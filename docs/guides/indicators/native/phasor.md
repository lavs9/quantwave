# Phasor

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">cycle</span> <span class="kw-badge">phase</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Extracts In-Phase (I) and Quadrature (Q) components using a Hilbert Transform.

## Usage

Use to measure the instantaneous phase and amplitude of the dominant market cycle. Phase crossings of key angles (90, 180 degrees) provide precise cycle turn timing signals.

## Background

> Ehlers borrows the concept of a phasor from electrical engineering to represent the amplitude and phase of a market cycle as a rotating vector. In Rocket Science for Traders (2001) he shows how measuring the instantaneous phasor angle gives more precise cycle timing than zero-crossing methods.

## Formula


\[
I = \text{Detrender}_{t-3}
\]
\[
Q = \text{HilbertFIR}(\text{Detrender}, \text{Period})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf)
