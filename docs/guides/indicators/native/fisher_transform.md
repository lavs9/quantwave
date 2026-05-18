# Fisher Transform

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">normalization</span> <span class="kw-badge">momentum</span></div>

Converts inputs to a nearly Gaussian probability distribution, creating sharp peaks at turning points.

## Usage

Apply to normalized prices or oscillators to sharpen turning-point signals. The near-Gaussian output makes extreme values statistically significant and easy to trade.

## Background

> Ehlers introduces the Fisher Transform in Cybernetic Analysis (2004) to convert any bounded indicator into a Gaussian normal distribution. Values beyond ±1.5 signal statistically significant price extremes, sharper than raw oscillators.

## Formula


\[
Fish(x) = 0.5 \times \ln\left(\frac{1 + x}{1 - x}\right) = \text{atanh}(x)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UsingTheFisherTransform.pdf)
