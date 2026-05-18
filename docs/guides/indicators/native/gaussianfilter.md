# GaussianFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">low-pass</span></div>

Multi-pole Gaussian low-pass filter for reduced lag.

## Usage

Use when smooth symmetric price averaging with near-zero phase shift is needed. Works well as a preprocessing step for spectral analysis indicators.

## Background

> Gaussian filters are the theoretically optimal lowpass filter for minimizing the product of time-domain duration and frequency-domain bandwidth. Ehlers implements them as cascaded pole filters with Gaussian-function-derived coefficients, achieving very smooth output with excellent stopband attenuation.

## Parameters

- `period` (default: 14): Critical period
- `poles` (default: 4): Number of poles (1-4)

## Formula


\[
\alpha = -\beta + \sqrt{\beta^2 + 2\beta}
\]
\[
\beta = \frac{1 - \cos(2\pi/P)}{2^{1/(2N)} - 1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/GaussianFilters.pdf)
