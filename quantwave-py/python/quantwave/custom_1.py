import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

@pl.api.register_expr_namespace("custom1")
class Custom1Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def gap_momentum(self, open: Union[str, pl.Expr], period: int, signal_period: int) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(open, str): open = pl.col(open)
        return register_plugin_function(
            args=[open, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="gap_momentum",
            is_elementwise=False,
            kwargs={"period": period, "signal_period": signal_period}
        )

    def pelt(self, penalty: float, min_dist: int) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="pelt",
            is_elementwise=False,
            kwargs={"penalty": penalty, "min_dist": min_dist}
        )

    def zlema(self, period: int) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="zlema",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def rodc(self, window_size: int, threshold: float, smooth_period: int) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="rodc",
            is_elementwise=False,
            kwargs={"window_size": window_size, "threshold": threshold, "smooth_period": smooth_period}
        )

    def heikin_ashi(self, open: Union[str, pl.Expr], high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(open, str): open = pl.col(open)
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[open, high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="heikin_ashi",
            is_elementwise=False
        )
