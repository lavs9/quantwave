# Weighted Moving Average

The Weighted Moving Average assigns linearly decreasing weights.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
WMA = \frac{P_1 \times n + P_2 \times (n-1) + \dots}{n + (n-1) + \dots + 1}
\]


[Source](https://www.investopedia.com/articles/technical/060401.asp)
