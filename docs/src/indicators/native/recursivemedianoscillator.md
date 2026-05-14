# RecursiveMedianOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">median</span> <span class="kw-badge">cycle</span> <span class="kw-badge">highpass</span></div>

Oscillator derived from the Recursive Median filter using a 2nd-order Highpass filter.

## Usage

Identify cyclic turning points with reduced lag and noise. The high-pass component removes the trend, leaving the cycle.

## Background

> By applying a 2nd-order Highpass filter to the Recursive Median output, we create an oscillator that is specifically tuned to the dominant cycle while remaining immune to the outlier spikes that would otherwise create false signals.

## Parameters

- `lp_period` (default: 12): Low-pass smoothing period
- `hp_period` (default: 30): High-pass cutoff period

## Formula


\[
\alpha_2 = \frac{\cos(0.707 \cdot 360/HP) + \sin(0.707 \cdot 360/HP) - 1}{\cos(0.707 \cdot 360/HP)}
\]
\[
RMO_t = (1-\alpha_2/2)^2(RM_t - 2RM_{t-1} + RM_{t-2}) + 2(1-\alpha_2)RMO_{t-1} - (1-\alpha_2)^2RMO_{t-2}
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2018/03/TradersTips.html)
