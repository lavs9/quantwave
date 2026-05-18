# Cyber Cycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

An oscillator introduced by John Ehlers that models the cyclical component of a time series using FIR smoothing.

## Usage

Use as a high-resolution short-term cycle oscillator to time entries and exits around cycle turns. Pair with a trend classifier to suppress signals in trending conditions.

## Background

> Ehlers introduces the Cyber Cycle in Cybernetic Analysis (2004) as a bandpass-like filter isolating the short-term cyclical component. The trigger line is the Cyber Cycle delayed by one bar, creating a clean crossover signal without derivative noise.

## Parameters

- `length` (default: 14): Alpha smoothing length parameter

## Formula


\[
\alpha = \frac{2}{\text{Length} + 1}
\]
\[
\text{Smooth} = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}
\]
\[
CC_t = \left(1 - \frac{\alpha}{2}\right)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2}) + 2(1 - \alpha)CC_{t-1} - (1 - \alpha)^2 CC_{t-2}
\]


[Source](Cybernetic Analysis for Stocks and Futures, John Ehlers, 2004, Chapter 4)
