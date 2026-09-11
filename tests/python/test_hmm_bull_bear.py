"""Input-contract tests for the ``hmm_bull_bear`` plugin.

``hmm_bull_bear``'s Gaussian emissions are hardcoded to daily-returns scale
(means ~0.001/-0.002, stds ~0.01/0.02). Feeding it a raw price series used to
silently underflow both emissions and decode as a constant Bull(``1``) column
forever; it now raises instead of returning a plausible-looking wrong answer.
See ``quantwave-core/src/regimes/hmm.rs`` (``HMM::bull_bear``,
``HMM::is_degenerate``) and ``quantwave-py/src/plugins/custom_4.rs``
(``hmm_bull_bear``).
"""

from __future__ import annotations

import polars as pl
import pytest

qw = pytest.importorskip("quantwave")


def test_hmm_bull_bear_rejects_price_scale_uptrend() -> None:
    """A raw uptrending price series must error, not decode as constant Bull."""
    prices = [100.0 + i * 0.5 for i in range(60)]
    df = pl.DataFrame({"close": prices})
    with pytest.raises(Exception, match="looks like price, not returns"):
        df.lazy().with_columns(pl.col("close").ta.hmm_bull_bear().alias("regime")).collect()


def test_hmm_bull_bear_rejects_price_scale_downtrend() -> None:
    """A raw downtrending price series must also error (not decode as constant Bull)."""
    prices = [100.0 - i * 0.3 for i in range(60)]
    df = pl.DataFrame({"close": prices})
    with pytest.raises(Exception, match="looks like price, not returns"):
        df.lazy().with_columns(pl.col("close").ta.hmm_bull_bear().alias("regime")).collect()


def test_hmm_bull_bear_rejects_real_price_index() -> None:
    """A realistic price index (not centered near zero) must also error."""
    prices = [4500.0, 4510.0, 4495.0, 4520.0, 4530.0, 4512.0, 4540.0, 4551.0] * 8
    df = pl.DataFrame({"close": prices})
    with pytest.raises(Exception, match="looks like price, not returns"):
        df.lazy().with_columns(pl.col("close").ta.hmm_bull_bear().alias("regime")).collect()


def test_hmm_bull_bear_varies_on_returns_scale_input() -> None:
    """Regression: a real returns series must still decode varying {1,2} states."""
    returns = [
        0.01, 0.009, 0.011, 0.008, 0.012, 0.01, 0.009,  # bull run
        -0.02, -0.018, -0.022, -0.019, -0.025, -0.021, -0.017,  # bear run
        0.011, 0.009, 0.012, 0.01, 0.008, 0.011,  # bull again
    ]
    df = pl.DataFrame({"ret": returns})
    out = (
        df.lazy()
        .with_columns(pl.col("ret").ta.hmm_bull_bear().alias("regime"))
        .collect()
    )
    regimes = out["regime"].to_list()

    assert set(regimes) <= {0, 1, 2}
    assert len(set(regimes)) > 1, f"expected varying Bull/Bear regimes, got constant {regimes}"
    # Bear (2) should show up during the sustained negative-return run, and Bull
    # (1) should show up during the positive-return runs.
    assert 2 in regimes[7:14]
    assert 1 in regimes[:7] or 1 in regimes[14:]
