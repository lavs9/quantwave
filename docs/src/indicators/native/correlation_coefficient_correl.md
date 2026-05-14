# Correlation Coefficient (CORREL)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">statistics</span> <span class="kw-badge">correlation</span> <span class="kw-badge">classic</span></div>

A statistical measure that determines the degree to which two securities move in relation to each other.

## Usage

Use to measure the strength and direction of the linear relationship between two assets. Values range from -1.0 (inverse correlation) to +1.0 (perfect correlation).

## Background

> The Pearson Correlation Coefficient measures the strength and direction of a linear relationship between two price series. It is a fundamental tool for pair trading and portfolio diversification, allowing traders to quantify how much of a security's movement is explained by another. — StockCharts ChartSchool

## Parameters

- `timeperiod` (default: 30): Lookback period

## Formula


\[
\rho_{X,Y} = \frac{\text{cov}(X,Y)}{\sigma_X \sigma_Y}
\]


[Source](https://www.investopedia.com/terms/c/correlationcoefficient.asp)
