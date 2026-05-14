# Butterworth3

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">low-pass</span></div>

3-pole Butterworth low-pass filter.

## Usage

Use to smooth price or intermediate indicator values with a flat passband and sharp rolloff. The 3-pole version provides steeper attenuation at the cost of marginally more lag.

## Background

> Butterworth filters are maximally flat in the passband, introducing no ripple. Ehlers implements 2-pole and 3-pole Butterworth IIR designs in Cycle Analytics for Traders, noting that the SuperSmoother is actually a critically-damped 2-pole Butterworth variant.

## Parameters

- `period` (default: 14): Critical period

## Formula


\[
a = \exp(-\pi/P)
\]
\[
b = 2a \cos(1.738\pi/P)
\]
\[
c = a^2
\]
\[
f = (b+c)f_{t-1} - (c+bc)f_{t-2} + c^2f_{t-3} + \frac{(1-b+c)(1-c)}{8}(g + 3g_{t-1} + 3g_{t-2} + g_{t-3})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf)
