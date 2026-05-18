# Noise Elimination Technology

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">noise</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span></div>

Nonlinear noise removal using Kendall correlation against a straight line.

## Usage

Use as a pre-filter to remove spike noise from price or intermediate indicator data without introducing lag. Particularly useful when raw tick or 1-minute data is used.

## Background

> Ehlers Noise Elimination Technology (NET) is a nonlinear filter that removes isolated noise spikes while leaving genuine price moves intact. It works by comparing each bar to its neighbors and replacing outliers with interpolated values, achieving noise reduction without the lag of conventional smoothers.

## Parameters

- `length` (default: 14): Correlation length

## Formula


\[
Num = \sum_{i=1}^{N-1} \sum_{j=0}^{i-1} -sgn(X_i - X_j)
\]
\[
Denom = \frac{N(N-1)}{2}
\]
\[
NET = \frac{Num}{Denom}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf)
