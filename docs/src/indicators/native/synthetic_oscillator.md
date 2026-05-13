# Synthetic Oscillator

A nonlinear oscillator designed to reduce lag while maintaining smoothness by adapting to the dominant cycle.

## Parameters

- `lower_bound` (default: 15): Lower bound of cycle period
- `upper_bound` (default: 25): Upper bound of cycle period

## Formula


\[
Price = \text{Hann}(Close, 12)
\]
\[
LP = \text{SuperSmoother}(\text{HighPass}(Price, UB), LB)
\]
\[
Re = \frac{LP}{RMS(LP, 100)}, \quad Im = \frac{Re - Re_{t-1}}{RMS(Re - Re_{t-1}, 100)}
\]
\[
DC = \frac{2\pi(Re^2 + Im^2)}{(Re - Re_{t-1})Im - (Im - Im_{t-1})Re}
\]
\[
BP = \text{UltimateSmoother}(\text{HighPass}(Close, Mid), Mid)
\]
\[
Phase = Phase_{t-1} + \frac{2\pi}{DC}
\]
\[
Synth = \sin(Phase)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202026.html)
