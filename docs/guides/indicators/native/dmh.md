# DMH

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">dsp</span></div>

An improved Directional Movement indicator using Hann windowing for smoother signals and reduced lag.

## Usage

Use as a momentum oscillator with high-pass filtering to isolate cyclical momentum while removing the trend bias that corrupts standard momentum indicators.

## Background

> Ehlers constructs the DMH by applying a high-pass filter to the momentum calculation, removing the low-frequency trend component that causes conventional momentum to drift. The result is a zero-centered momentum oscillator that oscillates cleanly around the cycle midpoint.

## Parameters

- `length` (default: 14): Smoothing period

## Formula


\[
\text{PlusDM} = \text{High} - \text{High}_{t-1} \text{ if } > (\text{Low}_{t-1} - \text{Low}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{MinusDM} = \text{Low}_{t-1} - \text{Low} \text{ if } > (\text{High} - \text{High}_{t-1}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{EMA} = \frac{1}{L}(\text{PlusDM} - \text{MinusDM}) + (1 - \frac{1}{L})\text{EMA}_{t-1}
\]
\[
\text{DMH} = \frac{\sum_{i=1}^{L} w_i \text{EMA}_{t-i+1}}{\sum_{i=1}^{L} w_i}, \text{ where } w_i = 1 - \cos\left(\frac{2\pi i}{L+1}\right)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202021.html)
