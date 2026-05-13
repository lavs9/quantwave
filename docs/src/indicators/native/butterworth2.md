# Butterworth2

2-pole Butterworth low-pass filter.

## Parameters

- `period` (default: 14): Critical period

## Formula


\[
a = \exp(-1.414\pi/P)
\]
\[
b = 2a \cos(1.414\pi/P)
\]
\[
f = bf_{t-1} - a^2f_{t-2} + \frac{1-b+a^2}{4}(g + 2g_{t-1} + g_{t-2})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf)
