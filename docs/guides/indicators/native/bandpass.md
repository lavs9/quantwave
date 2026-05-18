# BandPass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">bandpass</span></div>

A bandpass filter that isolates cycle components around a center period.

## Usage

Apply to isolate a specific cycle period in price, filtering out both trend and noise. Use zero crossings of the filtered output as entry and exit signals.

## Background

> Ehlers presents the BandPass filter in Cybernetic Analysis as a second-order IIR filter centred on a target cycle period with tunable bandwidth. It simultaneously attenuates lower and higher frequencies, leaving only the desired cycle band in the output.

## Parameters

- `period` (default: 20): Center period of the passband
- `bandwidth` (default: 0.1): Relative bandwidth (delta)

## Formula


\[
\beta = \cos(360/P), \gamma = 1/\cos(720\delta/P), \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf)
