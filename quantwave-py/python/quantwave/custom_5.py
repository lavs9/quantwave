import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union, List

class Custom5Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def ta_var(self, period: int, nbdev: float) -> pl.Expr:
        """Calculates Variance (ta_var).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="ta_var",
            is_elementwise=False,
            kwargs={"period": period, "nbdev": nbdev}
        )

    def vfi(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], period: int, coef: float, vcoef: float, smoothing_period: int) -> pl.Expr:
        """Calculates Volume Flow Indicator (VFI). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(
            args=[high, low, self._expr, volume],
            plugin_path=Path(__file__).parent,
            function_name="vfi",
            is_elementwise=False,
            kwargs={
                "period": period,
                "coef": coef,
                "vcoef": vcoef,
                "smoothing_period": smoothing_period
            }
        )

    def wavetrend(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], n1: int, n2: int, n3: int) -> pl.Expr:
        """Calculates WaveTrend. Returns struct: wt1, wt2. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(
            args=[high, low, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="wavetrend",
            is_elementwise=False,
            kwargs={"n1": n1, "n2": n2, "n3": n3}
        )

    def regimes_ensemble(self, columns: List[Union[str, pl.Expr]], weights: List[float]) -> pl.Expr:
        """Calculates Regimes Ensemble consensus.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        exprs = [self._expr]
        for c in columns:
            if isinstance(c, str):
                exprs.append(pl.col(c))
            else:
                exprs.append(c)
        return register_plugin_function(
            args=exprs,
            plugin_path=Path(__file__).parent,
            function_name="regimes_ensemble",
            is_elementwise=False,
            kwargs={"weights": weights}
        )

    def ta_stddev(self, period: int, nbdev: float) -> pl.Expr:
        """Calculates Standard Deviation (ta_stddev).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="ta_stddev",
            is_elementwise=False,
            kwargs={"period": period, "nbdev": nbdev}
        )
