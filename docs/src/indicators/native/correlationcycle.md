# CorrelationCycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">spectral</span></div>

Determines cycle phase angle by correlating price with orthogonal sinusoids.

## Usage

Use to measure the dominant cycle period via autocorrelation in an amplitude-independent way. Prefer over DFT methods when price amplitude varies significantly across the measurement window.

## Background

> Ehlers introduces Correlation Cycle measurement in Cycle Analytics for Traders (2013) as an improvement on DFT. By normalizing autocorrelation coefficients to unity variance, the resulting periodogram is independent of price amplitude variations, producing more consistent cycle period estimates.

## Parameters

- `period` (default: 20): Correlation wavelength

## Formula


\[
R = \text{Corr}(Price, \cos(2\pi n/P)), I = \text{Corr}(Price, -\sin(2\pi n/P))
\]
\[
\text{Angle} = 90 + \arctan(R/I) \text{ (with quadrant resolution)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/CORRELATION%20AS%20A%20CYCLE%20INDICATOR.pdf)
