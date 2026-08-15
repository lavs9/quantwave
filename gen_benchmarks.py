import polars as pl
import timeit
import numpy as np

def benchmark_sma():
    # Setup 1M rows
    num_rows = 1_000_000
    df = pl.DataFrame({
        "close": np.random.uniform(100, 200, num_rows)
    })
    
    # We'll use a simple rolling mean as a proxy for SMA if QuantWave isn't built yet
    # In a real scenario, we'd use pl.col("close").ta.sma(20)
    
    def run_qw():
        return df.with_columns(pl.col("close").rolling_mean(20))

    t = timeit.timeit(run_qw, number=10)
    print(f"QuantWave (Proxy) SMA 1M rows: {t/10 * 1000:.2f} ms")

if __name__ == "__main__":
    print("Running Benchmarks...")
    benchmark_sma()
    # In the future, this script would write results to a JSON or directly into benchmarks.md
