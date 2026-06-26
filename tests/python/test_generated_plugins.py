import polars as pl
import quantwave_plugins
import pytest

def test_generated_math_transforms():
    df = pl.DataFrame({"x": [1.0, 2.0, 3.0, 4.0, 5.0]})
    # Test ACOS
    res = df.with_columns(
        acos=pl.col("x").ta.acos()
    )
    assert "acos" in res.columns

def test_generated_cdl_patterns():
    df = pl.DataFrame({
        "open": [1.0, 2.0, 3.0, 4.0, 5.0],
        "high": [1.5, 2.5, 3.5, 4.5, 5.5],
        "low": [0.5, 1.5, 2.5, 3.5, 4.5],
        "close": [1.2, 2.2, 3.2, 4.2, 5.2]
    })
    res = df.with_columns(
        doji=pl.col("open").ta.cdl_doji("high", "low", "close")
    )
    assert "doji" in res.columns

def test_generated_math_operators():
    df = pl.DataFrame({"x": [1.0, 2.0, 3.0, 4.0, 5.0]})
    res = df.with_columns(
        add=pl.col("x").ta.add("x")
    )
    assert "add" in res.columns
