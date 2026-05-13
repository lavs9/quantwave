# Reversion Index

A mean-reversion oscillator that normalizes price changes by their absolute magnitude and applies SuperSmoother filtering.

## Parameters

- `length` (default: 20): Summation period (approx. half dominant cycle)

## Formula


\[
\Delta_t = \text{Close}_t - \text{Close}_{t-1}
\]
\[
\text{Ratio} = \frac{\sum_{i=0}^{L-1} \Delta_{t-i}}{\sum_{i=0}^{L-1} |\Delta_{t-i}|}
\]
\[
\text{Smooth} = SuperSmoother(\text{Ratio}, 8)
\]
\[
\text{Trigger} = SuperSmoother(\text{Ratio}, 4)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JANUARY%202026.html)
