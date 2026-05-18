# Classic Laguerre Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">laguerre</span></div>

The original Laguerre filter from John Ehlers' 2002 'Time Warp' paper.

## Usage

Use when a smooth trend estimate with controllable lag using only 4 state variables is needed. Preferred over long EMAs when computational memory is constrained.

## Background

> The Classic Laguerre Filter uses four first-order IIR sections sharing the same gamma coefficient. In Cybernetic Analysis (2004) Ehlers shows gamma maps directly to an effective period, making it highly tunable with minimal computation.

## Parameters

- `gamma` (default: 0.8): Smoothing factor (0.0 to 1.0)

## Formula


\[
L_0 = (1 - \gamma) \cdot Price + \gamma \cdot L_{0,t-1}
\]
\[
L_1 = -\gamma L_0 + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
L_2 = -\gamma L_1 + L_{1,t-1} + \gamma L_{2,t-1}
\]
\[
L_3 = -\gamma L_2 + L_{2,t-1} + \gamma L_{3,t-1}
\]
\[
Filt = \frac{L_0 + 2L_1 + 2L_2 + L_3}{6}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TimeWarp.pdf)
