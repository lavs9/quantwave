# Chande Momentum Oscillator (CMO)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span></div>

An advanced momentum oscillator developed by Tushar Chande that measures the difference between up and down days.

## Usage

Use to identify extreme overbought and oversold conditions. CMO is more sensitive to price action than RSI as it uses unsmoothed data in its internal calculations.

## Background

> Developed by Tushar Chande in 1994, the CMO is similar to the RSI but uses the net sum of up and down moves in both the numerator and denominator. This makes it more sensitive to price movements and useful for identifying short-term overextensions in the market. — The New Technical Trader

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
CMO = 100 \times \frac{S_u - S_d}{S_u + S_d}
\]


[Source](https://www.investopedia.com/terms/c/chandemomentumoscillator.asp)
