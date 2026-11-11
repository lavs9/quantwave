import math
import numpy as np
import pytest
import quantwave as qw

try:
    import polars as pl
except ImportError:
    pl = None


# ============================================================================
# GH-55: SMA/EMA Warmup Policy
# ============================================================================

def test_sma_warmup_partial_default():
    series = [10.0, 20.0, 30.0, 40.0]
    # Default is "partial"
    out = qw.sma(3, series)
    assert math.isclose(out[0], 10.0)
    assert math.isclose(out[1], 15.0)
    assert math.isclose(out[2], 20.0)
    assert math.isclose(out[3], 30.0)


def test_sma_warmup_nan():
    series = [10.0, 20.0, 30.0, 40.0]
    out = qw.sma(3, series, warmup="nan")
    assert math.isnan(out[0])
    assert math.isnan(out[1])
    assert math.isclose(out[2], 20.0)
    assert math.isclose(out[3], 30.0)

    # Streaming class with warmup="nan"
    inst = qw.Sma(3, warmup="nan")
    stream_out = [inst.next(x) for x in series]
    assert math.isnan(stream_out[0])
    assert math.isnan(stream_out[1])
    assert math.isclose(stream_out[2], 20.0)
    assert math.isclose(stream_out[3], 30.0)


def test_ema_warmup_nan():
    series = [10.0, 20.0, 30.0, 40.0]
    out = qw.ema(3, series, warmup="nan")
    assert math.isnan(out[0])
    assert math.isnan(out[1])
    # Bar 2 is seeded with SMA of first 3 bars: (10 + 20 + 30)/3 = 20.0
    assert math.isclose(out[2], 20.0)
    # Bar 3: alpha = 2/(3+1) = 0.5; EMA = 40.0 * 0.5 + 20.0 * 0.5 = 30.0
    assert math.isclose(out[3], 30.0)


def test_sma_invalid_warmup():
    with pytest.raises(ValueError, match="unknown warmup policy"):
        qw.sma(3, [1.0, 2.0], warmup="unsupported")


# ============================================================================
# GH-56: NaN Poisoning Fix & Non-stale SuperTrend
# ============================================================================

def test_ema_nan_recovery():
    series = [1.0, 2.0, 3.0, float("nan"), 5.0, 6.0, 7.0]
    out = qw.ema(3, series)
    assert not math.isnan(out[2])
    assert math.isnan(out[3])  # NaN input returns NaN
    assert not math.isnan(out[4])  # Resumes smoothly!
    assert not math.isnan(out[5])
    assert not math.isnan(out[6])


def test_beta_nan_recovery():
    # After NaN ages out of rolling window, Beta should recover to finite value
    n = 20
    market = [10.0 + i for i in range(n)]
    stock = [10.0 + i for i in range(n)]
    market[5] = float("nan")
    stock[5] = float("nan")

    out = qw.beta(5, market=market, stock=stock)
    assert math.isnan(out[5])
    # By index 15, the NaN at 5 has aged out of window of size 5
    assert not math.isnan(out[15])
    assert math.isclose(out[15], 1.0, rel_tol=1e-3)


def test_supertrend_nan_emits_nan_not_stale():
    high = [10.0, 11.0, 12.0, float("nan"), 14.0]
    low = [8.0, 9.0, 10.0, float("nan"), 12.0]
    close = [9.0, 10.0, 11.0, float("nan"), 13.0]
    res = qw.supertrend(3, 2.0, high, low, close)
    assert not math.isnan(res[2].value)
    assert math.isnan(res[3].value)  # Emits NaN, not stale value from bar 2!
    assert not math.isnan(res[4].value)


# ============================================================================
# GH-57: Parameter and Length Validation
# ============================================================================

def test_period_validation_negative_and_zero():
    # Negative periods must raise ValueError (not OverflowError!)
    with pytest.raises(ValueError):
        qw.sma(0, [1.0, 2.0])
    with pytest.raises(ValueError):
        qw.sma(-1, [1.0, 2.0])
    with pytest.raises(ValueError):
        qw.ema(0, [1.0, 2.0])
    with pytest.raises(ValueError):
        qw.ema(-5, [1.0, 2.0])
    with pytest.raises(ValueError):
        qw.atr(0, [1.0], [1.0], [1.0])
    with pytest.raises(ValueError):
        qw.atr(-3, [1.0], [1.0], [1.0])
    with pytest.raises(ValueError):
        qw.supertrend(0, 3.0, [1.0], [1.0], [1.0])
    with pytest.raises(ValueError):
        qw.supertrend(-2, 3.0, [1.0], [1.0], [1.0])


