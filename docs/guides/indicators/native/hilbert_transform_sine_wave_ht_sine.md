# Hilbert Transform - Sine Wave (HT_SINE)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">sine</span> <span class="kw-badge">dsp</span></div>

An indicator that plots a sine wave and a lead-sine wave (shifted by 45 degrees) to identify cyclical turns.

## Usage

Use to identify cycle turning points and trend regimes. When the two waves are separated and rhythmic, the market is in a cycle; when they are compressed or crossover erratically, the market is in a trend.

## Background

> The Hilbert Sine Wave is one of John Ehlers' most famous contributions. It provides a clear visual indication of market cycles. Crossovers of the Sine and Lead-Sine waves provide high-probability entry points in ranging markets while identifying when a strong trend has taken over. — Rocket Science for Traders

## Formula


\[
Sine = \sin(Phase) \\ LeadSine = \sin(Phase + 45^\circ)
\]


[Source](https://www.tradingview.com/support/solutions/43000502013-hilbert-transform-sine-wave-ht-sine/)
