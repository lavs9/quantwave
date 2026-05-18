# Sine Wave

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">phase</span></div>

Plots a sine wave and a lead-sine wave based on the cyclic phase of price movement.

## Usage

Use to confirm whether the market is in cycle or trend mode. When price follows the sine wave trade cycle reversals; when it diverges switch to trend-following.

## Background

> Introduced in Rocket Science for Traders, the Sine Wave Indicator plots the sine and cosine of measured instantaneous phase. In cycling markets price tracks the sine wave; in trending markets price breaks through the lead line signaling a mode change.

## Formula


\[
\text{Sine} = \sin(\text{Phase})
\]
\[
\text{LeadSine} = \sin(\text{Phase} + 45^\circ)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf)
