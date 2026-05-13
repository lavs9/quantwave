# BandPass

A bandpass filter that isolates cycle components around a center period.

## Parameters

- `period` (default: 20): Center period of the passband
- `bandwidth` (default: 0.1): Relative bandwidth (delta)

## Formula


\[
\beta = \cos(360/P), \gamma = 1/\cos(720\delta/P), \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf)
