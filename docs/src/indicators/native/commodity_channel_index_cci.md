# Commodity Channel Index (CCI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span> <span class="kw-badge">mean-reversion</span></div>

A versatile indicator that can be used to identify a new trend or warn of extreme conditions.

## Usage

Use to identify cyclical turns in commodities or stocks. Readings above +100 imply a strong uptrend, while readings below -100 imply a strong downtrend.

## Background

> Developed by Donald Lambert in 1980, the CCI measures the current price level relative to an average price level over a given period. CCI is relatively high when prices are far above their average and relatively low when prices are far below their average. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
CCI = \frac{Price - SMA}{0.015 \times \text{Mean Deviation}}
\]


[Source](https://www.investopedia.com/terms/c/commoditychannelindex.asp)
