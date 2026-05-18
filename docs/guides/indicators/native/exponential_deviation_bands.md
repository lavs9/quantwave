# Exponential Deviation Bands

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">bands</span> <span class="kw-badge">volatility</span> <span class="kw-badge">exponential-deviation</span> <span class="kw-badge">trend</span></div>

A price band indicator based on exponential deviation that applies more weight to recent data and generates fewer breakouts than standard deviation bands.

## Usage

Use as a tool to identify trends and potential trend reversals. Prices consistently above the upper band indicate a strong uptrend, while prices below the lower band indicate a strong downtrend.

## Background

> Introduced by Vitali Apirine, Exponential Deviation Bands use an EMA of the absolute deviation from a base moving average (SMA or EMA) to create volatility bands. This approach is more responsive to recent price changes than standard deviation-based Bollinger Bands.

## Parameters

- `period` (default: 20): Period for the base moving average and exponential deviation.
- `dev_mult` (default: 2.0): Multiplier for the exponential deviation.
- `use_sma` (default: false): Whether to use SMA (true) or EMA (false) as the base moving average.

## Formula


\[
BaseMA = \text{SMA or EMA}(Price, n) \\
Deviation = |BaseMA - Price| \\
ExpDev = EMA(Deviation, n) \\
Upper = BaseMA + ExpDev \times multiplier \\
Lower = BaseMA - ExpDev \times multiplier
\]


[Source](Technical Analysis of Stocks & Commodities, July 2019)
