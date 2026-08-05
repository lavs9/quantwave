import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

from .custom_0 import Custom0Extensions
from .custom_1 import Custom1Extensions
from .custom_2 import Custom2Extensions
from .custom_3 import Custom3Extensions
from .custom_4 import Custom4Extensions
from .custom_5 import Custom5Extensions
from .custom_6 import Custom6Extensions
from .custom_7 import Custom7Extensions
from .custom_8 import Custom8Extensions
from .custom_9 import Custom9Extensions
from .custom_10 import Custom10Extensions

@pl.api.register_expr_namespace("ta")
class TaNamespace(
    Custom0Extensions,
    Custom1Extensions,
    Custom2Extensions,
    Custom3Extensions,
    Custom4Extensions,
    Custom5Extensions,
    Custom6Extensions,
    Custom7Extensions,
    Custom8Extensions,
    Custom9Extensions,
    Custom10Extensions
):
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sma(self, period: int) -> pl.Expr:
        """Calculates the Simple Moving Average (SMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sma", is_elementwise=False, kwargs={"period": period})

    def ema(self, period: int) -> pl.Expr:
        """Calculates the Exponential Moving Average (EMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ema", is_elementwise=False, kwargs={"period": period})

    def rsi(self, timeperiod: int = 14) -> pl.Expr:
        """Calculates the Relative Strength Index (RSI).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rsi", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def macd(self, fast: int = 12, slow: int = 26, signal: int = 9) -> pl.Expr:
        """Calculates the Moving Average Convergence Divergence (MACD). Returns a struct containing: macd, signal, hist.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macd", is_elementwise=False, kwargs={"fast": fast, "slow": slow, "signal": signal})

    def bbands(self, timeperiod: int = 5, nbdevup: float = 2.0, nbdevdn: float = 2.0, matype: int = 0) -> pl.Expr:
        """Calculates Bollinger Bands. Returns a struct containing: upper, middle, lower. matype defaults to 0 (SMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="bbands", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdevup": nbdevup, "nbdevdn": nbdevdn, "matype": matype})

    def supertrend(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], period: int = 10, multiplier: float = 3.0) -> pl.Expr:
        """Calculates SuperTrend. Returns a struct containing: supertrend, direction. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="supertrend", is_elementwise=False, kwargs={"period": period, "multiplier": multiplier})

    # --------------------------------------------------------------------------------
    # Momentum & Trend
    # --------------------------------------------------------------------------------
    
    def cci(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Commodity Channel Index (CCI). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="cci", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def cmo(self, timeperiod: int = 14) -> pl.Expr:
        """Chande Momentum Oscillator (CMO).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="cmo", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def mom(self, timeperiod: int = 10) -> pl.Expr:
        """Momentum (MOM).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="mom", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def roc(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change (ROC).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="roc", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocp(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Percentage (ROCP).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocp", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocr(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Ratio (ROCR).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocr100(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Ratio 100 Scale (ROCR100).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocr100", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def trix(self, timeperiod: int = 30) -> pl.Expr:
        """1-day Rate-Of-Change (ROC) of a Triple Smooth EMA (TRIX).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="trix", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def willr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Williams' %R (WILLR). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="willr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def adx(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average Directional Movement Index (ADX). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="adx", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def adxr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average Directional Movement Index Rating (ADXR). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="adxr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def dx(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Directional Movement Index (DX). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="dx", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def plus_di(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Plus Directional Indicator (+DI). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="plus_di", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def minus_di(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Minus Directional Indicator (-DI). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="minus_di", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def plus_dm(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Plus Directional Movement (+DM). Note: self must be low.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="plus_dm", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def minus_dm(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Minus Directional Movement (-DM). Note: self must be low.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="minus_dm", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def aroonosc(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Aroon Oscillator. Note: self must be low.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="aroonosc", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def ultosc(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod1: int = 7, timeperiod2: int = 14, timeperiod3: int = 28) -> pl.Expr:
        """Ultimate Oscillator. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="ultosc", is_elementwise=False, kwargs={"timeperiod1": timeperiod1, "timeperiod2": timeperiod2, "timeperiod3": timeperiod3})
        
    def apo(self, fastperiod: int = 12, slowperiod: int = 26, matype: int = 0) -> pl.Expr:
        """Absolute Price Oscillator (APO).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="apo", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod, "matype": matype})
        
    def ppo(self, fastperiod: int = 12, slowperiod: int = 26, matype: int = 0) -> pl.Expr:
        """Percentage Price Oscillator (PPO).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ppo", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod, "matype": matype})
        
    def sar(self, high: Union[str, pl.Expr], optinacceleration: float = 0.02, optinmaximum: float = 0.2) -> pl.Expr:
        """Parabolic SAR (SAR). Note: self must be low.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="sar", is_elementwise=False, kwargs={"optinacceleration": optinacceleration, "optinmaximum": optinmaximum})
        
    def aroon(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Aroon. Returns struct: down, up. Note: self must be low.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="aroon", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def stoch(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], fastk_period: int = 5, slowk_period: int = 3, slowk_matype: int = 0, slowd_period: int = 3, slowd_matype: int = 0) -> pl.Expr:
        """Stochastic (STOCH). Returns struct: slowk, slowd. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="stoch", is_elementwise=False, kwargs={"fastk_period": fastk_period, "slowk_period": slowk_period, "slowk_matype": slowk_matype, "slowd_period": slowd_period, "slowd_matype": slowd_matype})
        
    def stochf(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], fastk_period: int = 5, fastd_period: int = 3, fastd_matype: int = 0) -> pl.Expr:
        """Stochastic Fast (STOCHF). Returns struct: fastk, fastd. Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="stochf", is_elementwise=False, kwargs={"fastk_period": fastk_period, "fastd_period": fastd_period, "fastd_matype": fastd_matype})
        
    def stochrsi(self, timeperiod: int = 14, fastk_period: int = 5, fastd_period: int = 3, fastd_matype: int = 0) -> pl.Expr:
        """Stochastic RSI (STOCHRSI). Returns struct: fastk, fastd.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="stochrsi", is_elementwise=False, kwargs={"timeperiod": timeperiod, "fastk_period": fastk_period, "fastd_period": fastd_period, "fastd_matype": fastd_matype})
        
    # --------------------------------------------------------------------------------
    # Volatility
    # --------------------------------------------------------------------------------
    def atr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average True Range (ATR). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="atr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def natr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Normalized Average True Range (NATR). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="natr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def trange(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        """True Range (TRANGE). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="trange", is_elementwise=False)

    # --------------------------------------------------------------------------------
    # Volume
    # --------------------------------------------------------------------------------
    def ad(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr]) -> pl.Expr:
        """Chaikin A/D Line (AD). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(args=[high, low, self._expr, volume], plugin_path=Path(__file__).parent, function_name="ad", is_elementwise=False)

    def adosc(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], fastperiod: int = 3, slowperiod: int = 10) -> pl.Expr:
        """Chaikin A/D Oscillator (ADOSC). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(args=[high, low, self._expr, volume], plugin_path=Path(__file__).parent, function_name="adosc", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod})

    # --------------------------------------------------------------------------------
    # Price Transform
    # --------------------------------------------------------------------------------
    def avgprice(self, open: Union[str, pl.Expr], high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        """Average Price (AVGPRICE). Note: self must be close.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(open, str): open = pl.col(open)
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[open, high, low, self._expr], plugin_path=Path(__file__).parent, function_name="avgprice", is_elementwise=False)

    def medprice(self, low: Union[str, pl.Expr]) -> pl.Expr:
        """Median Price (MEDPRICE). Note: self must be high.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[self._expr, low], plugin_path=Path(__file__).parent, function_name="medprice", is_elementwise=False)

    def typprice(self, low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        """Typical Price (TYPPRICE). Note: self must be high.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, low, close], plugin_path=Path(__file__).parent, function_name="typprice", is_elementwise=False)

    def wclprice(self, low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        """Weighted Close Price (WCLPRICE). Note: self must be high.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, low, close], plugin_path=Path(__file__).parent, function_name="wclprice", is_elementwise=False)

    # --------------------------------------------------------------------------------
    # Overlap Studies
    # --------------------------------------------------------------------------------
    def trima(self, timeperiod: int = 30) -> pl.Expr:
        """Triangular Moving Average (TRIMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="trima", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def midpoint(self, timeperiod: int = 14) -> pl.Expr:
        """MidPoint over period (MIDPOINT).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="midpoint", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def midprice(self, low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Midpoint Price over period (MIDPRICE). Note: self must be high.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[self._expr, low], plugin_path=Path(__file__).parent, function_name="midprice", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def kama(self, timeperiod: int = 30) -> pl.Expr:
        """Kaufman Adaptive Moving Average (KAMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="kama", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def t3(self, timeperiod: int = 5, vfactor: float = 0.7) -> pl.Expr:
        """Triple Exponential Moving Average (T3).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="t3", is_elementwise=False, kwargs={"timeperiod": timeperiod, "vfactor": vfactor})

    def dema(self, timeperiod: int = 30) -> pl.Expr:
        """Double Exponential Moving Average (DEMA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="dema", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def macdext(self, fastperiod: int = 12, fastmatype: int = 0, slowperiod: int = 26, slowmatype: int = 0, signalperiod: int = 9, signalmatype: int = 0) -> pl.Expr:
        """MACD with controllable MA type.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macdext", is_elementwise=False, kwargs={"fastperiod": fastperiod, "fastmatype": fastmatype, "slowperiod": slowperiod, "slowmatype": slowmatype, "signalperiod": signalperiod, "signalmatype": signalmatype})

    def macdfix(self, signalperiod: int = 9) -> pl.Expr:
        """Moving Average Convergence/Divergence Fix 12/26.

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macdfix", is_elementwise=False, kwargs={"signalperiod": signalperiod})

    # --------------------------------------------------------------------------------
    # Statistics
    # --------------------------------------------------------------------------------
    def stddev(self, timeperiod: int = 5, nbdev: float = 1.0) -> pl.Expr:
        """Standard Deviation (STDDEV).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="stddev", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdev": nbdev})

    def var(self, timeperiod: int = 5, nbdev: float = 1.0) -> pl.Expr:
        """Variance (VAR).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="var", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdev": nbdev})

    def linearreg(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression (LINEARREG).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_slope(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Slope (LINEARREG_SLOPE).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_slope", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_intercept(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Intercept (LINEARREG_INTERCEPT).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_intercept", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_angle(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Angle (LINEARREG_ANGLE).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_angle", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def tsf(self, timeperiod: int = 14) -> pl.Expr:
        """Time Series Forecast (TSF).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="tsf", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def correl(self, other: Union[str, pl.Expr], timeperiod: int = 30) -> pl.Expr:
        """Pearson's Correlation Coefficient (r) (CORREL).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="correl", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def beta(self, other: Union[str, pl.Expr], timeperiod: int = 5) -> pl.Expr:
        """Beta (BETA).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="beta", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    # --------------------------------------------------------------------------------
    # Hilbert Transform
    # --------------------------------------------------------------------------------
    def ht_dcperiod(self) -> pl.Expr:
        """Hilbert Transform - Dominant Cycle Period (HT_DCPERIOD).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_dcperiod", is_elementwise=False)

    def ht_dcphase(self) -> pl.Expr:
        """Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_dcphase", is_elementwise=False)

    def ht_phasor(self) -> pl.Expr:
        """Hilbert Transform - Phasor Components (HT_PHASOR). Returns Struct(inphase, quadrature).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_phasor", is_elementwise=False)

    def ht_sine(self) -> pl.Expr:
        """Hilbert Transform - SineWave (HT_SINE). Returns Struct(sine, leadsine).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_sine", is_elementwise=False)

    def ht_trendmode(self) -> pl.Expr:
        """Hilbert Transform - Trend vs Cycle Mode (HT_TRENDMODE).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_trendmode", is_elementwise=False)

    def ht_trendline(self) -> pl.Expr:
        """Hilbert Transform - Instantaneous Trendline (HT_TRENDLINE).

Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_trendline", is_elementwise=False)

    # Auto-generated functions

    def acos(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="acos", is_elementwise=False)

    def asin(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="asin", is_elementwise=False)

    def atan(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="atan", is_elementwise=False)

    def ceil(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ceil", is_elementwise=False)

    def cos(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="cos", is_elementwise=False)

    def cosh(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="cosh", is_elementwise=False)

    def exp(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="exp", is_elementwise=False)

    def floor(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="floor", is_elementwise=False)

    def ln(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ln", is_elementwise=False)

    def log10(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="log10", is_elementwise=False)

    def sin(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sin", is_elementwise=False)

    def sinh(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sinh", is_elementwise=False)

    def sqrt(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sqrt", is_elementwise=False)

    def tan(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="tan", is_elementwise=False)

    def tanh(self) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="tanh", is_elementwise=False)

    def max(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="max", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def maxindex(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="maxindex", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def min(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="min", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def minindex(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="minindex", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def sum(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sum", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_linearreg(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ta_linearreg", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_linearreg_angle(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ta_linearreg_angle", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_linearreg_intercept(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ta_linearreg_intercept", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_linearreg_slope(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ta_linearreg_slope", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_tsf(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ta_tsf", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def wma(self, timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="wma", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def add(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="add", is_elementwise=False)

    def div(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="div", is_elementwise=False)

    def mult(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="mult", is_elementwise=False)

    def obv(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="obv", is_elementwise=False)

    def sub(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="sub", is_elementwise=False)

    def ta_beta(self, other: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="ta_beta", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_correl(self, other: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="ta_correl", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_trange(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="ta_trange", is_elementwise=False)

    def ta_atr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="ta_atr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def ta_natr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="ta_natr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def bop(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="bop", is_elementwise=False)

    def cdl_2crows(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_2crows", is_elementwise=False)

    def cdl_3blackcrows(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3blackcrows", is_elementwise=False)

    def cdl_3inside(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3inside", is_elementwise=False)

    def cdl_3linestrike(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3linestrike", is_elementwise=False)

    def cdl_3outside(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3outside", is_elementwise=False)

    def cdl_3starsinsouth(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3starsinsouth", is_elementwise=False)

    def cdl_3whitesoldiers(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_3whitesoldiers", is_elementwise=False)

    def cdl_abandonedbaby(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_abandonedbaby", is_elementwise=False)

    def cdl_advanceblock(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_advanceblock", is_elementwise=False)

    def cdl_belthold(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_belthold", is_elementwise=False)

    def cdl_breakaway(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_breakaway", is_elementwise=False)

    def cdl_closingmarubozu(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_closingmarubozu", is_elementwise=False)

    def cdl_concealbabyswall(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_concealbabyswall", is_elementwise=False)

    def cdl_counterattack(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_counterattack", is_elementwise=False)

    def cdl_darkcloudcover(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_darkcloudcover", is_elementwise=False)

    def cdl_doji(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_doji", is_elementwise=False)

    def cdl_dojistar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_dojistar", is_elementwise=False)

    def cdl_dragonflydoji(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_dragonflydoji", is_elementwise=False)

    def cdl_engulfing(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_engulfing", is_elementwise=False)

    def cdl_eveningdojistar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_eveningdojistar", is_elementwise=False)

    def cdl_eveningstar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_eveningstar", is_elementwise=False)

    def cdl_gapsidesidewhite(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_gapsidesidewhite", is_elementwise=False)

    def cdl_gravestonedoji(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_gravestonedoji", is_elementwise=False)

    def cdl_hammer(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_hammer", is_elementwise=False)

    def cdl_hangingman(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_hangingman", is_elementwise=False)

    def cdl_harami(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_harami", is_elementwise=False)

    def cdl_haramicross(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_haramicross", is_elementwise=False)

    def cdl_highwave(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_highwave", is_elementwise=False)

    def cdl_hikkake(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_hikkake", is_elementwise=False)

    def cdl_hikkakemod(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_hikkakemod", is_elementwise=False)

    def cdl_homingpigeon(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_homingpigeon", is_elementwise=False)

    def cdl_identical3crows(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_identical3crows", is_elementwise=False)

    def cdl_inneck(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_inneck", is_elementwise=False)

    def cdl_invertedhammer(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_invertedhammer", is_elementwise=False)

    def cdl_kicking(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_kicking", is_elementwise=False)

    def cdl_kickingbylength(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_kickingbylength", is_elementwise=False)

    def cdl_ladderbottom(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_ladderbottom", is_elementwise=False)

    def cdl_longleggeddoji(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_longleggeddoji", is_elementwise=False)

    def cdl_longline(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_longline", is_elementwise=False)

    def cdl_marubozu(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_marubozu", is_elementwise=False)

    def cdl_matchinglow(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_matchinglow", is_elementwise=False)

    def cdl_mathold(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_mathold", is_elementwise=False)

    def cdl_morningdojistar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_morningdojistar", is_elementwise=False)

    def cdl_morningstar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_morningstar", is_elementwise=False)

    def cdl_onneck(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_onneck", is_elementwise=False)

    def cdl_piercing(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_piercing", is_elementwise=False)

    def cdl_rickshawman(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_rickshawman", is_elementwise=False)

    def cdl_risefall3methods(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_risefall3methods", is_elementwise=False)

    def cdl_separatinglines(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_separatinglines", is_elementwise=False)

    def cdl_shootingstar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_shootingstar", is_elementwise=False)

    def cdl_shortline(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_shortline", is_elementwise=False)

    def cdl_spinningtop(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_spinningtop", is_elementwise=False)

    def cdl_stalledpattern(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_stalledpattern", is_elementwise=False)

    def cdl_sticksandwich(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_sticksandwich", is_elementwise=False)

    def cdl_takuri(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_takuri", is_elementwise=False)

    def cdl_tasukigap(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_tasukigap", is_elementwise=False)

    def cdl_thrusting(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_thrusting", is_elementwise=False)

    def cdl_tristar(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_tristar", is_elementwise=False)

    def cdl_unique3river(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_unique3river", is_elementwise=False)

    def cdl_upsidegap2crows(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_upsidegap2crows", is_elementwise=False)

    def cdl_xsidegap3methods(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        
        """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="cdl_xsidegap3methods", is_elementwise=False)
