import polars as pl
from quantwave_plugins import dummy_multiply

def test_dummy_multiply():
    # Setup Polars dataframe
    df = pl.DataFrame({
        "values": [1.0, 2.0, 3.0]
    })
    
    # Run the dummy plugin via LazyFrame
    res = df.lazy().with_columns(
        dummy_multiply(pl.col("values")).alias("multiplied")
    ).collect()
    
    # Assert
    multiplied_vals = res.get_column("multiplied").to_list()
    assert multiplied_vals == [2.0, 4.0, 6.0], f"Expected [2.0, 4.0, 6.0], got {multiplied_vals}"
    print("Plugin successfully multiplied by 2 using Arrow zero-copy vectorization!")

if __name__ == "__main__":
    test_dummy_multiply()
