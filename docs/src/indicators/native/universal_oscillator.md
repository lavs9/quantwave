# Universal Oscillator

An adaptive oscillator that normalizes price momentum using a SuperSmoother filter and AGC.

## Parameters

- `band_edge` (default: 20): Critical period for the SuperSmoother filter

## Formula


\[
WN = (Price - Price_{t-2}) / 2
\]
\[
AvgWN = (WN + WN_{t-1}) / 2
\]
\[
Filt = c_1 AvgWN + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
Peak = \max(0.991 \times Peak_{t-1}, |Filt|)
\]
\[
Universal = Filt / Peak
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2015/01/TradersTips.html)
