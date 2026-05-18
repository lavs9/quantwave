# FM Demodulator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">amplitude</span> <span class="kw-badge">frequency</span></div>

Extracts market timing information by demodulating the frequency-modulated price spectrum.

## Usage

Use to extract the instantaneous amplitude and frequency of market cycles. The AM output measures cycle energy for position sizing; the FM output tracks cycle period for adaptive indicator tuning.

## Background

> Ehlers adapts AM and FM demodulation techniques from radio engineering in Cycle Analytics for Traders to extract cycle amplitude and instantaneous frequency from market data. The amplitude envelope measures how energetic the current cycle is, while FM reveals whether the cycle period is expanding or contracting.

## Parameters

- `period` (default: 30): SuperSmoother period

## Formula


\[
Deriv = Close - Open, HL = \text{Clip}(10 \times Deriv, -1, 1)
\]
\[
SS = c_1 \frac{HL + HL_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf)
