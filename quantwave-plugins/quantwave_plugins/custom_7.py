import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom7Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def regimes_ms_garch(self) -> pl.Expr:
        """Calculates Markov-Switching GARCH Regimes."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_ms_garch",
            is_elementwise=False
        )

    def adaptive_ema(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int, pds: int) -> pl.Expr:
        """Calculates Adaptive EMA. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="adaptive_ema",
            is_elementwise=False,
            kwargs={"period": period, "pds": pds}
        )

    def regimes_transition_matrix(self, num_states: int) -> pl.Expr:
        """Calculates Regime Transition Matrix."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_transition_matrix",
            is_elementwise=False,
            kwargs={"num_states": num_states}
        )

    def vpn(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], period: int, smooth_period: int) -> pl.Expr:
        """Calculates Volume Price Trend (VPN). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(
            args=[high, low, self._expr, volume],
            plugin_path=Path(__file__).parent,
            function_name="vpn",
            is_elementwise=False,
            kwargs={"period": period, "smooth_period": smooth_period}
        )

    def regimes_hsmm(self) -> pl.Expr:
        """Calculates Hidden Semi-Markov Model Regimes."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_hsmm",
            is_elementwise=False
        )
