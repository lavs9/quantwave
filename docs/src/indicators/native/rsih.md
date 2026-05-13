# RSIH

RSI enhanced with Hann windowing for superior smoothing and zero-centering.

## Parameters

- `length` (default: 14): RSI length

## Formula


\[
CU = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n+1} - Close_{t-n})
\]
\[
CD = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n} - Close_{t-n+1})
\]
\[
RSIH = \frac{CU - CD}{CU + CD}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202022.html)
