# Reverse EMA

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">ema</span> <span class="kw-badge">lag</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">zero-lag</span></div>

A causal forward and backward EMA indicator that minimizes lag using a series of alignment filters.

## Usage

Use to identify trends or cycles with minimal lag. Higher alpha values (e.g., 0.3) isolate cycles, while lower values (e.g., 0.05) isolate trends.

## Background

> Ehlers' Reverse EMA approximates a non-causal zero-lag filter by using a product series of Z-transform components. It achieves double smoothing at high frequencies and mitigates spectral dilation at low frequencies, providing a unique balance of smoothness and responsiveness.

## Parameters

- `alpha` (default: 0.1): Smoothing factor (0.0 to 1.0)

## Formula


\[
EMA = \alpha \cdot Price + (1 - \alpha) \cdot EMA_{t-1}
\]
\[
RE_1 = (1 - \alpha) \cdot EMA + EMA_{t-1}
\]
\[
RE_i = (1 - \alpha)^{2^{i-1}} \cdot RE_{i-1} + RE_{i-1, t-1} \text{ for } i=2..8
\]
\[
Wave = EMA - \alpha \cdot RE_8
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202017.html)
