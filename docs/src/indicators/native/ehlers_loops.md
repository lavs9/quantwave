# Ehlers Loops

Converts price and volume into normalized standard deviation units for scatter plot analysis.

## Parameters

- `lp_period` (default: 20): Low-pass filter period (SuperSmoother)
- `hp_period` (default: 125): High-pass filter period (Butterworth)

## Formula


\[
HP = c_1 (Price - 2 Price_{t-1} + Price_{t-2}) + c_2 HP_{t-1} + c_3 HP_{t-2}
\]
\[
SS = s_1 \frac{HP + HP_{t-1}}{2} + s_2 SS_{t-1} + s_3 SS_{t-2}
\]
\[
MS = \alpha SS^2 + (1 - \alpha) MS_{t-1}
\]
\[
RMS = \frac{SS}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202022.html)
