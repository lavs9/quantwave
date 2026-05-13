# Center of Gravity Oscillator

The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

## Parameters

- `period` (default: 10): Observation window length

## Formula


\[
CG = -\frac{\sum_{i=0}^{N-1} (i+1) \times Price_i}{\sum_{i=0}^{N-1} Price_i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheCGOscillator.pdf)
