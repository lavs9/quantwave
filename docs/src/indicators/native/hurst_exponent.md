# Hurst Exponent

<div class="indicator-meta"><span class="category-badge">ML Features</span> <span class="kw-badge">statistics</span> <span class="kw-badge">regime-detection</span> <span class="kw-badge">hurst</span> <span class="kw-badge">ml</span> <span class="kw-badge">trending</span> <span class="kw-badge">mean-reversion</span></div>

Measures the persistence or anti-persistence of a time series using R/S analysis.

## Usage

Use to classify the current market regime. H > 0.5 suggests a trending market (persistent); H < 0.5 suggests a mean-reverting market (anti-persistent). Useful as a filter for trend-following or mean-reversion strategies.

## Background

> The Hurst Exponent, pioneered by Harold Edwin Hurst in 1951, quantifies the 'memory' of a time series. In technical analysis, it distinguishes between trending, mean-reverting, and random walk price action. It is a critical feature for machine learning models to adapt their logic to the underlying market structure.

## Parameters

- `period` (default: 100): Lookback period for R/S analysis

## Formula


\[
H = \frac{\ln(R/S)}{\ln(N)}
\]


[Source](https://en.wikipedia.org/wiki/Hurst_exponent)
