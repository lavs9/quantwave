# MAD

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">volatility</span> <span class="kw-badge">statistics</span> <span class="kw-badge">robust</span> <span class="kw-badge">ehlers</span></div>

Moving Average Difference: 100 * (SMA(short) - SMA(long)) / SMA(long)

## Usage

Use as a robust volatility measure when outliers or fat-tailed distributions would distort standard deviation. Works well for position sizing and volatility-based stop placement.

## Background

> Mean Absolute Deviation measures dispersion as the average absolute difference from the median rather than the squared difference from the mean used by standard deviation. It is less sensitive to outliers, making it a more robust volatility estimate for financial time series with fat tails.

## Parameters

- `short_period` (default: 8): Short-term SMA period
- `long_period` (default: 23): Long-term SMA period

## Formula


\[
MAD = 100 \times \frac{SMA(short) - SMA(long)}{SMA(long)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - OCTOBER 2021.html)
