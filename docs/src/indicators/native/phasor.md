# Phasor

Extracts In-Phase (I) and Quadrature (Q) components using a Hilbert Transform.

## Formula


\[
I = \text{Detrender}_{t-3}
\]
\[
Q = \text{HilbertFIR}(\text{Detrender}, \text{Period})
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf)
