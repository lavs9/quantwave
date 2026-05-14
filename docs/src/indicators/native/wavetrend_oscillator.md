# WaveTrend Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">momentum</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">ehlers</span></div>

WaveTrend is an oscillator that helps identify overbought and oversold conditions.

## Usage

Use as a momentum oscillator to identify overbought and oversold conditions. WaveTrend crossovers at extreme levels provide high-probability mean-reversion entry signals.

## Background

> WaveTrend (popularized as LazyBear WaveTrend on TradingView) computes a channel index by normalizing price deviation from an EMA by the smoothed absolute deviation. A second EMA of this index produces the signal line. Extreme values (±60) with WT1-WT2 crossovers are the classic trade trigger.

## Parameters

- `n1` (default: 10): Channel Length
- `n2` (default: 21): Average Length

## Formula


\[
WT_1 = EMA(ESA, n_2)
\]


[Source](https://www.tradingview.com/script/2KE8wTuF-Indicator-WaveTrend-Oscillator-WT/)
