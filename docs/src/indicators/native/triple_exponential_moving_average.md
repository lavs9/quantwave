# Triple Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">ema</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

TEMA reduces the lag of traditional EMAs.

## Usage

Use to reduce the lag of a standard EMA by approximately two thirds. Drop-in replacement for EMA in trend-following systems where responsiveness is more important than smoothness.

## Background

> Patrick Mulloy introduced Triple EMA in Technical Analysis of Stocks and Commodities (1994) as a practical lag-reduction technique. TEMA = 3*EMA - 3*EMA(EMA) + EMA(EMA(EMA)), subtracting out two orders of the EMA lag while preserving most of the noise reduction.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
TEMA = (3 \times EMA_1) - (3 \times EMA_2) + EMA_3
\]


[Source](https://www.investopedia.com/terms/t/triple-exponential-moving-average.asp)
