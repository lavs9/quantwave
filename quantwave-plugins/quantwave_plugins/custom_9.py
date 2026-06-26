import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union, List

class Custom9Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sdo(self, lookback_period: int, period: int, ema_pds: int) -> pl.Expr:
        """Calculates the SDO."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="sdo",
            is_elementwise=False,
            kwargs={"lookback_period": lookback_period, "period": period, "ema_pds": ema_pds}
        )

    def regimes_tar(self, thresholds: List[float]) -> pl.Expr:
        """Calculates Regimes TAR."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_tar",
            is_elementwise=False,
            kwargs={"thresholds": thresholds}
        )

    def ichimoku_cloud(self, low: Union[str, pl.Expr], p1: int, p2: int, p3: int) -> pl.Expr:
        """Calculates Ichimoku Cloud. Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[self._expr, low],
            plugin_path=Path(__file__).parent,
            function_name="ichimoku_cloud",
            is_elementwise=False,
            kwargs={"p1": p1, "p2": p2, "p3": p3}
        )

    def mama(self, fastlimit: float, slowlimit: float) -> pl.Expr:
        """Calculates MAMA."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="mama",
            is_elementwise=False,
            kwargs={"fastlimit": fastlimit, "slowlimit": slowlimit}
        )

    def atr_trailing_stop(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int, multiplier: float) -> pl.Expr:
        """Calculates ATR Trailing Stop. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="atr_trailing_stop",
            is_elementwise=False,
            kwargs={"period": period, "multiplier": multiplier}
        )
