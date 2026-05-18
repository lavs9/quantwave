# OCPriceRSI

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">momentum</span></div>

RSI calculated using the average of Open and Close prices to reduce noise.

## Usage

Use to measure momentum on the open-to-close price differential rather than close-to-close, capturing intraday directional strength more directly.

## Background

> Ehlers computes this RSI variant on the difference between the open and close price of each bar rather than on the closing price series. The open-close differential captures the net directional pressure within each bar, producing a momentum oscillator more sensitive to intraday commitment than standard RSI.

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
