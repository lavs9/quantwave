# SuperTrend

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">atr</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.

## Usage

Use as a primary trend-following indicator and dynamic stop-loss. A SuperTrend flip from bearish to bullish (or vice versa) provides a clear, rule-based entry and exit signal.

## Background

> SuperTrend computes upper and lower ATR-based bands around the midpoint of each bar. The active line flips from upper to lower (and vice versa) only when price closes beyond the band, providing a clean directional bias and a trailing stop level in one indicator. — TradingView Community

## Parameters

- `period` (default: 10): ATR length
- `multiplier` (default: 3.0): ATR multiplier

## Formula


\[
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
\]


[Source](https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/)
