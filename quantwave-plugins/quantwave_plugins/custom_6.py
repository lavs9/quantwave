import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom6Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sarext(
        self,
        high: Union[str, pl.Expr],
        startvalue: float = 0.0,
        offsetonreverse: float = 0.0,
        accelerationinitlong: float = 0.0,
        accelerationlong: float = 0.0,
        accelerationmaxlong: float = 0.0,
        accelerationinitshort: float = 0.0,
        accelerationshort: float = 0.0,
        accelerationmaxshort: float = 0.0,
    ) -> pl.Expr:
        """Parabolic SAR - Extended (SAREXT). Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(
            args=[high, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="sarext",
            is_elementwise=False,
            kwargs={
                "startvalue": startvalue,
                "offsetonreverse": offsetonreverse,
                "accelerationinitlong": accelerationinitlong,
                "accelerationlong": accelerationlong,
                "accelerationmaxlong": accelerationmaxlong,
                "accelerationinitshort": accelerationinitshort,
                "accelerationshort": accelerationshort,
                "accelerationmaxshort": accelerationmaxshort,
            }
        )

    def keltner_channels(
        self,
        high: Union[str, pl.Expr],
        low: Union[str, pl.Expr],
        ema_period: int = 20,
        atr_period: int = 10,
        multiplier: float = 2.0,
    ) -> pl.Expr:
        """Keltner Channels. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="keltner_channels",
            is_elementwise=False,
            kwargs={
                "ema_period": ema_period,
                "atr_period": atr_period,
                "multiplier": multiplier,
            }
        )

    def regimes_duration_stats(
        self,
        num_states: int,
    ) -> pl.Expr:
        """Regime Duration Stats. Note: self must be the regime_col."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_duration_stats",
            is_elementwise=False,
            kwargs={
                "num_states": num_states,
            }
        )

    def market_structure(
        self,
        high: Union[str, pl.Expr],
        swing_strength: int = 2,
    ) -> pl.Expr:
        """Market Structure. Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(
            args=[high, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="market_structure",
            is_elementwise=False,
            kwargs={
                "swing_strength": swing_strength,
            }
        )

    def regimes_hmm_gas(self) -> pl.Expr:
        """Regimes HMM GAS. Note: self must be the input series."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_hmm_gas",
            is_elementwise=False,
        )
