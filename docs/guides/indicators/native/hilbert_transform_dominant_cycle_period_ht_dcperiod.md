# Hilbert Transform - Dominant Cycle Period (HT_DCPERIOD)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">dsp</span></div>

Identifies the period of the dominant cycle in the price data using the Hilbert Transform.

## Usage

Use to dynamically adjust the lookback periods of other indicators (e.g., adaptive moving averages). Knowing the current dominant cycle length allows for more accurate smoothing and trend detection.

## Background

> John Ehlers popularized the use of the Hilbert Transform to identify the dominant cycle in financial time series. The DCPERIOD indicator tracks the length of this cycle in bars, providing a crucial parameter for creating market-responsive technical indicators that adapt to changing volatility. — Rocket Science for Traders

## Formula


\[
\text{DCPERIOD}_t = \text{Recalculated Dominant Cycle using Hilbert Transform}
\]


[Source](https://www.tradingview.com/support/solutions/43000502011-hilbert-transform-dominant-cycle-period-ht-dcperiod/)
