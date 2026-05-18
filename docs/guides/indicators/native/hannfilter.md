# HannFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">windowing</span> <span class="kw-badge">spectral</span></div>

Hann windowed lowpass FIR filter.

## Usage

Use as a windowing function before FFT-based dominant cycle measurement to achieve clean spectral separation between market cycles.

## Background

> The Hann window provides a smooth bell-shaped taper achieving -31.5 dB first sidelobe suppression. Ehlers uses it in Cycle Analytics for Traders as the preferred DFT window because it offers the best trade-off between frequency resolution and leakage rejection.

## Parameters

- `length` (default: 20): Filter length

## Formula


\[
H(n) = 1 - \cos\left(\frac{2\pi n}{L+1}\right)
\]
\[
Filt = \frac{\sum_{n=1}^L H(n) \cdot Price_{t-n+1}}{\sum H(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf)
