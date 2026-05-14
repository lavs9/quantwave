# Triangular Moving Average (TRIMA)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

A double-smoothed simple moving average that gives more weight to the middle of the lookback period.

## Usage

Use for extremely smooth trend identification. TRIMA is significantly smoother than a standard SMA but introduces more lag; it is ideal for identifying long-term cycles.

## Background

> The Triangular Moving Average is an SMA of an SMA. For a period N, it averages the values over N/2 periods twice. This results in a weight distribution that is triangular, peaking at the center of the window, making it very effective at filtering out high-frequency noise. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 30): Smoothing period

## Formula


\[
TRIMA = SMA(SMA(Price, n/2), n/2)
\]


[Source](https://www.tradingview.com/support/solutions/43000591273-triangular-moving-average-tma/)
