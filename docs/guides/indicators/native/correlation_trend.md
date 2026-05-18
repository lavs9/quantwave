# Correlation Trend

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">correlation</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">statistics</span></div>

Calculates the Pearson correlation between price and a linear time ramp to identify trends.

## Usage

Use to confirm whether price is trending or cycling before applying directional strategies. High correlation indicates a strong trend; low correlation indicates a cycling market.

## Background

> In 'Correlation As A Trend Indicator' (2020), Ehlers uses the Pearson correlation coefficient between price and a linear ramp to identify trend strength. A coefficient near +1.0 indicates a consistent uptrend, while -1.0 indicates a consistent downtrend. Unlike standard moving averages, this approach is independent of price amplitude and focuses purely on the linearity of the move.

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
