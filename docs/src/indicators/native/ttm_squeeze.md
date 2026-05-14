# TTM Squeeze

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">momentum</span> <span class="kw-badge">breakout</span> <span class="kw-badge">squeeze</span> <span class="kw-badge">classic</span></div>

TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.

## Usage

Use to identify periods of compressed volatility (Bollinger Bands inside Keltner Channels) followed by high-energy breakouts. The momentum histogram direction at squeeze release indicates trade direction.

## Background

> The TTM Squeeze, developed by John Carter, identifies market consolidation by detecting when Bollinger Bands contract inside Keltner Channels — a squeeze condition indicating coiling energy. When the bands expand back outside the Keltner Channels, the squeeze releases and a momentum histogram shows the expected breakout direction. — Mastering the Trade, John Carter

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
