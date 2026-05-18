# Open-Close Average (OC2)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">dsp</span></div>

A simple average of the Open and Close prices.

## Usage

Use to reduce noise in technical indicators. Based on John Ehlers' recent research, averaging the open and close can significantly improve signal-to-noise ratios in DSP-based indicators.

## Background

> In his 2023 paper 'Every Little Bit Helps', John Ehlers demonstrates that using the average of the Open and Close as an input can enhance the performance of various filters and oscillators by providing a cleaner signal with reduced aliasing. — John Ehlers

## Formula


\[
OC2 = \frac{Open + Close}{2}
\]


[Source](Every Little Bit Helps (John Ehlers, 2023))
