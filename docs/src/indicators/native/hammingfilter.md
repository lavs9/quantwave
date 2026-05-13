# HammingFilter

Hamming windowed FIR filter with pedestal.

## Parameters

- `length` (default: 20): Filter length
- `pedestal` (default: 10.0): Pedestal in degrees

## Formula


\[
Deg(n) = Pedestal + (180 - 2 \times Pedestal) \times \frac{n}{L-1}
\]
\[
Coef(n) = \sin\left(\frac{Deg(n) \times \pi}{180}\right)
\]
\[
Filt = \frac{\sum_{n=0}^{L-1} Coef(n) \cdot Price_{t-n}}{\sum Coef(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html)
