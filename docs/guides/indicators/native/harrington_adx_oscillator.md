# Harrington ADX Oscillator

<div class="indicator-meta"><span class="category-badge">Wilder</span> <span class="kw-badge">adx</span> <span class="kw-badge">dmi</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">wilder</span> <span class="kw-badge">momentum</span></div>

An oscillator variant of the ADX where the sign reflects trend direction determined by DMI+ and DMI-.

## Usage

The oscillator is positive when DMI+ > DMI- and negative when DMI- > DMI+. The magnitude represents trend strength (ADX). Thresholds at 15 and 40 are often used to identify trend initiation and overextended states.

## Background

> While originally created by Wilder, this revisualization by Harrington transforms the unipolar ADX into a bipolar oscillator. This allows for simultaneous identification of trend strength and direction in a single histogram display, simplifying the interpretation of complex directional movement data.

## Parameters

- `adx_length` (default: 10): Wilder's ADX period
- `adx_smooth_length` (default: 1): SMA period for DMI components smoothing

## Formula


\[
TR = \max(H-L, |H-C_{t-1}|, |L-C_{t-1}|)
\]
\[
+DM = (H-H_{t-1} > L_{t-1}-L) \text{ and } (H-H_{t-1} > 0) ? H-H_{t-1} : 0
\]
\[
-DM = (L_{t-1}-L > H-H_{t-1}) \text{ and } (L_{t-1}-L > 0) ? L_{t-1}-L : 0
\]
\[
+DI = 100 \cdot \frac{EMA(+DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
-DI = 100 \cdot \frac{EMA(-DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
DX = 100 \cdot \frac{|+DI - -DI|}{+DI + -DI}
\]
\[
ADX = EMA(DX, 1/L)
\]
\[
Result = (SMA(+DI, S) \ge SMA(-DI, S)) ? ADX : -ADX
\]


[Source](https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202024.html)
