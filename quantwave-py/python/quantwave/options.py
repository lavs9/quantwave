"""
Options India helpers.

These were previously exposed at the top level of quantwave, which polluted
the indicator namespace. They are now properly namespaced.

For backward compatibility, the old top-level names (``quantwave.bs_call_price``,
etc.) still work via ``__getattr__`` but emit ``DeprecationWarning``.
"""

from typing import Any, Optional, Union
import numpy as np

from ._quantwave import (
    bs_call_price,
    bs_put_price,
    bs_delta,
    bs_gamma,
    bs_theta,
    bs_vega,
    bs_rho,
    black76_call_price,
    black76_put_price,
    black76_delta,
    black76_gamma,
    black76_theta,
    black76_vega,
    implied_vol as _raw_implied_vol,
    max_pain,
    strike_pcr,
    chain_pcr,
    oi_zones,
    gex_per_strike,
    gex_flip_strike,
    gextotal,
    atm_straddle,
    synthetic_futures,
    nse_lot_size as _raw_nse_lot_size,
    nse_risk_free_rate,
    moneyness,
)


def implied_vol(
    market_price: Any,
    s: Any = None,
    k: Any = None,
    r: Any = None,
    t: Any = None,
    is_call: Any = True,
    forward: Any = None,
) -> Any:
    """Calculate implied volatility under Black-Scholes / Black-76.

    Supports both scalar and vectorized (array/list) inputs.

    Parameters:
        market_price: Option market price (scalar, list, or array).
        s: Underlying spot price (optional if forward is provided).
        k: Strike price.
        r: Risk-free rate (annualized, default: 0.065 or 0.0).
        t: Time to expiry in years.
        is_call: True for Call option, False for Put (default: True).
        forward: Forward / futures price F (optional; if provided, spot s is not required).

    Returns:
        Implied volatility as annual decimal fraction (e.g. 0.18 for 18%), or None / NaN
        if market price is <= 0 or at/below intrinsic (zero/negative time value).
        Returns a numpy array if any input is array-like, otherwise a scalar float or None.
    """
    is_seq = any(
        hasattr(arg, "__len__") and not isinstance(arg, (str, bytes, dict))
        for arg in (market_price, s, k, r, t, is_call, forward)
        if arg is not None
    )

    if not is_seq:
        p = float(market_price) if market_price is not None else None
        s_val = float(s) if s is not None else None
        k_val = float(k) if k is not None else None
        r_val = float(r) if r is not None else 0.0
        t_val = float(t) if t is not None else 0.0
        c_val = bool(is_call) if is_call is not None else True
        f_val = float(forward) if forward is not None else None
        if p is None or p <= 0.0 or t is None or t_val <= 0.0:
            return None
        return _raw_implied_vol(p, s_val, k_val, r_val, t_val, c_val, f_val)

    # Vectorized path
    args = [market_price, s, k, r, t, is_call, forward]
    arrays = [np.asarray(a) if a is not None else None for a in args]
    shapes = [a.shape for a in arrays if a is not None]
    if not shapes:
        return np.array([])
    b_shape = np.broadcast_shapes(*shapes)
    b_arrays = [np.broadcast_to(a, b_shape) if a is not None else None for a in arrays]
    flat_arrays = [a.ravel() if a is not None else None for a in b_arrays]

    n = 0
    for a in flat_arrays:
        if a is not None:
            n = a.size
            break

    out = np.empty(n, dtype=np.float64)
    for i in range(n):
        p = float(flat_arrays[0][i]) if flat_arrays[0] is not None else None
        s_val = float(flat_arrays[1][i]) if flat_arrays[1] is not None else None
        k_val = float(flat_arrays[2][i]) if flat_arrays[2] is not None else None
        r_val = float(flat_arrays[3][i]) if flat_arrays[3] is not None else 0.0
        t_val = float(flat_arrays[4][i]) if flat_arrays[4] is not None else 0.0
        c_val = bool(flat_arrays[5][i]) if flat_arrays[5] is not None else True
        f_val = float(flat_arrays[6][i]) if flat_arrays[6] is not None else None

        if p is None or p <= 0.0 or flat_arrays[4] is None or t_val <= 0.0:
            out[i] = np.nan
        else:
            res = _raw_implied_vol(p, s_val, k_val, r_val, t_val, c_val, f_val)
            out[i] = res if res is not None else np.nan

    return out.reshape(b_shape)


def nse_lot_size(symbol: str, as_of: Optional[Union[str, Any]] = None) -> Optional[int]:
    """NSE standard lot size for index/equity derivatives contracts.

    Parameters:
        symbol: Underlier symbol (e.g. 'NIFTY', 'BANKNIFTY', 'FINNIFTY').
        as_of: Optional date string ('YYYY-MM-DD') or date object for historical lot sizes.
               Revisions supported for NIFTY:
               - pre-July 2021: 75
               - July 2021 to April 2024: 50
               - April 2024 to Nov 2024: 25
               - Nov 2024 to Nov 2025: 75
               - Nov 2025 onwards (current): 65
    """
    if as_of is not None and not isinstance(as_of, str):
        as_of = str(as_of)
    return _raw_nse_lot_size(symbol, as_of)


__all__ = [
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho",
    "black76_call_price", "black76_put_price", "black76_delta", "black76_gamma",
    "black76_theta", "black76_vega",
    "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike", "gextotal",
    "atm_straddle", "synthetic_futures", "nse_lot_size",
    "nse_risk_free_rate", "moneyness",
]
