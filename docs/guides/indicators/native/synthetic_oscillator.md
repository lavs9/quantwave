# Synthetic Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">cycle</span> <span class="kw-badge">synthetic</span></div>

A nonlinear oscillator designed to reduce lag while maintaining smoothness by adapting to the dominant cycle.

## Usage

Use to construct a synthetic oscillator from dominant cycle sine components when direct price oscillators are too noisy. Most effective in clearly cyclical markets.

## Background

> Ehlers constructs a Synthetic Oscillator by generating a synthetic sine wave at the measured dominant cycle period and comparing it to price. The phase difference between the synthetic sine and actual price reveals whether the market is ahead of or behind its expected cycle position.

## Parameters

- `lower_bound` (default: 15): Lower bound of cycle period
- `upper_bound` (default: 25): Upper bound of cycle period

## Formula


\[
Price = \text{Hann}(Close, 12)
\]
\[
LP = \text{SuperSmoother}(\text{HighPass}(Price, UB), LB)
\]
\[
Re = \frac{LP}{RMS(LP, 100)}, \quad Im = \frac{Re - Re_{t-1}}{RMS(Re - Re_{t-1}, 100)}
\]
\[
DC = \frac{2\pi(Re^2 + Im^2)}{(Re - Re_{t-1})Im - (Im - Im_{t-1})Re}
\]
\[
BP = \text{UltimateSmoother}(\text{HighPass}(Close, Mid), Mid)
\]
\[
Phase = Phase_{t-1} + \frac{2\pi}{DC}
\]
\[
Synth = \sin(Phase)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202026.html)
