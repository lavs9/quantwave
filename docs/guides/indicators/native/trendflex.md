# Trendflex

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">zero-lag</span> <span class="kw-badge">trend</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">oscillator</span></div>

A zero-lag averaging indicator designed to retain the trend component while reducing lag.

## Usage

Use to recognize enduring trends with minimal lag. It is better at identifying the start of a new trend than standard moving averages.

## Background

> Trendflex is the companion to Reflex. While Reflex focuses on the cyclic component by removing the trend slope, Trendflex retains the trend information by measuring the cumulative difference between the current smoothed value and its history without slope adjustment.

## Parameters

- `length` (default: 20): Assumed cycle period

## Formula


\[
Filt = \text{SuperSmoother}(Price, Length/2)
\]
\[
Sum = \frac{1}{Length} \sum_{n=1}^{Length} (Filt_t - Filt_{t-n})
\]
\[
MS = 0.04 \cdot Sum^2 + 0.96 \cdot MS_{t-1}
\]
\[
Trendflex = \frac{Sum}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS’ TIPS - FEBRUARY 2020.html)
