import os
import re

POLARS_LIB_PATH = "quantwave-polars/src/lib.rs"
PYTHON_INIT_PATH = "quantwave-py/python/quantwave/_ta_namespace.py"

with open(POLARS_LIB_PATH, "r") as f:
    polars_content = f.read()

with open(PYTHON_INIT_PATH, "r") as f:
    python_content = f.read()

implemented = set(re.findall(r'def\s+([a-zA-Z0-9_]+)\s*\(', python_content))

methods = re.findall(r'pub fn ([a-zA-Z0-9_]+)\s*\(\s*self[^)]*\)\s*->\s*LazyFrame', polars_content)
all_methods = set(m.lower() for m in methods)
missing = sorted(list(all_methods - implemented))

patterns = {}
for m in missing:
    body_match = re.search(r'pub fn ' + m + r'\s*\([^)]*\)\s*->\s*LazyFrame\s*\{([^}]*)\}', polars_content)
    if body_match:
        body = body_match.group(1).strip()
        pattern_match = re.search(r'self\.([a-zA-Z0-9_]+)::<', body)
        if pattern_match:
            pat = pattern_match.group(1)
            patterns.setdefault(pat, []).append(m)

# --------------------------------------------------------------------------
# Per-function default timeperiod.
#
# TA-Lib does NOT use one blanket default. BETA defaults to 5 and CORREL to 30,
# while the LINEARREG family, TSF, ATR and NATR default to 14. Emitting a single
# uniform default therefore made the ``ta_``-prefixed surface -- the one whose
# entire purpose is TA-Lib fidelity -- the surface that diverged from TA-Lib,
# and put it at odds with its own non-prefixed siblings (.ta.beta -> 5,
# .ta.correl -> 30) on the same data.
#
# Values are TA-Lib's own optInTimePeriod defaults (ta_abstract's
# TA_OptInputParameterInfo tables), corroborated by the hand-written siblings.
TALIB_DEFAULT_TIMEPERIOD = {
    "ta_atr": 14,
    "ta_natr": 14,
    "ta_beta": 5,
    "ta_correl": 30,
    "ta_linearreg": 14,
    "ta_linearreg_angle": 14,
    "ta_linearreg_intercept": 14,
    "ta_linearreg_slope": 14,
    "ta_tsf": 14,
}

# Non-prefixed slugs are quantwave's own surface, not a TA-Lib-fidelity promise,
# and they keep quantwave's house default of 14 even where TA-Lib itself uses 30
# (MAX/MIN/SUM/MAXINDEX/MININDEX/WMA). Changing those is a separate, wider
# behaviour change; recorded explicitly here so the divergence is deliberate
# rather than an accident of a blanket default.
QUANTWAVE_DEFAULT_TIMEPERIOD = {
    "max": 14,
    "maxindex": 14,
    "min": 14,
    "minindex": 14,
    "sum": 14,
    "wma": 14,
}


def default_timeperiod(method: str) -> int:
    """Look up a method's default period; refuse to invent a blanket one.

    Emitting a hard-coded default in the templates is what produced the
    ``ta_beta``/``ta_correl`` divergence. Any new period-taking plugin must
    declare its default in one of the tables above, so a regeneration cannot
    silently reintroduce a uniform value.
    """
    if method in TALIB_DEFAULT_TIMEPERIOD:
        return TALIB_DEFAULT_TIMEPERIOD[method]
    if method in QUANTWAVE_DEFAULT_TIMEPERIOD:
        return QUANTWAVE_DEFAULT_TIMEPERIOD[method]
    raise SystemExit(
        f"gen_pyo3_plugins_py: no default timeperiod recorded for {method!r}. "
        "Add it to TALIB_DEFAULT_TIMEPERIOD (ta_-prefixed: use TA-Lib's own "
        "optInTimePeriod default) or QUANTWAVE_DEFAULT_TIMEPERIOD. Do not "
        "reintroduce a blanket default."
    )


py_code = "\n    # Auto-generated functions\n"

for m in patterns.get("math_transform_1_in_1_out", []):
    py_code += f"""
    def {m}(self) -> pl.Expr:
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

for m in patterns.get("math_operator_1_in_1_out_period", []):
    py_code += f"""
    def {m}(self, timeperiod: int = {default_timeperiod(m)}) -> pl.Expr:
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
"""

for m in patterns.get("math_operator_2_in_1_out", []):
    py_code += f"""
    def {m}(self, other: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

for m in patterns.get("math_operator_2_in_1_out_period", []):
    py_code += f"""
    def {m}(self, other: Union[str, pl.Expr], timeperiod: int = {default_timeperiod(m)}) -> pl.Expr:
        if isinstance(other, str): other = pl.col(other)
        return register_plugin_function(args=[self._expr, other], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
"""

# The underlying TA-Lib plugin consumes (high, low, close) positionally. The
# receiver is CLOSE so these read the same way as their non-prefixed siblings
# (e.g. pl.col("close").ta.atr("high", "low")); the args list restores the
# order the Rust side expects. Naming the params high/low — rather than the
# generated in2/in3 — is what makes a mis-ordered call visible at the call site.
for m in patterns.get("ta_3_in_1_out_default", []):
    py_code += f"""
    def {m}(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

# See the note above: receiver is CLOSE, args restore (high, low, close).
for m in patterns.get("ta_3_in_1_out_period", []):
    py_code += f"""
    def {m}(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], timeperiod: int = {default_timeperiod(m)}) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        return register_plugin_function(args=[high, low, self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
"""

for m in patterns.get("ta_4_in_1_out_default", []):
    py_code += f"""
    def {m}(self, high: Union[str, pl.Expr], low: Union[str, pl.Expr], close: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(high, str): high = pl.col(high)
        if isinstance(low, str): low = pl.col(low)
        if isinstance(close, str): close = pl.col(close)
        return register_plugin_function(args=[self._expr, high, low, close], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

with open(PYTHON_INIT_PATH, "a") as f:
    f.write(py_code)

print("Appended python bindings")
