# GriffithsPredictor

Adaptive LMS linear predictive filter for signal forecasting.

## Parameters

- `lower_bound` (default: 18): Lower frequency bound (SS length)
- `upper_bound` (default: 40): Upper frequency bound (HP length)
- `length` (default: 18): LMS filter length
- `bars_fwd` (default: 2): Number of bars to predict forward

## Formula


\[
\mu = 1/L
\]
\[
\bar{x} = \sum_{i=1}^L xx_{L-i} \cdot coef_i
\]
\[
coef_i = coef_i + \mu(xx_L - \bar{x})xx_{L-i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
