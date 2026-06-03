import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path

@pl.api.register_expr_namespace("ta")
class TaNamespace:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sma(self, period: int) -> pl.Expr:
        """
        Calculates the Simple Moving Average (SMA).
        """
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="sma",
            is_elementwise=False, # State machine relies on sequential order
            kwargs={"period": period}
        )
