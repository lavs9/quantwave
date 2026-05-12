# ATR Trailing Stop

A trailing stop based on Average True Range to keep trades in a trend.

## Parameters

- `period` (default: 10): ATR period
- `multiplier` (default: 3.0): ATR Multiplier

## Formula


\[
Stop = P_{high} - (Multiplier \times ATR)
\]


[Source](https://www.tradingview.com/support/solutions/43000589105-average-true-range-atr/)
