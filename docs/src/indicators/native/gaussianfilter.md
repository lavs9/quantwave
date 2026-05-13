# GaussianFilter

Multi-pole Gaussian low-pass filter for reduced lag.

## Parameters

- `period` (default: 14): Critical period
- `poles` (default: 4): Number of poles (1-4)

## Formula


\[
\alpha = -\beta + \sqrt{\beta^2 + 2\beta}
\]
\[
\beta = \frac{1 - \cos(2\pi/P)}{2^{1/(2N)} - 1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/GaussianFilters.pdf)
