# RSIH

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">cycle</span></div>

RSI enhanced with Hann windowing for superior smoothing and zero-centering.

## Usage

Use to measure momentum exclusively on the cyclical (high-pass filtered) component of price, eliminating the trend bias that makes standard RSI drift.

## Background

> RSIH applies RSI computation to the high-pass filtered price rather than raw price. By removing the trend component first, the RSI calculation operates only on the cyclical content of the market, producing an oscillator that is centered around zero regardless of the prevailing trend direction.

## Parameters

- `length` (default: 14): RSI length

## Formula


\[
CU = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n+1} - Close_{t-n})
\]
\[
CD = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n} - Close_{t-n+1})
\]
\[
RSIH = \frac{CU - CD}{CU + CD}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202022.html)
