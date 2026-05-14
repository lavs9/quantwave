# Volume Profile

<div class="indicator-meta"><span class="category-badge">Volume</span> <span class="kw-badge">volume</span> <span class="kw-badge">profile</span> <span class="kw-badge">poc</span> <span class="kw-badge">support-resistance</span> <span class="kw-badge">auction-market-theory</span></div>

Calculates the price level with the highest traded volume (Point of Control) over a sliding window.

## Usage

Use to identify significant support and resistance levels. The POC represents the price where most market activity occurred, often acting as a magnet for price or a strong barrier. Essential for volume spread analysis and auction market theory.

## Background

> Volume Profile is an advanced charting study that displays trading activity over a specified time period at specified price levels. The Point of Control (POC) is the single most important level in the profile, representing the price at which the most volume was traded. It serves as a key benchmark for identifying value areas and potential trend reversals.

## Parameters

- `period` (default: 200): Sliding window size
- `bins` (default: 50): Number of price bins in the histogram

## Formula


\[
BinIdx = \lfloor \frac{Price - Price_{min}}{BinSize} \rfloor
\]
\[
POC = Price_{min} + (Idx_{max\_vol} + 0.5) \times BinSize
\]


[Source](https://www.tradingview.com/support/solutions/43000502040-volume-profile-visible-range-vpvr/)
