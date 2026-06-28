# Batch & Streaming Examples

QuantWave allows you to use the same logic for both batch processing and streaming.

## Batch Processing (Polars)

=== "Python"

    ```python
    import polars as pl
    import quantwave  # registers pl.col().ta (bundled in PyPI wheel)

    df = pl.read_parquet("data.parquet")
    df = df.lazy().with_columns(
        pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),
    ).collect()
    ```

=== "Rust"

    ```rust
    use polars::prelude::*;
    use quantwave_polars::TA;

    let df = df.lazy().ta().rsi("close", 14).collect()?;
    ```

## Streaming Processing

QuantWave indicators implement the universal `Next<T>` trait. This is the single source of mathematical truth for every indicator — the exact same logic used by the Polars expressions. As a result, streaming and batch results are guaranteed to be bit-identical.

=== "Python"

    ```python
    import quantwave as qw

    # Simple streaming example
    Rsi = qw.streaming_class("rsi")
    rsi = qw.wrap_streaming(Rsi(14), name="rsi")
    for price in prices:
        val = rsi.next(price)
        if rsi.is_ready:
            print(val)

    # SuperTrend streaming example
    St = qw.streaming_class("supertrend")
    st = qw.wrap_streaming(St(period=10, multiplier=3.0), name="supertrend")
    for high, low, close in ohlcv_data:
        signal = st.next((high, low, close))
        if st.is_ready:
            print(signal)
    ```

=== "Rust"

    ```rust
    use quantwave_core::indicators::{RSI, SuperTrend};
    use quantwave_core::traits::Next;

    // Simple streaming example
    let mut rsi = RSI::new(14);
    for price in prices {
        println!("{:?}", rsi.next(price));
    }

    // SuperTrend streaming example
    let mut st = SuperTrend::new(10, 3.0);
    for (high, low, close) in ohlcv_data {
        let signal = st.next((high, low, close));
        println!("{:?}", signal);
    }
    ```

## ML Features + Rich Backtest (Cross-Epic Canonical Example)

The complete demonstration of the 4ps/gwx integration contract (locked `.ta().features()` surface for batch signal prep + `FeatureToSignal` adapter implementing `Next<&Bar, Output=StrategySignal>` with rich metadata for `run_streaming_simulation` + verified parity + metadata in `Trade.entry_metadata`) is the runnable notebook:

- `docs/examples/notebooks/ml_feature_backtest_parity.py`

It is the primary evidence artifact for closure of both epics. Run instructions and full source provenance are inside the notebook.