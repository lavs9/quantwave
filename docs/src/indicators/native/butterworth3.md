# Butterworth3

3-pole Butterworth low-pass filter.

## Parameters

- `period` (default: 14): Critical period

## Formula


\[
a = \exp(-\pi/P)
\]
\[
b = 2a \cos(1.738\pi/P)
\]
\[
c = a^2
\]
\[
f = (b+c)f_{t-1} - (c+bc)f_{t-2} + c^2f_{t-3} + \frac{(1-b+c)(1-c)}{8}(g + 3g_{t-1} + 3g_{t-2} + g_{t-3})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf)
