# SuperSmoother

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">low-pass</span></div>

A second-order IIR filter with a maximally flat Butterworth response for superior smoothing with minimal lag.

## Usage

Use as a drop-in replacement for any moving average when maximum smoothing with minimal lag is needed. Ideal as a pre-filter before oscillators to eliminate high-frequency noise.

## Background

> Ehlers describes the SuperSmoother as a two-pole Butterworth filter achieving the same smoothing as a longer SMA with far less lag. It uses a critically-damped design to eliminate Gibbs phenomenon overshoot while retaining cycle information. — Cybernetic Analysis for Stocks and Futures, 2004

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
c_1 = 1 - c_2 - c_3
\]
\[
SS = c_1 \frac{Price + Price_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf)
