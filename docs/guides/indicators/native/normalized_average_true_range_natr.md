# Normalized Average True Range (NATR)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">atr</span> <span class="kw-badge">normalization</span> <span class="kw-badge">classic</span></div>

A normalized version of ATR that represents volatility as a percentage of price.

## Usage

Use to compare volatility across different securities with varying price levels. NATR allows for normalized risk assessment and position sizing.

## Background

> Normalized ATR (NATR) was developed to allow traders to compare the volatility of high-priced stocks with low-priced stocks. By dividing the ATR by the closing price and multiplying by 100, the result is a percentage that can be used consistently across all assets. — TA-Lib Documentation

## Parameters

- `timeperiod` (default: 14): Smoothing period

## Formula


\[
NATR = \frac{ATR(n)}{Close} \times 100
\]


[Source](https://www.tradingtechnologies.com/help/x-study/technical-indicator-definitions/normalized-average-true-range-natr/)
