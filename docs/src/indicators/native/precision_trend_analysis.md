# Precision Trend Analysis

Trend identification using the difference between two high-pass filters.

## Parameters

- `length1` (default: 250): First HighPass filter period
- `length2` (default: 40): Second HighPass filter period

## Formula


\[
HP1 = HighPass(Price, Length1)
\]
\[
HP2 = HighPass(Price, Length2)
\]
\[
Trend = HP1 - HP2
\]
\[
ROC = \frac{Length2}{6.28} \cdot (Trend - Trend_{t-1})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20SEPTEMBER%202024.html)
