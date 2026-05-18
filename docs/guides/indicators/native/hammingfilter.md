# HammingFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">windowing</span> <span class="kw-badge">spectral</span></div>

Hamming windowed FIR filter with pedestal.

## Usage

Apply as a windowing function before DFT-based cycle detection to reduce sidelobe leakage and obtain cleaner dominant cycle estimates.

## Background

> The Hamming window is a raised-cosine weighting function that reduces spectral leakage by tapering the edges of a data block. Ehlers uses it in DFT-based cycle measurement tools to prevent energy in one frequency bin from contaminating adjacent bins, improving cycle period resolution.

## Parameters

- `length` (default: 20): Filter length
- `pedestal` (default: 10.0): Pedestal in degrees

## Formula


\[
Deg(n) = Pedestal + (180 - 2 \times Pedestal) \times \frac{n}{L-1}
\]
\[
Coef(n) = \sin\left(\frac{Deg(n) \times \pi}{180}\right)
\]
\[
Filt = \frac{\sum_{n=0}^{L-1} Coef(n) \cdot Price_{t-n}}{\sum Coef(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html)
