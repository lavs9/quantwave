# Roofing Filter

Combines a 2-pole HighPass filter and a SuperSmoother to isolate specific cyclic components.

## Parameters

- `hp_period` (default: 48): HighPass critical period
- `ss_period` (default: 10): SuperSmoother critical period

## Formula


\[
\alpha_1 = \frac{\cos(\sqrt{2}\pi/P_{hp}) + \sin(\sqrt{2}\pi/P_{hp}) - 1}{\cos(\sqrt{2}\pi/P_{hp})}
\]
\[
HP = (1 - \alpha_1/2)^2 (Price - 2 Price_{t-1} + Price_{t-2}) + 2(1 - \alpha_1) HP_{t-1} - (1 - \alpha_1)^2 HP_{t-2}
\]
\[
Filt = c_1 \frac{HP + HP_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PredictiveIndicators.pdf)
