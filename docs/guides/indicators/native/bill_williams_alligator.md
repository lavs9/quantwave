# Bill Williams Alligator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span> <span class="kw-badge">williams</span></div>

Trend-following indicator using three delayed smoothed moving averages.

## Usage

Use to identify trend presence and direction. When the three Alligator lines are separated and fanning, the market is trending; when they converge or intertwine, the market is ranging.

## Background

> Bill Williams introduced the Alligator in Trading Chaos (1995) as three offset SMAs with periods 13, 8, and 5 and offsets of 8, 5, and 3 bars. The three lines represent the Jaw, Teeth, and Lips of the Alligator. When the Alligator is sleeping (lines intertwined) no trade is taken; when it wakes and opens its mouth a trend trade is entered. — StockCharts ChartSchool

## Formula


\[
\text{Jaw} = \text{SMMA}(13, 8), \text{Teeth} = \text{SMMA}(8, 5), \text{Lips} = \text{SMMA}(5, 3)
\]


[Source](https://chartschool.stockcharts.com/table-of-contents/technical-indicators-and-overlays/alligator)
