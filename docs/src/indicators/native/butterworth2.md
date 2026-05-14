# Butterworth2

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">low-pass</span></div>

2-pole Butterworth low-pass filter.

## Usage

Use to smooth price or intermediate indicator values with a flat passband and sharp rolloff. The 3-pole version provides steeper attenuation at the cost of marginally more lag.

## Background

> Butterworth filters are maximally flat in the passband, introducing no ripple. Ehlers implements 2-pole and 3-pole Butterworth IIR designs in Cycle Analytics for Traders, noting that the SuperSmoother is actually a critically-damped 2-pole Butterworth variant.

## Parameters

- `period` (default: 14): Critical period

## Formula


\[
a = \exp(-1.414\pi/P)
\]
\[
b = 2a \cos(1.414\pi/P)
\]
\[
f = bf_{t-1} - a^2f_{t-2} + \frac{1-b+a^2}{4}(g + 2g_{t-1} + g_{t-2})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf)
