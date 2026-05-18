# Ichimoku Cloud

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">support-resistance</span> <span class="kw-badge">classic</span> <span class="kw-badge">japanese</span> <span class="kw-badge">momentum</span></div>

Ichimoku Kinko Hyo is a comprehensive indicator that defines support and resistance, identifies trend direction, gauges momentum and provides trading signals.

## Usage

Use as a complete trend system providing support, resistance, momentum, and cloud-based bias in a single indicator. The Kumo cloud thickness indicates trend strength.

## Background

> Ichimoku Kinko Hyo was developed by Goichi Hosoda in the 1960s. The system comprises five components: Tenkan-sen (9-period midpoint), Kijun-sen (26-period midpoint), Senkou Span A and B (cloud), and Chikou Span (lagged close). Price above the cloud is bullish; the cloud thickness quantifies the strength of support or resistance. — Ichimoku Charts, Nicole Elliott

## Parameters

- `tenkan_period` (default: 9): Tenkan-sen period
- `kijun_period` (default: 26): Kijun-sen period
- `senkou_span_b_period` (default: 52): Senkou Span B period

## Formula


\[
\text{Tenkan-sen} = \frac{\text{Highest High} + \text{Lowest Low}}{2} \text{ for past 9 periods}
\]


[Source](https://www.investopedia.com/terms/i/ichimoku-cloud.asp)
