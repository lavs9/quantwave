# Gap Momentum

<div class="indicator-meta"><span class="category-badge">Momentum</span> <span class="kw-badge">momentum</span> <span class="kw-badge">gap</span> <span class="kw-badge">kaufman</span> <span class="kw-badge">oscillator</span></div>

Accumulates positive and negative opening gaps to derive a cumulative gap ratio, smoothed by a signal line.

## Usage

Used to identify momentum shifts based on price gaps. Buy when the signal line is rising and sell when it is falling.

## Background

> Perry J. Kaufman introduced Gap Momentum as a way to quantify price gaps relative to their cumulative volatility, similar to an On-Balance Volume (OBV) logic applied to opening gaps. It helps traders identify if gap-driven momentum is increasing or decreasing by comparing the sum of upward gaps against downward gaps over a rolling window. — Perry Kaufman, S&C 2024

## Parameters

- `period` (default: 40): Rolling window for gap accumulation
- `signal_period` (default: 20): Smoothing period for the gap ratio

## Formula


\[
Gap = Open_t - Close_{t-1}
\]
\[
UpGaps = \sum_{i=0}^{Period-1} \max(0, Gap_{t-i})
\]
\[
DnGaps = \sum_{i=0}^{Period-1} \max(0, -Gap_{t-i})
\]
\[
GapRatio = \begin{cases} 1 & \text{if } DnGaps = 0 \\ 100 \times \frac{UpGaps}{DnGaps} & \text{otherwise} \end{cases}
\]
\[
Signal = SMA(GapRatio, SignalPeriod)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JANUARY%202024.html)
