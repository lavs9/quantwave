# Ehlers Stochastic

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">stochastic</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">cycle</span> <span class="kw-badge">adaptive</span></div>

A Stochastic oscillator applied to the output of a Roofing Filter to eliminate Spectral Dilation.

## Usage

Use as a cycle-aware stochastic oscillator that adapts its lookback window to the current dominant cycle period rather than using a fixed period.

## Background

> Ehlers computes the stochastic oscillator using the measured dominant cycle period as the lookback window. This adaptive approach ensures the stochastic spans exactly one full market cycle, making overbought and oversold conditions consistently meaningful.

## Parameters

- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period
- `stoch_period` (default: 20): Stochastic lookback period

## Formula


\[
Roof = RoofingFilter(HP, SS)
\]
\[
Stoch = 100 \times \frac{Roof - \min(Roof, L)}{\max(Roof, L) - \min(Roof, L)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating Turning Points.pdf)