def test_mismatched_lengths_validation():
    # Mismatched array lengths must raise ValueError (no silent truncation!)
    with pytest.raises(ValueError, match="lengths must match"):
        qw.atr(3, [10.0, 11.0], [8.0], [9.0, 10.0])

    with pytest.raises(ValueError, match="lengths must match"):
        qw.supertrend(3, 2.0, [10.0, 11.0], [8.0, 9.0], [9.0])

    with pytest.raises(ValueError, match="lengths must match"):
        qw.correl(3, [1.0, 2.0, 3.0], [1.0, 2.0])

    with pytest.raises(ValueError, match="lengths must match"):
        qw.beta(3, market=[1.0, 2.0, 3.0], stock=[1.0, 2.0])


# ============================================================================
# GH-58: Docstrings, Beta Arguments, ATR Smoothing
# ============================================================================

def test_docstrings_populated():
    assert qw.sma.__doc__ is not None and len(qw.sma.__doc__) > 20
    assert qw.ema.__doc__ is not None and len(qw.ema.__doc__) > 20
    assert qw.supertrend.__doc__ is not None and len(qw.supertrend.__doc__) > 20
    assert qw.atr.__doc__ is not None and len(qw.atr.__doc__) > 20
    assert qw.beta.__doc__ is not None and len(qw.beta.__doc__) > 20


def test_beta_argument_orders():
    m = [10.0, 11.0, 12.0, 13.0, 14.0]
    s = [20.0, 22.0, 24.0, 26.0, 28.0]
    # Positional
    res1 = qw.beta(3, m, s)
    # Named market, stock
    res2 = qw.beta(3, market=m, stock=s)
    # Named real0, real1
    res3 = qw.beta(3, real0=m, real1=s)

    assert np.allclose(res1[3:], res2[3:])
    assert np.allclose(res1[3:], res3[3:])


def test_atr_wilder_and_ta_atr_exposed():
    high = [10.0, 11.0, 12.0, 13.0]
    low = [8.0, 9.0, 10.0, 11.0]
    close = [9.0, 10.0, 11.0, 12.0]

    out_wilder = qw.atr_wilder(3, high, low, close)
    out_ta_atr = qw.ta_atr(3, high, low, close)

    assert len(out_wilder) == len(high)
    assert np.allclose(out_wilder[3:], out_ta_atr[3:])
    # Streaming classes
    inst1 = qw.AtrWilder(3)
    inst2 = qw.TaAtr(3)
    for h, l, c in zip(high, low, close):
        r1 = inst1.next(h, l, c)
        r2 = inst2.next(h, l, c)
        if not math.isnan(r1):
            assert math.isclose(r1, r2)


# ============================================================================
# GH-59: SuperTrend IPO Breakout & Initial Direction
# ============================================================================

def test_supertrend_ipo_breakout_down_to_up():
    # IPO listing starts in downtrend (-1) with Wilder smoothing
    # Bars 0, 1, 2: Wilder ATR warmup -> emits NaN
    # Bar 3: First valid ATR, downtrend (-1), upper band established
    # Bar 4: Price breaks out above upper band -> flips to +1!
    high = [10.0, 11.0, 11.0, 10.0, 16.0]
    low = [8.0, 9.0, 9.0, 8.0, 14.0]
    close = [9.0, 10.0, 10.0, 9.5, 15.0]

    res = qw.supertrend(
        3,
        2.0,
        high,
        low,
        close,
        initial_direction="down",
        smoothing="wilder",
    )
    # Warmup bars emit NaN
    assert math.isnan(res[0].value)
    assert math.isnan(res[1].value)
    assert math.isnan(res[2].value)

    # Bar 3: Valid band, direction is -1 (downtrend)
    assert not math.isnan(res[3].value)
    assert res[3].direction == -1
    upper_band = res[3].value

    # Bar 4: Close (15.0) > Upper Band -> flips to +1 (Uptrend Breakout!)
    assert res[4].direction == 1
    assert close[4] > upper_band


@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_supertrend_polars_plugin_options():
    df = pl.DataFrame({
        "high": [10.0, 11.0, 11.0, 10.0, 16.0],
        "low": [8.0, 9.0, 9.0, 8.0, 14.0],
        "close": [9.0, 10.0, 10.0, 9.5, 15.0],
    })
    out = df.select(
        pl.col("close")
        .ta.supertrend(
            high="high",
            low="low",
            period=3,
            multiplier=2.0,
            initial_direction="down",
            smoothing="wilder",
        )
        .alias("st")
    ).unnest("st")

    dirs = out["direction"].to_list()
    # At bar 3: -1.0; at bar 4: 1.0
    assert dirs[3] == -1.0
    assert dirs[4] == 1.0


