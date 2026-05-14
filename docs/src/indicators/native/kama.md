# KAMA

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

Kaufman's Adaptive Moving Average adjusts its sensitivity based on market volatility.

## Usage

Use as an adaptive moving average that is fast in trending markets and slow in choppy, sideways conditions. Reduces whipsaws that plague fixed-period moving averages in ranging markets.

## Background

> Perry Kaufman designed KAMA using an Efficiency Ratio that measures how directionally price has moved versus total path length. A high ratio (strong trend) produces a fast-reacting EMA; a low ratio (choppy market) produces a near-flat line, dramatically reducing false signals during consolidation. — New Trading Systems and Methods, 4th ed.

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
