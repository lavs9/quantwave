# ATR Trailing Stop

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">atr</span> <span class="kw-badge">classic</span></div>

A trailing stop based on Average True Range to keep trades in a trend.

## Usage

Use as a dynamic trailing stop that widens in volatile markets and tightens in calm ones, automatically adjusting stop distance to current market conditions.

## Background

> ATR Trailing Stop uses Average True Range to set a stop distance that scales with market volatility. During high-volatility regimes the stop moves further from price to avoid premature exit; during low-volatility regimes it tightens to lock in more profit. It is one of the most robust mechanical stop methods in systematic trading.

## Parameters

- `period` (default: 10): ATR period
- `multiplier` (default: 3.0): ATR Multiplier

## Formula


\[
Stop = P_{high} - (Multiplier \times ATR)
\]


[Source](https://www.tradingview.com/support/solutions/43000589105-average-true-range-atr/)
