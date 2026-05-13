# SimplePredictor

A fixed-coefficient 2-pole linear predictive filter.

## Parameters

- `hp_len` (default: 15): HighPass filter length
- `lp_len` (default: 30): LowPass (SuperSmoother) length
- `q` (default: 0.35): Damping/Predictor coefficient

## Formula


\[
Predict = \frac{Signal - 1.8Q \cdot Signal_{t-1} + Q^2 \cdot Signal_{t-2}}{1 - 1.8Q + Q^2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
