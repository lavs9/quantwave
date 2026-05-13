# FisherHighPass

Fisher Transform applied to normalized HighPass filtered prices.

## Parameters

- `hp_len` (default: 20): HighPass filter length
- `norm_len` (default: 20): Normalization lookback period

## Formula


\[
HP = \text{HighPass}(Price, hp\_len)
\]
\[
N = 2 \cdot \frac{HP - Low(HP, norm\_len)}{High(HP, norm\_len) - Low(HP, norm\_len)} - 1
\]
\[
S = \frac{N + N_{t-1} + N_{t-2}}{3}
\]
\[
Fisher = 0.5 \cdot \ln\left(\frac{1+S}{1-S}\right)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf)
