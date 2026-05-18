# Aroon Indicator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

An indicator system that identifies when a new trend is beginning and the strength of the trend.

## Usage

Use to identify when a security is trending and when it is in a range-bound period. Aroon Up crossing above Aroon Down signals the start of a new uptrend.

## Background

> Developed by Tushar Chande in 1995, the Aroon indicator focuses on the time between highs and the time between lows over a given period. The idea is that strong uptrends will regularly see new highs, and strong downtrends will regularly see new lows. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 25): Lookback period

## Formula


\[
\text{Aroon Up} = \frac{n - \text{Periods since n-period High}}{n} \times 100
\]


[Source](https://www.investopedia.com/terms/a/aroon.asp)
