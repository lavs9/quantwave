import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

class Custom4Extensions:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def regimes_next_state_prob(self, num_states: int, steps: int) -> pl.Expr:
        """Forecast the next state probabilities for market regimes."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_next_state_prob",
            is_elementwise=False,
            kwargs={"num_states": num_states, "steps": steps}
        )

    def hmm_bull_bear(self) -> pl.Expr:
        """Classify market regime into Bull (1) or Bear (2) using a Hidden Markov Model."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="hmm_bull_bear",
            is_elementwise=False
        )

    def alma(self, period: int, offset: float, sigma: float) -> pl.Expr:
        """Calculate the Arnaud Legoux Moving Average (ALMA)."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="alma",
            is_elementwise=False,
            kwargs={"period": period, "offset": offset, "sigma": sigma}
        )

    def regimes_stability_score(self) -> pl.Expr:
        """Calculate the stability score for market regimes."""
        return register_plugin_function(
            args=[self._expr],
            plugin_path=Path(__file__).parent,
            function_name="regimes_stability_score",
            is_elementwise=False
        )

    def geometric_patterns(self, high: Union[str, pl.Expr], swing_strength: int) -> pl.Expr:
        """Scan for geometric chart patterns. Returns a struct with pattern flags. Note: self must be low."""
        if isinstance(high, str):
            high = pl.col(high)
        return register_plugin_function(
            args=[high, self._expr],
            plugin_path=Path(__file__).parent,
            function_name="geometric_patterns",
            is_elementwise=False,
            kwargs={"swing_strength": swing_strength}
        )
