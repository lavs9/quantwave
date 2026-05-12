# Native Indicators

Native indicators in QuantWave are written entirely in safe, zero-cost Rust.

These algorithms are compiled as native Polars Expressions, allowing them to benefit from vectorized execution, multi-threading, and query optimization without serialization overhead.

Here you will find our implementations of algorithms like `SuperTrend`, `WaveTrend`, `ALMA`, and more.
