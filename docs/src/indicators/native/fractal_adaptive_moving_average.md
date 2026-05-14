# Fractal Adaptive Moving Average

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">fractal</span> <span class="kw-badge">smoothing</span></div>

An adaptive moving average that uses the fractal dimension of prices to dynamically change its smoothing constant.

## Usage

Use as an adaptive moving average that slows dramatically during consolidation and speeds up during trending phases. Outperforms fixed-period MAs in ranging markets by avoiding false crossovers.

## Background

> The Fractal Adaptive Moving Average uses the fractal dimension of recent price action to adapt its smoothing constant. During trending markets the fractal dimension approaches 1 (a line) producing a fast-reacting EMA; during ranging markets the dimension approaches 2 (a plane) slowing the average dramatically to filter chop.

## Parameters

- `length` (default: 16): Length (must be an even number; odd values will be incremented by 1).

## Formula


\[
D = \frac{\log(N_1 + N_2) - \log(N_3)}{\log(2)}
\]
\[
\alpha = \exp(-4.6 (D - 1))
\]
\[
\text{FRAMA}_t = \alpha P_t + (1 - \alpha) \text{FRAMA}_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/FRAMA.pdf)
