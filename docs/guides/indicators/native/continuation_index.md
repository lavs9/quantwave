# Continuation Index

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">cycle</span></div>

An oscillator that identifies trend onset and exhaustion by comparing a fast UltimateSmoother with a Generalized Laguerre filter.

## Usage

Use to measure whether a price move is likely to continue or reverse based on cycle analysis. High index values suggest trend continuation; low values suggest an impending cycle turn.

## Background

> The Continuation Index measures the persistence of directional price movement relative to the dominant cycle. Ehlers derives it from the cycle phase velocity — when phase advances quickly in one direction, momentum is strong and continuation is likely; slow or reversing phase suggests the move is exhausting.

## Parameters

- `gamma` (default: 0.8): Laguerre gamma parameter
- `order` (default: 8): Laguerre filter order
- `length` (default: 40): Base smoothing length

## Formula


\[
US = UltimateSmoother(Close, Length/2)
\]
\[
LG = Laguerre(Close, \gamma, Order, Length)
\]
\[
Variance = SMA(|US - LG|, Length)
\]
\[
Ref = 2 \times (US - LG) / Variance
\]
\[
CI = \tanh(Ref)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html)
