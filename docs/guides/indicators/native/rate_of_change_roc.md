# Rate of Change (ROC)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">oscillator</span></div>

A momentum-based technical indicator that measures the percentage change in price between the current price and the price n periods ago.

## Usage

Use to measure the speed at which price is changing. It is often used to identify overbought/oversold conditions and trend reversals.

## Background

> The Rate of Change (ROC) indicator is a pure momentum oscillator that measures the percentage change in price from one period to the next. It is highly effective at identifying the velocity of a move and anticipating when that velocity is slowing down. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 10): Lookback period

## Formula


\[
ROC = \frac{Price_t - Price_{t-n}}{Price_{t-n}} \times 100
\]


[Source](https://www.investopedia.com/terms/r/rateofchange.asp)
