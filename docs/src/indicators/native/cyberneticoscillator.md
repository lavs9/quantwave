# CyberneticOscillator

Combined HighPass and SuperSmoother filters normalized by RMS.

## Parameters

- `hp_length` (default: 30): HighPass filter length
- `lp_length` (default: 20): LowPass (SuperSmoother) length
- `rms_len` (default: 100): RMS normalization length

## Formula


\[
HP = HighPass(Price, HPLen)
\]
\[
LP = SuperSmoother(HP, LPLen)
\]
\[
RMS = \sqrt{\frac{1}{N} \sum_{i=0}^{N-1} LP_{t-i}^2}
\]
\[
CO = \frac{LP}{RMS}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202025.html)
