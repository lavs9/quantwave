# Rate of Directional Change

<div class="indicator-meta"><span class="category-badge">Volatility</span> <span class="kw-badge">zigzag</span> <span class="kw-badge">whipsaw</span> <span class="kw-badge">momentum</span> <span class="kw-badge">volatility</span> <span class="kw-badge">directional change</span></div>

Measures the frequency of directional changes (zigzag flips) within a moving window to identify whipsaw market conditions.

## Usage

Use to filter out false signals in trend-following strategies. High RODC values indicate a whipsaw environment, while low values suggest a trending market.

## Background

> RODC tracks the number of alternating up and down zigzag segments within a fixed window. By normalizing this count and smoothing it, the indicator provides a measure of how 'noisy' the price action is. It declines in trending environments and increases during whipsaws. — Richard Poster, TASC March 2024

## Parameters

- `window_size` (default: 30): Lookback window for zigzag calculation
- `threshold` (default: 0.0015): Zigzag reversal threshold (absolute price change)
- `smooth_period` (default: 3): Smoothing period for the resulting rate

## Formula


\[
RODC = SMA(100 \times \frac{NumUD}{WindowSize}, SmoothPeriod)
\]


[Source](TASC March 2024)
