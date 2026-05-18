# Adaptive Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Moving Averages</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span></div>

An adaptive moving average that adjusts its smoothing factor based on volatility.

## Usage

Use to identify overall trends. AEMA reacts faster to large price movements by adapting the smoothing factor using the highest high and lowest low of a lookback period.

## Background

> Introduced by Vitali Apirine in TASC April 2019, AEMA alters the EMA's alpha (smoothing factor) by comparing the distance of the close from the lowest low and highest high. This amplifies the smoothing factor during strong price moves while reducing it during sideways chop, yielding a moving average with less lag when it matters most.

## Parameters

- `period` (default: 10): Smoothing period
- `pds` (default: 10): Lookback period for volatility

## Formula


\[
Rate = \frac{2}{P+1} \times \left(1 + \frac{|(C - L_{min}) - (H_{max} - C)|}{H_{max} - L_{min}}\right) \\ AEMA_t = AEMA_{t-1} + Rate \times (C - AEMA_{t-1})
\]


[Source](Technical Analysis of Stocks & Commodities, April 2019)
