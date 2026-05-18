# MyRSI

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">momentum</span> <span class="kw-badge">smoothing</span></div>

Ehlers' version of RSI that swings between -1 and +1.

## Usage

Use as Ehlers smoothed RSI variant that applies cycle-aware filtering to reduce whipsaws while maintaining RSI-style overbought/oversold interpretation.

## Background

> Ehlers presents a smoothed RSI formulation that applies a Laguerre or SuperSmoother filter to the up/down ratio before computing the RSI index. This reduces the noise and oscillation of standard RSI without significantly increasing lag, producing more reliable overbought and oversold readings.

## Parameters

- `length` (default: 14): Smoothing length

## Formula


\[
CU = \sum_{i=0}^{length-1} \max(0, Price_i - Price_{i+1})
\]
\[
CD = \sum_{i=0}^{length-1} \max(0, Price_{i+1} - Price_i)
\]
\[
MyRSI = \frac{CU - CD}{CU + CD}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf)
