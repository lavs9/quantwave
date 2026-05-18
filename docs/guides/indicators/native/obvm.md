# OBVM

<div class="indicator-meta"><span class="category-badge">Volume Indicators</span> <span class="kw-badge">volume</span> <span class="kw-badge">obv</span> <span class="kw-badge">momentum</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">apirine</span></div>

On-Balance Volume Modified - a smoothed version of OBV with an additional signal line.

## Usage

Used to identify divergences between price and volume flow, and to generate signals via crossovers with its signal line. Values typically follow the trend of buying and selling pressure.

## Background

> While originally developed by Joe Granville, this modified version by Vitali Apirine applies exponential smoothing to the OBV values to filter out noise and adds a signal line for better trend identification and crossover signals. It provides a clearer picture of volume-price relationships by reducing high-frequency fluctuations. — TASC April 2020

## Parameters

- `obvm_period` (default: 7): EMA period for smoothing OBV
- `signal_period` (default: 10): EMA period for the signal line

## Formula


\begin{aligned}
TP &= \frac{High + Low + Close}{3} \\
OBV_t &= OBV_{t-1} + \begin{cases} Volume, & \text{if } TP_t > TP_{t-1} \\ -Volume, & \text{if } TP_t < TP_{t-1} \\ 0, & \text{otherwise} \end{cases} \\
OBVM &= EMA(OBV, Period_1) \\
Signal &= EMA(OBVM, Period_2)
\end{aligned}


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2020/04/TradersTips.html)
