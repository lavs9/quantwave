import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom0Extensions:
    def anchored_vwap(self, volume: Union[str, pl.Expr], anchor: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(volume, str): volume = pl.col(volume)
        if isinstance(anchor, str): anchor = pl.col(anchor)
        return register_plugin_function(
            args=[self._expr, volume, anchor],
            plugin_path=Path(__file__).parent,
            function_name="anchored_vwap",
            is_elementwise=False
        )

    def kinematic_kalman(self, q_pos: float, q_vel: float, r: float) -> pl.Expr:
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="kinematic_kalman",
            is_elementwise=False,
            kwargs={"q_pos": q_pos, "q_vel": q_vel, "r": r}
        )

    def vortex_indicator(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="vortex_indicator",
            is_elementwise=False,
            kwargs={"period": period}
        )

    def sve_volatility_bands(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], bands_period: int, bands_deviation: float, low_band_adjust: float, mid_line_length: int) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="sve_volatility_bands",
            is_elementwise=False,
            kwargs={
                "bands_period": bands_period,
                "bands_deviation": bands_deviation,
                "low_band_adjust": low_band_adjust,
                "mid_line_length": mid_line_length
            }
        )

    def ttm_squeeze(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int, multiplier_bb: float, multiplier_kc: float) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="ttm_squeeze",
            is_elementwise=False,
            kwargs={
                "period": period,
                "multiplier_bb": multiplier_bb,
                "multiplier_kc": multiplier_kc
            }
        )
