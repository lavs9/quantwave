# CorrelationCycle

Determines cycle phase angle by correlating price with orthogonal sinusoids.

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
