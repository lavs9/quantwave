# EhlersUltimateOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">momentum</span> <span class="kw-badge">adaptive</span></div>

A highly responsive oscillator created from the difference of two highpass filters, normalized by RMS.

## Usage

Use as a multi-scale momentum oscillator that combines signals from multiple cycle-aware timeframes to reduce false signals from any single period.

## Background

> Ehlers Ultimate Oscillator combines the outputs of multiple cycle-synchronized oscillators operating at different dominant cycle harmonics. By averaging across scales, it reduces the likelihood of false signals that occur when any single oscillator is temporarily misaligned with the market cycle.

## Parameters

- `band_edge` (default: 20): Critical period (shorter period)
- `bandwidth` (default: 2.0): Multiplier for the longer period

## Formula


\[
HP_1 = \text{HighPass}(Price, BandEdge \cdot Bandwidth)
\]
\[
HP_2 = \text{HighPass}(Price, BandEdge)
\]
\[
Signal = HP_1 - HP_2
\]
\[
UO = \frac{Signal}{RMS(Signal, 100)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202025.html)
