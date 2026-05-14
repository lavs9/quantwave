# UltimateSmoother

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">zero-lag</span></div>

An Ehlers filter with zero lag in the Pass Band, constructed by subtracting High Pass response from the input data.

## Usage

Use when you need near-zero phase lag smoothing with very low ripple. It is Ehlers preferred smoother for applications where timing precision is critical.

## Background

> Ehlers designs the Ultimate Smoother in Cycle Analytics for Traders to minimize both lag and ripple simultaneously. It achieves near-zero phase shift across the passband while providing excellent attenuation of high-frequency noise, making it his preferred general-purpose smoother for cycle-sensitive applications.

## Parameters

- `period` (default: 20): Critical period (wavelength)

## Formula


\[
a_1 = \exp\left(-\frac{1.414\pi}{Period}\right)
\]
\[
c_2 = 2a_1 \cos\left(\frac{1.414\pi}{Period}\right)
\]
\[
c_3 = -a_1^2
\]
\[
c_1 = (1 + c_2 - c_3) / 4
\]
\[
US = (1 - c_1) Price + (2c_1 - c_2) Price_{t-1} - (c_1 + c_3) Price_{t-2} + c_2 US_{t-1} + c_3 US_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf)
