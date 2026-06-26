import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom8Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def autotune_filter(self, window: int, bandwidth: float) -> pl.Expr:
        """Calculates AutoTune Filter."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="autotune_filter",
            is_elementwise=False,
            kwargs={"window": window, "bandwidth": bandwidth}
        )

    def tradj_ema(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int, pds: int, mltp: float) -> pl.Expr:
        """Calculates True Range Adjusted EMA (TRAdjEMA). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="tradj_ema",
            is_elementwise=False,
            kwargs={"period": period, "pds": pds, "mltp": mltp}
        )

    def tema(self, period: int) -> pl.Expr:
        """Calculates Triple Exponential Moving Average (TEMA)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="tema",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def rsmk(self, benchmark: Union[str, pl.Expr], length: int, ema_length: int) -> pl.Expr:
        """Calculates Relative Strength Mansfield (RSMK). Note: self must be price."""
        if isinstance(benchmark, str): benchmark = pl.col(benchmark)
        return register_plugin_function(
            args=[self._expr, benchmark],
            plugin_path=Path(__file__).parent,
            function_name="rsmk",
            is_elementwise=False,
            kwargs={"length": length, "ema_length": ema_length}
        )

    def exp_dev_bands(self, period: int, multiplier: float, use_sma: bool) -> pl.Expr:
        """Calculates Exponential Deviation Bands. Returns a struct containing: upper, middle, lower."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="exp_dev_bands",
            is_elementwise=False,
            kwargs={"period": period, "multiplier": multiplier, "use_sma": use_sma}
        )
