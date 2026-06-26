import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union, List

class Custom3Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def hma(self, period: int) -> pl.Expr:
        """Calculates the Hull Moving Average (HMA)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="hma",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def obvm(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], obvm_period: int, signal_period: int) -> pl.Expr:
        """On Balance Volume Modified (OBVM). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(
            args=[high, low, self._expr, volume],
            plugin_path=Path(__file__).parent,
            function_name="obvm",
            is_elementwise=False,
            kwargs={"obvm_period": obvm_period, "signal_period": signal_period}
        )

    def bill_williams_fractals(self, low: Union[str, pl.Expr]) -> pl.Expr:
        """Bill Williams Fractals. Returns struct: bearish, bullish. Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[self._expr, low],
            plugin_path=Path(__file__).parent,
            function_name="bill_williams_fractals",
            is_elementwise=False
        )

    def donchian_channels(self, low: Union[str, pl.Expr], period: int) -> pl.Expr:
        """Donchian Channels. Returns struct: upper, middle, lower. Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[self._expr, low],
            plugin_path=Path(__file__).parent,
            function_name="donchian_channels",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def gmm(self, other_cols: List[Union[str, pl.Expr]], k: int) -> pl.Expr:
        """Gaussian Mixture Model. Note: self is the first column."""
        exprs = [self._expr]
        for c in other_cols:
            if isinstance(c, str):
                exprs.append(pl.col(c))
            else:
                exprs.append(c)
        return register_plugin_function(
            args=exprs,
            plugin_path=Path(__file__).parent,
            function_name="gmm",
            is_elementwise=False,
            kwargs={"k": k}
        )
