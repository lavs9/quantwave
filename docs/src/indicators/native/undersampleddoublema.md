# UndersampledDoubleMA

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">internal</span> <span class="kw-badge">utility</span></div>

Undersampled price data smoothed by dual Hann filters to eliminate high frequency noise.

## Usage

Internal implementation module — not intended as a standalone trading indicator.

## Background

> This module contains internal utility functions used by other indicators in the library. It is not intended to be used directly as a standalone trading indicator.

## Parameters

- `fast_len` (default: 6): Fast Hann filter length
- `slow_len` (default: 12): Slow Hann filter length
- `sampling_period` (default: 5): Undersampling rate (bars)

## Formula


\[
Sample = \begin{cases} Price & \text{if } t \pmod N = 0 \\ Sample_{t-1} & \text{otherwise} \end{cases}
\]
\[
Fast = Hann(Sample, FastLen)
\]
\[
Slow = Hann(Sample, SlowLen)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf)
