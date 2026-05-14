# GriffithsDominantCycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">spectral</span></div>

Dominant cycle estimation using Griffiths adaptive spectral analysis.

## Usage

Use as a robust dominant cycle estimator less sensitive to amplitude changes than DFT-based methods, making it reliable across different market volatility regimes.

## Background

> The Griffiths method computes the dominant cycle by solving the real-roots of an autocorrelation polynomial. Adapted by Ehlers in Cycle Analytics for Traders, it remains stable even when market amplitude changes rapidly, unlike power-spectrum methods that can shift with volatility.

## Parameters

- `lower_bound` (default: 18): Lower period bound
- `upper_bound` (default: 40): Upper period bound
- `length` (default: 40): LMS filter length

## Formula


\[
Pwr(Period) = \frac{0.1}{(1-Real)^2 + Imag^2}
\]
\[
Real = \sum coef_i \cos(2\pi i / Period)
\]
\[
Imag = \sum coef_i \sin(2\pi i / Period)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html)
