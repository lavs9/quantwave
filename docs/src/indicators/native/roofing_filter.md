# Roofing Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">cycle</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">low-pass</span></div>

Combines a 2-pole HighPass filter and a SuperSmoother to isolate specific cyclic components.

## Usage

Apply before oscillators to remove both low-frequency trend drift and high-frequency noise, leaving only the tradable cycle band (roughly 10-48 bars).

## Background

> Introduced in Cycle Analytics for Traders (2013), the Roofing Filter first applies a high-pass filter to remove the dominant trend component, then a SuperSmoother to remove short-term noise. The result is a cycle-only signal with controlled bandwidth, ideal for use as input to oscillators and cycle indicators.

## Parameters

- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period

## Formula


\[
\alpha_1 = \frac{\cos(\sqrt{2}\pi/P_{hp}) + \sin(\sqrt{2}\pi/P_{hp}) - 1}{\cos(\sqrt{2}\pi/P_{hp})}
\]
\[
HP = (1 - \alpha_1/2)^2 (Price - 2 Price_{t-1} + Price_{t-2}) + 2(1 - \alpha_1) HP_{t-1} - (1 - \alpha_1)^2 HP_{t-2}
\]
\[
Filt = c_1 \frac{HP + HP_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PredictiveIndicators.pdf)
