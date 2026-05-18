# Percentage Price Oscillator (PPO)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span> <span class="kw-badge">normalization</span></div>

A momentum oscillator that measures the difference between two moving averages as a percentage of the larger moving average.

## Usage

Use to compare trend momentum across different securities with varying price levels. PPO is the percentage version of MACD.

## Background

> The Percentage Price Oscillator (PPO) is identical to the MACD, except that it measures the difference between two moving averages as a percentage. This allows for comparison across different stocks regardless of their price, making it a superior tool for relative strength analysis. — StockCharts ChartSchool

## Parameters

- `fastperiod` (default: 12): Fast period
- `slowperiod` (default: 26): Slow period

## Formula


\[
PPO = \frac{EMA(12) - EMA(26)}{EMA(26)} \times 100
\]


[Source](https://www.investopedia.com/terms/p/ppo.asp)
