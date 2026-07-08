#!/usr/bin/env python3
"""Post-install smoke test for unified quantwave wheels (release gate)."""

from __future__ import annotations

import sys


def main() -> int:
    import polars as pl
    import quantwave  # noqa: F401 — registers .ta namespace
    import quantwave_plugins  # noqa: F401

    names = quantwave.indicators()
    if not names:
        print("FAIL: quantwave.indicators() returned empty", file=sys.stderr)
        return 1

    n = 200
    close = [100.0 + i * 0.1 for i in range(n)]
    df = pl.DataFrame({"close": close})

    batch = df.lazy().with_columns(
        pl.col("close").ta.rsi(timeperiod=14).alias("rsi")
    ).collect()["rsi"].to_list()

    stream_cls = quantwave.streaming_class("rsi")
    stream = stream_cls(period=14)
    streamed = [stream.next(v) for v in close]

    # Compare non-null tail (warmup may differ in leading nulls)
    batch_vals = [v for v in batch if v is not None]
    stream_vals = [v for v in streamed if v is not None]
    if not batch_vals or not stream_vals:
        print("FAIL: RSI produced no values", file=sys.stderr)
        return 1

    pairs = zip(batch_vals[-20:], stream_vals[-20:])
    for b, s in pairs:
        if abs(b - s) > 1e-6:
            print(f"FAIL: RSI batch/stream mismatch {b} vs {s}", file=sys.stderr)
            return 1

    print(
        f"wheel_smoke_test: OK ({len(names)} indicators, RSI batch/stream parity)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())