# One Euro Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">real-time</span> <span class="kw-badge">low-pass</span></div>

A speed-based adaptive low-pass filter that dynamically adjusts its smoothing coefficient.

## Usage

Use in real-time systems where you need low lag at high speeds and low noise at low speeds. The adaptive cutoff frequency makes it self-tuning for different signal velocities.

## Background

> The One Euro Filter, developed by Casiez et al. (2012), is an adaptive lowpass filter that adjusts its cutoff frequency based on the signal derivative. When the signal changes quickly (high speed) the cutoff is raised to reduce lag; when it changes slowly the cutoff is lowered to reduce noise — automatically balancing the speed-accuracy trade-off.

## Parameters

- `period_min` (default: 10): Minimum cutoff period
- `beta` (default: 0.2): Responsiveness factor

## Formula


\[
\alpha_{dx} = \frac{2\pi}{4\pi + 10}
\]
\[
SmoothedDX = \alpha_{dx}(Price - Price_{t-1}) + (1 - \alpha_{dx})SmoothedDX_{t-1}
\]
\[
Cutoff = PeriodMin + \beta |SmoothedDX|
\]
\[
\alpha_3 = \frac{2\pi}{4\pi + Cutoff}
\]
\[
Smoothed = \alpha_3 Price + (1 - \alpha_3)Smoothed_{t-1}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20DECEMBER%202025.html)
