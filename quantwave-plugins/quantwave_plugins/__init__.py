import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path

@pl.api.register_expr_namespace("ta")
class TaNamespace:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sma(self, period: int) -> pl.Expr:
        """Calculates the Simple Moving Average (SMA)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="sma",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def ema(self, period: int) -> pl.Expr:
        """Calculates the Exponential Moving Average (EMA)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="ema",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def rsi(self, timeperiod: int = 14) -> pl.Expr:
        """Calculates the Relative Strength Index (RSI)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="rsi",
            is_elementwise=False,
            kwargs={"timeperiod": timeperiod}
        )

    def macd(self, fast: int = 12, slow: int = 26, signal: int = 9) -> pl.Expr:
        """Calculates the Moving Average Convergence Divergence (MACD).
        Returns a struct containing: macd, signal, hist.
        """
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="macd",
            is_elementwise=False,
            kwargs={"fast": fast, "slow": slow, "signal": signal}
        )

    def bbands(self, timeperiod: int = 5, nbdevup: float = 2.0, nbdevdn: float = 2.0, matype: int = 0) -> pl.Expr:
        """Calculates Bollinger Bands.
        Returns a struct containing: upper, middle, lower.
        matype defaults to 0 (SMA).
        """
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="bbands",
            is_elementwise=False,
            kwargs={
                "timeperiod": timeperiod,
                "nbdevup": nbdevup,
                "nbdevdn": nbdevdn,
                "matype": matype
            }
        )
