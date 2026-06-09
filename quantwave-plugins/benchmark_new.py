import polars as pl
import quantwave_plugins
import time
import numpy as np

def run_benchmark():
    n_rows = 1_000_000
    print(f"Generating {n_rows} rows of data...")
    df = pl.DataFrame({
        "open": np.random.rand(n_rows) * 100,
        "high": np.random.rand(n_rows) * 100,
        "low": np.random.rand(n_rows) * 100,
        "close": np.random.rand(n_rows) * 100,
        "volume": np.random.rand(n_rows) * 1000,
    })
    
    ldf = df.lazy()
    
    indicators = [
        ("ADX (14)", ldf.with_columns(pl.col("close").ta.adx("high", "low", timeperiod=14).alias("res"))),
        ("CCI (14)", ldf.with_columns(pl.col("close").ta.cci("high", "low", timeperiod=14).alias("res"))),
        ("STOCH (5,3,3)", ldf.with_columns(pl.col("close").ta.stoch("high", "low").alias("res"))),
        ("ATR (14)", ldf.with_columns(pl.col("close").ta.atr("high", "low", timeperiod=14).alias("res"))),
        ("ADOSC", ldf.with_columns(pl.col("close").ta.adosc("high", "low", "volume").alias("res"))),
        ("KAMA (30)", ldf.with_columns(pl.col("close").ta.kama(timeperiod=30).alias("res"))),
        ("HT_PHASOR", ldf.with_columns(pl.col("close").ta.ht_phasor().alias("res"))),
        ("LINEARREG (14)", ldf.with_columns(pl.col("close").ta.linearreg(timeperiod=14).alias("res"))),
    ]
    
    print("\nBenchmarking New Indicators (1,000,000 rows):")
    print("-" * 40)
    for name, expr in indicators:
        t0 = time.time()
        expr.collect()
        t1 = time.time()
        
        diff = t1 - t0
        print(f"{name:.<20} {diff:.4f} seconds ({diff*1000:.1f} ms)")

if __name__ == "__main__":
    run_benchmark()
