# Weighted Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">ema</span></div>

The Weighted Moving Average assigns linearly decreasing weights.

## Usage

Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.

## Background

> The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
WMA = \frac{P_1 \times n + P_2 \times (n-1) + \dots}{n + (n-1) + \dots + 1}
\]


[Source](https://www.investopedia.com/articles/technical/060401.asp)
