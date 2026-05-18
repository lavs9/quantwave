# Ultimate Strength Index

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">strength</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">momentum</span></div>

A lag-reduced version of the RSI using UltimateSmoother on smoothed up/down components.

## Usage

Use to measure the relative strength of the current market move normalized to the dominant cycle amplitude, giving a volatility-adjusted momentum reading.

## Background

> The Ultimate Strength Index measures directional momentum as a fraction of the total cycle amplitude. By normalizing momentum to the RMS energy of the dominant cycle, it produces a consistent 0-100 reading that is comparable across different instruments and volatility regimes.

## Parameters

- `length` (default: 14): UltimateSmoother period

## Formula


\[
\text{SU} = \max(0, \text{Close} - \text{Close}_{t-1})
\]
\[
\text{SD} = \max(0, \text{Close}_{t-1} - \text{Close})
\]
\[
\text{USU} = UltimateSmoother(SMA(\text{SU}, 4), Length)
\]
\[
\text{USD} = UltimateSmoother(SMA(\text{SD}, 4), Length)
\]
\[
\text{USI} = \frac{\text{USU} - \text{USD}}{\text{USU} + \text{USD}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20NOVEMBER%202024.html)
