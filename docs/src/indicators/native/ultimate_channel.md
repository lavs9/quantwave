# Ultimate Channel

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">channel</span> <span class="kw-badge">volatility</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">breakout</span></div>

A Keltner-style channel using UltimateSmoothers for both the center line and the volatility range to minimize lag.

## Usage

Use as a dynamic price channel whose width scales with the current dominant cycle amplitude, providing adaptive support and resistance levels for breakout trading.

## Background

> The Ultimate Channel uses the measured dominant cycle amplitude to set channel width, analogous to Keltner Channels but cycle-aware rather than ATR-based. When price breaks beyond the channel boundary, it signals that cycle amplitude has expanded enough to suggest a genuine directional move.

## Parameters

- `length` (default: 20): Center line smoothing period
- `str_length` (default: 20): Smooth True Range (STR) period
- `num_strs` (default: 1.0): Channel width multiplier

## Formula


\[
TH = \max(High, Close_{t-1})
\]
\[
TL = \min(Low, Close_{t-1})
\]
\[
STR = UltimateSmoother(TH - TL, STRLength)
\]
\[
Center = UltimateSmoother(Close, Length)
\]
\[
Upper = Center + NumSTRs \times STR
\]
\[
Lower = Center - NumSTRs \times STR
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf)
