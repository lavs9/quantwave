# Bollinger Bands

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">bands</span></div>

A volatility indicator consisting of a middle SMA and two outer bands based on standard deviation.

## Usage

Use to identify overbought/oversold levels and volatility breakouts. Prices near the upper band suggest overbought conditions, while prices near the lower band suggest oversold conditions. Narrowing bands (The Squeeze) often precede large price moves.

## Background

> Developed by John Bollinger in the 1980s, Bollinger Bands adapt to volatility by using standard deviation. The middle band is typically a 20-period SMA, and the outer bands are set 2 standard deviations away. This ensures that 95% of price action typically stays within the bands, making escapes highly significant. — BollingerOnBollingerBands.com

## Parameters

- `timeperiod` (default: 20): SMA period
- `nbdevup` (default: 2.0): Upper deviation multiplier
- `nbdevdn` (default: 2.0): Lower deviation multiplier

## Formula


\[
Middle = SMA(n) \\ Upper = Middle + (k \times \sigma) \\ Lower = Middle - (k \times \sigma)
\]


[Source](https://www.investopedia.com/terms/b/bollingerbands.asp)
