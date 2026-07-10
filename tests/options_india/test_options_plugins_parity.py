"""Parity: vectorized options plugins vs scalar FFI (quantwave-5ipk.6)."""

from __future__ import annotations

import numpy as np
import polars as pl
import pytest
import quantwave as qw
from quantwave.polars import options as opt_expr


def _reference_row(spot: float, k: float, r: float, t: float, sigma: float, is_call: bool) -> dict:
    iv = qw.implied_vol(
        qw.bs_call_price(spot, k, r, t, sigma) if is_call else qw.bs_put_price(spot, k, r, t, sigma),
        spot,
        k,
        r,
        t,
        is_call,
    )
    return {
        "delta": qw.bs_delta(spot, k, r, t, sigma, is_call),
        "gamma": qw.bs_gamma(spot, k, r, t, sigma),
        "vega": qw.bs_vega(spot, k, r, t, sigma),
        "theta": qw.bs_theta(spot, k, r, t, sigma, is_call),
        "rho": qw.bs_rho(spot, k, r, t, sigma, is_call),
        "call": qw.bs_call_price(spot, k, r, t, sigma),
        "put": qw.bs_put_price(spot, k, r, t, sigma),
        "iv": iv if iv is not None else float("nan"),
    }


@pytest.mark.parametrize("is_call", [True, False])
def test_bs_plugins_match_scalar_reference(is_call: bool) -> None:
    rng = np.random.default_rng(42)
    n = 2_000
    spot = 25_000.0
    strikes = rng.uniform(22_000, 28_000, size=n)
    r = rng.uniform(0.04, 0.08, size=n)
    t = rng.uniform(1 / 365, 90 / 365, size=n)
    sigma = rng.uniform(0.12, 0.35, size=n)

    ref = [_reference_row(spot, float(k), float(rv), float(tv), float(sv), is_call)
           for k, rv, tv, sv in zip(strikes, r, t, sigma)]

    df = pl.DataFrame({
        "strike": strikes,
        "r": r,
        "t": t,
        "sigma": sigma,
        "price": [row["call" if is_call else "put"] for row in ref],
    })

    out = df.with_columns([
        opt_expr.bs_delta("sigma", spot, "strike", "r", "t", is_call=is_call).alias("delta"),
        opt_expr.bs_gamma("sigma", spot, "strike", "r", "t").alias("gamma"),
        opt_expr.bs_vega("sigma", spot, "strike", "r", "t").alias("vega"),
        opt_expr.bs_theta("sigma", spot, "strike", "r", "t", is_call=is_call).alias("theta"),
        opt_expr.bs_rho("sigma", spot, "strike", "r", "t", is_call=is_call).alias("rho"),
        opt_expr.bs_call_price(spot, "strike", "r", "t", "sigma").alias("call"),
        opt_expr.bs_put_price(spot, "strike", "r", "t", "sigma").alias("put"),
        opt_expr.implied_vol("price", spot, "strike", "r", "t", is_call=is_call).alias("iv"),
    ])

    for col in ("delta", "gamma", "vega", "theta", "rho", "call", "put", "iv"):
        got = out[col].to_numpy()
        want = np.array([row[col] for row in ref], dtype=np.float64)
        mask = ~(np.isnan(got) & np.isnan(want))
        assert np.allclose(got[mask], want[mask], rtol=1e-10, atol=1e-10), col