# SuperTrend

Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.

## Parameters
- `period` (default: 10): ATR length
- `multiplier` (default: 3.0): ATR multiplier

## Formula
$$
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
$$

[Source](https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/)
