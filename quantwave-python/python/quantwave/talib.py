"""
TA-Lib compatible interface.

This module provides functions with naming and (where reasonable) parameter
compatibility with the classic `talib` library.

Example:
    from quantwave import talib as ta
    rsi = ta.RSI(close, timeperiod=14)
"""

# Resilient for gqem / namespace work: some talib-style names (bbands etc) may have
# different casing or not be direct pyfns in every build. Load what exists.
_loaded = {}
for _src, _alias in [
    ("rsi", "RSI"), ("macd", "MACD"), ("bbands", "BBANDS"), ("atr", "ATR"),
    ("adx", "ADX"), ("stoch", "STOCH"), ("willr", "WILLR"),
    ("obv", "OBV"), ("ad", "AD"), ("adosc", "ADOSC"),
]:
    try:
        from ._quantwave import __dict__ as _d
        if _src in _d:
            _loaded[_alias] = _d[_src]
        else:
            # try direct
            exec(f"from ._quantwave import {_src} as {_alias}")
            _loaded[_alias] = locals().get(_alias)
    except Exception:
        _loaded[_alias] = None

# Bind to module globals
for _a, _v in _loaded.items():
    globals()[_a] = _v

__all__ = list(_loaded.keys())
