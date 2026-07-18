"""Tests for leakage-safe cross-validation split helpers.

The single non-negotiable guarantee under test: after purge + embargo,
there is zero overlap between any train fold and its corresponding test
fold, and no train sample's label span (or the embargo band) touches the
test interval.
"""

from __future__ import annotations

import numpy as np
import pytest

from quantwave.feature_splits import purged_kfold, walk_forward


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _random_t1(rng: np.random.Generator, n_samples: int, max_horizon: int) -> np.ndarray:
    """Random non-decreasing label horizons: t1[i] in [i, min(n_samples-1, i+H)]."""
    horizons = rng.integers(0, max_horizon + 1, size=n_samples)
    t1 = np.minimum(np.arange(n_samples) + horizons, n_samples - 1)
    return t1


# ---------------------------------------------------------------------------
# 1. ZERO-OVERLAP PROPERTY (the core guarantee) -- write first.
# ---------------------------------------------------------------------------


@pytest.mark.parametrize("seed", range(20))
def test_purged_kfold_zero_overlap_randomized(seed: int) -> None:
    rng = np.random.default_rng(seed)
    n_samples = int(rng.integers(20, 300))
    n_splits = int(rng.integers(2, 8))
    embargo = float(rng.choice([0.0, 0.01, 0.02, 0.05, 0.1]))
    use_t1 = bool(rng.integers(0, 2))
    t1 = None
    if use_t1:
        max_horizon = int(rng.integers(0, n_samples // 2 + 1))
        t1 = _random_t1(rng, n_samples, max_horizon)

    folds = purged_kfold(n_samples, n_splits=n_splits, t1=t1, embargo=embargo)
    embargo_count = int(embargo * n_samples)

    for train, test in folds:
        train_set = set(train.tolist())
        test_set = set(test.tolist())

        # No index shared between train and test.
        assert train_set & test_set == set()

        if len(test) == 0:
            continue
        test_lo, test_hi = int(test[0]), int(test[-1])

        span = t1 if t1 is not None else np.arange(n_samples)

        for i in train:
            i = int(i)
            # Purge: train sample's label span must not overlap test interval.
            lo, hi = i, int(span[i])
            overlaps = (i <= test_hi) and (hi >= test_lo)
            assert not overlaps, (
                f"train idx {i} label span [{lo},{hi}] overlaps test "
                f"interval [{test_lo},{test_hi}]"
            )
            # Embargo: no train sample immediately after the test fold
            # within the embargo band.
            in_embargo_band = test_hi < i <= test_hi + embargo_count
            assert not in_embargo_band, (
                f"train idx {i} falls inside embargo band after test "
                f"fold ending at {test_hi} (embargo_count={embargo_count})"
            )


@pytest.mark.parametrize("seed", range(10))
def test_walk_forward_zero_overlap_randomized(seed: int) -> None:
    rng = np.random.default_rng(seed)
    n_samples = int(rng.integers(20, 300))
    n_splits = int(rng.integers(1, 6))
    embargo = float(rng.choice([0.0, 0.01, 0.02, 0.05]))
    expanding = bool(rng.integers(0, 2))

    folds = walk_forward(
        n_samples, n_splits=n_splits, expanding=expanding, embargo=embargo
    )

    for train, test in folds:
        train_set = set(train.tolist())
        test_set = set(test.tolist())
        assert train_set & test_set == set()
        if len(train) == 0 or len(test) == 0:
            continue
        assert int(train.max()) < int(test.min())


# ---------------------------------------------------------------------------
# 2. walk_forward strict time ordering.
# ---------------------------------------------------------------------------


def test_walk_forward_train_strictly_before_test() -> None:
    folds = walk_forward(200, n_splits=5, expanding=True, embargo=0.0)
    assert len(folds) > 0
    for train, test in folds:
        assert len(train) > 0 and len(test) > 0
        assert int(train.max()) < int(test.min())


def test_walk_forward_expanding_non_decreasing_train_size() -> None:
    folds = walk_forward(300, n_splits=6, expanding=True, embargo=0.0)
    sizes = [len(train) for train, _ in folds]
    assert all(a <= b for a, b in zip(sizes, sizes[1:]))


def test_walk_forward_embargo_gap_respected() -> None:
    n_samples = 300
    embargo = 0.02
    embargo_count = int(embargo * n_samples)
    folds = walk_forward(n_samples, n_splits=5, expanding=True, embargo=embargo)
    for train, test in folds:
        if len(train) == 0 or len(test) == 0:
            continue
        gap = int(test.min()) - int(train.max()) - 1
        assert gap == embargo_count


def test_walk_forward_rolling_window_train_size_bounded() -> None:
    folds = walk_forward(300, n_splits=6, expanding=False, embargo=0.0)
    sizes = [len(train) for train, _ in folds if len(train) > 0]
    # Rolling window should never exceed the initial (max) train size.
    assert max(sizes) == sizes[0] or all(s <= sizes[0] for s in sizes)


# ---------------------------------------------------------------------------
# 3. Coverage / partition sanity for purged_kfold test folds.
# ---------------------------------------------------------------------------


def test_purged_kfold_test_folds_partition_timeline() -> None:
    n_samples = 137
    n_splits = 5
    folds = purged_kfold(n_samples, n_splits=n_splits, t1=None, embargo=0.0)

    test_indices_concat: list[int] = []
    for _, test in folds:
        test_indices_concat.extend(test.tolist())

    # Disjoint.
    assert len(test_indices_concat) == len(set(test_indices_concat))
    # Union covers all samples.
    assert set(test_indices_concat) == set(range(n_samples))


# ---------------------------------------------------------------------------
# 4. Determinism + edge cases.
# ---------------------------------------------------------------------------


def test_purged_kfold_deterministic() -> None:
    folds_a = purged_kfold(100, n_splits=4, t1=None, embargo=0.03)
    folds_b = purged_kfold(100, n_splits=4, t1=None, embargo=0.03)
    assert len(folds_a) == len(folds_b)
    for (train_a, test_a), (train_b, test_b) in zip(folds_a, folds_b):
        assert np.array_equal(train_a, train_b)
        assert np.array_equal(test_a, test_b)


def test_purged_kfold_embargo_zero_matches_plain_purged(monkeypatch=None) -> None:
    n_samples = 60
    t1 = np.arange(n_samples)  # trivial point labels
    folds_embargo0 = purged_kfold(n_samples, n_splits=5, t1=t1, embargo=0.0)
    folds_no_embargo_arg = purged_kfold(n_samples, n_splits=5, t1=t1)
    assert len(folds_embargo0) == len(folds_no_embargo_arg)
    for (tr0, te0), (tr1, te1) in zip(folds_embargo0, folds_no_embargo_arg):
        assert np.array_equal(tr0, tr1)
        assert np.array_equal(te0, te1)


def test_purged_kfold_tiny_n_samples() -> None:
    folds = purged_kfold(5, n_splits=2, t1=None, embargo=0.0)
    assert len(folds) == 2
    all_test = sorted(idx for _, test in folds for idx in test.tolist())
    assert all_test == list(range(5))


def test_purged_kfold_n_splits_one() -> None:
    folds = purged_kfold(20, n_splits=1, t1=None, embargo=0.0)
    assert len(folds) == 1
    train, test = folds[0]
    assert len(test) == 20
    assert len(train) == 0


def test_walk_forward_n_splits_one() -> None:
    folds = walk_forward(50, n_splits=1, expanding=True, embargo=0.0)
    assert len(folds) == 1
    train, test = folds[0]
    assert len(train) > 0
    assert len(test) > 0
    assert int(train.max()) < int(test.min())


def test_walk_forward_min_train_respected() -> None:
    folds = walk_forward(100, n_splits=3, expanding=True, min_train=40, embargo=0.0)
    first_train, _ = folds[0]
    assert len(first_train) >= 40


def test_purged_kfold_invalid_n_splits_raises() -> None:
    with pytest.raises(ValueError):
        purged_kfold(10, n_splits=0)


def test_walk_forward_invalid_n_splits_raises() -> None:
    with pytest.raises(ValueError):
        walk_forward(10, n_splits=0)
