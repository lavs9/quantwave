# FourierSeriesModel

Synthesized market model using fundamental and harmonic frequency components.

## Parameters

- `fundamental` (default: 20): Fundamental cycle period

## Formula


\[
BP_k = \text{BandPass}(Price, Fundamental/k)
\]
\[
Q_k = \frac{Fundamental}{2\pi} (BP_{k} - BP_{k,t-1})
\]
\[
P_k = \sum_{n=0}^{F-1} (BP_{k,t-n}^2 + Q_{k,t-n}^2)
\]
\[
Wave = BP_1 + \sqrt{P_2/P_1}BP_2 + \sqrt{P_3/P_1}BP_3
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FOURIER%20SERIES%20MODEL%20OF%20THE%20MARKET.pdf)
