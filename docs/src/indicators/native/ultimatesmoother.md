# UltimateSmoother

An Ehlers filter with zero lag in the Pass Band, constructed by subtracting High Pass response from the input data.

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
c_1 = (1 + c_2 - c_3) / 4
\]
\[
US = (1 - c_1) Price + (2c_1 - c_2) Price_{t-1} - (c_1 + c_3) Price_{t-2} + c_2 US_{t-1} + c_3 US_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf)
