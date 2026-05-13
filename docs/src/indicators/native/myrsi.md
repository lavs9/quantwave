# MyRSI

Ehlers' version of RSI that swings between -1 and +1.

## Parameters

- `length` (default: 14): Smoothing length

## Formula


\[
CU = \sum_{i=0}^{length-1} \max(0, Price_i - Price_{i+1})
\]
\[
CD = \sum_{i=0}^{length-1} \max(0, Price_{i+1} - Price_i)
\]
\[
MyRSI = \frac{CU - CD}{CU + CD}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf)
