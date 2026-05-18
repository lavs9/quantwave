# AutoTune Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">filter</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">autotune</span></div>

An adaptive BandPass filter that dynamically tunes itself to the market's dominant cycle.

## Usage

Use to isolate the cyclical component of price while automatically adapting to changes in cycle length. Zero crossings of the output or its rate of change can be used as trading signals.

## Background

> The AutoTune filter provides a bridge between the time domain and frequency domain by using a rolling autocorrelation function to measure the Dominant Cycle in real time. By dynamically tuning a Bandpass filter to twice the lag at which autocorrelation is minimized, it maintains consistent performance and avoids the destructive phase shifts typical of fixed-tuned filters.

## Parameters

- `window` (default: 20): Window length for autocorrelation and HighPass filter
- `bandwidth` (default: 0.25): Bandwidth of the tuned BandPass filter

## Formula


\[
R(lag) = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{(n \sum X_i^2 - (\sum X_i)^2)(n \sum Y_i^2 - (\sum Y_i)^2)}}
\]
\[
DC = 2 \times \text{argmin}_{lag} R(lag)
\]
\[
BP = \text{BandPass}(Price, DC, BW)
\]


[Source](references/Ehlers Papers/The AutoTune Filter.pdf)
