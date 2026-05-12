# SuperSmoother

A second-order IIR filter with a maximally flat Butterworth response for superior smoothing with minimal lag.

## Parameters

- `period` (default: 20): Critical period (wavelength)

## Formula


\[
a_1 = \exp\left(-\frac{1.414\pi}{Period}\right)
\]
\[
c_2 = 2a_1 \cos\left(\frac{1.414\pi}{Period}\right)
\]
\[
c_3 = -a_1^2
\]
\[
c_1 = 1 - c_2 - c_3
\]
\[
SS = c_1 \frac{Price + Price_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf)
