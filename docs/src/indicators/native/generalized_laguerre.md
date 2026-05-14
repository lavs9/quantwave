# Generalized Laguerre

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">laguerre</span></div>

A generalized Laguerre filter of arbitrary order using an UltimateSmoother as the primary component.

## Usage

Use when the standard 4-element Laguerre filter needs further customization. The additional gamma2 parameter allows independent control of the pole spacing for more flexible frequency response shaping.

## Background

> The Generalized Laguerre Filter extends the classic 4-element Laguerre design with an additional parameter that controls the distribution of poles across the frequency spectrum. This gives finer control over the transition band slope and passband flatness, useful for specialized spectral analysis applications.

## Parameters

- `length` (default: 40): UltimateSmoother period
- `gamma` (default: 0.8): Smoothing factor (0.0 to 1.0)
- `order` (default: 8): Filter order (1 to 10)

## Formula


\[
LG_1 = UltimateSmoother(Price, Length)
\]
\[
LG_i = -\gamma LG_{i-1,t-1} + LG_{i-1,t-1} + \gamma LG_{i,t-1} \text{ for } i=2 \dots Order
\]
\[
Filter = \frac{1}{Order} \sum_{i=1}^{Order} LG_i
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html)
