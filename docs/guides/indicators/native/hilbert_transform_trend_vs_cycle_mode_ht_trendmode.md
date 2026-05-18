# Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">trend</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">regime-detection</span> <span class="kw-badge">dsp</span></div>

A binary indicator that determines if the market is currently in a trending state (1) or a cyclical state (0).

## Usage

Use as a master filter for strategy selection. Deploy trend-following tools when TRENDMODE is 1, and mean-reversion tools when TRENDMODE is 0.

## Background

> Determining the current market regime is the 'holy grail' of technical analysis. The HT_TRENDMODE indicator uses the rate of change of the dominant cycle phase to distinguish between trending and ranging price action, allowing traders to avoid 'whipsaws' in non-conducive environments. — Rocket Science for Traders

## Formula


\[
\text{TRENDMODE} = \begin{cases} 1 & \text{if trend detected} \\ 0 & \text{if cycle detected} \end{cases}
\]


[Source](https://www.tradingview.com/support/solutions/43000502014-hilbert-transform-trend-vs-cycle-mode-ht-trendmode/)
