# AM Detector

Recovers market volatility from the amplitude-modulated whitened price spectrum.

## Parameters

- `highest_len` (default: 4): Envelope lookback length
- `avg_len` (default: 8): Smoothing length

## Formula


\[
Deriv = |Close - Open|, Envel = \max(Deriv, 4), Volatil = \text{Avg}(Envel, 8)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf)
