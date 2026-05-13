# Noise Elimination Technology

Nonlinear noise removal using Kendall correlation against a straight line.

## Parameters

- `length` (default: 14): Correlation length

## Formula


\[
Num = \sum_{i=1}^{N-1} \sum_{j=0}^{i-1} -sgn(X_i - X_j)
\]
\[
Denom = \frac{N(N-1)}{2}
\]
\[
NET = \frac{Num}{Denom}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf)
