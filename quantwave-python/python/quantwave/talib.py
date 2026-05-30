"""
TA-Lib compatible interface.

This module provides functions with naming and (where reasonable) parameter
compatibility with the classic `talib` library.

Example:
    from quantwave import talib as ta
    rsi = ta.RSI(close, timeperiod=14)
"""

from ._quantwave import (
    # Common TA-Lib style wrappers (we expose the native ones with similar names)
    rsi as RSI,
    macd as MACD,
    bbands as BBANDS,
    atr as ATR,
    adx as ADX,
    stoch as STOCH,
    willr as WILLR,
    obv as OBV,
    ad as AD,
    adosc as ADOSC,
    # Add more aliases as we implement proper wrappers
)

__all__ = [
    "RSI", "MACD", "BBANDS", "ATR", "ADX", "STOCH", "WILLR",
    "OBV", "AD", "ADOSC",
]
