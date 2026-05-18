# System Evaluator

<div class="indicator-meta"><span class="category-badge">Statistics</span> <span class="kw-badge">system</span> <span class="kw-badge">performance</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">statistics</span></div>

Calculates robust statistical performance metrics for a trading system based on a stream of trade profits.

## Usage

Use to assess the performance quality of a trading system output using signal processing metrics. Helps distinguish systems with genuine edge from those that merely overfit.

## Background

> Ehlers applies signal processing metrics to evaluate trading system quality in Cybernetic Analysis. Metrics such as the Signal-to-Noise Ratio of the equity curve quantify whether a system is generating genuine signal above the noise floor of random entry and exit.

## Formula


\[
AveTrade = \% \cdot (PF + 1) - 1
\]
\[
PF_{breakeven} = \frac{1 - \%}{\%}
\]
\[
N_{losers} = \frac{\ln(0.0027)}{\ln(1 - \%)}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/SystemEvaluation.pdf)
