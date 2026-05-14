# Schaff Trend Cycle

<div class="indicator-meta"><span class="category-badge">Modern</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span></div>

A hybrid indicator that applies a double-smoothed stochastic to MACD for faster trend identification.

## Usage

Use as a faster trend-cycle momentum indicator. STC typically reaches overbought/oversold levels sooner than MACD while generating fewer false signals than a raw stochastic.

## Background

> The Schaff Trend Cycle, developed by Doug Schaff, applies the stochastic oscillator formula twice to MACD values rather than to price. This double stochastic smoothing produces faster, more defined overbought and oversold levels than MACD alone, while the cycle component reduces the lag of a conventional stochastic. — investopedia.com

## Parameters

- `cycle_period` (default: 10): Stochastic lookback period
- `fast_period` (default: 23): Fast EMA period for MACD
- `slow_period` (default: 50): Slow EMA period for MACD

## Formula


\[
MACD = EMA(23) - EMA(50)
\]
\[
STC = EMA(Stochastic(EMA(Stochastic(MACD, 10), 3), 10), 3)
\]


[Source](https://www.investopedia.com/articles/forex/10/schaff-trend-cycle-indicator.asp)
