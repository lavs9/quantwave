# TriangleFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">triangle</span></div>

Triangle windowed FIR filter.

## Usage

Use as a pre-smoother to reduce noise before applying cycle or momentum indicators when a symmetric low-ripple response is needed.

## Background

> The Triangle (Bartlett) window is a linearly-tapered FIR filter equivalent to applying two rectangular windows in sequence. It provides moderate sidelobe suppression and is useful when computational simplicity is preferred over maximum spectral attenuation.

## Parameters

- `length` (default: 20): Filter length

## Formula


\[
Coef(n) = \begin{cases} n & n < L/2 \\ L/2 & n = L/2 \\ L + 1 - n & n > L/2 \end{cases}
\]
\[
Filt = \frac{\sum_{n=1}^L Coef(n) \cdot Price_{t-n+1}}{\sum Coef(n)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html)
