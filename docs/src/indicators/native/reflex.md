# Reflex

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">zero-lag</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">oscillator</span></div>

A zero-lag averaging indicator designed to synchronize with the cycle component in price data.

## Usage

Use to identify cyclic reversals with minimal lag. It is more sensitive to significant reversals than standard moving averages.

## Background

> Ehlers introduces Reflex as a way to reduce lag in averaging indicators by measuring the difference between the current SuperSmoother value and its historical values, adjusted for a linear slope. This 'reflexes' the indicator to show reversals as they happen rather than after the fact.

## Parameters

- `length` (default: 20): Assumed cycle period

## Formula


\[
Filt = \text{SuperSmoother}(Price, Length/2)
\]
\[
Slope = \frac{Filt_{t-Length} - Filt_t}{Length}
\]
\[
Sum = \frac{1}{Length} \sum_{n=1}^{Length} (Filt_t + n \cdot Slope - Filt_{t-n})
\]
\[
MS = 0.04 \cdot Sum^2 + 0.96 \cdot MS_{t-1}
\]
\[
Reflex = \frac{Sum}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS’ TIPS - FEBRUARY 2020.html)
