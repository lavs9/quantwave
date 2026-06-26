import polars as pl
import quantwave_plugins
import pytest

def test_all_methods():
    df = pl.DataFrame({
        "open": [1.0]*20,
        "high": [1.5]*20,
        "low": [0.5]*20,
        "close": [1.2]*20,
        "volume": [100.0]*20,
    })
    
    methods = [m for m in dir(pl.Expr.ta) if not m.startswith("_")]
    
    success = 0
    failed = []
    
    for m in methods:
        try:
            func = getattr(pl.col("close").ta, m)
            # Try to call it with appropriate arguments based on name
            if m.startswith("cdl_"):
                res = df.with_columns(func("open", "high", "low").alias("out"))
            elif m in ["add", "sub", "mul", "div"]:
                res = df.with_columns(func("open").alias("out"))
            elif m in ["acos", "asin", "atan", "ceil", "cos", "cosh", "exp", "floor", "ln", "log10"]:
                res = df.with_columns(func().alias("out"))
            else:
                # Skip ones we don't know the exact signature of for this brute force test
                continue
            success += 1
        except Exception as e:
            failed.append(f"{m}: {e}")
            
    assert len(failed) == 0, f"Failed methods: {failed}"
    print(f"Successfully ran {success} methods")
