# HannFilter

Hann windowed lowpass FIR filter.

## Parameters

- `length` (default: 20): Filter length

## Formula


\[
H(n) = 1 - \cos\left(\frac{2\pi n}{L+1}\right)
\]
\[
Filt = \frac{\sum_{n=1}^L H(n) \cdot Price_{t-n+1}}{\sum H(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf)
