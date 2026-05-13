# GriffithsDominantCycle

Dominant cycle estimation using Griffiths adaptive spectral analysis.

## Parameters

- `lower_bound` (default: 18): Lower period bound
- `upper_bound` (default: 40): Upper period bound
- `length` (default: 40): LMS filter length

## Formula


\[
Pwr(Period) = \frac{0.1}{(1-Real)^2 + Imag^2}
\]
\[
Real = \sum coef_i \cos(2\pi i / Period)
\]
\[
Imag = \sum coef_i \sin(2\pi i / Period)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
