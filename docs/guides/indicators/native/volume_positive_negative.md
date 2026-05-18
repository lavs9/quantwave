# Volume Positive Negative

<div class="indicator-meta"><span class="category-badge">Volume</span> <span class="kw-badge">volume</span> <span class="kw-badge">breakout</span> <span class="kw-badge">katsanos</span> <span class="kw-badge">vpn</span> <span class="kw-badge">momentum</span></div>

Detects high-volume breakouts by comparing volume on up days vs down days, normalized between -100 and 100.

## Usage

Use to confirm breakouts. A VPN value crossing above a critical threshold (e.g., 10) signals a high-volume positive breakout.

## Background

> While originally using EMA for smoothing, this implementation employs the UltimateSmoother to further reduce lag in detecting volume-driven trend shifts, aligning with modern DSP standards for technical indicators.

## Parameters

- `period` (default: 30): Calculation period for volume sums and ATR
- `smooth_period` (default: 3): Smoothing period for the final VPN value

## Formula


\[
TP = \frac{High + Low + Close}{3}
\]
\[
MF = TP - TP_{t-1}
\]
\[
MC = 0.1 \times ATR(Period)
\]
\[
VP = \sum_{i=0}^{Period-1} (\text{if } MF_{t-i} > MC_{t-i} \text{ then } Volume_{t-i} \text{ else } 0)
\]
\[
VN = \sum_{i=0}^{Period-1} (\text{if } MF_{t-i} < -MC_{t-i} \text{ then } Volume_{t-i} \text{ else } 0)
\]
\[
MAV = \text{Average}(Volume, Period)
\]
\[
VPN = \frac{VP - VN}{MAV \times Period} \times 100
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2021/04/TradersTips.html)
