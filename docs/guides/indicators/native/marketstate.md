# MarketState

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">cycle</span> <span class="kw-badge">regime</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

Identifies trend vs cycle regimes using Correlation Cycle phase angle.

## Usage

Returns 1 for uptrend, -1 for downtrend, and 0 for cycle mode. Use to switch between trend-following and mean-reversion strategies.

## Background

> In 'Correlation As A Cycle Indicator' (2020), Ehlers defines a Market State variable based on the rate of change of the Correlation Cycle phase angle. When the angle changes slowly (less than 9 degrees per bar), the market is in a trend regime (positive angle for uptrend, negative for downtrend). Rapid angle changes indicate a cycle regime.

## Parameters

- `period` (default: 14): Correlation wavelength
- `threshold` (default: 9.0): Angle rate of change threshold for trend detection

## Formula


\[
\text{State} = 
\begin{cases} 
1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} \geq 0 \\
-1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} < 0 \\
0 & \text{otherwise}
\end{cases}
\]


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2020/06/TradersTips.html)
