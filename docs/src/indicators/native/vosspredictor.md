# VossPredictor

A predictive filter with negative group delay for band-limited signals.

## Parameters

- `period` (default: 20): Center period of the BandPass filter
- `predict` (default: 3): Number of bars of prediction

## Formula


\[
Filt = \text{BandPass}(Price, Period, 0.25)
\]
\[
Order = 3 \cdot Predict
\]
\[
SumC = \sum_{n=0}^{Order-1} \frac{n+1}{Order} Voss_{t-(Order-n)}
\]
\[
Voss = \frac{3 + Order}{2} Filt - SumC
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/A%20PEEK%20INTO%20THE%20FUTURE.pdf)
