# Average Price (AVGPRICE)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">classic</span> <span class="kw-badge">smoothing</span></div>

The simple average of the Open, High, Low, and Close prices for a given period.

## Usage

Use as a smoothed price input for other indicators. It provides a more balanced view of the period's price action than the Close price alone.

## Background

> Average Price is the arithmetic mean of the four key price points in a bar. In technical analysis, using Average Price instead of Close can help filter out erratic price spikes and provide a more stable foundation for trend-following algorithms. — TA-Lib Documentation

## Formula


\[
AVGPRICE = \frac{Open + High + Low + Close}{4}
\]


[Source](https://www.tradingview.com/support/solutions/43000502588-average-price-avgprice/)
