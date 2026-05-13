# DMH

An improved Directional Movement indicator using Hann windowing for smoother signals and reduced lag.

## Parameters

- `length` (default: 14): Smoothing period

## Formula


\[
\text{PlusDM} = \text{High} - \text{High}_{t-1} \text{ if } > (\text{Low}_{t-1} - \text{Low}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{MinusDM} = \text{Low}_{t-1} - \text{Low} \text{ if } > (\text{High} - \text{High}_{t-1}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{EMA} = \frac{1}{L}(\text{PlusDM} - \text{MinusDM}) + (1 - \frac{1}{L})\text{EMA}_{t-1}
\]
\[
\text{DMH} = \frac{\sum_{i=1}^{L} w_i \text{EMA}_{t-i+1}}{\sum_{i=1}^{L} w_i}, \text{ where } w_i = 1 - \cos\left(\frac{2\pi i}{L+1}\right)
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202021.html)
