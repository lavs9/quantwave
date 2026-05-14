# Simple Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">ema</span></div>

The Simple Moving Average calculates the unweighted mean of the previous N data points.

## Usage

Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.

## Background

> The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
SMA = \frac{1}{n} \sum_{i=1}^{n} P_i
\]


[Source](https://www.investopedia.com/terms/s/sma.asp)
