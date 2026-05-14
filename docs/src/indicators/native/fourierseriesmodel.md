# FourierSeriesModel

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">prediction</span> <span class="kw-badge">fourier</span></div>

Synthesized market model using fundamental and harmonic frequency components.

## Usage

Use to model price as a sum of sine wave harmonics for short-term prediction. Most effective in clearly cyclical markets; combine with a cycle mode detector to disable it in trends.

## Background

> The Fourier Series Model fits harmonically related sine waves to recent price history using least-squares coefficients. Ehlers shows that projecting this model one bar forward gives a price forecast useful for anticipatory entry timing at predicted cycle turns.

## Parameters

- `fundamental` (default: 20): Fundamental cycle period

## Formula


\[
BP_k = \text{BandPass}(Price, Fundamental/k)
\]
\[
Q_k = \frac{Fundamental}{2\pi} (BP_{k} - BP_{k,t-1})
\]
\[
P_k = \sum_{n=0}^{F-1} (BP_{k,t-n}^2 + Q_{k,t-n}^2)
\]
\[
Wave = BP_1 + \sqrt{P_2/P_1}BP_2 + \sqrt{P_3/P_1}BP_3
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FOURIER%20SERIES%20MODEL%20OF%20THE%20MARKET.pdf)
