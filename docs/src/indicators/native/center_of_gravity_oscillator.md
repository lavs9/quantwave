# Center of Gravity Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">momentum</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">zero-lag</span></div>

The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

## Usage

Use as a zero-lag momentum oscillator to detect cycle turning points. Crossovers of the trigger line provide high-accuracy entry and exit signals.

## Background

> Ehlers introduces the Center of Gravity oscillator in Cybernetic Analysis (2004) as a near-zero-lag indicator. It computes the center of mass of a price series over a lookback window, producing an oscillator whose turning points lead price turns — a reversal of the usual indicator lag relationship.

## Parameters

- `period` (default: 10): Observation window length

## Formula


\[
CG = -\frac{\sum_{i=0}^{N-1} (i+1) \times Price_i}{\sum_{i=0}^{N-1} Price_i}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheCGOscillator.pdf)
