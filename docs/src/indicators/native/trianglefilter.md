# TriangleFilter

Triangle windowed FIR filter.

## Parameters

- `length` (default: 20): Filter length

## Formula


\[
Coef(n) = \begin{cases} n & n < L/2 \\ L/2 & n = L/2 \\ L + 1 - n & n > L/2 \end{cases}
\]
\[
Filt = \frac{\sum_{n=1}^L Coef(n) \cdot Price_{t-n+1}}{\sum Coef(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html)
