# MESA Adaptive Moving Average

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">trend</span></div>

MAMA adapts to price movement in an entirely new and unique way based on the rate change of phase.

## Usage

Use as an adaptive trend filter that automatically speeds up in fast markets and slows in choppy ones. The FAMA line crossing MAMA provides high-probability trend change signals.

## Background

> Presented in Rocket Science for Traders (2001), MAMA adapts its alpha based on the rate of phase change measured by the Hilbert Transform Discriminator. Fast cycles produce large alpha for responsiveness; slow cycles produce small alpha to reduce noise.

## Parameters

- `fast_limit` (default: 0.5): Fast limit for alpha
- `slow_limit` (default: 0.05): Slow limit for alpha

## Formula


\[
\text{MAMA} = \alpha \cdot \text{Price} + (1 - \alpha) \cdot \text{MAMA}_{1}
\]
\[
\text{FAMA} = 0.5\alpha \cdot \text{MAMA} + (1 - 0.5\alpha) \cdot \text{FAMA}_{1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/MAMA.pdf)
