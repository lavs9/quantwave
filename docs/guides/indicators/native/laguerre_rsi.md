# Laguerre RSI

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">laguerre</span> <span class="kw-badge">momentum</span></div>

RSI calculated over Laguerre-warped time for faster response.

## Usage

Use as a faster lower-lag alternative to traditional RSI. Laguerre smoothing produces fewer whipsaws while remaining responsive to genuine momentum shifts.

## Background

> Ehlers constructs the Laguerre RSI in Cybernetic Analysis by computing RSI on the four outputs of a Laguerre filter bank. The result has RSI-like scaling (0 to 1) but dramatically less lag and smoother behaviour than conventional RSI.

## Parameters

- `gamma` (default: 0.5): Smoothing factor (0.0 to 1.0)

## Formula


\[
L_0 = (1 - \gamma) \cdot Close + \gamma \cdot L_{0,t-1}
\]
\[
L_1 = -\gamma L_0 + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
L_2 = -\gamma L_1 + L_{1,t-1} + \gamma L_{2,t-1}
\]
\[
L_3 = -\gamma L_2 + L_{2,t-1} + \gamma L_{3,t-1}
\]
\[
CU = \sum \max(L_{i} - L_{i+1}, 0)
\]
\[
CD = \sum \max(L_{i+1} - L_{i}, 0)
\]
\[
RSI = \frac{CU}{CU + CD}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TimeWarp.pdf)
