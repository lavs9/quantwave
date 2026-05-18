# Moving Average Convergence Divergence (MACD)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span></div>

A trend-following momentum indicator that shows the relationship between two moving averages.

## Usage

Use to identify trend direction and momentum. Crossovers of the MACD line and signal line provide entry and exit signals, while the histogram shows the strength of the trend.

## Background

> Gerald Appel developed the MACD in the late 1970s. It is calculated by subtracting the 26-period EMA from the 12-period EMA. A nine-day EMA of the MACD, called the 'signal line,' is then plotted on top of the MACD line, which can function as a trigger for buy and sell signals. — Investopedia

## Parameters

- `fastperiod` (default: 12): Fast EMA period
- `slowperiod` (default: 26): Slow EMA period
- `signalperiod` (default: 9): Signal EMA period

## Formula


\[
MACD = EMA(12) - EMA(26) \\ Signal = EMA(MACD, 9)
\]


[Source](https://www.investopedia.com/terms/m/macd.asp)
