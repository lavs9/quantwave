# Generalized Laguerre

A generalized Laguerre filter of arbitrary order using an UltimateSmoother as the primary component.

## Parameters

- `length` (default: 40): UltimateSmoother period
- `gamma` (default: 0.8): Smoothing factor (0.0 to 1.0)
- `order` (default: 8): Filter order (1 to 10)

## Formula


\[
LG_1 = UltimateSmoother(Price, Length)
\]
\[
LG_i = -\gamma LG_{i-1,t-1} + LG_{i-1,t-1} + \gamma LG_{i,t-1} \text{ for } i=2 \dots Order
\]
\[
Filter = \frac{1}{Order} \sum_{i=1}^{Order} LG_i
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html)
