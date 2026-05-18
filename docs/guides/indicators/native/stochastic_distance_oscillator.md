# Stochastic Distance Oscillator

<div class="indicator-meta"><span class="category-badge">Momentum</span> <span class="kw-badge">momentum</span> <span class="kw-badge">stochastic</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">apirine</span> <span class="kw-badge">trend</span></div>

A momentum indicator based on the classic stochastic oscillator applied to price distances.

## Usage

Identify bull and bear trend changes through overbought (+40) and oversold (-40) levels. Suitable for both trending and ranging markets.

## Background

> The Stochastic Distance Oscillator (SDO) by Vitali Apirine adapts the stochastic formula to measure the current price distance relative to its historical range. By smoothing this relative distance with an EMA, it provides a cleaner momentum signal that identifies potential trend reversals when crossing extreme thresholds.

## Parameters

- `lookback_period` (default: 200): Range lookback for stochastic calculation
- `period` (default: 12): Distance calculation period
- `ema_pds` (default: 3): Smoothing EMA period

## Formula


\[
Dist = |Price_t - Price_{t-n}|
\]
\[
DVal = \frac{Dist - \min(Dist_{lookback})}{\max(Dist_{lookback}) - \min(Dist_{lookback})}
\]
\[
DDVal = \begin{cases} DVal & \text{if } Price_t > Price_{t-n} \\ -DVal & \text{if } Price_t < Price_{t-n} \\ 0 & \text{otherwise} \end{cases}
\]
\[
SDO = EMA(DDVal, smoothing) \times 100
\]


[Source](https://traders.com/Documentation/FEEDbk_docs/2023/06/TradersTips.html)
