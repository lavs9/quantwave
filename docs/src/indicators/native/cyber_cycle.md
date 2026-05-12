# Cyber Cycle

An oscillator introduced by John Ehlers that models the cyclical component of a time series using FIR smoothing.

## Parameters

- `length` (default: 14): Alpha smoothing length parameter

## Formula


\[
\alpha = \frac{2}{\text{Length} + 1}
\]
\[
\text{Smooth} = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}
\]
\[
CC_t = \left(1 - \frac{\alpha}{2}\right)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2}) + 2(1 - \alpha)CC_{t-1} - (1 - \alpha)^2 CC_{t-2}
\]


[Source](Cybernetic Analysis for Stocks and Futures, John Ehlers, 2004, Chapter 4)
