# On-Balance Volume (OBV)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volume</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">accumulation</span> <span class="kw-badge">distribution</span></div>

A momentum indicator that uses volume flow to predict changes in stock price.

## Usage

Use to identify accumulation by institutions. When price is flat but OBV is rising, a breakout to the upside is likely. Conversely, when price is flat but OBV is falling, a breakdown is likely.

## Background

> Introduced by Joe Granville in his 1963 book 'Granville's New Key to Stock Market Profits', OBV is one of the oldest and most respected volume indicators. It operates on the principle that volume precedes price, and that institutional money flow leaves a detectable trail in the volume data before the price move occurs. — StockCharts ChartSchool

## Formula


\[
OBV_t = OBV_{t-1} + \begin{cases} Volume & \text{if } Close_t > Close_{t-1} \\ 0 & \text{if } Close_t = Close_{t-1} \\ -Volume & \text{if } Close_t < Close_{t-1} \end{cases}
\]


[Source](https://www.investopedia.com/terms/o/onbalancevolume.asp)
