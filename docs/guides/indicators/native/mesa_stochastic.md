# MESA Stochastic

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">stochastic</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">cycle</span> <span class="kw-badge">adaptive</span></div>

Standard Stochastic calculation applied to Roofing Filtered data, followed by SuperSmoothing.

## Usage

Use as a cycle-synchronized stochastic that automatically scales its lookback to the measured dominant cycle period for consistent overbought/oversold signals.

## Background

> The MESA Stochastic extends Ehlers adaptive stochastic concept by using the MESA-measured dominant cycle period as the lookback window. Unlike traditional stochastics with fixed periods, it adapts to the current market rhythm, keeping the oscillator calibrated to one full cycle at all times.

## Parameters

- `length` (default: 20): Stochastic lookback length
- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period

## Formula


\[
Filt = \text{RoofingFilter}(Price, P_{hp}, P_{ss})
\]
\[
Stoc = \frac{Filt - \min(Filt, L)}{\max(Filt, L) - \min(Filt, L)}
\]
\[
MESAStoch = \text{SuperSmoother}(Stoc \times 100, P_{ss})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating%20Turning%20Points.pdf)
