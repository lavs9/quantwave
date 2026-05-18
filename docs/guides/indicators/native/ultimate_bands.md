# Ultimate Bands

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">bands</span> <span class="kw-badge">volatility</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">adaptive</span></div>

A Bollinger-style band using UltimateSmoother for the center line and standard deviation of the price-smooth difference for width.

## Usage

Use as volatility bands that automatically widen during high-energy cycle phases and narrow during quiet phases. Better than fixed-multiple ATR bands in strongly cyclical markets.

## Background

> Ehlers Ultimate Bands compute upper and lower price envelopes using the RMS amplitude of the dominant cycle rather than a fixed ATR multiple. This makes the bands proportional to the current cycle energy, expanding when the market is actively cycling and contracting when it enters a low-energy consolidation.

## Parameters

- `length` (default: 20): Smoothing and SD period
- `num_sds` (default: 1.0): Standard Deviation multiplier

## Formula


\[
Smooth = UltimateSmoother(Close, Length)
\]
\[
SD = \sqrt{\frac{1}{n}\sum_{i=0}^{n-1} (Close_{t-i} - Smooth_{t-i})^2}
\]
\[
Upper = Smooth + NumSDs \times SD
\]
\[
Lower = Smooth - NumSDs \times SD
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf)
