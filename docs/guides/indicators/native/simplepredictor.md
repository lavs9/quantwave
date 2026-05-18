# SimplePredictor

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">prediction</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

A fixed-coefficient 2-pole linear predictive filter.

## Usage

Use as a lightweight one-bar-ahead price predictor for cycle-mode markets. Its low computational cost makes it suitable for real-time streaming at high frequency.

## Background

> Ehlers derives a Simple Predictor that extrapolates price one bar forward using only the current and prior bars weighted by the dominant cycle coefficient. Despite its simplicity it provides useful one-bar forecasts in cycling markets, demonstrating the predictive value of cycle measurement.

## Parameters

- `hp_len` (default: 15): HighPass filter length
- `lp_len` (default: 30): LowPass (SuperSmoother) length
- `q` (default: 0.35): Damping/Predictor coefficient

## Formula


\[
Predict = \frac{Signal - 1.8Q \cdot Signal_{t-1} + Q^2 \cdot Signal_{t-2}}{1 - 1.8Q + Q^2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
