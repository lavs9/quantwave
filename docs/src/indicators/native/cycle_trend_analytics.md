# Cycle/Trend Analytics

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">trend</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">classification</span> <span class="kw-badge">adaptive</span></div>

A set of oscillators (Price - SMA) with lengths from 5 to 30 used to visualize cycles and trends.

## Usage

Use to classify the current market mode as trending or cycling before selecting your strategy. Apply trend-following systems in trend mode and mean-reversion systems in cycle mode.

## Background

> Ehlers presents Cycle/Trend Analytics in Cycle Analytics for Traders as a framework for determining the dominant market mode. By measuring the correlation between price and the best-fit dominant cycle, the indicator classifies market behavior, enabling traders to switch between trend and cycle trading strategies dynamically.

## Parameters

- `min_length` (default: 5): Minimum SMA length
- `max_length` (default: 30): Maximum SMA length

## Formula


\[
Osc(L) = Price - SMA(Price, L) \quad \text{for } L \in [min, max]
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - OCTOBER 2021.html)