# ---------------------------------------------------------------------------
# GH-60 Dogfood Fixes
# ---------------------------------------------------------------------------


@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_gh60_sma_nan_parity_plain_and_polars():
    data = [1.0, 2.0, float("nan"), 4.0, 5.0, 6.0]
    plain1 = qw.sma(3, data)
    plain2 = qw.sma(data, 3)
    df = pl.DataFrame({"x": data})
    polars_res = df.select(pl.col("x").ta.sma(3))["x"].to_list()

    assert np.allclose(plain1, polars_res, equal_nan=True)
    assert np.allclose(plain2, polars_res, equal_nan=True)
    # NaN in bar 2 propagates for the duration of the rolling window (bars 2, 3, 4)
    # and then cleanly recovers on bar 5
    assert math.isnan(polars_res[2])
    assert math.isnan(polars_res[3])
    assert math.isnan(polars_res[4])
    assert abs(polars_res[5] - 5.0) < 1e-6


@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_gh60_atr_output_naming():
    df = pl.DataFrame({"h": [10.0, 11.0, 12.0], "l": [8.0, 9.0, 10.0], "c": [9.0, 10.0, 11.0]})
    # In a select with h and ATR, it must not collide with h
    res = df.select([pl.col("c").ta.atr(pl.col("h"), pl.col("l"), 2), pl.col("h")])
    assert res.columns == ["c", "h"]


def test_gh60_atr_argument_order_flexibility():
    h = [10.0, 11.0, 12.0, 13.0, 14.0]
    l = [8.0, 9.0, 10.0, 11.0, 12.0]
    c = [9.0, 10.0, 11.0, 12.0, 13.0]
    res1 = qw.atr(3, h, l, c)
    res2 = qw.atr(h, l, c, 3)
    res3 = qw.atr(h, l, c, period=3)
    assert np.allclose(res1, res2, equal_nan=True)
    assert np.allclose(res1, res3, equal_nan=True)

    ta_res1 = qw.ta_atr(3, h, l, c)
    ta_res2 = qw.ta_atr(h, l, c, 3)
    assert np.allclose(ta_res1, ta_res2, equal_nan=True)


def test_gh60_max_min_module_and_docstrings():
    c = [1.0, 5.0, 2.0, 8.0, 3.0]
    m1 = qw.max(c, 3)
    m2 = qw.max(3, c)
    assert np.allclose(m1, m2, equal_nan=True)

    min1 = qw.min(c, 3)
    min2 = qw.min(3, c)
    assert np.allclose(min1, min2, equal_nan=True)

    if pl is not None:
        from quantwave._ta_namespace import TaNamespace
        assert "Rolling Highest Value" in TaNamespace.max.__doc__
        assert "Rolling Lowest Value" in TaNamespace.min.__doc__


@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_gh60_cross_sectional_rank():
    df = pl.DataFrame({
        "date": ["2026-01-01"] * 3 + ["2026-01-02"] * 3,
        "symbol": ["A", "B", "C"] * 2,
        "score": [10.0, 20.0, 30.0, 100.0, 50.0, 75.0],
    })
    ranked = df.with_columns(
        pl.col("score").ta.cross_sectional_rank(group_by="date").alias("rank")
    )
    # On 2026-01-01: A=10 -> 1/3, B=20 -> 2/3, C=30 -> 1.0
    r = ranked["rank"].to_list()
    assert abs(r[0] - 1 / 3) < 1e-5
    assert abs(r[1] - 2 / 3) < 1e-5
    assert abs(r[2] - 1.0) < 1e-5
    # On 2026-01-02: B=50 -> 1/3, C=75 -> 2/3, A=100 -> 1.0
    assert abs(r[3] - 1.0) < 1e-5
    assert abs(r[4] - 1 / 3) < 1e-5
    assert abs(r[5] - 2 / 3) < 1e-5

    # Module-level sequence rank
    module_ranked = qw.cross_sectional_rank([10.0, 20.0, 30.0])
    assert np.allclose(module_ranked, [1 / 3, 2 / 3, 1.0])


# ============================================================================
# GH-61: ATR Parity, Warmup in Polars MAs, ROCP, Gap Reset Helper
# ============================================================================

