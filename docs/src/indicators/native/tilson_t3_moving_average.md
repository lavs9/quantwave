# Tilson T3 Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">lag-reduction</span> <span class="kw-badge">classic</span></div>

A smooth, low-lag moving average that uses multiple exponential smoothing.

## Usage

Use for trend following in noisy markets. T3 offers a superior balance between lag reduction and smoothness compared to DEMA or TEMA.

## Background

> Developed by Tim Tilson in 1998, the T3 moving average uses a 'v-factor' (volume factor) to control how much the indicator reacts to price changes. It is essentially a sextuple EMA smoothing process that provides a very smooth curve with remarkably little lag. — Technical Analysis of Stocks & Commodities

## Parameters

- `timeperiod` (default: 5): Smoothing period
- `v_factor` (default: 0.7): Volume factor (0.0 to 1.0)

## Formula


\[
e1 = EMA(Price, n) \\ e2 = EMA(e1, n) \\ \dots \\ e6 = EMA(e5, n) \\ T3 = c1 \times e6 + c2 \times e5 + c3 \times e4 + c4 \times e3
\]


[Source](https://www.tradingview.com/script/667W2a8n-T3-Moving-Average/)
