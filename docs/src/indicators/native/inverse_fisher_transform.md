# Inverse Fisher Transform

A compressive transform that forces oscillator values towards +1 or -1, creating clear buy/sell signals.

## Formula


\[
IFT(x) = \frac{e^{2x} - 1}{e^{2x} + 1} = \tanh(x)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheInverseFisherTransform.pdf)
