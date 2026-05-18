# Relative Strength Index (RSI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">classic</span></div>

A momentum oscillator that measures the speed and change of price movements.

## Usage

Use to identify overbought (>70) and oversold (<30) conditions. RSI divergences against price often signal impending trend reversals.

## Background

> Developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), the RSI compares the magnitude of recent gains to recent losses to determine overbought and oversold conditions of an asset. It remains the most widely used momentum oscillator in modern technical analysis.

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
RS = \frac{Average Gain}{Average Loss} \\ RSI = 100 - \frac{100}{1 + RS}
\]


[Source](https://www.investopedia.com/terms/r/rsi.asp)
