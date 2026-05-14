# Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">ema</span></div>

The Exponential Moving Average gives more weight to recent prices.

## Usage

Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.

## Background

> The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
EMA = P_t \times \alpha + EMA_{t-1} \times (1 - \alpha)
\]


[Source](https://www.investopedia.com/terms/e/ema.asp)
