# Precision Trend Analysis

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">filter</span></div>

Trend identification using the difference between two high-pass filters.

## Usage

Use as a high-precision trend indicator that applies DSP filtering to remove cycle noise before measuring trend direction, giving fewer but more reliable trend signals.

## Background

> Ehlers Precision Trend analysis applies a roofing-filter style preprocessing to price before computing the trend indicator, removing the cyclical component that causes premature trend reversals in standard indicators. The result is a trend signal that changes state only when the genuine trend direction changes.

## Parameters

- `length1` (default: 250): First HighPass filter period
- `length2` (default: 40): Second HighPass filter period

## Formula


\[
HP1 = HighPass(Price, Length1)
\]
\[
HP2 = HighPass(Price, Length2)
\]
\[
Trend = HP1 - HP2
\]
\[
ROC = \frac{Length2}{6.28} \cdot (Trend - Trend_{t-1})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20SEPTEMBER%202024.html)
