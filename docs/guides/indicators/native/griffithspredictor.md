# GriffithsPredictor

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">prediction</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

Adaptive LMS linear predictive filter for signal forecasting.

## Usage

Use for short-horizon price prediction by projecting the dominant market cycle one or two bars forward. Works best in oscillating markets; disable in strong trends.

## Background

> The Griffiths Predictor uses autoregressive coefficients from the Griffiths cycle measurement to extrapolate the current dominant cycle one bar ahead. By fitting an AR model to cycle-filtered price, it generates a one-step-ahead forecast useful for anticipatory entries at predicted cycle turns.

## Parameters

- `lower_bound` (default: 18): Lower frequency bound (SS length)
- `upper_bound` (default: 40): Upper frequency bound (HP length)
- `length` (default: 18): LMS filter length
- `bars_fwd` (default: 2): Number of bars to predict forward

## Formula


\[
\mu = 1/L
\]
\[
\bar{x} = \sum_{i=1}^L xx_{L-i} \cdot coef_i
\]
\[
coef_i = coef_i + \mu(xx_L - \bar{x})xx_{L-i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
