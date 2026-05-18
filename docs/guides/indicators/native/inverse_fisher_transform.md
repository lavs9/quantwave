# Inverse Fisher Transform

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">normalization</span> <span class="kw-badge">momentum</span></div>

A compressive transform that forces oscillator values towards +1 or -1, creating clear buy/sell signals.

## Usage

Apply to RSI or other oscillators to rescale them to a ±1 range with sharp threshold behaviour. Values near ±1 indicate high-confidence overbought/oversold conditions.

## Background

> The Inverse Fisher Transform maps input values to (-1, +1) via a hyperbolic tangent function. Ehlers uses it in Cybernetic Analysis to create oscillators whose output clusters near the extremes, making crossovers of fixed thresholds reliable trading signals.

## Formula


\[
IFT(x) = \frac{e^{2x} - 1}{e^{2x} + 1} = \tanh(x)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheInverseFisherTransform.pdf)
