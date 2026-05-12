# Keltner Channels

Keltner Channels are volatility-based envelopes set above and below an exponential moving average.

## Parameters

- `period` (default: 20): EMA Period
- `multiplier` (default: 2.0): ATR Multiplier

## Formula


\[
UC = EMA + (Multiplier \times ATR)
\]


[Source](https://www.investopedia.com/terms/k/keltnerchannel.asp)
