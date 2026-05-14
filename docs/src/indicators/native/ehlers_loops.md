# Ehlers Loops

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">phase</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">visualization</span></div>

Converts price and volume into normalized standard deviation units for scatter plot analysis.

## Usage

Use to visualize cycle dynamics in phase-space by plotting the indicator value against its derivative. Loop patterns reveal cycle turns before they appear in the price chart.

## Background

> Ehlers describes phase-space loops in Cybernetic Analysis as a powerful visualization technique where an indicator is plotted against its first derivative. In cycle mode the path traces elliptical loops; in trend mode the path collapses to a line, enabling visual market mode identification.

## Parameters

- `lp_period` (default: 20): Low-pass filter period (SuperSmoother)
- `hp_period` (default: 125): High-pass filter period (Butterworth)

## Formula


\[
HP = c_1 (Price - 2 Price_{t-1} + Price_{t-2}) + c_2 HP_{t-1} + c_3 HP_{t-2}
\]
\[
SS = s_1 \frac{HP + HP_{t-1}}{2} + s_2 SS_{t-1} + s_3 SS_{t-2}
\]
\[
MS = \alpha SS^2 + (1 - \alpha) MS_{t-1}
\]
\[
RMS = \frac{SS}{\sqrt{MS}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202022.html)
