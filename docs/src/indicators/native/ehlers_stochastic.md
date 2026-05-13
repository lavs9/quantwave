# Ehlers Stochastic

A Stochastic oscillator applied to the output of a Roofing Filter to eliminate Spectral Dilation.

## Parameters

- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period
- `stoch_period` (default: 20): Stochastic lookback period

## Formula


\[
Roof = RoofingFilter(HP, SS)
\]
\[
Stoch = 100 \times \frac{Roof - \min(Roof, L)}{\max(Roof, L) - \min(Roof, L)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating Turning Points.pdf)
