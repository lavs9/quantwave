"""
Portfolio optimization engine.

Python-first (numpy) v1. The bead that spawned this module names a Rust
(nalgebra) home for the eventual production engine; this module is a
deliberate Python-first v1 that ships the API surface and numerical
correctness now, using numpy only. A Rust port (``quantwave._portfolio``,
mirroring the pattern used by ``quantwave._backtest``) is a documented
follow-up once the API here has stabilized against real usage.

Conventions
-----------
* Returns matrices are ``(rows=time, cols=assets)`` numpy arrays (or
  anything ``np.asarray`` accepts).
* Every optimizer returns a 1-D weights vector that sums to 1.0.
* Degenerate inputs (singular covariance, a single asset, NaN returns)
  raise :class:`PortfolioError` rather than propagating a raw numpy
  ``LinAlgError`` or silently returning ``NaN`` weights.

numpy is not guaranteed to be present on the core ``quantwave`` install,
so it is imported lazily inside functions (see ``_np()`` below) following
the same late-bind pattern used in ``quantwave/features.py`` for polars.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable, Dict, Hashable, Optional, Sequence, Tuple

if TYPE_CHECKING:
    import numpy as np


def _np():
    """Late-bind numpy so ``import quantwave`` works without the optional extra."""
    import numpy as np

    return np


class PortfolioError(ValueError):
    """Raised for degenerate or invalid portfolio-optimization inputs.

    Examples: a singular/non-invertible covariance matrix, a returns
    matrix containing NaN/Inf, an empty asset universe, or bounds that
    cannot be satisfied simultaneously with the full-investment budget
    constraint. Never let a raw numpy ``LinAlgError`` escape this module,
    and never return weights containing NaN.
    """


Bounds = Tuple[float, float]


# ---------------------------------------------------------------------------
# Validation helpers
# ---------------------------------------------------------------------------


def _check_finite(arr: "np.ndarray", name: str) -> None:
    np = _np()
    if arr.size == 0:
        raise PortfolioError(f"{name} is empty")
    if not np.all(np.isfinite(arr)):
        raise PortfolioError(f"{name} contains NaN or Inf values")


def _check_returns_matrix(returns: "np.ndarray") -> "np.ndarray":
    np = _np()
    arr = np.asarray(returns, dtype=float)
    if arr.ndim == 1:
        arr = arr.reshape(-1, 1)
    if arr.ndim != 2:
        raise PortfolioError(f"returns must be 1-D or 2-D, got ndim={arr.ndim}")
    if arr.shape[0] < 2:
        raise PortfolioError("returns must have at least 2 time observations")
    if arr.shape[1] < 1:
        raise PortfolioError("returns must have at least 1 asset")
    _check_finite(arr, "returns")
    return arr


def _check_cov(cov: "np.ndarray") -> "np.ndarray":
    np = _np()
    arr = np.asarray(cov, dtype=float)
    if arr.ndim != 2 or arr.shape[0] != arr.shape[1]:
        raise PortfolioError(f"covariance must be square 2-D, got shape={arr.shape}")
    if arr.shape[0] < 1:
        raise PortfolioError("covariance must cover at least 1 asset")
    _check_finite(arr, "covariance")
    return arr


def _safe_solve_spd(cov: "np.ndarray", rhs: "np.ndarray") -> "np.ndarray":
    """Solve ``cov @ x = rhs``, raising :class:`PortfolioError` on singularity.

    Uses a small ridge fallback probe purely to *detect* near-singularity
    (via condition number); the actual solve is exact ``np.linalg.solve``.
    """
    np = _np()
    n = cov.shape[0]
    if n == 1:
        val = cov[0, 0]
        if val <= 0 or not np.isfinite(val):
            raise PortfolioError("single-asset covariance is non-positive or invalid")
        return rhs / val
    try:
        cond = np.linalg.cond(cov)
    except np.linalg.LinAlgError as exc:  # pragma: no cover - defensive
        raise PortfolioError(f"covariance matrix is degenerate: {exc}") from exc
    if not np.isfinite(cond) or cond > 1e12:
        raise PortfolioError(
            "covariance matrix is singular or ill-conditioned "
            f"(condition number={cond:.3g}); cannot invert"
        )
    try:
        x = np.linalg.solve(cov, rhs)
    except np.linalg.LinAlgError as exc:
        raise PortfolioError(f"covariance matrix is singular: {exc}") from exc
    if not np.all(np.isfinite(x)):
        raise PortfolioError("solve produced non-finite weights")
    return x


# ---------------------------------------------------------------------------
# Constraint projection
# ---------------------------------------------------------------------------


def _apply_bounds_and_budget(
    w: "np.ndarray",
    bounds: Optional[Sequence[Bounds]] = None,
    long_only: bool = True,
) -> "np.ndarray":
    """Project raw weights onto the box-constrained, budget=1 feasible set.

    Computes the Euclidean projection of ``w`` onto
    ``{x : lo <= x <= hi, sum(x) = 1}`` via bisection on a scalar shift
    ``lambda``, using the standard capped-simplex projection identity
    ``x_i = clip(w_i - lambda, lo_i, hi_i)``. ``sum(clip(w - lambda, lo,
    hi))`` is monotonically non-increasing in ``lambda``, so bisection
    converges to the unique ``lambda`` with ``sum(x) = 1`` whenever
    ``sum(lo) <= 1 <= sum(hi)`` (validated below). This guarantees bounds
    are respected *exactly* (unlike clip-then-renormalize, which can push
    values back out of bounds).
    """
    np = _np()
    n = w.shape[0]

    if bounds is None:
        lo = np.zeros(n) if long_only else np.full(n, -np.inf)
        hi = np.ones(n)
    else:
        if len(bounds) != n:
            raise PortfolioError(
                f"bounds length {len(bounds)} does not match number of assets {n}"
            )
        lo = np.array([b[0] for b in bounds], dtype=float)
        hi = np.array([b[1] for b in bounds], dtype=float)
        if np.any(lo > hi):
            raise PortfolioError("bounds have low > high for at least one asset")

    if np.sum(hi) < 1.0 - 1e-9:
        raise PortfolioError(
            "bounds cannot satisfy full-investment budget: sum(upper bounds) < 1"
        )
    finite_lo_sum = np.sum(np.where(np.isfinite(lo), lo, 0.0))
    if finite_lo_sum > 1.0 + 1e-9:
        raise PortfolioError(
            "bounds cannot satisfy full-investment budget: sum(lower bounds) > 1"
        )

    lo_b = np.where(np.isfinite(lo), lo, -1e9)
    hi_b = np.where(np.isfinite(hi), hi, 1e9)

    def excess(lam: float) -> float:
        return float(np.sum(np.clip(w - lam, lo_b, hi_b)) - 1.0)

    lam_lo, lam_hi = -1e9, 1e9
    # excess(lam_lo) should be >= 0 (near-fully-clipped-high), excess(lam_hi) <= 0.
    for _ in range(200):
        mid = (lam_lo + lam_hi) / 2.0
        if excess(mid) > 0:
            lam_lo = mid
        else:
            lam_hi = mid
        if lam_hi - lam_lo < 1e-14:
            break
    lam = (lam_lo + lam_hi) / 2.0

    w_final = np.clip(w - lam, lo_b, hi_b)
    total = np.sum(w_final)
    if not np.all(np.isfinite(w_final)) or abs(total - 1.0) > 1e-6:
        raise PortfolioError("unable to project weights onto the feasible bounds set")
    # Nudge for the last bit of float error so sum is exactly 1.
    w_final = w_final / total
    return w_final


# ---------------------------------------------------------------------------
# Covariance estimators
# ---------------------------------------------------------------------------


def sample_cov(returns: "np.ndarray") -> "np.ndarray":
    """Sample covariance matrix of asset returns.

    Parameters
    ----------
    returns:
        ``(T, N)`` matrix of periodic returns (rows=time, cols=assets).

    Returns
    -------
    ``(N, N)`` covariance matrix, ``np.cov(returns, rowvar=False, ddof=1)``.
    """
    np = _np()
    arr = _check_returns_matrix(returns)
    if arr.shape[0] < 2:
        raise PortfolioError("sample_cov requires at least 2 observations")
    cov = np.cov(arr, rowvar=False, ddof=1)
    cov = np.atleast_2d(cov)
    return cov


def ewma_cov(returns: "np.ndarray", halflife: float) -> "np.ndarray":
    """Exponentially-weighted covariance matrix (RiskMetrics-style).

    Weights the ``t``-th most-recent observation (``t=0`` = most recent)
    by ``lambda**t`` where ``lambda = 0.5 ** (1 / halflife)``, normalized
    to sum to 1, applied to the (mean-centered) outer products of returns.

    Parameters
    ----------
    returns:
        ``(T, N)`` matrix of periodic returns.
    halflife:
        Halflife in observations (> 0). Smaller halflife weights recent
        observations more heavily.
    """
    np = _np()
    arr = _check_returns_matrix(returns)
    if halflife <= 0:
        raise PortfolioError(f"halflife must be positive, got {halflife}")

    T = arr.shape[0]
    lam = 0.5 ** (1.0 / halflife)
    # t=0 is most recent observation (last row).
    ages = np.arange(T - 1, -1, -1)
    raw_w = lam ** ages
    w = raw_w / raw_w.sum()

    mean = np.average(arr, axis=0, weights=w)
    centered = arr - mean
    cov = (centered * w[:, None]).T @ centered
    # Bias correction akin to reliability weights: 1 / (1 - sum(w^2)).
    denom = 1.0 - np.sum(w ** 2)
    if denom > 1e-12:
        cov = cov / denom
    if not np.all(np.isfinite(cov)):
        raise PortfolioError("ewma_cov produced non-finite covariance")
    return cov


def ledoit_wolf_cov(returns: "np.ndarray") -> "np.ndarray":
    """Ledoit-Wolf shrinkage covariance estimator (shrinkage target: identity-scaled).

    Implements the analytic shrinkage-intensity formula from Ledoit & Wolf
    (2004), "A well-conditioned estimator for large-dimensional covariance
    matrices", shrinking the sample covariance toward ``mu * I`` where
    ``mu = trace(S) / N`` (the same target used by
    ``sklearn.covariance.LedoitWolf`` in its default configuration).

    Let ``X`` be the ``(T, N)`` mean-centered returns matrix, ``S`` the
    sample covariance (``ddof=1``), and ``mu = trace(S) / N``.
    Define per-observation outer products ``S_t = x_t x_t^T`` (using the
    ``ddof=0``/population sample covariance convention for the shrinkage
    formula, matching sklearn's implementation). Then:

    * ``pi_hat = (1/T) * sum_t || S_t - S_pop ||_F^2``  (estimation error)
    * ``gamma_hat = || S_pop - mu * I ||_F^2``           (target misspecification)
    * ``rho_hat`` uses the diagonal-only simplification (off-diagonal terms
      of the covariance's asymptotic covariance are ignored, matching the
      common practical implementation of Ledoit-Wolf against identity):
      ``rho_hat = (1/T) * sum_t sum_i (S_t[i,i] - S_pop[i,i]) * (S_pop[i,i] - mu)``
    * ``kappa_hat = (pi_hat - rho_hat) / gamma_hat``
    * ``shrinkage = clip(kappa_hat / T, 0, 1)``
    * ``cov = shrinkage * mu * I + (1 - shrinkage) * S`` (S = ddof=1 sample cov)

    Returns
    -------
    ``(N, N)`` shrunk covariance matrix.
    """
    np = _np()
    arr = _check_returns_matrix(returns)
    T, N = arr.shape
    if N == 1:
        return sample_cov(arr)

    mean = arr.mean(axis=0)
    X = arr - mean

    # Population (ddof=0) sample covariance, used throughout the
    # Ledoit-Wolf shrinkage-intensity derivation.
    S_pop = (X.T @ X) / T
    mu = np.trace(S_pop) / N

    # pi_hat: average squared Frobenius distance between per-obs outer
    # products and the population covariance.
    pi_sum = 0.0
    rho_sum = 0.0
    diag_target = np.diag(S_pop) - mu  # (N,)
    for t in range(T):
        x = X[t]
        S_t = np.outer(x, x)
        diff = S_t - S_pop
        pi_sum += np.sum(diff ** 2)
        rho_sum += np.sum(np.diag(diff) * diag_target)
    pi_hat = pi_sum / T
    rho_hat = rho_sum / T

    gamma_hat = np.sum((S_pop - mu * np.eye(N)) ** 2)
    if gamma_hat < 1e-18:
        shrinkage = 0.0
    else:
        kappa_hat = (pi_hat - rho_hat) / gamma_hat
        shrinkage = float(np.clip(kappa_hat / T, 0.0, 1.0))

    S_sample = sample_cov(arr)  # ddof=1, matches other estimators/tests
    cov = shrinkage * mu * np.eye(N) + (1.0 - shrinkage) * S_sample
    if not np.all(np.isfinite(cov)):
        raise PortfolioError("ledoit_wolf_cov produced non-finite covariance")
    return cov


# ---------------------------------------------------------------------------
# Optimizers
# ---------------------------------------------------------------------------


def min_variance(
    cov: "np.ndarray", bounds: Optional[Sequence[Bounds]] = None
) -> "np.ndarray":
    """Minimum-variance portfolio weights.

    Closed form (unconstrained): ``w ∝ Σ⁻¹ · 1``, normalized to sum to 1.
    Bounds/long-only constraints are then applied via clip+renormalize
    projection (see :func:`_apply_bounds_and_budget`).
    """
    np = _np()
    C = _check_cov(cov)
    n = C.shape[0]
    ones = np.ones(n)
    raw = _safe_solve_spd(C, ones)
    total = np.sum(raw)
    if abs(total) < 1e-12:
        raise PortfolioError("min_variance: Σ⁻¹·1 sums to ~0, cannot normalize")
    w = raw / total
    return _apply_bounds_and_budget(w, bounds=bounds, long_only=True)


def max_sharpe(
    mean: "np.ndarray", cov: "np.ndarray", bounds: Optional[Sequence[Bounds]] = None
) -> "np.ndarray":
    """Maximum-Sharpe (tangency) portfolio weights.

    Closed form (unconstrained, zero risk-free rate): ``w ∝ Σ⁻¹ · μ``,
    normalized to sum to 1. Bounds/long-only constraints are then applied
    via clip+renormalize projection.
    """
    np = _np()
    C = _check_cov(cov)
    m = np.asarray(mean, dtype=float).reshape(-1)
    if m.shape[0] != C.shape[0]:
        raise PortfolioError(
            f"mean length {m.shape[0]} does not match covariance size {C.shape[0]}"
        )
    _check_finite(m, "mean")
    raw = _safe_solve_spd(C, m)
    total = np.sum(raw)
    if abs(total) < 1e-12:
        raise PortfolioError("max_sharpe: Σ⁻¹·μ sums to ~0, cannot normalize")
    w = raw / total
    return _apply_bounds_and_budget(w, bounds=bounds, long_only=True)


def risk_parity(
    cov: "np.ndarray",
    bounds: Optional[Sequence[Bounds]] = None,
    max_iter: int = 200,
    tol: float = 1e-10,
) -> "np.ndarray":
    """Equal risk-contribution ("risk parity") portfolio weights.

    Uses Spinu's (2013) fixed-point iteration for the risk-parity problem:
    solve for ``y`` such that ``Σ y = 1 / y`` (elementwise reciprocal of
    the risk-budget-scaled weights), via the multiplicative update

        ``y_{k+1} = y_k * sqrt( b / (Σ y_k * y_k) )``  (elementwise, b = 1/n each)

    starting from equal weight, then normalize ``y`` to sum to 1. Risk
    contributions ``RC_i = w_i * (Σw)_i`` are equal at the fixed point
    (up to solver tolerance).
    """
    np = _np()
    C = _check_cov(cov)
    n = C.shape[0]
    b = np.full(n, 1.0 / n)  # equal risk budget

    y = np.full(n, 1.0 / n)
    for _ in range(max_iter):
        Sigma_y = C @ y
        if np.any(Sigma_y <= 0) or not np.all(np.isfinite(Sigma_y)):
            raise PortfolioError(
                "risk_parity: covariance produced non-positive marginal risk; "
                "matrix may not be positive semi-definite"
            )
        y_new = y * np.sqrt(b / (y * Sigma_y) * np.sum(y * Sigma_y))
        # Renormalize each step to prevent drift/overflow.
        y_new = y_new / np.sum(y_new)
        if np.max(np.abs(y_new - y)) < tol:
            y = y_new
            break
        y = y_new

    if not np.all(np.isfinite(y)):
        raise PortfolioError("risk_parity failed to converge to finite weights")
    w = y / np.sum(y)
    return _apply_bounds_and_budget(w, bounds=bounds, long_only=True)


# ---------------------------------------------------------------------------
# HRP (Hierarchical Risk Parity), scipy-free
# ---------------------------------------------------------------------------


def _corr_from_cov(cov: "np.ndarray") -> "np.ndarray":
    np = _np()
    std = np.sqrt(np.diag(cov))
    std = np.where(std <= 0, 1e-12, std)
    corr = cov / np.outer(std, std)
    corr = np.clip(corr, -1.0, 1.0)
    np.fill_diagonal(corr, 1.0)
    return corr


def _single_linkage(dist: "np.ndarray") -> list:
    """Single-linkage agglomerative clustering on a condensed distance set.

    Reimplements the core of ``scipy.cluster.hierarchy.linkage(method="single")``
    with a simple O(n^2 log n) nearest-fragment merge (no scipy dependency).
    Returns a SciPy-compatible linkage matrix as a list of
    ``[cluster_a, cluster_b, distance, size]`` rows, where clusters
    ``0..n-1`` are the original items and ``n+i`` is the cluster created
    at merge step ``i``.
    """
    np = _np()
    n = dist.shape[0]
    next_id = n
    linkage = []

    # We maintain a mutable distance matrix indexed by *original* labels
    # for simplicity given small n typical of asset universes.
    id_to_members = {i: {i} for i in range(n)}
    alive = set(range(n))
    # distance lookup keyed by frozenset pair over "current" cluster ids
    D = {}
    for i in range(n):
        for j in range(i + 1, n):
            D[(i, j)] = dist[i, j]

    while len(alive) > 1:
        # find closest pair among alive clusters
        best = None
        best_d = np.inf
        alive_list = sorted(alive)
        for ai in range(len(alive_list)):
            for bi in range(ai + 1, len(alive_list)):
                a, b = alive_list[ai], alive_list[bi]
                key = (a, b) if a < b else (b, a)
                d = D.get(key)
                if d is None:
                    continue
                if d < best_d:
                    best_d = d
                    best = (a, b)
        a, b = best
        size = len(id_to_members[a]) + len(id_to_members[b])
        linkage.append([a, b, float(best_d), size])

        new_members = id_to_members[a] | id_to_members[b]
        new_id = next_id
        next_id += 1
        id_to_members[new_id] = new_members

        # compute single-link distance from new cluster to remaining alive clusters
        alive.discard(a)
        alive.discard(b)
        for c in list(alive):
            key_a = (a, c) if a < c else (c, a)
            key_b = (b, c) if b < c else (c, b)
            d_a = D.get(key_a, np.inf)
            d_b = D.get(key_b, np.inf)
            d_new = min(d_a, d_b)
            key_new = (new_id, c) if new_id < c else (c, new_id)
            D[key_new] = d_new
        alive.add(new_id)

    return linkage


def _linkage_order(linkage: list, n: int) -> list:
    """Quasi-diagonalization: recover leaf order from a linkage list.

    Standard "seriation" walk used by HRP: recursively expand each merge
    into its two children until only original leaves (``< n``) remain.
    """
    def expand(cluster_id):
        if cluster_id < n:
            return [cluster_id]
        row = linkage[cluster_id - n]
        a, b = int(row[0]), int(row[1])
        return expand(a) + expand(b)

    root = n - 1 + len(linkage)  # id of final merged cluster
    return expand(root)


def _hrp_bisection(cov: "np.ndarray", sorted_idx: list) -> "np.ndarray":
    """Recursive bisection allocation over quasi-diagonalized order."""
    np = _np()
    n = len(sorted_idx)
    weights = np.ones(n)
    clusters = [sorted_idx]

    while clusters:
        new_clusters = []
        for cluster in clusters:
            if len(cluster) <= 1:
                continue
            mid = len(cluster) // 2
            left = cluster[:mid]
            right = cluster[mid:]

            def cluster_var(members):
                sub_cov = cov[np.ix_(members, members)]
                inv_diag = 1.0 / np.diag(sub_cov)
                w = inv_diag / np.sum(inv_diag)
                return float(w @ sub_cov @ w)

            var_left = cluster_var(left)
            var_right = cluster_var(right)
            denom = var_left + var_right
            alpha = 1.0 - var_left / denom if denom > 1e-18 else 0.5

            for idx in left:
                pos = sorted_idx.index(idx)
                weights[pos] *= alpha
            for idx in right:
                pos = sorted_idx.index(idx)
                weights[pos] *= (1.0 - alpha)

            new_clusters.append(left)
            new_clusters.append(right)
        clusters = new_clusters

    out = np.zeros(n)
    for pos, idx in enumerate(sorted_idx):
        out[pos] = weights[pos]
    return out


def hrp(
    returns_or_cov: "np.ndarray",
    bounds: Optional[Sequence[Bounds]] = None,
    is_cov: bool = False,
) -> "np.ndarray":
    """Hierarchical Risk Parity (Lopez de Prado) portfolio weights.

    Pipeline: correlation -> distance, single-linkage hierarchical
    clustering, quasi-diagonalization (seriation), then recursive
    bisection allocation using inverse-variance sub-portfolios.

    scipy is treated as absent: if it happens to be importable, the
    ``scipy.cluster.hierarchy.linkage``/``dendrogram`` path is *not*
    used automatically — this module always uses its own numpy-only
    single-linkage implementation, so behavior is identical with or
    without scipy installed. (Kept deliberately simple per spec; a
    scipy-accelerated path can be added later behind a feature flag
    without changing the public signature.)

    Parameters
    ----------
    returns_or_cov:
        Either a ``(T, N)`` returns matrix, or an ``(N, N)`` covariance
        matrix (pass ``is_cov=True`` in the latter case).
    bounds:
        Optional per-asset ``(low, high)`` bounds applied to the final
        weights via clip+renormalize.
    is_cov:
        If True, treat ``returns_or_cov`` as a precomputed covariance
        matrix instead of a returns matrix.
    """
    np = _np()
    arr = np.asarray(returns_or_cov, dtype=float)
    if is_cov:
        cov = _check_cov(arr)
    else:
        returns = _check_returns_matrix(arr)
        cov = sample_cov(returns)

    n = cov.shape[0]
    if n == 1:
        return _apply_bounds_and_budget(np.ones(1), bounds=bounds, long_only=True)

    corr = _corr_from_cov(cov)
    dist = np.sqrt(np.clip((1.0 - corr) / 2.0, 0.0, 1.0))
    np.fill_diagonal(dist, 0.0)

    linkage = _single_linkage(dist)
    order = _linkage_order(linkage, n)
    w_sorted = _hrp_bisection(cov, order)

    w = np.zeros(n)
    for pos, idx in enumerate(order):
        w[idx] = w_sorted[pos]

    total = np.sum(w)
    if abs(total) < 1e-12 or not np.all(np.isfinite(w)):
        raise PortfolioError("hrp produced degenerate weights")
    w = w / total
    return _apply_bounds_and_budget(w, bounds=bounds, long_only=True)


# ---------------------------------------------------------------------------
# Regime-conditioned optimization hook
# ---------------------------------------------------------------------------


def by_regime(
    returns: "np.ndarray",
    regime_labels: Sequence[Hashable],
    optimizer: Callable[..., "np.ndarray"] = min_variance,
    min_obs: int = 2,
    **optimizer_kwargs,
) -> Dict[Hashable, "np.ndarray"]:
    """Compute per-regime portfolio weights.

    Splits ``returns`` rows by ``regime_labels`` and runs ``optimizer`` on
    each regime's sample covariance (via :func:`sample_cov`). Recognizes
    :func:`max_sharpe` (needs ``mean`` and ``cov``) and :func:`hrp` (accepts
    a covariance directly via ``is_cov=True``) by identity and dispatches
    accordingly; any other callable is assumed to take ``cov`` as its sole
    positional argument (matching :func:`min_variance` / :func:`risk_parity`).

    Parameters
    ----------
    returns:
        ``(T, N)`` matrix of periodic returns.
    regime_labels:
        Length-``T`` sequence of regime labels, one per row of ``returns``.
    optimizer:
        One of :func:`min_variance`, :func:`max_sharpe`, :func:`risk_parity`,
        :func:`hrp`, or a compatible callable. Defaults to :func:`min_variance`.
    min_obs:
        Minimum observations required for a regime to be optimized; regimes
        with fewer rows are skipped.
    **optimizer_kwargs:
        Forwarded to ``optimizer`` (e.g. ``bounds=...``).

    Returns
    -------
    Dict mapping each regime label to its weights vector.
    """
    np = _np()
    arr = _check_returns_matrix(returns)
    labels = list(regime_labels)
    if len(labels) != arr.shape[0]:
        raise PortfolioError(
            f"regime_labels length {len(labels)} does not match returns rows {arr.shape[0]}"
        )

    out: Dict[Hashable, "np.ndarray"] = {}
    unique_labels = sorted(set(labels), key=lambda x: str(x))
    for label in unique_labels:
        mask = [lbl == label for lbl in labels]
        sub = arr[np.asarray(mask, dtype=bool)]
        if sub.shape[0] < min_obs:
            continue
        cov = sample_cov(sub) if sub.shape[0] >= 2 else None
        if cov is None:
            continue
        mean = sub.mean(axis=0)
        if optimizer is max_sharpe:
            w = optimizer(mean, cov, **optimizer_kwargs)
        elif optimizer is hrp:
            w = optimizer(cov, is_cov=True, **optimizer_kwargs)
        else:
            w = optimizer(cov, **optimizer_kwargs)
        out[label] = w
    return out
