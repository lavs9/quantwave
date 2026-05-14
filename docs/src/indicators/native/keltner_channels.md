# Keltner Channels

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span> <span class="kw-badge">breakout</span> <span class="kw-badge">channels</span> <span class="kw-badge">classic</span></div>

Keltner Channels are volatility-based envelopes set above and below an exponential moving average.

## Usage

Use as volatility-adjusted envelope bands around an EMA. When Keltner Channels contract inside Bollinger Bands (the Squeeze), a high-energy breakout move is typically imminent.

## Background

> Keltner Channels, updated by Linda Raschke in the 1980s from Chester Keltner original design, use ATR to set channel width around an EMA. Unlike Bollinger Bands which use standard deviation, ATR-based channels adapt to average bar range rather than statistical volatility, producing smoother and more stable channel boundaries. — StockCharts ChartSchool

## Parameters

- `period` (default: 20): EMA Period
- `multiplier` (default: 2.0): ATR Multiplier

## Formula


\[
UC = EMA + (Multiplier \times ATR)
\]


[Source](https://www.investopedia.com/terms/k/keltnerchannel.asp)
