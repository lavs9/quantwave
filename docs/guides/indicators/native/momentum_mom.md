# Momentum (MOM)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">trend</span></div>

A simple indicator that measures the amount that a security's price has changed over a given span of time.

## Usage

Use to measure the velocity of price changes. Positive values indicate an uptrend, while negative values indicate a downtrend.

## Background

> Momentum is one of the most basic and powerful concepts in technical analysis. It measures the rate of change of an asset's price, providing a clear indication of trend strength and potential exhaustion before the actual price reversal occurs. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 10): Lookback period

## Formula


\[
MOM = Price_t - Price_{t-n}
\]


[Source](https://www.investopedia.com/terms/m/momentum.asp)
