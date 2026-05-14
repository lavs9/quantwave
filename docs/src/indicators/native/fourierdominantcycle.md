# FourierDominantCycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">fourier</span></div>

Dominant cycle period estimation using resolution-enhanced DFT and center of gravity.

## Usage

Use to compute the dominant market cycle period via DFT. Feed the output period into adaptive indicators like DSMA or Ehlers Stochastic to make them cycle-synchronized.

## Background

> Ehlers implements a Discrete Fourier Transform cycle measurement in Cybernetic Analysis using a Hann-windowed data segment. The DFT computes power across periods from 6 to 50 bars, and the peak power identifies the dominant cycle period driving price movement.

## Parameters

- `window_len` (default: 50): DFT window length

## Formula


\[
HP = \text{HighPass}(Price, 40)
\]
\[
Cleaned = \frac{HP + 2HP_{t-1} + 3HP_{t-2} + 3HP_{t-3} + 2HP_{t-4} + HP_{t-5}}{12}
\]
\[
Pwr(P) = \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \cos\left(\frac{2\pi n}{P}\right)\right)^2 + \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \sin\left(\frac{2\pi n}{P}\right)\right)^2
\]
\[
DB(P) = \min\left(20, -10 \log_{10}\left(\frac{0.01}{1 - 0.99 \frac{Pwr(P)}{\max(Pwr)}}\right)\right)
\]
\[
DC = \frac{\sum_{P=8}^{50} P \cdot (3 - DB(P)) \text{ where } DB(P) < 3}{\sum (3 - DB(P))}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FourierTransformForTraders.pdf)
