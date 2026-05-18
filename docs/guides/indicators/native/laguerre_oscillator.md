# Laguerre Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">laguerre</span> <span class="kw-badge">momentum</span></div>

A low-lag trend oscillator derived from Laguerre polynomials and normalized by RMS volatility.

## Usage

Use to detect overbought and oversold conditions with very low lag. The single gamma parameter lets you tune it from aggressive to smooth.

## Background

> Ehlers describes the Laguerre Oscillator in Cybernetic Analysis as measuring the difference between the first and last elements of a 4-element Laguerre filter bank, extracting the high-frequency component as a zero-lag momentum measure.

## Parameters

- `length` (default: 30): UltimateSmoother period
- `gamma` (default: 0.5): Smoothing factor
- `rms_period` (default: 100): RMS normalization period

## Formula


\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_0 + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
RMS = \sqrt{\frac{1}{n}\sum (L_0 - L_1)^2}
\]
\[
Osc = (L_0 - L_1) / RMS
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html)
