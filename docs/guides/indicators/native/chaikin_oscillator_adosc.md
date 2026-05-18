# Chaikin Oscillator (ADOSC)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volume</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span></div>

An indicator that measures the momentum of the Accumulation/Distribution Line using the difference between two exponential moving averages.

## Usage

Use to anticipate changes in the AD Line. Positive values indicate increasing buying pressure, while negative values indicate increasing selling pressure.

## Background

> Marc Chaikin developed this oscillator to identify momentum shifts in the AD Line. By applying EMAs of different lengths to the AD Line, it highlights changes in money flow before they become apparent in the cumulative total, providing an early warning system for trend exhaustion. — StockCharts ChartSchool

## Parameters

- `fastperiod` (default: 3): Fast EMA period
- `slowperiod` (default: 10): Slow EMA period

## Formula


\[
ADOSC = EMA(AD, 3) - EMA(AD, 10)
\]


[Source](https://www.investopedia.com/terms/c/chaikinoscillator.asp)
