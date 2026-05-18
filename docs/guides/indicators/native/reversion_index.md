# Reversion Index

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">mean-reversion</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">cycle</span></div>

A mean-reversion oscillator that normalizes price changes by their absolute magnitude and applies SuperSmoother filtering.

## Usage

Use to identify mean-reversion opportunities when price has deviated significantly from its cycle trend. High index values signal overextended moves ripe for reversal.

## Background

> Ehlers Reversion Index measures how far price has deviated from its Instantaneous Trendline in units of cycle amplitude. Because it normalizes by the current cycle energy, the index provides consistent overbought/oversold thresholds regardless of the absolute price level or volatility regime.

## Parameters

- `length` (default: 20): Summation period (approx. half dominant cycle)

## Formula


\[
\Delta_t = \text{Close}_t - \text{Close}_{t-1}
\]
\[
\text{Ratio} = \frac{\sum_{i=0}^{L-1} \Delta_{t-i}}{\sum_{i=0}^{L-1} |\Delta_{t-i}|}
\]
\[
\text{Smooth} = SuperSmoother(\text{Ratio}, 8)
\]
\[
\text{Trigger} = SuperSmoother(\text{Ratio}, 4)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JANUARY%202026.html)
