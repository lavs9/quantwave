# Universal Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">universal</span> <span class="kw-badge">momentum</span></div>

An adaptive oscillator that normalizes price momentum using a SuperSmoother filter and AGC.

## Usage

Use as a generic oscillator framework that works on any pre-filtered input. Feed it the output of any smoother or filter to produce a normalized zero-centered oscillator.

## Background

> Ehlers Universal Oscillator is a generic momentum computation that can be applied to any filtered price input. It computes the rate of change of the filtered series normalized by its RMS amplitude, producing a consistently scaled oscillator that works regardless of the underlying filter or price instrument.

## Parameters

- `band_edge` (default: 20): Critical period for the SuperSmoother filter

## Formula


\[
WN = (Price - Price_{t-2}) / 2
\]
\[
AvgWN = (WN + WN_{t-1}) / 2
\]
\[
Filt = c_1 AvgWN + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
Peak = \max(0.991 \times Peak_{t-1}, |Filt|)
\]
\[
Universal = Filt / Peak
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2015/01/TradersTips.html)
