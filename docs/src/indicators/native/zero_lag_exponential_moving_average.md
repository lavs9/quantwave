# Zero Lag Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">ema</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

ZLEMA attempts to eliminate the inherent lag associated with moving averages.

## Usage

Use to reduce the lag of a standard EMA by approximately two thirds. Drop-in replacement for EMA in trend-following systems where responsiveness is more important than smoothness.

## Background

> Patrick Mulloy introduced Triple EMA in Technical Analysis of Stocks and Commodities (1994) as a practical lag-reduction technique. TEMA = 3*EMA - 3*EMA(EMA) + EMA(EMA(EMA)), subtracting out two orders of the EMA lag while preserving most of the noise reduction.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
ZLEMA = EMA(Price + (Price - Price_{t - (period - 1)/2}))
\]


[Source](https://en.wikipedia.org/wiki/Zero_lag_exponential_moving_average)
