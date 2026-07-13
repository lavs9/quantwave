import quantwave
import quantwave._ta_namespace
import polars as pl
import inspect

methods = [m for m in dir(quantwave._ta_namespace.TaNamespace) if not m.startswith("_")]
print(f"Total ta methods implemented: {len(methods)}")
