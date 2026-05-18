# MADH

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">volatility</span> <span class="kw-badge">statistics</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">high-pass</span></div>

Moving Average Difference with Hann Windowing: 100 * (Hann(short) - Hann(long)) / Hann(long)

## Usage

Use to measure the volatility of the cyclical price component only, filtering out trend-driven amplitude changes that inflate standard volatility measures in trending markets.

## Background

> MADH applies Mean Absolute Deviation to the high-pass filtered price series rather than raw price. By isolating the cyclical component before measuring dispersion, it quantifies the noise level within the current market cycle rather than conflating it with trend amplitude.

## Parameters

- `short_length` (default: 8): Short-term filter length
- `dominant_cycle` (default: 27): Dominant cycle for calculating long length

## Formula


\[
LongLength = \lfloor ShortLength + DominantCycle / 2 \rfloor
\]
\[
Filt1 = HannWindow(Price, ShortLength)
\]
\[
Filt2 = HannWindow(Price, LongLength)
\]
\[
MADH = 100 \times \frac{Filt1 - Filt2}{Filt2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - NOVEMBER 2021.html)
