# MESA Stochastic

Standard Stochastic calculation applied to Roofing Filtered data, followed by SuperSmoothing.

## Parameters

- `length` (default: 20): Stochastic lookback length
- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period

## Formula


\[
Filt = \text{RoofingFilter}(Price, P_{hp}, P_{ss})
\]
\[
Stoc = \frac{Filt - \min(Filt, L)}{\max(Filt, L) - \min(Filt, L)}
\]
\[
MESAStoch = \text{SuperSmoother}(Stoc \times 100, P_{ss})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating%20Turning%20Points.pdf)
