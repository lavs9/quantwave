# HighPass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">cycle</span></div>

A second-order High Pass filter that rejects low-frequency components.

## Usage

Apply to price to isolate the cyclical component by attenuating the low-frequency trend. Use as the first stage before an oscillator or spectrum analyser.

## Background

> Ehlers derives the one-pole high-pass filter in Cycle Analytics for Traders analogously to EMA derivation, but applied to price differences rather than levels. It removes the DC component and low-frequency trend, leaving the cyclical content for downstream analysis.

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
HP = c_1 (Price - 2 Price_{t-1} + Price_{t-2}) + c_2 HP_{t-1} + c_3 HP_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf)
