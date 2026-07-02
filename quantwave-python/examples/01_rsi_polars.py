#!/usr/bin/env python3
"""Minimal Polars batch example — RSI via pl.col().ta."""

import polars as pl
import quantwave  # noqa: F401 — registers .ta

df = pl.DataFrame({"close": [float(x) for x in range(1, 50)]})
out = df.lazy().with_columns(pl.col("close").ta.rsi(timeperiod=14).alias("rsi")).collect()
print(out.tail())