# Beta (BETA)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">statistics</span> <span class="kw-badge">risk</span> <span class="kw-badge">classic</span> <span class="kw-badge">volatility</span></div>

A measure of a security's volatility in relation to the overall market.

## Usage

Use to understand the systematic risk of an asset. A beta of 1.0 indicates the asset moves with the market; >1.0 means it is more volatile, and <1.0 means it is less volatile.

## Background

> Beta is a measure of the volatility—or systematic risk—of a security or portfolio compared to the market as a whole. It is used in the Capital Asset Pricing Model (CAPM) to calculate the expected return of an asset based on its beta and expected market returns. — Investopedia

## Parameters

- `timeperiod` (default: 30): Lookback period

## Formula


\[
\beta = \frac{\text{Cov}(R_i, R_m)}{\text{Var}(R_m)}
\]


[Source](https://www.investopedia.com/terms/b/beta.asp)
