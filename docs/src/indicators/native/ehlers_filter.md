# Ehlers Filter

A non-linear FIR filter using distance coefficients to adapt to price transitions while maintaining smoothness.

## Parameters

- `length` (default: 15): Filter window length

## Formula


\[
C_i = \sum_{j=1}^{L-1} (Price_{t-i} - Price_{t-i-j})^2
\]
\[
Filt = \frac{\sum_{i=0}^{L-1} C_i Price_{t-i}}{\sum_{i=0}^{L-1} C_i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EhlersFilters.pdf)
