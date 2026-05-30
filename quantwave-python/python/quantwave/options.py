"""
Options India helpers.

These were previously exposed at the top level of quantwave, which polluted
the indicator namespace. They are now properly namespaced.

For backward compatibility, the old top-level names still work but will
emit DeprecationWarnings in a future release.
"""

from ._quantwave import (
    bs_call_price,
    bs_put_price,
    bs_delta,
    bs_gamma,
    bs_theta,
    bs_vega,
    bs_rho,
    implied_vol,
    max_pain,
    strike_pcr,
    chain_pcr,
    oi_zones,
    gex_per_strike,
    gex_flip_strike,
    atm_straddle,
    synthetic_futures,
    nse_lot_size,
    nse_risk_free_rate,
    moneyness,
)

__all__ = [
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho", "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike",
    "atm_straddle", "synthetic_futures", "nse_lot_size",
    "nse_risk_free_rate", "moneyness",
]