@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_gh61_atr_plain_polars_parity():
    rng = np.random.default_rng(1)
    cc = 100 + np.cumsum(rng.normal(0, 1, 120))
    hh = cc + rng.uniform(0, 2, 120)
    ll = cc - rng.uniform(0, 2, 120)

    # 1. Plain default is Wilder RMA
    a1 = np.array(qw.atr(14, hh, ll, cc))
    # 2. Polars default is Wilder RMA
    df = pl.DataFrame({"h": hh, "l": ll, "c": cc})
    a2 = df.select(pl.col("c").ta.atr(pl.col("h"), pl.col("l"), 14))["c"].to_numpy()

    # Both must match bit-for-bit (GH-61 repro value: ~2.1427)
    np.testing.assert_allclose(a1, a2, equal_nan=True)
    assert abs(a1[100] - 2.1427) < 1e-3

    # 3. Plain and Polars both support smoothing="ema" (GH-61 repro value: ~2.2230)
    a1_ema = np.array(qw.atr(14, hh, ll, cc, smoothing="ema"))
    a2_ema = df.select(
        pl.col("c").ta.atr(pl.col("h"), pl.col("l"), 14, smoothing="ema")
    )["c"].to_numpy()
    np.testing.assert_allclose(a1_ema, a2_ema, equal_nan=True)
    assert abs(a1_ema[100] - 2.2230) < 1e-3

    # 4. Streaming class matches both
    s_wilder = qw.Atr(14)
    res_wilder = [s_wilder.next(h, l, c) for h, l, c in zip(hh, ll, cc)]
    np.testing.assert_allclose(a1, res_wilder, equal_nan=True)

    s_ema = qw.Atr(14, smoothing="ema")
    res_ema = [s_ema.next(h, l, c) for h, l, c in zip(hh, ll, cc)]
    np.testing.assert_allclose(a1_ema, res_ema, equal_nan=True)


@pytest.mark.skipif(pl is None, reason="polars not installed")
def test_gh61_polars_moving_averages_warmup():
    df = pl.DataFrame({"x": [1.0, 2.0, 3.0, 4.0]})

    # SMA
    sma_nan = df.select(pl.col("x").ta.sma(3, warmup="nan"))["x"].to_list()
    assert np.isnan(sma_nan[0]) and np.isnan(sma_nan[1])
    assert abs(sma_nan[2] - 2.0) < 1e-9
    assert abs(sma_nan[3] - 3.0) < 1e-9

    sma_partial = df.select(pl.col("x").ta.sma(3, warmup="partial"))["x"].to_list()
    assert abs(sma_partial[0] - 1.0) < 1e-9
    assert abs(sma_partial[1] - 1.5) < 1e-9

    # EMA
    ema_nan = df.select(pl.col("x").ta.ema(3, warmup="nan"))["x"].to_list()
    assert np.isnan(ema_nan[0]) and np.isnan(ema_nan[1])
    assert not np.isnan(ema_nan[2])

    ema_partial = df.select(pl.col("x").ta.ema(3, warmup="partial"))["x"].to_list()
    assert abs(ema_partial[0] - 1.0) < 1e-9

    # ALMA
    alma_nan = df.select(pl.col("x").ta.alma(3, warmup="nan"))["x"].to_list()
    assert np.isnan(alma_nan[0]) and np.isnan(alma_nan[1])
    assert not np.isnan(alma_nan[2])

    alma_partial = df.select(pl.col("x").ta.alma(3, warmup="partial"))["x"].to_list()
    assert not np.isnan(alma_partial[0])


def test_gh61_rocp():
    # Plain rocp returns fractional ratio: (p - p_prev) / p_prev
    res = qw.rocp(1, [1.0, 2.0])
    assert np.isnan(res[0])
    assert abs(res[1] - 1.0) < 1e-9  # 1.0 = +100%

    # roc returns percent (100 * fraction)
    roc_res = qw.roc(1, [1.0, 2.0])
    assert np.isnan(roc_res[0])
    assert abs(roc_res[1] - 100.0) < 1e-9

    # Streaming Rocp
    stream = qw.Rocp(1)
    v0 = stream.next(1.0)
    v1 = stream.next(2.0)
    assert np.isnan(v0)
    assert abs(v1 - 1.0) < 1e-9


def test_gh61_apply_with_gap_reset():
    # Create series with a 6-bar NaN gap (exceeds default max_gap=5)
    data = [10.0, 11.0, 12.0, 13.0, 14.0] + [float("nan")] * 6 + [20.0, 21.0, 22.0]
    out = qw.apply_with_gap_reset(lambda: qw.Sma(3, warmup="nan"), data, max_gap=5)

    assert len(out) == len(data)
    # Bars 0..4: normal warmup for period 3
    assert np.isnan(out[0]) and np.isnan(out[1])
    assert abs(out[2] - 11.0) < 1e-9
    # Gap bars: all NaN
    for i in range(5, 11):
        assert np.isnan(out[i])
    # Bar 11 (index 11, value 20.0): indicator was reset! Fresh warmup means NaN
    assert np.isnan(out[11])
    assert np.isnan(out[12])
    # Bar 13 (index 13, value 22.0): 3 bars accumulated -> (20+21+22)/3 = 21.0
    assert abs(out[13] - 21.0) < 1e-9


