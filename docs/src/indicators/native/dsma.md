# DSMA

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Deviation Scaled Moving Average adapts to price variations using standard deviation scaled oscillators.

## Usage

Use as a highly adaptive moving average that tracks price closely during trends and large moves but provides heavy filtering during consolidation. Ideal for trend-following entries and trailing stops.

## Background

> In 'The Deviation-Scaled Moving Average' (2018), Ehlers introduces an adaptive EMA where the alpha (smoothing factor) is dynamically adjusted based on a deviation-scaled oscillator. By scaling the SuperSmoother-filtered momentum by its RMS, the indicator becomes reactive to significant price deviations while remaining smooth during low-volatility periods.

## Parameters

- `period` (default: 40): Critical period for smoothing and RMS calculation

## Formula


\[
Zeros = Close - Close_{t-2}
\]
\[
Filt = c_1 \frac{Zeros + Zeros_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
RMS = \sqrt{\frac{1}{P} \sum_{i=0}^{P-1} Filt_{t-i}^2}
\]
\[
\alpha = \min\left(1.0, \left| \frac{Filt}{RMS} \right| \frac{5}{P}\right)
\]
\[
DSMA = \alpha \cdot Close + (1 - \alpha) \cdot DSMA_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/DEVIATION%20SCALED%20MOVING%20AVERAGE.pdf)
