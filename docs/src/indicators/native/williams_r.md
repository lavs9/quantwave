# Williams %R

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">classic</span></div>

A momentum indicator that measures overbought and oversold levels, similar to a stochastic oscillator.

## Usage

Use to identify entry and exit points in the market. Readings from 0 to -20 are considered overbought, while readings from -80 to -100 are considered oversold.

## Background

> Developed by Larry Williams, %R compares the closing price of a stock to the high-low range over a specific period, typically 14 days. It is used to determine when a stock might be overbought or oversold and to identify potential trend reversals. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
\%R = \frac{\text{Highest High} - \text{Close}}{\text{Highest High} - \text{Lowest Low}} \times -100
\]


[Source](https://www.investopedia.com/terms/w/williamsr.asp)
