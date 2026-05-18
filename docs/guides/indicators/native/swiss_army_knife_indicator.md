# Swiss Army Knife Indicator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">multi-purpose</span> <span class="kw-badge">smoothing</span></div>

A versatile indicator that can be configured as EMA, SMA, Gaussian, Butterworth, High Pass, Band Pass, or Band Stop filter.

## Usage

Use as a single configurable filter that can emulate SMA, EMA, Gaussian, Butterworth, or bandpass responses by switching mode flags. Ideal for prototyping different filter designs without code changes.

## Background

> Ehlers presents the Swiss Army Knife Indicator in Cycle Analytics for Traders as a unified filter framework. A set of boolean flags selects the operating mode, making it possible to compare multiple filter responses on the same data by simply toggling flags rather than reimplementing each filter.

## Parameters

- `mode` (default: BandPass): Filter mode (EMA, SMA, Gauss, Butter, Smooth, HP, 2PHP, BP, BS)
- `period` (default: 20): Filter period
- `delta` (default: 0.1): Bandwidth parameter for BP and BS modes

## Formula


\[
Filt = c_0(b_0 x_t + b_1 x_{t-1} + b_2 x_{t-2}) + a_1 Filt_{t-1} + a_2 Filt_{t-2} - c_1 x_{t-N}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/SwissArmyKnifeIndicator.pdf)
