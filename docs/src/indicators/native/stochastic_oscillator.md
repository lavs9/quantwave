# Stochastic Oscillator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">classic</span></div>

A momentum indicator comparing a particular closing price of a security to a range of its prices over a certain period of time.

## Usage

Use to identify trend reversals by looking for crossovers and overbought/oversold levels. The %K and %D lines indicate when the momentum is shifting relative to the recent price range.

## Background

> George Lane developed the Stochastic Oscillator in the 1950s. It is based on the observation that in an uptrend, prices tend to close near their high, and in a downtrend, they tend to close near their low. The sensitivity of the oscillator to market movements is reducible by adjusting the time period or by taking a moving average of the result. — StockCharts ChartSchool

## Parameters

- `fastk_period` (default: 5): Fast %K period
- `slowk_period` (default: 3): Slow %K period
- `slowd_period` (default: 3): Slow %D period

## Formula


\[
\%K = 100 \times \frac{C - L14}{H14 - L14} \\ \%D = 3\text{-period SMA of } \%K
\]


[Source](https://www.investopedia.com/terms/s/stochasticoscillator.asp)
