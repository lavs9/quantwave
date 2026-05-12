# Zero Lag EC

Zero Lag Error Corrected EMA attempts to eliminate lag by adding an error term to the EMA.

## Parameters

- `length` (default: 20): Equivalent SMA length
- `gain_limit` (default: 50.0): Gain limit (divided by 10 for actual gain)

## Formula


\[
\alpha = \frac{2}{Length + 1}
\]
\[
EMA = \alpha \times Close + (1 - \alpha) \times EMA_{t-1}
\]
\[
EC = \alpha \times (EMA + Gain \times (Close - EC_{t-1})) + (1 - \alpha) \times EC_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/ZeroLag.pdf)
