# RecursiveMedian

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">median</span> <span class="kw-badge">robust</span> <span class="kw-badge">smoothing</span></div>

EMA of a 5-bar median filter for smooth tracking with minimal jitter.

## Usage

Use to filter out extreme outliers and noise while maintaining trend sensitivity. Excellent as a baseline for other oscillators.

## Background

> Standard filters like SMA or EMA are distorted by price spikes. The recursive median filter uses the median to reject outliers and an EMA to provide smoothness, offering a cleaner trend representation than standard moving averages.

## Parameters

- `lp_period` (default: 12): Low-pass smoothing period

## Formula


\[
\alpha = \frac{\cos(360/P) + \sin(360/P) - 1}{\cos(360/P)}
\]
\[
RM_t = \alpha \cdot \text{Median}(Price, 5)_t + (1 - \alpha) \cdot RM_{t-1}
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2018/03/TradersTips.html)
