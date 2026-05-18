# Money Flow Index (MFI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">volume</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span></div>

A technical oscillator that uses price and volume data for identifying overbought or oversold signals.

## Usage

Use as a volume-weighted RSI. Divergences between MFI and price can signal potential reversals, especially when the MFI is in extreme territory (>80 or <20).

## Background

> The Money Flow Index (MFI) is a momentum indicator that measures the inflow and outflow of money into an asset over a specific period of time. It is related to the RSI but incorporates volume, whereas the RSI only considers price. — Investopedia

## Parameters

- `timeperiod` (default: 14): Lookback period

## Formula


\[
\text{Money Ratio} = \frac{\text{Positive Money Flow}}{\text{Negative Money Flow}} \\ MFI = 100 - \frac{100}{1 + \text{Money Ratio}}
\]


[Source](https://www.investopedia.com/terms/m/mfi.asp)
