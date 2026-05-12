# Arnaud Legoux Moving Average

ALMA is designed to reduce lag while providing high smoothness.

## Parameters

- `period` (default: 9): Period
- `offset` (default: 0.85): Offset
- `sigma` (default: 6.0): Sigma

## Formula


\[
ALMA = \sum (W_i \times P_i) / \sum W_i
\]


[Source](https://www.prorealcode.com/prorealtime-indicators/arnaud-legoux-moving-average-alma/)
