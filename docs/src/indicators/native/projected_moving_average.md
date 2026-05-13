# Projected Moving Average

A lag-compensated moving average that uses linear regression slope to project the average forward.

## Parameters

- `length` (default: 20): Calculation length

## Formula


\[
Slope = -\frac{n \sum xy - \sum x \sum y}{n \sum x^2 - (\sum x)^2}
\]
\[
PMA = SMA + Slope \cdot \frac{n}{2}
\]
\[
Predict = PMA + 0.5 \cdot (Slope - Slope_{t-2}) \cdot n
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20MARCH%202025.html)
