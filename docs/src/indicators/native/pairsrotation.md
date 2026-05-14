# PairsRotation

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">pairs-trading</span> <span class="kw-badge">rotation</span> <span class="kw-badge">relative-strength</span> <span class="kw-badge">ehlers</span></div>

Relative rotation of two securities using normalized roofing filters.

## Usage

Use to detect and trade rotation between two correlated assets. When one asset leads and the other lags, the indicator signals a rotation trade opportunity.

## Background

> Pairs Rotation analysis measures the relative cycle phase between two correlated assets. When one asset is at a cycle peak while its correlated partner is at a trough, a statistical rotation trade can be placed — long the laggard, short the leader — anticipating mean reversion of the spread.

## Parameters

- `hp_len` (default: 125): HighPass filter length
- `lp_len` (default: 20): LowPass (SuperSmoother) length

## Formula


\[
Filt = SuperSmoother(HighPass(Price, HPLen), LPLen)
\]
\[
MS = 0.0242 \cdot Filt^2 + 0.9758 \cdot MS_{t-1}
\]
\[
Normalized = \frac{Filt}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PAIRS%20ROTATION.pdf)
