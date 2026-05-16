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

=== "Python"

    ```python
    from quantwave import RSI

    rsi = RSI(14)
    for price in prices:
        print(rsi.next(price))
    ```

=== "Rust"

    ```rust
    use quantwave_core::indicators::RSI;
    use quantwave_core::traits::Next;

    let mut rsi = RSI::new(14);
    for price in prices {
        println!("{:?}", rsi.next(price));
    }
    ```
