# Ehlers Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span></div>

A non-linear FIR filter using distance coefficients to adapt to price transitions while maintaining smoothness.

## Usage

Use as a configurable digital filter from Ehlers DSP toolkit when you need a specific frequency response not covered by the standard smoother or Butterworth designs.

## Background

> The Ehlers Filter is a generalized IIR filter design drawn from Ehlers digital signal processing framework for markets. Its coefficients can be tuned to approximate different filter types (lowpass, highpass, bandpass), making it a flexible building block for custom indicator pipelines.

## Parameters

- `length` (default: 15): Filter window length

## Formula


\[
C_i = \sum_{j=1}^{L-1} (Price_{t-i} - Price_{t-i-j})^2
\]
\[
Filt = \frac{\sum_{i=0}^{L-1} C_i Price_{t-i}}{\sum_{i=0}^{L-1} C_i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EhlersFilters.pdf)
