# TruncatedBandpass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">bandpass</span> <span class="kw-badge">cycle</span> <span class="kw-badge">robust</span></div>

Truncated Bandpass filter for handling sharp price movements.

## Usage

Use to isolate cyclic components while minimizing 'ringing' effects caused by sudden price shocks. Ideal for cycle-based trading systems in volatile markets.

## Background

> Finite Impulse Response (FIR) filters have a fixed history, while Infinite Impulse Response (IIR) filters technically have an infinite history. Truncation limits the IIR feedback loop to a specific length, combining the sharp selectivity of IIR with the outlier-rejection of FIR.

## Parameters

- `period` (default: 20): Cycle period to isolate
- `bandwidth` (default: 0.1): Bandwidth of the filter
- `length` (default: 10): Truncation length

## Formula


\[
L1 = \cos(360/P), \quad G1 = \cos(BW \cdot 360/P), \quad S1 = 1/G1 - \sqrt{1/G1^2 - 1}
\]
\[
BPT_t = \text{IIR window of length } L \text{ with zero initial conditions}
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2020/07/TradersTips.html)
