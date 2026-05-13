# ChannelCycle

Extracts cyclic components and a leading function using channel-normalized bandpass filtering.

## Parameters

- `period` (default: 20): Channel and Bandpass period

## Formula


\[
Detrended = \frac{Price - Low}{High - Low} - 0.5
\]
\[
BP = \text{Bandpass}(Detrended, Period)
\]
\[
Leading = \frac{BP - BP_{t-1}}{2\pi/Period}
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf)
