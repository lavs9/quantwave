# Ultimate Bands

A Bollinger-style band using UltimateSmoother for the center line and standard deviation of the price-smooth difference for width.

## Parameters

- `length` (default: 20): Smoothing and SD period
- `num_sds` (default: 1.0): Standard Deviation multiplier

## Formula


\[
Smooth = UltimateSmoother(Close, Length)
\]
\[
SD = \sqrt{\frac{1}{n}\sum_{i=0}^{n-1} (Close_{t-i} - Smooth_{t-i})^2}
\]
\[
Upper = Smooth + NumSDs \times SD
\]
\[
Lower = Smooth - NumSDs \times SD
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf)
