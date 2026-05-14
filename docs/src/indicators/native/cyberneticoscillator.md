# CyberneticOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">cycle</span> <span class="kw-badge">momentum</span></div>

Combined HighPass and SuperSmoother filters normalized by RMS.

## Usage

Use as a generalized Ehlers cycle oscillator when you need a configurable bandpass response tuned to a specific dominant cycle period.

## Background

> The Cybernetic Oscillator is derived from the bandpass filter framework in Ehlers Cybernetic Analysis for Stocks and Futures (2004). By tuning the filter center frequency to the measured dominant cycle period, it extracts only the cyclical component and presents it as an oscillator ranging above and below zero.

## Parameters

- `hp_length` (default: 30): HighPass filter length
- `lp_length` (default: 20): LowPass (SuperSmoother) length
- `rms_len` (default: 100): RMS normalization length

## Formula


\[
HP = HighPass(Price, HPLen)
\]
\[
LP = SuperSmoother(HP, LPLen)
\]
\[
RMS = \sqrt{\frac{1}{N} \sum_{i=0}^{N-1} LP_{t-i}^2}
\]
\[
CO = \frac{LP}{RMS}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202025.html)
