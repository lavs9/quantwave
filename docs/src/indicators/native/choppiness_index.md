# Choppiness Index

<div class="indicator-meta"><span class="category-badge">Modern</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend-strength</span> <span class="kw-badge">classic</span> <span class="kw-badge">range</span></div>

Determines if the market is trending (low values) or ranging/choppy (high values).

## Usage

Use to determine whether a market is trending or choppy before selecting a trading strategy. Values above 61.8 indicate chop; values below 38.2 indicate a strong trend.

## Background

> The Choppiness Index, developed by E.W. Dreiss, measures how much of the total ATR-based range is consumed by the actual net price move over N bars. A value near 100 means price wandered back and forth using all available range without net progress (maximum chop); near 0 means a straight directional move with minimal retracement. — StockCharts ChartSchool

## Parameters

- `period` (default: 14): Lookback period

## Formula


\[
CHOP = 100 \times \frac{\log_{10}(\sum_{i=1}^n ATR(1)_i / (\max(H, n) - \min(L, n)))}{\log_{10}(n)}
\]


[Source](https://www.tradingview.com/support/solutions/43000501980-choppiness-index-chop/)
