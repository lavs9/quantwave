# System Evaluator

Calculates robust statistical performance metrics for a trading system based on a stream of trade profits.

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
