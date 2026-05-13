# FM Demodulator

Extracts market timing information by demodulating the frequency-modulated price spectrum.

## Parameters

- `period` (default: 30): SuperSmoother period

## Formula


\[
Deriv = Close - Open, HL = \text{Clip}(10 \times Deriv, -1, 1)
\]
\[
SS = c_1 \frac{HL + HL_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf)
