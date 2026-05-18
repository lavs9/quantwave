# Ehlers Autocorrelation

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Computes Pearson correlation of smoothed price with its lags to identify market structure.

## Usage

Use to generate an autocorrelation periodogram showing which cycle periods are currently dominant. Visualise as a heatmap to track cycle period shifts over time.

## Background

> Ehlers introduces autocorrelation-based cycle measurement in Cycle Analytics for Traders (2013) as a more robust alternative to DFT. By computing autocorrelation of Roofing-filtered price at each lag, then applying a spectral DFT to the lag series, he obtains a periodogram insensitive to amplitude variations.

## Parameters

- `length` (default: 20): Correlation window length
- `num_lags` (default: 100): Number of lags to compute

## Formula


\[
\rho(lag) = \frac{N \sum X Y - \sum X \sum Y}{\sqrt{(N \sum X^2 - (\sum X)^2)(N \sum Y^2 - (\sum Y)^2)}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - FEBRUARY 2025.html)
