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

QuantWave provides a helper for vectorized option analytics in Polars DataFrames.

```python
import polars as pl
from quantwave.polars import options as opt_expr

df = pl.DataFrame({
    "ltp": [150.5, 200.0],
    "strike": [25000, 25100],
    "t_years": [7/365, 7/365]
})

df = df.with_columns([
    opt_expr.implied_vol("ltp", spot=25050, strike_col="strike", r=0.065, t_col="t_years").alias("iv")
])
```
