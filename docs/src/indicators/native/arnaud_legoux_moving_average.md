# Arnaud Legoux Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">low-latency</span> <span class="kw-badge">adaptive</span></div>

ALMA is designed to reduce lag while providing high smoothness.

## Usage

Use as a low-latency moving average that reduces lag compared to EMA while controlling overshoot through the Gaussian offset parameter. Well-suited for momentum systems.

## Background

> The Arnaud Legoux Moving Average applies a Gaussian-shaped weight distribution offset toward the recent end of the lookback window. The sigma parameter controls weight spread and the offset parameter controls how far the Gaussian peak is positioned from the current bar, enabling a lag-accuracy trade-off unavailable in standard MAs.

## Parameters

- `period` (default: 9): Period
- `offset` (default: 0.85): Offset
- `sigma` (default: 6.0): Sigma

## Formula


\[
ALMA = \sum (W_i \times P_i) / \sum W_i
\]


[Source](https://www.prorealcode.com/prorealtime-indicators/arnaud-legoux-moving-average-alma/)
