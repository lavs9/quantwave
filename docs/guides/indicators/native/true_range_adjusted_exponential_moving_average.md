# True Range Adjusted Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Moving Averages</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">true-range</span> <span class="kw-badge">volatility</span></div>

An exponential moving average that incorporates true range to measure volatility and adapt to price movements.

## Usage

Use to identify trend turning points and filter price movements. Comparing TRAdj EMA with a standard EMA of the same length provides insights into the overall trend.

## Background

> Introduced by Vitali Apirine in TASC January 2023, TRAdj EMA modifies the standard exponential moving average by adjusting the smoothing factor using the True Range. The normalized true range modifies the rate, making the indicator more responsive during volatile periods while filtering out noise when volatility drops.

## Parameters

- `period` (default: 40): Smoothing period
- `pds` (default: 40): Lookback period for True Range
- `mltp` (default: 10.0): Multiplier

## Formula


\[
TRAdj = \frac{TR - TR_{min}}{TR_{max} - TR_{min}} \\ Rate = \frac{2}{P+1} \times (1 + TRAdj \times Multiplier)
\]


[Source](Technical Analysis of Stocks & Commodities, January 2023)
