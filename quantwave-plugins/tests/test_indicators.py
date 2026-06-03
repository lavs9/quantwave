import polars as pl
import quantwave_plugins
import math

def test_all_indicators():
    df = pl.DataFrame({
        "close": [10.0, 12.0, 15.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0]
    })
    
    res = df.lazy().with_columns([
        pl.col("close").ta.sma(period=3).alias("sma"),
        pl.col("close").ta.ema(period=3).alias("ema"),
        pl.col("close").ta.rsi(timeperiod=3).alias("rsi"),
        pl.col("close").ta.macd(fast=3, slow=6, signal=3).alias("macd"),
        pl.col("close").ta.bbands(timeperiod=3).alias("bbands")
    ]).collect()
    
    # Just checking they run and output correctly formatted structs/floats
    print(res)
    
    assert "sma" in res.columns
    assert "ema" in res.columns
    assert "rsi" in res.columns
    assert "macd" in res.columns
    assert "bbands" in res.columns
    
    macd_struct = res.get_column("macd")[0]
    assert "macd" in macd_struct
    assert "signal" in macd_struct
    assert "hist" in macd_struct
    
    bbands_struct = res.get_column("bbands")[0]
    assert "upper" in bbands_struct
    assert "middle" in bbands_struct
    assert "lower" in bbands_struct
    
    print("All indicators successfully bound and executed!")

if __name__ == "__main__":
    test_all_indicators()
