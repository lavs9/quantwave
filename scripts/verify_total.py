import quantwave
import quantwave_plugins
import polars as pl
import inspect

methods = [m for m in dir(quantwave_plugins.TaNamespace) if not m.startswith("_")]
print(f"Total ta methods implemented: {len(methods)}")
