# Average Directional Index (ADX)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">volatility</span> <span class="kw-badge">classic</span> <span class="kw-badge">wilder</span></div>

An indicator used to quantify trend strength without regard to trend direction.

## Usage

Use to determine if the market is trending or ranging. ADX values above 25 indicate a strong trend, while values below 20 indicate a weak or non-trending market.

## Background

> Developed by J. Welles Wilder, the ADX is derived from two other indicators, also developed by Wilder: the Positive Directional Indicator (+DI) and the Negative Directional Indicator (-DI). While +DI and -DI indicate trend direction, ADX measures the strength of that trend. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
ADX = 100 \times \frac{\text{EMA}(|(+DI) - (-DI)| / |(+DI) + (-DI)|, n)}{n}
\]


[Source](https://www.investopedia.com/terms/a/adx.asp)
