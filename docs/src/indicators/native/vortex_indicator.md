# Vortex Indicator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

The Vortex Indicator helps identify the start of a new trend or the continuation of an existing one.

## Usage

Use to detect the start of new trends. A Vortex Indicator crossover (VI+ crossing above VI-) signals the beginning of an uptrend; the reverse signals a downtrend.

## Background

> The Vortex Indicator, developed by Etienne Botes and Douglas Siepman (2010), is inspired by the vortex flow of water discovered by Viktor Schauberger. VI+ measures upward movement relative to the prior bar low; VI- measures downward movement relative to the prior bar high. Normalized by ATR, they produce two oscillating lines whose crossovers signal trend changes. — Technical Analysis of Stocks and Commodities, 2010

## Parameters

- `period` (default: 14): Period

## Formula


\[
VI+ = \frac{\sum VM+}{\sum TR} \\ VI- = \frac{\sum VM-}{\sum TR}
\]


[Source](https://www.investopedia.com/terms/v/vortex-indicator-vi.asp)
