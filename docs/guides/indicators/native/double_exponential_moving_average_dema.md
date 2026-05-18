# Double Exponential Moving Average (DEMA)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">lag-reduction</span> <span class="kw-badge">classic</span></div>

A fast-acting moving average that reduces lag by using two exponential moving averages.

## Usage

Use as a replacement for EMA when faster signal generation is required without excessive noise. DEMA reacts more quickly to price changes than a standard EMA.

## Background

> Developed by Patrick Mulloy in 1994, DEMA provides a less-laggy alternative to traditional moving averages. It is calculated by taking a single EMA and then subtracting it from a double EMA of the same period. This effectively cancels out some of the lag inherent in the EMA calculation. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 30): Smoothing period

## Formula


\[
DEMA = 2 \times EMA - EMA(EMA)
\]


[Source](https://www.investopedia.com/terms/d/double-exponential-moving-average.asp)
