# PairsRotation

Relative rotation of two securities using normalized roofing filters.

## Parameters

- `hp_len` (default: 125): HighPass filter length
- `lp_len` (default: 20): LowPass (SuperSmoother) length

## Formula


\[
Filt = SuperSmoother(HighPass(Price, HPLen), LPLen)
\]
\[
MS = 0.0242 \cdot Filt^2 + 0.9758 \cdot MS_{t-1}
\]
\[
Normalized = \frac{Filt}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PAIRS%20ROTATION.pdf)
