# EhlersUltimateOscillator

A highly responsive oscillator created from the difference of two highpass filters, normalized by RMS.

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
