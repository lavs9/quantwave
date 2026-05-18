# Hull Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

## Usage

Use as a near-zero-lag moving average for trend-following systems where entry timing is critical. The HMA substantially reduces the lag of a same-period WMA.

## Background

> Alan Hull designed the Hull Moving Average to nearly eliminate lag while maintaining smoothness. It achieves this by computing a WMA of doubled period, subtracting a WMA of full period, then applying a final WMA to the difference over the square-root period, combining speed with noise reduction. — AlanHull.com

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
HMA = WMA(2 \times WMA(\frac{n}{2}) - WMA(n), \sqrt{n})
\]


[Source](https://alanhull.com/hull-moving-average)
