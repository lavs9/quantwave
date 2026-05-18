# VossPredictor

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">prediction</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">filter</span></div>

A predictive filter with negative group delay for band-limited signals.

## Usage

Use for multi-bar price prediction based on a bandpass-filtered dominant cycle. More accurate than simple linear extrapolation due to its IIR filter pole placement.

## Background

> The Voss Predictor is a predictive filter developed by J.F. Voss and adapted by Ehlers in Cycle Analytics for Traders. Its IIR bandpass design inherently extrapolates the filtered signal several bars into the future by virtue of pole placement inside the unit circle, enabling lookahead without buffer access.

## Parameters

- `period` (default: 20): Center period of the BandPass filter
- `predict` (default: 3): Number of bars of prediction

## Formula


\[
Filt = \text{BandPass}(Price, Period, 0.25)
\]
\[
Order = 3 \cdot Predict
\]
\[
SumC = \sum_{n=0}^{Order-1} \frac{n+1}{Order} Voss_{t-(Order-n)}
\]
\[
Voss = \frac{3 + Order}{2} Filt - SumC
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/A%20PEEK%20INTO%20THE%20FUTURE.pdf)
