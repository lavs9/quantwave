import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom2Extensions:
    def mavp(self, in2: Union[str, pl.Expr], minperiod: int, maxperiod: int, matype: int = 0) -> pl.Expr:
        """Moving Average with Variable Period (MAVP). Note: self is in1.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="mavp", is_elementwise=False, kwargs={"minperiod": minperiod, "maxperiod": maxperiod, "matype": matype})

    def mfi(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], period: int = 14) -> pl.Expr:
        """Money Flow Index (MFI). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(args=[high, low, self._expr, volume], plugin_path=Path(__file__).parent, function_name="mfi", is_elementwise=False, kwargs={"period": period})

    def reverse_ema(self, alpha: float) -> pl.Expr:
        """Reverse Exponential Moving Average (REVERSE_EMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="reverse_ema", is_elementwise=False, kwargs={"alpha": alpha})

    def volatility_clusterer(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], atr_period: int, window_size: int, k: int) -> pl.Expr:
        """Volatility Clusterer. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="volatility_clusterer", is_elementwise=False, kwargs={"atr_period": atr_period, "window_size": window_size, "k": k})

    def pivot_points(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        """Pivot Points. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="pivot_points", is_elementwise=False)
