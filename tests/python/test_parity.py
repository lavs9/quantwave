import polars as pl
import quantwave as qw
import pytest


def test_sma_parity():
    # Test data
    data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    df = pl.DataFrame({"close": data})
    period = 3

    # Expected results (SMA period 3), partial averages during warmup:
    # 1.0 -> 1.0 / 1 = 1.0
    # 2.0 -> (1+2) / 2 = 1.5
    # 3.0 -> (1+2+3) / 3 = 2.0
    # 4.0 -> (2+3+4) / 3 = 3.0
    # ...
    expected_sma = [1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]

    # 1. Polars expression namespace. `.ta` is registered on pl.Expr — not on
    # DataFrame or LazyFrame — so it is reached through pl.col(...), which is
    # then usable anywhere an expression is (select/with_columns, eager or lazy).
    result_df = df.lazy().select(pl.col("close").ta.sma(period).alias("sma")).collect()
    actual_sma = result_df["sma"].to_list()
    assert actual_sma == pytest.approx(expected_sma)

    # 2. Streaming API — same Rust core, one value at a time.
    sma = qw.streaming_class("sma")(period)
    streaming_results = [sma.next(v) for v in data]
    assert streaming_results == pytest.approx(expected_sma)


if __name__ == "__main__":
    test_sma_parity()
