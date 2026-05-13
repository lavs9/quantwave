# Swiss Army Knife Indicator

A versatile indicator that can be configured as EMA, SMA, Gaussian, Butterworth, High Pass, Band Pass, or Band Stop filter.

## Parameters

- `mode` (default: BandPass): Filter mode (EMA, SMA, Gauss, Butter, Smooth, HP, 2PHP, BP, BS)
- `period` (default: 20): Filter period
- `delta` (default: 0.1): Bandwidth parameter for BP and BS modes

## Formula


\[
Filt = c_0(b_0 x_t + b_1 x_{t-1} + b_2 x_{t-2}) + a_1 Filt_{t-1} + a_2 Filt_{t-2} - c_1 x_{t-N}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/SwissArmyKnifeIndicator.pdf)
