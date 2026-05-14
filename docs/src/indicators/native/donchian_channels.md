# Donchian Channels

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">breakout</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">support-resistance</span></div>

Donchian Channels are volatility indicators formed by taking the highest high and the lowest low of the last N periods.

## Usage

Use for breakout trading systems: a close above the N-period high signals a long entry; below the N-period low signals a short entry. The Turtle Traders famously used 20 and 55-day Donchian channels.

## Background

> Developed by Richard Donchian in the 1970s, Donchian Channels plot the highest high and lowest low over N bars. They define the current trading range and signal breakouts when price escapes the channel. The Turtle Trading system of Richard Dennis built its entire entry and exit logic on 20 and 55-day Donchian channels. — TurtleTrader.com

## Parameters

- `period` (default: 20): Channel period

## Formula


\[
UC = \max(H_{t-n \dots t}) \\ LC = \min(L_{t-n \dots t})
\]


[Source](https://www.investopedia.com/terms/d/donchianchannels.asp)
