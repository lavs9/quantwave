"""Conditional-outcome query layer: "given these conditions on my data, how
often did this outcome occur" — answered honestly.

This module is a first, deliberately narrow slice of an edge-stats-style
tool (inspired by LuxAlgo's Edge Stats, but not a port of it — no bespoke
query-string DSL, no session/calendar machinery, no continuous-outcome
distribution stats). It composes over plain Polars boolean expressions the
caller already has lying around (typically built with ``.ta`` indicators or
``.bt`` outputs), the same house style as :mod:`quantwave.robustness` and
:mod:`quantwave.tearsheet`: pure Python + Polars, no numpy/scipy dependency,
and a "can we honestly compute this" discipline — when the sample is too
small to say anything meaningful, the result reports ``"computed": False``
and leaves the numeric fields as ``None`` rather than returning a bare
percentage that looks identical whether it was computed from 3 rows or
3000.

What "condition" and "outcome" mean here
-----------------------------------------
Both are Polars **boolean expressions**, evaluated per-row against the
caller's DataFrame:

* ``condition`` selects the *eligible* rows — e.g. ``pl.col("rsi") < 30``
  ("RSI was oversold on this bar").
* ``outcome`` is the event whose rate you want to know, evaluated only on
  the eligible rows — e.g. ``pl.col("fwd_return_5") > 0`` ("price was higher
  five bars later").

"Outcome rate given condition" = among rows where ``condition`` is true,
what fraction also have ``outcome`` true. This is exactly ``P(outcome |
condition)`` estimated empirically from the data, i.e. a per-row hit rate,
not a time-series/backtest return.

Statistical honesty
--------------------
A bare hit rate (``x / n``) says nothing about how much to trust it. This
module always reports it alongside:

* ``n`` — the eligible sample size (rows where ``condition`` is true).
* A 95% **Wilson score interval** (closed-form, not bootstrapped) around the
  point estimate. The Wilson interval is preferred over the naive normal
  ("Wald") interval because the Wald interval behaves badly near 0 or 1
  (it can produce bounds outside ``[0, 1]`` or a zero-width interval at
  ``p̂ == 0``/``1``); Wilson does not have this problem and is the standard
  textbook recommendation for a binomial proportion CI. Reference: Wilson,
  E.B. (1927), "Probable Inference, the Law of Succession, and Statistical
  Inference", *JASA* 22(158): 209-212. Formula (with continuity-uncorrected
  z = 1.96 for 95%)::

      center = (p̂ + z²/(2n)) / (1 + z²/n)
      half_width = z * sqrt(p̂(1-p̂)/n + z²/(4n²)) / (1 + z²/n)
      CI = [center - half_width, center + half_width]

* A **minimum-sample-size guard** (``min_samples``, default 30): below this,
  the result reports ``"computed": False`` rather than a misleading number.
* A **stability check**: the eligible rows are split in half chronologically
  (by row order — callers should pass an already time-sorted DataFrame,
  which is the norm for OHLCV/backtest data in this library) and the same
  point estimate + CI + guard is computed independently on each half. This
  answers "does the edge exist throughout history, or only in one half of
  it" — a classic overfitting tell that a single aggregate number hides.

Worked example
---------------
::

    import polars as pl
    from quantwave.research import conditional_outcome

    df = pl.DataFrame({
        "rsi": [10, 15, 25, 40, 60, 80, 5, 12, 90, 8] * 5,
        "fwd_return_5": [0.02, 0.01, -0.01, 0.00, 0.01, -0.02, 0.03, 0.015, -0.03, 0.025] * 5,
    })
    result = conditional_outcome(
        df,
        outcome=pl.col("fwd_return_5") > 0,
        condition=pl.col("rsi") < 20,
        min_samples=10,
    )
    # result["n"] == 30           (rows where rsi < 20)
    # result["point_estimate"] ~= 1.0   (fwd_return_5 > 0 on every such row here)
    # result["wilson_ci_low"], result["wilson_ci_high"] bracket that estimate
    # result["computed"] is True (n >= min_samples)
    # result["first_half"] / result["second_half"] give the same shape on
    # each chronological half of those 30 rows, so you can see whether the
    # ~100% hit rate holds up throughout or was concentrated early/late.

Out of scope for this first slice (tracked as follow-up work, not built
here): per-year breakdown, exchange-timezone session-boundary/holiday
calendar machinery, futures roll-day exclusion, and continuous-outcome
distribution stats (median/quartiles of a numeric outcome rather than a
boolean one). All of those are part of the broader edge-stats feature set
but are not needed for a first working version, and none of them require
changes to this module's shape to add later.
"""

