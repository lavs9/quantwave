# Instantaneous Trendline

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">trend</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

Removes the dominant cycle to reveal the underlying trend with minimal lag.

## Usage

Use as an adaptive trend line that automatically adjusts to the current dominant cycle period, replacing fixed-period moving averages in trend-following systems.

## Background

> Defined in Rocket Science for Traders (2001), the Instantaneous Trendline is derived from Hilbert Transform phasors and synchronized to the current market cycle. It is computed as a 3-bar weighted average adjusted by the instantaneous period, giving a zero-lag trend estimate.

## Formula


\[
Trendline = \text{WMA}(\text{SMA}(Price, DCPeriod), 4)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf)
