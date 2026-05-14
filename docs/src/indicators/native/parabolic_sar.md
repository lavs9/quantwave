# Parabolic SAR

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">wilder</span></div>

A trend-following indicator used to determine price direction and potential reversals.

## Usage

Use for setting trailing stop losses and identifying trend reversals. Dots below price indicate an uptrend, while dots above price indicate a downtrend.

## Background

> Developed by J. Welles Wilder, the Parabolic Stop and Reverse (SAR) uses an acceleration factor that increases as the trend persists. This 'parabolic' nature allows the indicator to stay close to price action and provide timely exit signals when a trend exhausts. — StockCharts ChartSchool

## Parameters

- `acceleration` (default: 0.02): Acceleration factor
- `maximum` (default: 0.2): Maximum acceleration

## Formula


\[
SAR_{t+1} = SAR_t + AF \times (EP - SAR_t)
\]


[Source](https://www.investopedia.com/terms/p/parabolicindicator.asp)
