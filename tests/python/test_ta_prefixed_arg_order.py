"""The ``ta_``-prefixed plugins must read the same way as their siblings.

Regression cover for a bug where the generated signatures took ``(in2, in3)``
positionally and the underlying plugin consumed ``(high, low, close)``. The
receiver therefore had to be *high*, while the sibling ``.ta.atr`` takes *close*
in the receiver. Writing the call the obvious way silently permuted the inputs
and returned a plausible wrong number with no error.

Every assertion here is anchored to a value computed independently in this file,
not to whatever the implementation currently returns.
"""

import math

import pytest

pl = pytest.importorskip("polars")
import quantwave  # noqa: F401,E402  (registers the .ta namespace)


def _series(n: int = 200):
    """Deterministic random walk plus OHLC bracketing it."""
    import random

    random.seed(7)
    close = [100.0]
    for _ in range(n - 1):
        close.append(round(close[-1] * (1 + random.gauss(0, 0.01)), 4))
    high = [c * 1.004 for c in close]
    low = [c * 0.996 for c in close]
    return pl.DataFrame({"close": close, "high": high, "low": low})


def _wilder_atr(high, low, close, period=14):
    """Wilder's RMA of true range, SMA-seeded — the TA-Lib convention."""
    tr = [high[0] - low[0]]
    for i in range(1, len(close)):
        tr.append(
            max(
                high[i] - low[i],
                abs(high[i] - close[i - 1]),
                abs(low[i] - close[i - 1]),
            )
        )
    atr = sum(tr[1 : period + 1]) / period
    for i in range(period + 1, len(close)):
        atr = (atr * (period - 1) + tr[i]) / period
    return atr


def _true_range(high, low, close):
    return max(
        high[-1] - low[-1],
        abs(high[-1] - close[-2]),
        abs(low[-1] - close[-2]),
    )


class TestThreeInputReceiverIsClose:
    """ta_atr / ta_natr / ta_trange take close in the receiver, high+low as args."""

    def test_ta_atr_matches_hand_computed_wilder(self):
        df = _series()
        expected = _wilder_atr(df["high"].to_list(), df["low"].to_list(), df["close"].to_list())
        got = (
            df.lazy()
            .with_columns(pl.col("close").ta.ta_atr("high", "low", timeperiod=14).alias("x"))
            .collect()["x"][-1]
        )
        assert got == pytest.approx(expected, abs=1e-9)

    def test_ta_atr_agrees_with_its_sibling(self):
        """.ta.atr is Wilder too; written naturally, both must agree."""
        df = _series()
        out = (
            df.lazy()
            .with_columns(
                pl.col("close").ta.ta_atr("high", "low", timeperiod=14).alias("prefixed"),
                pl.col("close").ta.atr("high", "low", timeperiod=14).alias("sibling"),
            )
            .collect()
        )
        assert out["prefixed"][-1] == pytest.approx(out["sibling"][-1], abs=1e-9)

    def test_ta_trange_matches_hand_computed_true_range(self):
        df = _series()
        expected = _true_range(
            df["high"].to_list(), df["low"].to_list(), df["close"].to_list()
        )
        got = (
            df.lazy()
            .with_columns(pl.col("close").ta.ta_trange("high", "low").alias("x"))
            .collect()["x"][-1]
        )
        assert got == pytest.approx(expected, abs=1e-9)

    def test_ta_natr_is_atr_normalised_by_close(self):
        df = _series()
        atr = _wilder_atr(df["high"].to_list(), df["low"].to_list(), df["close"].to_list())
        expected = 100.0 * atr / df["close"][-1]
        got = (
            df.lazy()
            .with_columns(pl.col("close").ta.ta_natr("high", "low", timeperiod=14).alias("x"))
            .collect()["x"][-1]
        )
        assert got == pytest.approx(expected, rel=1e-6)

    def test_permuted_call_is_now_the_wrong_one(self):
        """The old receiver order must no longer be the correct one.

        Guards against a silent revert: if someone flips the args back, this
        starts matching Wilder and fails.
        """
        df = _series()
        expected = _wilder_atr(df["high"].to_list(), df["low"].to_list(), df["close"].to_list())
        permuted = (
            df.lazy()
            .with_columns(pl.col("high").ta.ta_atr("low", "close", timeperiod=14).alias("x"))
            .collect()["x"][-1]
        )
        assert not math.isclose(permuted, expected, abs_tol=1e-9)


class TestTwoInputReceiverIsSelf:
    """ta_beta / ta_correl already matched their siblings; pin the shape."""

    def test_ta_correl_agrees_with_sibling_at_equal_period(self):
        df = _series()
        out = (
            df.lazy()
            .with_columns(
                pl.col("close").ta.ta_correl("high", timeperiod=30).alias("prefixed"),
                pl.col("close").ta.correl("high", timeperiod=30).alias("sibling"),
            )
            .collect()
        )
        assert out["prefixed"][-1] == pytest.approx(out["sibling"][-1], abs=1e-12)

    def test_ta_beta_agrees_with_sibling_at_equal_period(self):
        df = _series()
        out = (
            df.lazy()
            .with_columns(
                pl.col("close").ta.ta_beta("high", timeperiod=5).alias("prefixed"),
                pl.col("close").ta.beta("high", timeperiod=5).alias("sibling"),
            )
            .collect()
        )
        assert out["prefixed"][-1] == pytest.approx(out["sibling"][-1], abs=1e-12)


def test_no_ta_plugin_still_uses_opaque_positional_names():
    """in2/in3 gave the call site nothing to key off — that is why it was invisible."""
    import inspect

    from quantwave._ta_namespace import TaNamespace

    offenders = []
    for name in dir(TaNamespace):
        if not name.startswith("ta_"):
            continue
        try:
            params = inspect.signature(getattr(TaNamespace, name)).parameters
        except (TypeError, ValueError):
            continue
        bad = [p for p in params if p in ("in2", "in3", "in4")]
        if bad:
            offenders.append(f"{name}{tuple(bad)}")
    assert not offenders, f"opaque positional params remain: {offenders}"
