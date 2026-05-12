# MESA Adaptive Moving Average

MAMA adapts to price movement in an entirely new and unique way based on the rate change of phase.

## Parameters

- `fast_limit` (default: 0.5): Fast limit for alpha
- `slow_limit` (default: 0.05): Slow limit for alpha

## Formula


\[
\text{MAMA} = \alpha \cdot \text{Price} + (1 - \alpha) \cdot \text{MAMA}_{1}
\]
\[
\text{FAMA} = 0.5\alpha \cdot \text{MAMA} + (1 - 0.5\alpha) \cdot \text{FAMA}_{1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/MAMA.pdf)
