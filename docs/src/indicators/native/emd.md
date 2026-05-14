# EMD

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">decomposition</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">dsp</span></div>

Empirical Mode Decomposition separates cycles from trends using bandpass filtering and identifies market modes via adaptive thresholds.

## Usage

Use to decompose price into Intrinsic Mode Functions to separate cycles of different periods without any a priori period assumption. Useful for multi-timescale analysis.

## Background

> Empirical Mode Decomposition is a data-driven method developed by Huang et al. (1998) that decomposes a signal into Intrinsic Mode Functions by iteratively sifting local extrema. Unlike Fourier methods, it requires no predetermined basis functions, making it adaptive to non-stationary market data.

## Parameters

- `period` (default: 20): Bandpass center period
- `delta` (default: 0.5): Bandwidth half-width
- `fraction` (default: 0.1): Threshold multiplier for peaks/valleys

## Formula


\[
\beta = \cos\left(\frac{360}{P}\right), \gamma = \frac{1}{\cos\left(\frac{720\delta}{P}\right)}, \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]
\[
Mean = \text{SMA}(BP, 2P)
\]
\[
Threshold = \text{Fraction} \cdot \text{SMA}(\text{Peak/Valley}, 50)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf)
