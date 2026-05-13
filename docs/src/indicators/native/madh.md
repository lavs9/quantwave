# MADH

Moving Average Difference with Hann Windowing: 100 * (Hann(short) - Hann(long)) / Hann(long)

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
