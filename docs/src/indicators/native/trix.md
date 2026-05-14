# TRIX

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

A momentum oscillator that shows the percent rate of change of a triple exponentially smoothed moving average.

## Usage

Use to filter out market noise and identify trend reversals. TRIX crossings of the zero line or a signal line can provide trade entries.

## Background

> Developed by Jack Hutson in the early 1980s, TRIX is a powerful momentum oscillator that effectively filters out minor price fluctuations. By triple-smoothing an EMA, it emphasizes the underlying trend and provides a clear signal when the trend changes direction. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 15): Smoothing period

## Formula


\[
TRIX = \frac{EMA3_t - EMA3_{t-1}}{EMA3_{t-1}} \times 100
\]


[Source](https://www.investopedia.com/terms/t/trix.asp)
