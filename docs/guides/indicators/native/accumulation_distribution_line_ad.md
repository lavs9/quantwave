# Accumulation/Distribution Line (AD)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volume</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">accumulation</span> <span class="kw-badge">distribution</span></div>

A volume-based indicator designed to measure the cumulative flow of money into and out of a security.

## Usage

Use to confirm price trends or identify potential reversals through divergences. Rising AD confirms an uptrend; falling AD confirms a downtrend.

## Background

> Developed by Marc Chaikin, the AD line uses the relationship between price and volume to determine whether a security is being accumulated or distributed. It is calculated by multiplying the Money Flow Multiplier by the period's volume and adding it to a cumulative total. — StockCharts ChartSchool

## Formula


\[
\text{MFM} = \frac{(Close - Low) - (High - Close)}{High - Low} \\ \text{MFV} = \text{MFM} \times Volume \\ AD_t = AD_{t-1} + \text{MFV}
\]


[Source](https://www.investopedia.com/terms/a/accumulationdistributioncurve.asp)
