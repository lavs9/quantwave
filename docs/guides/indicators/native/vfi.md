# VFI

<div class="indicator-meta"><span class="category-badge">Volume Indicators</span> <span class="kw-badge">volume</span> <span class="kw-badge">vfi</span> <span class="kw-badge">money-flow</span> <span class="kw-badge">katsanos</span> <span class="kw-badge">oscillator</span></div>

Volume Flow Indicator - a volume-based indicator that uses price and volume relative to a cutoff to measure money flow.

## Usage

Used to identify trend direction and potential reversals. Values above 0 are bullish, below 0 are bearish. Extreme readings and divergences are also significant.

## Background

> Katsanos' Volume Flow Indicator (VFI) is based on the popular On Balance Volume (OBV) but with three main modifications: it is bounded, it filters out small price changes, and it caps volume extremes. It provides a more balanced view of buying and selling pressure by accounting for price volatility and volume outliers. — TASC June 2004

## Parameters

- `period` (default: 130): Lookback period for Vave and Summation
- `coef` (default: 0.2): Coefficient for minimal price cut-off
- `vcoef` (default: 2.5): Coefficient for volume cut-off
- `smoothing_period` (default: 3): EMA period for final smoothing

## Formula


\begin{aligned}
TP &= \frac{H+L+C}{3} \\
Inter &= \ln(TP) - \ln(TP_{t-1}) \\
VInter &= StdDev(Inter, 30) \\
Cutoff &= Coef \cdot VInter \cdot Close \\
Vave &= SMA(Volume, Period)_{t-1} \\
Vmax &= Vave \cdot Vcoef \\
VC &= \min(Volume, Vmax) \\
MF &= TP - TP_{t-1} \\
VCP &= \begin{cases} VC, & \text{if } MF > Cutoff \\ -VC, & \text{if } MF < -Cutoff \\ 0, & \text{otherwise} \end{cases} \\
VFI_{raw} &= \frac{\sum_{i=0}^{Period-1} VCP_{t-i}}{Vave} \\
VFI &= EMA(VFI_{raw}, 3)
\end{aligned}


[Source](https://www.traders.com/Documentation/FEEDbk_docs/2022/04/TradersTips.html)
