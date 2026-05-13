# Laguerre Filter

A trend-following filter that excels at smoothing long-wavelength components using Laguerre polynomials and an UltimateSmoother base.

## Parameters

- `length` (default: 40): UltimateSmoother period
- `gamma` (default: 0.8): Smoothing factor (0.0 to 1.0)

## Formula


\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_{0,t-1} + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
...
\]
\[
Laguerre = (L_0 + 4L_1 + 6L_2 + 4L_3 + L_5) / 16
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html)
