import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path

def dummy_multiply(expr: pl.Expr) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=Path(__file__).parent,
        function_name="dummy_multiply",
        is_elementwise=True,
    )
