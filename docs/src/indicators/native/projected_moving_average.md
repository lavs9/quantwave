# Projected Moving Average

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">prediction</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">zero-lag</span></div>

A lag-compensated moving average that uses linear regression slope to project the average forward.

## Usage

Use as a predictive moving average that uses linear regression projection to anticipate where price will be rather than where it has been, reducing effective lag.

## Background

> The Projected Moving Average uses linear regression over the lookback window to project the best-fit line forward to the current bar. This predictive approach shifts the MA output toward the leading edge of price movement, achieving reduced lag compared to conventional MAs of the same period.

## Parameters

- `length` (default: 20): Calculation length

## Formula


\[
Slope = -\frac{n \sum xy - \sum x \sum y}{n \sum x^2 - (\sum x)^2}
\]
\[
PMA = SMA + Slope \cdot \frac{n}{2}
\]
\[
Predict = PMA + 0.5 \cdot (Slope - Slope_{t-2}) \cdot n
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20MARCH%202025.html)
