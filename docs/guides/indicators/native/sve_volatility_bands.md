# SVE Volatility Bands

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">bands</span> <span class="kw-badge">volatility</span> <span class="kw-badge">renko</span> <span class="kw-badge">vervoort</span></div>

Volatility bands designed to highlight volatility changes especially when using non-time-related charts like Renko.

## Usage

Use to identify extreme price excursions and volatility contraction/expansion. The bands adapt to volatility using a smoothed ATR-like calculation.

## Background

> Introduced by Sylvain Vervoort, SVE Volatility Bands use a weighted moving average of price and a smoothed True Range to create dynamic bands. It includes a specific adjustment for the lower band and a midline based on typical price.

## Parameters

- `bands_period` (default: 20): Period for the price WMA and the ATR smoothing basis.
- `bands_deviation` (default: 2.4): Multiplier for the volatility range.
- `low_band_adjust` (default: 0.9): Adjustment factor for the lower band.
- `mid_line_length` (default: 20): Period for the midline WMA.

## Formula


\[
ATR\_MA = SMA(TrueRange, bands\_period \times 2 - 1) \\
WtdAvgVal = WMA(Close, bands\_period) \\
Upper = WtdAvgVal \times (1 + (ATR\_MA \times bands\_deviation) / Close) \\
Lower = WtdAvgVal \times (1 - (ATR\_MA \times bands\_deviation \times low\_band\_adjust) / Close) \\
MidLine = WMA(TypicalPrice, mid\_line\_length)
\]


[Source](Technical Analysis of Stocks & Commodities, January 2019)
