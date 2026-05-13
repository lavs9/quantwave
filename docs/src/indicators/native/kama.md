# KAMA

Kaufman's Adaptive Moving Average adjusts its sensitivity based on market volatility.

## Parameters

- `period` (default: 10): Efficiency Ratio lookback period
- `fast_period` (default: 2): Fastest smoothing period
- `slow_period` (default: 30): Slowest smoothing period

## Formula


\[
ER = \frac{|Price - Price_{t-n}|}{\sum |Price - Price_{t-1}|}
\]
\[
SC = [ER(FastSC - SlowSC) + SlowSC]^2
\]
\[
KAMA = KAMA_{t-1} + SC(Price - KAMA_{t-1})
\]


[Source](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:kaufman_s_adaptive_moving_average)