from __future__ import annotations

import math
from typing import Optional

import polars as pl

_Z_95 = 1.96  # z-score for a 95% two-sided confidence interval


def wilson_interval(x: int, n: int, z: float = _Z_95) -> tuple[float, float]:
    """95%-by-default Wilson score confidence interval for a binomial
    proportion ``x / n``.

    Closed-form (Wilson 1927) — see module docstring for the formula and
    why it's preferred over the naive normal-approximation interval.
    Returns ``(low, high)``, both clamped to ``[0.0, 1.0]``. ``n == 0``
    returns ``(0.0, 1.0)`` (maximally uninformative — the caller's
    ``min_samples`` guard is expected to prevent this case from being
    reported as a real result upstream).
    """
    if n <= 0:
        return (0.0, 1.0)
    p_hat = x / n
    z2 = z * z
    denom = 1.0 + z2 / n
    center = (p_hat + z2 / (2 * n)) / denom
    half_width = (z * math.sqrt(p_hat * (1 - p_hat) / n + z2 / (4 * n * n))) / denom
    low = max(0.0, center - half_width)
    high = min(1.0, center + half_width)
    return (low, high)


def _point_result(x: int, n: int, min_samples: int) -> dict:
    """One "shape" of result (no stability sub-splits) for x successes out
    of n eligible rows. Shared by the top-level call and each half of the
    stability split."""
    if n <= 0 or n < min_samples:
        return {
            "n": n,
            "point_estimate": None,
            "wilson_ci_low": None,
            "wilson_ci_high": None,
            "min_samples_met": n >= min_samples,
            "computed": False,
        }
    low, high = wilson_interval(x, n)
    return {
        "n": n,
        "point_estimate": x / n,
        "wilson_ci_low": low,
        "wilson_ci_high": high,
        "min_samples_met": True,
        "computed": True,
    }


def conditional_outcome(
    df: pl.DataFrame,
    outcome: pl.Expr,
    condition: pl.Expr,
    min_samples: int = 30,
) -> dict:
    """"Given these conditions on my data, how often did this outcome
    occur" — a statistically honest answer.

    Parameters
    ----------
    df:
        A (typically time-sorted) Polars DataFrame — e.g. OHLCV bars with
        ``.ta`` indicator columns and/or forward-return/label columns
        already computed on it.
    outcome:
        A boolean Polars expression evaluated per-row, e.g.
        ``pl.col("fwd_return_5") > 0``. Evaluated only on rows where
        ``condition`` is true.
    condition:
        A boolean Polars expression selecting the eligible rows, e.g.
        ``pl.col("rsi") < 30``.
    min_samples:
        Minimum eligible row count (``n``) required before a point estimate
        and confidence interval are reported. Below this, ``computed`` is
        ``False`` and the numeric fields are ``None`` — see module
        docstring.

    Returns
    -------
    dict with keys:
        ``n`` (int): number of rows where ``condition`` is true.
        ``point_estimate`` (float | None): ``P(outcome | condition)``, i.e.
            fraction of eligible rows where ``outcome`` is also true. ``None``
            if ``computed`` is ``False``.
        ``wilson_ci_low`` / ``wilson_ci_high`` (float | None): 95% Wilson
            score interval around ``point_estimate``. ``None`` if
            ``computed`` is ``False``.
        ``min_samples_met`` (bool): whether ``n >= min_samples``.
        ``first_half`` / ``second_half`` (dict | None): same shape as this
            top-level dict (without nested halves), computed independently
            on the chronological first/second half of the eligible rows —
            the stability check. ``None`` only when there are zero eligible
            rows (nothing to split).
        ``computed`` (bool): ``True`` iff a point estimate/CI was actually
            computed for the full sample (``n >= min_samples``). Mirrors
            :mod:`quantwave.robustness`'s "never fabricate a number" pattern.

    See the module docstring for a full worked example and the definitions
    of "condition"/"outcome"/"eligible" used above.
    """
    eligible = df.filter(condition)
    n = eligible.height

    if n == 0:
        top = _point_result(0, 0, min_samples)
        top["first_half"] = None
        top["second_half"] = None
        return top

    x = int(eligible.select(outcome.sum()).item())
    top = _point_result(x, n, min_samples)

    mid = n // 2
    first = eligible.slice(0, mid)
    second = eligible.slice(mid, n - mid)

    def _half(half_df: pl.DataFrame) -> dict:
        hn = half_df.height
        hx = int(half_df.select(outcome.sum()).item()) if hn else 0
        return _point_result(hx, hn, min_samples)

    top["first_half"] = _half(first)
    top["second_half"] = _half(second)
    return top
