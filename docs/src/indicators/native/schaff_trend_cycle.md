# Schaff Trend Cycle

A hybrid indicator that applies a double-smoothed stochastic to MACD for faster trend identification.

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
