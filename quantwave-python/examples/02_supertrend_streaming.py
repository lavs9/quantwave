#!/usr/bin/env python3
"""Minimal streaming example — RSI via streaming_class + wrap_streaming."""

import quantwave as qw

closes = [100.0 + i * 0.3 for i in range(30)]

cls = qw.streaming_class("rsi")
rsi = qw.wrap_streaming(cls(period=14), name="rsi")

for price in closes:
    val = rsi.next(price)
    if rsi.is_ready:
        print(f"close={price:.1f} rsi={val:.2f}")