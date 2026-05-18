# True Range

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">atr</span> <span class="kw-badge">classic</span> <span class="kw-badge">range</span></div>

True Range measures daily volatility.

## Usage

Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels.

## Background

> Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.

## Formula


\[
TR = \max(H - L, |H - C_{t-1}|, |L - C_{t-1}|)
\]


[Source](https://www.investopedia.com/terms/a/atr.asp)
