# Batch & Streaming Examples

QuantWave allows you to use the same logic for both batch processing and streaming.

## Batch Processing (Polars)

=== "Python"

    ```python
    import polars as pl
    from quantwave import ta

    df = pl.read_parquet("data.parquet")
    df = df.with_columns(ta.rsi("close", 14).alias("rsi"))
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
    from quantwave import RSI, SuperTrend

    # Simple streaming example
    rsi = RSI(14)
    for price in prices:
        print(rsi.next(price))

    # SuperTrend streaming example
    st = SuperTrend(10, 3.0)
    for high, low, close in ohlcv_data:
        signal = st.next(high, low, close)
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
