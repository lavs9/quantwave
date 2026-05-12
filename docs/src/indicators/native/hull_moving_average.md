# Hull Moving Average

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
HMA = WMA(2 \times WMA(\frac{n}{2}) - WMA(n), \sqrt{n})
\]


[Source](https://alanhull.com/hull-moving-average)
