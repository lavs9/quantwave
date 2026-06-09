import polars as pl
from polars.plugins import register_plugin_function
from pathlib import Path
from typing import Union

@pl.api.register_expr_namespace("ta")
class TaNamespace:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sma(self, period: int) -> pl.Expr:
        """Calculates the Simple Moving Average (SMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="sma", is_elementwise=False, kwargs={"period": period})

    def ema(self, period: int) -> pl.Expr:
        """Calculates the Exponential Moving Average (EMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ema", is_elementwise=False, kwargs={"period": period})

    def rsi(self, timeperiod: int = 14) -> pl.Expr:
        """Calculates the Relative Strength Index (RSI)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rsi", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def macd(self, fast: int = 12, slow: int = 26, signal: int = 9) -> pl.Expr:
        """Calculates the Moving Average Convergence Divergence (MACD). Returns a struct containing: macd, signal, hist."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macd", is_elementwise=False, kwargs={"fast": fast, "slow": slow, "signal": signal})

    def bbands(self, timeperiod: int = 5, nbdevup: float = 2.0, nbdevdn: float = 2.0, matype: int = 0) -> pl.Expr:
        """Calculates Bollinger Bands. Returns a struct containing: upper, middle, lower. matype defaults to 0 (SMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="bbands", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdevup": nbdevup, "nbdevdn": nbdevdn, "matype": matype})

    # --------------------------------------------------------------------------------
    # Momentum & Trend
    # --------------------------------------------------------------------------------
    
    def cci(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Commodity Channel Index (CCI). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="cci", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def cmo(self, timeperiod: int = 14) -> pl.Expr:
        """Chande Momentum Oscillator (CMO)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="cmo", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def mom(self, timeperiod: int = 10) -> pl.Expr:
        """Momentum (MOM)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="mom", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def roc(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change (ROC)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="roc", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocp(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Percentage (ROCP)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocp", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocr(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Ratio (ROCR)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def rocr100(self, timeperiod: int = 10) -> pl.Expr:
        """Rate of Change Ratio 100 Scale (ROCR100)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="rocr100", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def trix(self, timeperiod: int = 30) -> pl.Expr:
        """1-day Rate-Of-Change (ROC) of a Triple Smooth EMA (TRIX)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="trix", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def willr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Williams' %R (WILLR). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="willr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def adx(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average Directional Movement Index (ADX). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="adx", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def adxr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average Directional Movement Index Rating (ADXR). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="adxr", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def dx(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Directional Movement Index (DX). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="dx", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def plus_di(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Plus Directional Indicator (+DI). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="plus_di", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def minus_di(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Minus Directional Indicator (-DI). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="minus_di", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def plus_dm(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Plus Directional Movement (+DM). Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="plus_dm", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def minus_dm(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Minus Directional Movement (-DM). Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="minus_dm", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def aroonosc(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Aroon Oscillator. Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="aroonosc", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def ultosc(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod1: int = 7, timeperiod2: int = 14, timeperiod3: int = 28) -> pl.Expr:
        """Ultimate Oscillator. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="ultosc", is_elementwise=False, kwargs={"timeperiod1": timeperiod1, "timeperiod2": timeperiod2, "timeperiod3": timeperiod3})
        
    def apo(self, fastperiod: int = 12, slowperiod: int = 26, matype: int = 0) -> pl.Expr:
        """Absolute Price Oscillator (APO)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="apo", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod, "matype": matype})
        
    def ppo(self, fastperiod: int = 12, slowperiod: int = 26, matype: int = 0) -> pl.Expr:
        """Percentage Price Oscillator (PPO)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ppo", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod, "matype": matype})
        
    def sar(self, high: Union[str, pl.Expr], optinacceleration: float = 0.02, optinmaximum: float = 0.2) -> pl.Expr:
        """Parabolic SAR (SAR). Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="sar", is_elementwise=False, kwargs={"optinacceleration": optinacceleration, "optinmaximum": optinmaximum})
        
    def aroon(self, high: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Aroon. Returns struct: down, up. Note: self must be low."""
        if isinstance(high, str): high = pl.col(high)
        return register_plugin_function(args=[high, self._expr], plugin_path=Path(__file__).parent, function_name="aroon", is_elementwise=False, kwargs={"timeperiod": timeperiod})
        
    def stoch(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], fastk_period: int = 5, slowk_period: int = 3, slowk_matype: int = 0, slowd_period: int = 3, slowd_matype: int = 0) -> pl.Expr:
        """Stochastic (STOCH). Returns struct: slowk, slowd. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="stoch", is_elementwise=False, kwargs={"fastk_period": fastk_period, "slowk_period": slowk_period, "slowk_matype": slowk_matype, "slowd_period": slowd_period, "slowd_matype": slowd_matype})
        
    def stochf(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], fastk_period: int = 5, fastd_period: int = 3, fastd_matype: int = 0) -> pl.Expr:
        """Stochastic Fast (STOCHF). Returns struct: fastk, fastd. Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="stochf", is_elementwise=False, kwargs={"fastk_period": fastk_period, "fastd_period": fastd_period, "fastd_matype": fastd_matype})
        
    def stochrsi(self, timeperiod: int = 14, fastk_period: int = 5, fastd_period: int = 3, fastd_matype: int = 0) -> pl.Expr:
        """Stochastic RSI (STOCHRSI). Returns struct: fastk, fastd."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="stochrsi", is_elementwise=False, kwargs={"timeperiod": timeperiod, "fastk_period": fastk_period, "fastd_period": fastd_period, "fastd_matype": fastd_matype})
        
    # --------------------------------------------------------------------------------
    # Volatility
    # --------------------------------------------------------------------------------
    def atr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Average True Range (ATR). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="atr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def natr(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Normalized Average True Range (NATR). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="natr", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def trange(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        """True Range (TRANGE). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="trange", is_elementwise=False)

    # --------------------------------------------------------------------------------
    # Volume
    # --------------------------------------------------------------------------------
    def ad(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr]) -> pl.Expr:
        """Chaikin A/D Line (AD). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(args=[high, low, self._expr, volume], plugin_path=Path(__file__).parent, function_name="ad", is_elementwise=False)

    def adosc(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], volume: Union[str, pl.Expr], fastperiod: int = 3, slowperiod: int = 10) -> pl.Expr:
        """Chaikin A/D Oscillator (ADOSC). Note: self must be close."""
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(volume, str): volume = pl.col(volume)
        return register_plugin_function(args=[high, low, self._expr, volume], plugin_path=Path(__file__).parent, function_name="adosc", is_elementwise=False, kwargs={"fastperiod": fastperiod, "slowperiod": slowperiod})

    # --------------------------------------------------------------------------------
    # Price Transform
    # --------------------------------------------------------------------------------
    def avgprice(self, open: Union[str, pl.Expr], high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        """Average Price (AVGPRICE). Note: self must be close."""
        if isinstance(open, str): open = pl.col(open)
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[open, high, low, self._expr], plugin_path=Path(__file__).parent, function_name="avgprice", is_elementwise=False)

    def medprice(self, low: Union[str, pl.Expr]) -> pl.Expr:
        """Median Price (MEDPRICE). Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[self._expr, low], plugin_path=Path(__file__).parent, function_name="medprice", is_elementwise=False)

    def typprice(self, low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        """Typical Price (TYPPRICE). Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, low, close], plugin_path=Path(__file__).parent, function_name="typprice", is_elementwise=False)

    def wclprice(self, low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        """Weighted Close Price (WCLPRICE). Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, low, close], plugin_path=Path(__file__).parent, function_name="wclprice", is_elementwise=False)

    # --------------------------------------------------------------------------------
    # Overlap Studies
    # --------------------------------------------------------------------------------
    def trima(self, timeperiod: int = 30) -> pl.Expr:
        """Triangular Moving Average (TRIMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="trima", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def midpoint(self, timeperiod: int = 14) -> pl.Expr:
        """MidPoint over period (MIDPOINT)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="midpoint", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def midprice(self, low: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        """Midpoint Price over period (MIDPRICE). Note: self must be high."""
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[self._expr, low], plugin_path=Path(__file__).parent, function_name="midprice", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def kama(self, timeperiod: int = 30) -> pl.Expr:
        """Kaufman Adaptive Moving Average (KAMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="kama", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def t3(self, timeperiod: int = 5, vfactor: float = 0.7) -> pl.Expr:
        """Triple Exponential Moving Average (T3)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="t3", is_elementwise=False, kwargs={"timeperiod": timeperiod, "vfactor": vfactor})

    def dema(self, timeperiod: int = 30) -> pl.Expr:
        """Double Exponential Moving Average (DEMA)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="dema", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def macdext(self, fastperiod: int = 12, fastmatype: int = 0, slowperiod: int = 26, slowmatype: int = 0, signalperiod: int = 9, signalmatype: int = 0) -> pl.Expr:
        """MACD with controllable MA type."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macdext", is_elementwise=False, kwargs={"fastperiod": fastperiod, "fastmatype": fastmatype, "slowperiod": slowperiod, "slowmatype": slowmatype, "signalperiod": signalperiod, "signalmatype": signalmatype})

    def macdfix(self, signalperiod: int = 9) -> pl.Expr:
        """Moving Average Convergence/Divergence Fix 12/26."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="macdfix", is_elementwise=False, kwargs={"signalperiod": signalperiod})

    # --------------------------------------------------------------------------------
    # Statistics
    # --------------------------------------------------------------------------------
    def stddev(self, timeperiod: int = 5, nbdev: float = 1.0) -> pl.Expr:
        """Standard Deviation (STDDEV)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="stddev", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdev": nbdev})

    def var(self, timeperiod: int = 5, nbdev: float = 1.0) -> pl.Expr:
        """Variance (VAR)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="var", is_elementwise=False, kwargs={"timeperiod": timeperiod, "nbdev": nbdev})

    def linearreg(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression (LINEARREG)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_slope(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Slope (LINEARREG_SLOPE)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_slope", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_intercept(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Intercept (LINEARREG_INTERCEPT)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_intercept", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def linearreg_angle(self, timeperiod: int = 14) -> pl.Expr:
        """Linear Regression Angle (LINEARREG_ANGLE)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="linearreg_angle", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def tsf(self, timeperiod: int = 14) -> pl.Expr:
        """Time Series Forecast (TSF)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="tsf", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def correl(self, other: Union[str, pl.Expr], timeperiod: int = 30) -> pl.Expr:
        """Pearson's Correlation Coefficient (r) (CORREL)."""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="correl", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    def beta(self, other: Union[str, pl.Expr], timeperiod: int = 5) -> pl.Expr:
        """Beta (BETA)."""
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="beta", is_elementwise=False, kwargs={"timeperiod": timeperiod})

    # --------------------------------------------------------------------------------
    # Hilbert Transform
    # --------------------------------------------------------------------------------
    def ht_dcperiod(self) -> pl.Expr:
        """Hilbert Transform - Dominant Cycle Period (HT_DCPERIOD)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_dcperiod", is_elementwise=False)

    def ht_dcphase(self) -> pl.Expr:
        """Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_dcphase", is_elementwise=False)

    def ht_phasor(self) -> pl.Expr:
        """Hilbert Transform - Phasor Components (HT_PHASOR). Returns Struct(inphase, quadrature)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_phasor", is_elementwise=False)

    def ht_sine(self) -> pl.Expr:
        """Hilbert Transform - SineWave (HT_SINE). Returns Struct(sine, leadsine)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_sine", is_elementwise=False)

    def ht_trendmode(self) -> pl.Expr:
        """Hilbert Transform - Trend vs Cycle Mode (HT_TRENDMODE)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_trendmode", is_elementwise=False)

    def ht_trendline(self) -> pl.Expr:
        """Hilbert Transform - Instantaneous Trendline (HT_TRENDLINE)."""
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="ht_trendline", is_elementwise=False)
