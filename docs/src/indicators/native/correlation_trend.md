# Correlation Trend

Calculates the Pearson correlation between price and a linear time ramp to identify trends.

## Parameters

- `length` (default: 20): Correlation window length

## Formula


\[
X_i = Price_{t-i}, Y_i = -i
\]
\[
R = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{(n \sum X_i^2 - (\sum X_i)^2)(n \sum Y_i^2 - (\sum Y_i)^2)}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/CORRELATION%20AS%20A%20TREND%20INDICATOR.pdf)