def test_gh61_smoothing_convention_docstrings():
    # Document default smoothing differences so callers don't assume atr and supertrend agree
    assert "Wilder" in qw.atr.__doc__
    assert "supertrend" in qw.atr.__doc__
    assert "EMA" in qw.supertrend.__doc__
    assert "Wilder" in qw.supertrend.__doc__

    if pl is not None:
        from quantwave._ta_namespace import TaNamespace
        assert "Wilder" in TaNamespace.atr.__doc__
        assert "supertrend" in TaNamespace.atr.__doc__
        assert "EMA" in TaNamespace.supertrend.__doc__
        assert "Wilder" in TaNamespace.supertrend.__doc__


def test_gh62_atr_seed_options_and_wilder1978_parity():
    # 20-bar synthetic series
    high = [10.0, 11.0, 12.0, 11.5, 12.5, 13.0, 12.8, 13.5, 14.0, 13.8, 14.2, 14.5, 14.1, 14.8, 15.0, 15.2, 15.1, 15.5, 15.8, 16.0]
    low = [8.0, 9.0, 10.0, 9.5, 10.5, 11.0, 10.8, 11.5, 12.0, 11.8, 12.2, 12.5, 12.1, 12.8, 13.0, 13.2, 13.1, 13.5, 13.8, 14.0]
    close = [9.0, 10.5, 11.0, 10.0, 12.0, 11.5, 12.5, 13.0, 12.5, 13.5, 13.0, 14.0, 13.5, 14.5, 14.0, 14.8, 14.2, 15.0, 14.5, 15.5]

    # Classic Wilder 1978 hand formula:
    # TR[0] = H[0] - L[0]
    # ATR[13] = mean(TR[0:14])
    # ATR[t] = (13 * ATR[t-1] + TR[t]) / 14
    tr = [high[0] - low[0]]
    for i in range(1, 20):
        tr.append(max(high[i] - low[i], abs(high[i] - close[i-1]), abs(low[i] - close[i-1])))

    atr_hand = [float("nan")] * 20
    atr_hand[13] = sum(tr[0:14]) / 14.0
    for i in range(14, 20):
        atr_hand[i] = (13.0 * atr_hand[i-1] + tr[i]) / 14.0

    # 1. Plain qw.atr seed='classic' / 'wilder1978'
    atr_classic = qw.atr(high, low, close, period=14, seed="classic")
    atr_w1978 = qw.atr(14, high, low, close, seed="wilder1978")
    assert np.isnan(atr_classic[12])
    assert not np.isnan(atr_classic[13])
    assert np.allclose(atr_classic[13:], atr_hand[13:], rtol=1e-10)
    assert np.allclose(atr_w1978[13:], atr_hand[13:], rtol=1e-10)

    # 2. Warmup alias
    atr_warmup_classic = qw.atr(high, low, close, period=14, warmup="classic")
    assert np.allclose(atr_warmup_classic[13:], atr_hand[13:], rtol=1e-10)

    # 3. Default seed='talib' has 14 NaNs, seeds at index 14
    atr_talib = qw.atr(high, low, close, period=14)
    assert np.isnan(atr_talib[13])
    assert not np.isnan(atr_talib[14])

    # 4. Polars parity
    if pl is not None:
        df = pl.DataFrame({"h": high, "l": low, "c": close})
        polars_classic = df.select(
            pl.col("c").ta.atr(pl.col("h"), pl.col("l"), timeperiod=14, seed="classic")
        )["c"].to_list()
        assert np.allclose(polars_classic[13:], atr_classic[13:], rtol=1e-10)

    # 5. Streaming parity
    stream_classic = qw.Atr(14, seed="classic")
    stream_out = [stream_classic.next(h, l, c) for h, l, c in zip(high, low, close)]
    assert np.allclose(stream_out[13:], atr_classic[13:], rtol=1e-10)

    # 6. EMA warmup='nan' option
    atr_ema_nan = qw.atr(high, low, close, period=14, smoothing="ema", warmup="nan")
    assert np.isnan(atr_ema_nan[12])
    assert not np.isnan(atr_ema_nan[13])



