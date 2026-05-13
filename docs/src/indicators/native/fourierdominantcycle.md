# FourierDominantCycle

Dominant cycle period estimation using resolution-enhanced DFT and center of gravity.

## Parameters

- `window_len` (default: 50): DFT window length

## Formula


\[
HP = \text{HighPass}(Price, 40)
\]
\[
Cleaned = \frac{HP + 2HP_{t-1} + 3HP_{t-2} + 3HP_{t-3} + 2HP_{t-4} + HP_{t-5}}{12}
\]
\[
Pwr(P) = \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \cos\left(\frac{2\pi n}{P}\right)\right)^2 + \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \sin\left(\frac{2\pi n}{P}\right)\right)^2
\]
\[
DB(P) = \min\left(20, -10 \log_{10}\left(\frac{0.01}{1 - 0.99 \frac{Pwr(P)}{\max(Pwr)}}\right)\right)
\]
\[
DC = \frac{\sum_{P=8}^{50} P \cdot (3 - DB(P)) \text{ where } DB(P) < 3}{\sum (3 - DB(P))}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FourierTransformForTraders.pdf)
