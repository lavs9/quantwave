# TTM Squeeze

TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.

## Parameters

- `bb_period` (default: 20): Bollinger Bands Period
- `bb_mult` (default: 2.0): Bollinger Bands Multiplier
- `kc_period` (default: 20): Keltner Channel Period
- `kc_mult` (default: 1.5): Keltner Channel Multiplier

## Formula


\[
\text{Squeeze} = BB_{width} < KC_{width}
\]


[Source](https://www.investopedia.com/articles/active-trading/110714/intro-ttm-squeeze-indicator.asp)
