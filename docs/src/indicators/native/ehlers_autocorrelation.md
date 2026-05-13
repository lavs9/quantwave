# Ehlers Autocorrelation

Computes Pearson correlation of smoothed price with its lags to identify market structure.

## Parameters

- `length` (default: 20): Correlation window length
- `num_lags` (default: 100): Number of lags to compute

## Formula


\[
\rho(lag) = \frac{N \sum X Y - \sum X \sum Y}{\sqrt{(N \sum X^2 - (\sum X)^2)(N \sum Y^2 - (\sum Y)^2)}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - FEBRUARY 2025.html)
