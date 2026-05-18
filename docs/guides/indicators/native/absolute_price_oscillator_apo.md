# Absolute Price Oscillator (APO)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span></div>

Shows the absolute difference between two moving averages of different periods.

## Usage

Use to identify trend crossovers and momentum. It is essentially a MACD without the signal line, showing the raw distance between fast and slow averages.

## Background

> The Absolute Price Oscillator (APO) is based on the difference between two exponential moving averages. It is a trend-following indicator that signals a change in direction when the fast EMA crosses the slow EMA, providing a clear visual of trend development. — TA-Lib Documentation

## Parameters

- `fastperiod` (default: 12): Fast period
- `slowperiod` (default: 26): Slow period

## Formula


\[
APO = EMA(fast) - EMA(slow)
\]


[Source](https://www.tradingview.com/support/solutions/43000501826-absolute-price-oscillator-apo/)
