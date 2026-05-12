# Fractal Adaptive Moving Average

An adaptive moving average that uses the fractal dimension of prices to dynamically change its smoothing constant.

## Parameters

- `length` (default: 16): Length (must be an even number; odd values will be incremented by 1).

## Formula


\[
D = \frac{\log(N_1 + N_2) - \log(N_3)}{\log(2)}
\]
\[
\alpha = \exp(-4.6 (D - 1))
\]
\[
\text{FRAMA}_t = \alpha P_t + (1 - \alpha) \text{FRAMA}_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/FRAMA.pdf)
