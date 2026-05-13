# Ultimate Channel

A Keltner-style channel using UltimateSmoothers for both the center line and the volatility range to minimize lag.

## Parameters

- `length` (default: 20): Center line smoothing period
- `str_length` (default: 20): Smooth True Range (STR) period
- `num_strs` (default: 1.0): Channel width multiplier

## Formula


\[
TH = \max(High, Close_{t-1})
\]
\[
TL = \min(Low, Close_{t-1})
\]
\[
STR = UltimateSmoother(TH - TL, STRLength)
\]
\[
Center = UltimateSmoother(Close, Length)
\]
\[
Upper = Center + NumSTRs \times STR
\]
\[
Lower = Center - NumSTRs \times STR
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf)
