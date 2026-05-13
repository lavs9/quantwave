# Continuation Index

An oscillator that identifies trend onset and exhaustion by comparing a fast UltimateSmoother with a Generalized Laguerre filter.

## Parameters

- `gamma` (default: 0.8): Laguerre gamma parameter
- `order` (default: 8): Laguerre filter order
- `length` (default: 40): Base smoothing length

## Formula


\[
US = UltimateSmoother(Close, Length/2)
\]
\[
LG = Laguerre(Close, \gamma, Order, Length)
\]
\[
Variance = SMA(|US - LG|, Length)
\]
\[
Ref = 2 \times (US - LG) / Variance
\]
\[
CI = \tanh(Ref)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html)
