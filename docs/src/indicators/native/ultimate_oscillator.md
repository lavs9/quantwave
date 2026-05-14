# Ultimate Oscillator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span> <span class="kw-badge">multi-timeframe</span></div>

A momentum oscillator designed to capture momentum across three different timeframes.

## Usage

Use to avoid the pitfalls of oscillators that are limited to a single timeframe. Buy signals are generated when there is bullish divergence between price and the indicator.

## Background

> Developed by Larry Williams in 1976, the Ultimate Oscillator uses weighted averages of three different timeframes to reduce the volatility and false signals common in other oscillators. It remains a staple for identifying divergence across short, medium, and long-term price action. — StockCharts ChartSchool

## Parameters

- `timeperiod1` (default: 7): Short period
- `timeperiod2` (default: 14): Medium period
- `timeperiod3` (default: 28): Long period

## Formula


\[
\text{BP} = \text{Close} - \min(\text{Low}, \text{PrevClose}) \\ \text{TR} = \max(\text{High}, \text{PrevClose}) - \min(\text{Low}, \text{PrevClose})
\]


[Source](https://www.investopedia.com/terms/u/ultimateoscillator.asp)
