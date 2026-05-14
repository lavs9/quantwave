# GriffithsSpectrum

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">spectrum</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">periodogram</span></div>

Normalized power spectrum estimation using Griffiths adaptive filters.

## Usage

Use to generate a high-resolution periodogram for cycle analysis. Best visualized as a heatmap to identify and track multiple market cycles simultaneously.

## Background

> The Griffiths Spectrum is an adaptive spectral estimation method that provides higher resolution than a standard DFT for short data segments. It fits an all-pole model to the signal using an LMS algorithm, allowing for instantaneous frequency measurement without the windowing artifacts of FFT-based methods.

## Parameters

- `lower_bound` (default: 18): Lower period bound
- `upper_bound` (default: 40): Upper period bound
- `length` (default: 40): LMS filter length

## Formula


\[
Pwr(P) = \frac{0.1}{(1 - \sum coef_i \cos(2\pi i/P))^2 + (\sum coef_i \sin(2\pi i/P))^2}
\]
\[
Pwr_{norm}(P) = \frac{Pwr(P)}{\max(Pwr)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
