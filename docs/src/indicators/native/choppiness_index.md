# Choppiness Index

Determines if the market is trending (low values) or ranging/choppy (high values).

## Parameters

- `period` (default: 14): Lookback period

## Formula


\[
CHOP = 100 \times \frac{\log_{10}(\sum_{i=1}^n ATR(1)_i / (\max(H, n) - \min(L, n)))}{\log_{10}(n)}
\]


[Source](https://www.tradingview.com/support/solutions/43000501980-choppiness-index-chop/)
