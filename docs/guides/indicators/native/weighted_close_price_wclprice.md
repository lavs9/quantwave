# Weighted Close Price (WCLPRICE)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">classic</span> <span class="kw-badge">weighted</span></div>

An average of the High, Low, and Close prices, with double weight given to the Close price.

## Usage

Use to emphasize the importance of the closing price while still accounting for the total range of the bar.

## Background

> Weighted Close Price gives additional significance to the Close, reflecting the widely held belief that the closing price is the most important data point in a trading session. It provides a more nuanced input for smoothing algorithms. — TA-Lib Documentation

## Formula


\[
WCLPRICE = \frac{High + Low + 2 \times Close}{4}
\]


[Source](https://www.tradingview.com/support/solutions/43000502590-weighted-close-wclprice/)
