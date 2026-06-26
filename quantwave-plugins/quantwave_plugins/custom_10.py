import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom10Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def harrington_adx(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], adx_length: int, adx_smooth_length: int) -> pl.Expr:
        """Calculates the Harrington ADX Oscillator."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="harrington_adx",
            is_elementwise=False,
            kwargs={"adx_length": adx_length, "adx_smooth_length": adx_smooth_length}
        )

    def kalman(self, q: float, r: float) -> pl.Expr:
        """Calculates the Kalman Filter."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="kalman",
            is_elementwise=False,
            kwargs={"q": q, "r": r}
        )
