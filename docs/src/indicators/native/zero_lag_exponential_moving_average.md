# Zero Lag Exponential Moving Average

ZLEMA attempts to eliminate the inherent lag associated with moving averages.

## Parameters

- `period` (default: 14): Smoothing period

## Formula


\[
ZLEMA = EMA(Price + (Price - Price_{t - (period - 1)/2}))
\]


[Source](https://en.wikipedia.org/wiki/Zero_lag_exponential_moving_average)
