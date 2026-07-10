"""Parity: vectorized chain-analytics wrappers vs scalar core FFI (quantwave-5ipk.6).

The Polars wrappers in ``quantwave.polars.options`` must reproduce the exact
``quantwave_core::options_india::chain_analytics`` / ``india`` results. Per-strike
and lookup analytics are native Polars expressions; whole-chain reductions call
the same Rust core once per chain. Both are checked here.
"""

from __future__ import annotations

import numpy as np
import polars as pl
import pytest

from quantwave.polars import options as opt
from quantwave._quantwave import (
    max_pain as core_max_pain,
    strike_pcr as core_strike_pcr,
    chain_pcr as core_chain_pcr,
    oi_zones as core_oi_zones,
    gex_per_strike as core_gex_per_strike,
    gex_flip_strike as core_gex_flip_strike,
    atm_straddle as core_atm_straddle,
    synthetic_futures as core_synthetic_futures,
    moneyness as core_moneyness,
    nse_lot_size as core_nse_lot_size,
)


def _elementwise_chain(seed: int, n: int = 10_000) -> pl.DataFrame:
    rng = np.random.default_rng(seed)
    return pl.DataFrame({
        "k": rng.uniform(100.0, 5000.0, n),
        "ce_oi": rng.integers(0, 50_000, n).astype("int64"),
        "pe_oi": rng.integers(0, 50_000, n).astype("int64"),
        "ce_ltp": rng.uniform(0.0, 500.0, n),
        "pe_ltp": rng.uniform(0.0, 500.0, n),
        "ce_gamma": rng.uniform(0.0, 0.01, n),
        "pe_gamma": rng.uniform(0.0, 0.01, n),
    })


def test_strike_pcr_parity() -> None:
    df = _elementwise_chain(1)
    got = df.select(opt.strike_pcr("ce_oi", "pe_oi").alias("x"))["x"].to_numpy()
    want = np.array(core_strike_pcr(df["ce_oi"].to_list(), df["pe_oi"].to_list()))
    assert np.max(np.abs(got - want)) < 1e-10


def test_synthetic_futures_parity() -> None:
    df = _elementwise_chain(2)
    got = df.select(opt.synthetic_futures("k", "ce_ltp", "pe_ltp").alias("x"))["x"].to_numpy()
    want = np.array(core_synthetic_futures(
        df["k"].to_list(), df["ce_ltp"].to_list(), df["pe_ltp"].to_list()))
    assert np.max(np.abs(got - want)) < 1e-10


def test_gex_per_strike_parity() -> None:
    df = _elementwise_chain(3)
    spot, lot = 2500.0, 50
    out = df.select(
        opt.gex_per_strike(spot, "k", "ce_gamma", "pe_gamma", "ce_oi", "pe_oi", lot).alias("g")
    )["g"]
    core = core_gex_per_strike(
        spot, df["k"].to_list(), df["ce_gamma"].to_list(), df["pe_gamma"].to_list(),
        df["ce_oi"].to_list(), df["pe_oi"].to_list(), lot)
    for field, attr in (("ce_gex", "ce_gex"), ("pe_gex", "pe_gex"), ("net_gex", "net_gex")):
        got = out.struct.field(field).to_numpy()
        want = np.array([getattr(r, attr) for r in core])
        assert np.max(np.abs(got - want)) < 1e-10, field


def test_moneyness_parity() -> None:
    df = _elementwise_chain(4)
    spot = 2500.0
    got = df.select(opt.moneyness(spot, "k").alias("x"))["x"].to_list()
    want = [core_moneyness(spot, float(k)) for k in df["k"].to_list()]
    assert got == want


def test_nse_lot_size_parity() -> None:
    symbols = ["NIFTY", "banknifty", "FinNifty", "MIDCPNIFTY", "SENSEX", "UNKNOWN", "reliance"]
    df = pl.DataFrame({"sym": symbols})
    got = df.select(opt.nse_lot_size("sym").alias("x"))["x"].to_list()
    want = [core_nse_lot_size(s) for s in symbols]
    assert got == want


def test_chain_pcr_parity() -> None:
    for seed in range(5):
        df = _elementwise_chain(seed, n=200)
        got = df.select(opt.chain_pcr("ce_oi", "pe_oi").alias("x"))["x"][0]
        want = core_chain_pcr(df["ce_oi"].to_list(), df["pe_oi"].to_list())
        assert abs(got - want) < 1e-10


@pytest.mark.parametrize("seed", range(20))
def test_reduction_parity(seed: int) -> None:
    """Whole-chain reductions (single native call per chain) match core exactly."""
    rng = np.random.default_rng(100 + seed)
    n = int(rng.integers(10, 80))
    k = np.sort(rng.uniform(20_000.0, 30_000.0, n))
    ce_oi = rng.integers(0, 100_000, n).astype("int64")
    pe_oi = rng.integers(0, 100_000, n).astype("int64")
    ce_ltp = rng.uniform(0.0, 500.0, n)
    pe_ltp = rng.uniform(0.0, 500.0, n)
    net_gex = rng.uniform(-10.0, 10.0, n)
    spot, lot = 25_000.0, 50
    df = pl.DataFrame({
        "k": k, "ce_oi": ce_oi, "pe_oi": pe_oi,
        "ce_ltp": ce_ltp, "pe_ltp": pe_ltp, "net_gex": net_gex,
    })

    # max_pain
    got = df.select(opt.max_pain("k", "ce_oi", "pe_oi", lot).alias("x"))["x"][0]
    want = core_max_pain(k.tolist(), ce_oi.tolist(), pe_oi.tolist(), lot)
    assert got == pytest.approx(want, abs=1e-9)

    # oi_zones
    zones = df.select(opt.oi_zones("k", "ce_oi", "pe_oi", 3).alias("z"))["z"][0]
    core_z = core_oi_zones(k.tolist(), ce_oi.tolist(), pe_oi.tolist(), 3)
    assert zones["resistance_strikes"] == pytest.approx(core_z.resistance_strikes)
    assert zones["support_strikes"] == pytest.approx(core_z.support_strikes)

    # gex_flip_strike
    got_flip = df.select(opt.gex_flip_strike("k", "net_gex").alias("x"))["x"][0]
    want_flip = core_gex_flip_strike(k.tolist(), net_gex.tolist())
    assert got_flip == (pytest.approx(want_flip) if want_flip is not None else None)

    # atm_straddle
    strd = df.select(opt.atm_straddle(spot, "k", "ce_ltp", "pe_ltp").alias("s"))["s"][0]
    core_s = core_atm_straddle(spot, k.tolist(), ce_ltp.tolist(), pe_ltp.tolist())
    assert strd["atm_strike"] == pytest.approx(core_s.atm_strike)
    assert strd["straddle_premium"] == pytest.approx(core_s.straddle_premium)
    assert strd["implied_move_pct"] == pytest.approx(core_s.implied_move_pct)
