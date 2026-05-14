# Bill Williams Fractals

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">pattern</span> <span class="kw-badge">support-resistance</span> <span class="kw-badge">classic</span> <span class="kw-badge">williams</span></div>

Fractals are indicators on candlestick charts that identify reversal points in the market.

## Usage

Use to mark potential support and resistance levels at local price extremes. Williams Fractals are commonly combined with Alligator lines to filter valid fractal signals.

## Background

> Bill Williams introduced Fractals in Trading Chaos (1995) as a pattern-recognition tool identifying local price extremes. A bullish fractal is a bar whose low is lower than the two bars on either side; a bearish fractal is a bar whose high is higher than the two bars on either side. Combined with the Alligator indicator, fractals provide entry triggers. — StockCharts ChartSchool

## Formula


\[
\text{Up Fractal} = \text{High} > \text{High}_{t-1, t-2, t+1, t+2}
\]


[Source](https://www.investopedia.com/terms/f/fractal.asp)
