# Anchored VWAP

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">volume</span> <span class="kw-badge">classic</span> <span class="kw-badge">support-resistance</span></div>

Volume Weighted Average Price anchored to a specific starting point.

## Usage

Use as an intraday fair value benchmark. Institutional traders buy below VWAP and sell above it; breakouts above VWAP on heavy volume signal bullish institutional interest.

## Background

> Volume Weighted Average Price calculates the average price weighted by volume transacted at each level throughout the trading session. It serves as the primary execution benchmark for institutional orders — TWAP and VWAP algorithms are the two most common order execution strategies in equity markets. — Investopedia

## Formula


\[
VWAP = \frac{\sum (Price \times Volume)}{\sum Volume}
\]


[Source](https://www.investopedia.com/terms/v/vwap.asp)
