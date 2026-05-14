# MESA Adaptive Moving Average (MAMA)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">phase</span></div>

A moving average that adapts to price movement based on the rate of change of phase.

## Usage

Use as a highly responsive moving average that virtually eliminates overshoot while providing rapid response to price changes. The companion 'FAMA' (Following Adaptive Moving Average) provides a secondary line for crossover signals.

## Background

> MAMA adapts to the price movement based on the Hilbert Transform phase rate of change. It provides a unique combination of fast response to price changes while remaining smooth during congested market periods. It is one of the most sophisticated adaptive moving averages available. — Rocket Science for Traders

## Parameters

- `fastlimit` (default: 0.5): Fast limit
- `slowlimit` (default: 0.05): Slow limit

## Formula


\[
\alpha = \frac{\text{FastLimit}}{\text{PhaseRate}}
\]


[Source](http://www.mesasoftware.com/Papers/MAMA.pdf)
