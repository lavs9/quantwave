# Ultimate Strength Index

A lag-reduced version of the RSI using UltimateSmoother on smoothed up/down components.

## Parameters

- `length` (default: 14): UltimateSmoother period

## Formula


\[
\text{SU} = \max(0, \text{Close} - \text{Close}_{t-1})
\]
\[
\text{SD} = \max(0, \text{Close}_{t-1} - \text{Close})
\]
\[
\text{USU} = UltimateSmoother(SMA(\text{SU}, 4), Length)
\]
\[
\text{USD} = UltimateSmoother(SMA(\text{SD}, 4), Length)
\]
\[
\text{USI} = \frac{\text{USU} - \text{USD}}{\text{USU} + \text{USD}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20NOVEMBER%202024.html)
