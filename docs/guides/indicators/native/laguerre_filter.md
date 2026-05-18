# Laguerre Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">laguerre</span></div>

A trend-following filter that excels at smoothing long-wavelength components using Laguerre polynomials and an UltimateSmoother base.

## Usage

Use as a low-lag smoothing filter with only 4 elements of state. Ideal when memory-efficiency matters or when a highly responsive smoother for real-time streaming is needed.

## Background

> Ehlers introduces Laguerre filters in Cybernetic Analysis (2004), noting they achieve the response of much longer conventional filters using only four coefficients. The single gamma parameter controls the trade-off between lag and smoothness.

## Parameters

- `length` (default: 40): UltimateSmoother period
- `gamma` (default: 0.8): Smoothing factor (0.0 to 1.0)

## Formula


\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_{0,t-1} + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
...
\]
\[
Laguerre = (L_0 + 4L_1 + 6L_2 + 4L_3 + L_5) / 16
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html)
