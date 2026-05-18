# Heikin-Ashi

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">candlestick</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">visualization</span></div>

Heikin-Ashi candles filter market noise to reveal the underlying trend.

## Usage

Use to smooth candlestick charts and reduce noise for trend identification. Two or more consecutive same-colored HA candles with no lower/upper wicks confirm a strong trend.

## Background

> Heikin-Ashi candles, developed by Munehisa Homma in the 18th century, use averaged OHLC values to produce smoother candles that better represent the prevailing trend. Each HA bar open equals the midpoint of the previous HA bar, while close equals the OHLC average, creating continuity that raw candles lack. — StockCharts ChartSchool

## Formula


\[
HA_{Close} = \frac{O + H + L + C}{4} \\ HA_{Open} = \frac{HA_{Open, t-1} + HA_{Close, t-1}}{2}
\]


[Source](https://www.investopedia.com/trading/heikin-ashi-better-candlestick/)
