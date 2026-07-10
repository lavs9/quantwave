# Options India Module

The `options_india` module provides high-performance Black-Scholes pricing, robust Implied Volatility (IV) solving, and comprehensive option chain analytics tailored for the Indian market (NSE/BSE).

## Features

- **Black-Scholes Core**: Precise pricing and Greeks (Delta, Gamma, Theta, Vega, Rho).
- **LetsBeRational IV Solver**: Implementation of Peter Jäckel's gold-standard algorithm for stable and fast IV calculation across all moneyness levels.
- **India-Specific Helpers**: Automatic handling of NSE lot sizes, risk-free rates (91-day T-bill), and calendar-based Days to Expiry (DTE).
- **Chain Analytics**: Tools for market sentiment analysis including Max Pain, Put-Call Ratio (PCR), Gamma Exposure (GEX), and Synthetic Futures.

## Implementation Details

### Implied Volatility

QuantWave uses the "PJ-2024-Inverse-Normal" algorithm and Householder iterations as described in "Let's Be Rational" by Peter Jäckel to ensure maximum accuracy and stability.

!!! info "Attribution"
    The Implied Volatility implementation in QuantWave is based on the reference implementation of the "Let's Be Rational" algorithm.
    **Copyright © 2013-2024 Peter Jäckel.**
    The source code resides at [www.jaeckel.org/LetsBeRational.7z](http://www.jaeckel.org/LetsBeRational.7z).

### Indian Market Conventions

- **Risk-Free Rate**: Hardcoded at 6.5% (NSE 91-day T-bill rate). Updated quarterly.
- **Time to Expiry**: Calculated using calendar days / 365.0, matching NSE retail trading expectations.
- **Lot Sizes**: Hardcoded for major indices (NIFTY=50, BANKNIFTY=15, etc.).
- **Theta**: Reported as decay per calendar day (negative sign for long positions).

## Usage

### Python

```python
import quantwave as qw
from datetime import date

# Basic Pricing
call_price = qw.bs_call_price(s=25000, k=25000, r=0.065, t=7/365, sigma=0.18)

# IV Solving
iv = qw.implied_vol(market_price=call_price, s=25000, k=25000, r=0.065, t=7/365, is_call=True)

# India Helpers
lot_size = qw.nse_lot_size("NIFTY") # returns 50
```

### Polars Integration

QuantWave exposes the full options surface as **vectorized Polars expressions** via
`quantwave.polars.options`. Black–Scholes price/greeks/IV run as native Rust
expression plugins; per-strike and lookup analytics (`strike_pcr`,
`synthetic_futures`, `gex_per_strike`, `moneyness`, `nse_lot_size`) are native
Polars expressions; whole-chain reductions (`max_pain`, `oi_zones`,
`gex_flip_strike`, `atm_straddle`) execute the exact core math in a single call
per chain. There are **no per-row Python loops** anywhere in the wrappers.

Greeks / IV (per-strike):

```python
import polars as pl
from quantwave.polars import options as opt_expr

df = pl.DataFrame({
    "ltp": [150.5, 200.0],
    "strike": [25000.0, 25100.0],
    "t_years": [7 / 365, 7 / 365],
})

df = df.with_columns(
    # signature: implied_vol(price_col, spot, strike_col, r_col_or_val, t_col_or_val, is_call=True)
    opt_expr.implied_vol("ltp", 25050.0, "strike", 0.065, "t_years").alias("iv")
)
```

Chain analytics (apply per expiry snapshot, or inside `group_by(expiry)`):

```python
chain = pl.DataFrame({
    "strike": [24800.0, 25000.0, 25200.0],
    "ce_oi": [10000, 5000, 2000],
    "pe_oi": [2000, 5000, 10000],
    "ce_ltp": [250.0, 100.0, 20.0],
    "pe_ltp": [10.0, 60.0, 180.0],
})

chain.select(
    opt_expr.max_pain("strike", "ce_oi", "pe_oi", 50).alias("max_pain"),
    opt_expr.chain_pcr("ce_oi", "pe_oi").alias("pcr"),
)
chain.with_columns(
    opt_expr.strike_pcr("ce_oi", "pe_oi").alias("strike_pcr"),
    opt_expr.synthetic_futures("strike", "ce_ltp", "pe_ltp").alias("syn_fut"),
    opt_expr.moneyness(25000.0, "strike").alias("moneyness"),
)
```

Every method carries a full docstring (parameters, conventions, and a runnable
`Examples` block verified by `--doctest-modules` in CI).
