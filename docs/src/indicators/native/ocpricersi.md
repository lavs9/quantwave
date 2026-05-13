# OCPriceRSI

RSI calculated using the average of Open and Close prices to reduce noise.

## Parameters

- `period` (default: 14): RSI period

## Formula


\[
Input = \frac{Open + Close}{2}
\]
\[
RSI = \text{Wilder's RSI}(Input, Period)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EveryLittleBitHelps.pdf)
