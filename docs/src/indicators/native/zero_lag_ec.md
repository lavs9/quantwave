# Zero Lag EC

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">zero-lag</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">ema</span> <span class="kw-badge">smoothing</span></div>

Zero Lag Error Corrected EMA attempts to eliminate lag by adding an error term to the EMA.

## Usage

Use as a near-zero-lag moving average for trend-following systems. The error-correction term removes the lag inherent in the standard EMA without introducing significant overshoot.

## Background

> Ehlers introduces the Zero Lag indicator in Cybernetic Analysis as an EMA with an added error-correction term that subtracts the average lag from the output. The resulting EC (Error Corrected) line tracks price with near-zero delay while the ZL-EMA provides a smoothed reference, with crossovers between them providing trade signals.

## Parameters

- `length` (default: 20): Equivalent SMA length
- `gain_limit` (default: 50.0): Gain limit (divided by 10 for actual gain)

## Formula


\[
\alpha = \frac{2}{Length + 1}
\]
\[
EMA = \alpha \times Close + (1 - \alpha) \times EMA_{t-1}
\]
\[
EC = \alpha \times (EMA + Gain \times (Close - EC_{t-1})) + (1 - \alpha) \times EC_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/ZeroLag.pdf)
