# Homodyne Discriminator

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">cycle</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">spectral</span></div>

Estimates the dominant cycle period using a homodyne approach.

## Usage

Use to measure the instantaneous dominant cycle period from price data. Feed its output into adaptive indicators as the dynamic period parameter.

## Background

> Described in Rocket Science for Traders (2001), the Homodyne Discriminator borrows from radio engineering to measure instantaneous frequency by multiplying the analytic signal by its one-bar-delayed conjugate, giving cycle period without DFT latency.

## Formula


\[
\text{Period} = \frac{360}{\text{atan}(Im / Re)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf)
