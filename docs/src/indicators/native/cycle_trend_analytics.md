# Cycle/Trend Analytics

A set of oscillators (Price - SMA) with lengths from 5 to 30 used to visualize cycles and trends.

## Parameters

- `min_length` (default: 5): Minimum SMA length
- `max_length` (default: 30): Maximum SMA length

## Formula


\[
Osc(L) = Price - SMA(Price, L) \quad \text{for } L \in [min, max]
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - OCTOBER 2021.html)
