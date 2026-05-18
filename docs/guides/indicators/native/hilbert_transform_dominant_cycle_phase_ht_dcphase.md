# Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">phase</span> <span class="kw-badge">dsp</span></div>

Calculates the phase angle (0 to 360 degrees) of the dominant cycle identified by the Hilbert Transform.

## Usage

Use to identify the current position within a market cycle. It is the core component for generating the Hilbert Sine Wave indicator, which signals trend vs. cycle regimes.

## Background

> The Dominant Cycle Phase represents the instantaneous position within a detected cycle. By measuring the phase angle, traders can determine if the market is at a peak, trough, or mid-cycle, enabling more precise timing for entry and exit signals. — Rocket Science for Traders

## Formula


\[
Phase = \arctan\left(\frac{\text{Quadrature}}{\text{InPhase}}\right)
\]


[Source](https://www.tradingview.com/support/solutions/43000502010-hilbert-transform-dominant-cycle-phase-ht-dcphase/)
