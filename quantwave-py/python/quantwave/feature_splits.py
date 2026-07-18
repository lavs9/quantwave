"""Leakage-safe cross-validation split helpers for time-ordered features.

Implements the purged K-fold (with embargo) and walk-forward splitters
described by Lopez de Prado (*Advances in Financial Machine Learning*).
Both splitters are pure-index (numpy) based: they take a sample count and
return ``(train_idx, test_idx)`` index arrays rather than touching any
DataFrame directly, so callers can apply them to any array-like of the same
length.

The guarantee both splitters exist to provide: after purge + embargo (or,
for walk-forward, after the embargo gap), no train index can leak
information about a test index -- either because it shares a timestamp with
the test fold, because its label window overlaps the test interval, or
because it falls inside the embargo band immediately following the test
fold.
"""

from __future__ import annotations

from typing import List, Optional, Tuple

import numpy as np

FoldList = List[Tuple[np.ndarray, np.ndarray]]


def purged_kfold(
    n_samples: int,
    n_splits: int = 5,
    t1: Optional[np.ndarray] = None,
    embargo: float = 0.0,
) -> FoldList:
    """K-fold splits over time-ordered samples with purging and embargo.

    Samples ``0..n_samples-1`` are assumed to be in time order. The
    timeline is partitioned into ``n_splits`` contiguous, disjoint test
    folds (a plain contiguous K-fold partition, before any purging). For
    each test fold ``[test_lo, test_hi]``:

    - **Purge**: a train candidate ``i`` is removed if its label span
      ``[i, t1[i]]`` overlaps the test interval, i.e. ``i <= test_hi and
      t1[i] >= test_lo``. If ``t1`` is ``None`` each sample is treated as a
      point label (``t1[i] = i``), so purging only removes the
      fold-adjacent boundary -- i.e. the test fold itself (already
      excluded); no additional overlap is possible without label-horizon
      information.
    - **Embargo**: train candidates in ``(test_hi, test_hi + embargo_count]``
      are removed, where ``embargo_count = int(embargo * n_samples)``. This
      is independent of ``t1`` and only ever trims samples immediately
      *after* the test fold.

    Args:
        n_samples: total number of time-ordered samples.
        n_splits: number of contiguous test folds (>= 1).
        t1: optional array of length ``n_samples`` mapping each sample
            index to the index at which its label is realized (its label
            horizon). ``t1[i] >= i`` is expected.
        embargo: fraction of ``n_samples`` to embargo immediately after
            each test fold (0.0 disables embargo).

    Returns:
        A list of ``(train_idx, test_idx)`` numpy index array pairs, one
        per non-empty test fold.
    """
    if n_samples <= 0:
        raise ValueError("n_samples must be positive")
    if n_splits < 1:
        raise ValueError("n_splits must be >= 1")
    if embargo < 0:
        raise ValueError("embargo must be non-negative")

    indices = np.arange(n_samples)

    if t1 is None:
        span_end = indices.copy()
    else:
        span_end = np.asarray(t1)
        if span_end.shape[0] != n_samples:
            raise ValueError("t1 must have length n_samples")

    embargo_count = int(embargo * n_samples)
    fold_bounds = np.array_split(indices, n_splits)

    folds: FoldList = []
    for test_idx in fold_bounds:
        if len(test_idx) == 0:
            continue
        test_lo = int(test_idx[0])
        test_hi = int(test_idx[-1])

        keep_mask = np.ones(n_samples, dtype=bool)
        keep_mask[test_idx] = False  # exclude the test fold itself

        candidates = indices[keep_mask]
        candidate_span_end = span_end[candidates]

        # Purge: label span [i, t1[i]] overlaps [test_lo, test_hi].
        purge_mask = (candidates <= test_hi) & (candidate_span_end >= test_lo)

        # Embargo: fixed band strictly after the test fold.
        embargo_mask = (candidates > test_hi) & (
            candidates <= test_hi + embargo_count
        )

        train = candidates[~(purge_mask | embargo_mask)]
        folds.append((train, test_idx))

    return folds


def walk_forward(
    n_samples: int,
    n_splits: int,
    expanding: bool = True,
    min_train: Optional[int] = None,
    embargo: float = 0.0,
) -> FoldList:
    """Time-ordered forward-chaining splits with an embargo gap.

    The timeline is split so that an initial training block of at least
    ``min_train`` samples is followed by ``n_splits`` contiguous test
    folds carved out of the remaining samples. For fold ``k`` the test
    fold always starts strictly after the corresponding train fold ends,
    separated by an embargo gap of ``embargo_count = int(embargo *
    n_samples)`` samples:

    - ``expanding=True``: train is every sample from ``0`` up to
      ``test_start - embargo_count`` (train grows each fold).
    - ``expanding=False``: train is a rolling window of (up to) the
      initial train size immediately preceding the embargo gap (train
      size is capped, not grown).

    Args:
        n_samples: total number of time-ordered samples.
        n_splits: number of forward test folds (>= 1).
        expanding: expanding-window train (default) vs. rolling-window
            train.
        min_train: minimum number of samples in the initial training
            block. Defaults to ``n_samples // (n_splits + 1)`` (at least
            1).
        embargo: fraction of ``n_samples`` to leave as a gap between the
            end of train and the start of test.

    Returns:
        A list of ``(train_idx, test_idx)`` numpy index array pairs.
        Folds where the embargo gap would consume the entire train
        window are skipped (empty-train folds are not returned).
    """
    if n_samples <= 0:
        raise ValueError("n_samples must be positive")
    if n_splits < 1:
        raise ValueError("n_splits must be >= 1")
    if embargo < 0:
        raise ValueError("embargo must be non-negative")

    if min_train is None:
        min_train = max(1, n_samples // (n_splits + 1))
    if min_train >= n_samples:
        raise ValueError("min_train must be smaller than n_samples")

    embargo_count = int(embargo * n_samples)
    remaining = np.arange(min_train, n_samples)
    if len(remaining) == 0:
        raise ValueError("no samples remain for test folds after min_train")

    test_fold_bounds = np.array_split(remaining, n_splits)
    window_size = min_train

    folds: FoldList = []
    for test_idx in test_fold_bounds:
        if len(test_idx) == 0:
            continue
        test_start = int(test_idx[0])
        train_end = test_start - embargo_count
        if train_end <= 0:
            continue

        if expanding:
            train = np.arange(0, train_end)
        else:
            train_start = max(0, train_end - window_size)
            train = np.arange(train_start, train_end)

        if len(train) == 0:
            continue

        folds.append((train, test_idx))

    return folds
