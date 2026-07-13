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

py_code = "\n    # Auto-generated functions\n"

for m in patterns.get("math_transform_1_in_1_out", []):
    py_code += f"""
    def {m}(self) -> pl.Expr:
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

for m in patterns.get("math_operator_1_in_1_out_period", []):
    py_code += f"""
    def {m}(self, timeperiod: int = 14) -> pl.Expr:
        return register_plugin_function(args=[self._expr], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
"""

for m in patterns.get("math_operator_2_in_1_out", []):
    py_code += f"""
    def {m}(self, in2: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

for m in patterns.get("math_operator_2_in_1_out_period", []):
    py_code += f"""
    def {m}(self, in2: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        if isinstance(in2, str): in2 = pl.col(in2)
        return register_plugin_function(args=[self._expr, in2], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
"""

for m in patterns.get("ta_3_in_1_out_default", []):
    py_code += f"""
    def {m}(self, in2: Union[str, pl.Expr], in3: Union[str, pl.Expr]) -> pl.Expr:
        if isinstance(in2, str): in2 = pl.col(in2)
        if isinstance(in3, str): in3 = pl.col(in3)
        return register_plugin_function(args=[self._expr, in2, in3], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False)
"""

for m in patterns.get("ta_3_in_1_out_period", []):
    py_code += f"""
    def {m}(self, in2: Union[str, pl.Expr], in3: Union[str, pl.Expr], timeperiod: int = 14) -> pl.Expr:
        if isinstance(in2, str): in2 = pl.col(in2)
        if isinstance(in3, str): in3 = pl.col(in3)
        return register_plugin_function(args=[self._expr, in2, in3], plugin_path=Path(__file__).parent, function_name="{m}", is_elementwise=False, kwargs={{"timeperiod": timeperiod}})
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
