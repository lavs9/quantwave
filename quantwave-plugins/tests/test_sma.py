import polars as pl
import quantwave_plugins
import math

def test_sma():
    # Setup Polars dataframe
    df = pl.DataFrame({
        "close": [10.0, 20.0, 30.0, 40.0, 50.0]
    })
    
    # Run the SMA plugin via LazyFrame
    res = df.lazy().with_columns(
        pl.col("close").ta.sma(period=3).alias("sma_3")
    ).collect()
    
    # Assert
    sma_vals = res.get_column("sma_3").to_list()
    print("SMA Values:", sma_vals)
    
    assert math.isnan(sma_vals[0])
    assert math.isnan(sma_vals[1])
    assert abs(sma_vals[2] - 20.0) < 1e-9
    assert abs(sma_vals[3] - 30.0) < 1e-9
    assert abs(sma_vals[4] - 40.0) < 1e-9
    print("SMA Plugin works flawlessly via Polars Expression!")

if __name__ == "__main__":
    test_sma()
