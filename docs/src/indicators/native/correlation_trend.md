# Correlation Trend

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">correlation</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">statistics</span></div>

Calculates the Pearson correlation between price and a linear time ramp to identify trends.

## Usage

Use to confirm whether price is trending or cycling before applying directional strategies. High correlation indicates a strong trend; low correlation indicates a cycling market.

## Background

> Ehlers uses the correlation between price and the best-fit sine wave as a trend indicator in Cycle Analytics for Traders. A high correlation coefficient (near 1.0) means price closely follows a sine wave and is cycling; a low coefficient indicates the dominant market mode is a trend.

## Parameters

- `length` (default: 20): Correlation window length

## Formula


\[
X_i = Price_{t-i}, Y_i = -i
\]
\[
R = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{(n \sum X_i^2 - (\sum X_i)^2)(n \sum Y_i^2 - (\sum Y_i)^2)}}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/CORRELATION%20AS%20A%20TREND%20INDICATOR.pdf)
