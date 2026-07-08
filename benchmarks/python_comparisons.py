"""Python-side throughput comparisons (pandas / polars / quantwave / optional TA-Lib)."""

from __future__ import annotations

import importlib
import time
from typing import Any

import numpy as np
import polars as pl

from benchmarks.data import OhlcvConfig, generate_ohlcv

TOLERANCE = 1e-3
PREVIEW_ROWS = 1_000
SMA_PERIOD = 20


def _library_versions() -> dict[str, str]:
    versions: dict[str, str] = {}
    for name in ("polars", "pandas", "numpy", "quantwave"):
        try:
            mod = importlib.import_module(name)
            versions[name] = getattr(mod, "__version__", "unknown")
        except ImportError:
            versions[name] = "not_installed"
    for opt in ("talib", "pandas_ta"):
        try:
            mod = importlib.import_module(opt)
            versions[opt] = getattr(mod, "__version__", "installed")
        except ImportError:
            versions[opt] = "not_installed"
    return versions


def _tail_sma_pandas(close: np.ndarray, period: int) -> np.ndarray:
    import pandas as pd

    return pd.Series(close).rolling(period).mean().to_numpy()


def _tail_sma_polars(df: pl.DataFrame, period: int) -> np.ndarray:
    return (
        df.select(pl.col("close").rolling_mean(period).alias("sma"))
        .to_series()
        .to_numpy()
    )


def _tail_sma_quantwave(df: pl.DataFrame, period: int) -> np.ndarray | None:
    try:
        import quantwave  # noqa: F401 — registers .ta

        out = (
            df.lazy()
            .with_columns(pl.col("close").ta.sma(period=period).alias("sma"))
            .collect()["sma"]
            .to_numpy()
        )
        return out
    except Exception:
        return None


def _tail_sma_talib(close: np.ndarray, period: int) -> np.ndarray | None:
    try:
        import talib

        return talib.SMA(close, timeperiod=period)
    except ImportError:
        return None


def correctness_precheck(df: pl.DataFrame) -> dict[str, Any]:
    """Compare SMA outputs on a small slice before timing."""
    close = df["close"].head(PREVIEW_ROWS).to_numpy()
    small = df.head(PREVIEW_ROWS)
    ref = _tail_sma_polars(small, SMA_PERIOD)
    results: dict[str, Any] = {"indicator": "SMA", "period": SMA_PERIOD, "rows": PREVIEW_ROWS}

    checks: dict[str, str] = {}
    pd_sma = _tail_sma_pandas(close, SMA_PERIOD)
    mask = ~np.isnan(ref) & ~np.isnan(pd_sma)
    if mask.any():
        ok = np.allclose(ref[mask], pd_sma[mask], rtol=TOLERANCE, atol=TOLERANCE)
        checks["pandas_rolling"] = "ok" if ok else "mismatch"
    else:
        checks["pandas_rolling"] = "insufficient_warmup"

    qw = _tail_sma_quantwave(small, SMA_PERIOD)
    if qw is not None:
        mask = ~np.isnan(ref) & ~np.isnan(qw)
        if mask.any():
            ok = np.allclose(ref[mask], qw[mask], rtol=TOLERANCE, atol=TOLERANCE)
            checks["quantwave_ta"] = "ok" if ok else "mismatch"
        else:
            checks["quantwave_ta"] = "insufficient_warmup"
    else:
        checks["quantwave_ta"] = "skipped"

    tl = _tail_sma_talib(close, SMA_PERIOD)
    if tl is not None:
        mask = ~np.isnan(ref) & ~np.isnan(tl)
        if mask.any():
            ok = np.allclose(ref[mask], tl[mask], rtol=TOLERANCE, atol=TOLERANCE)
            checks["talib"] = "ok" if ok else "mismatch"
        else:
            checks["talib"] = "insufficient_warmup"
    else:
        checks["talib"] = "not_installed"

    results["checks"] = checks
    failed = [k for k, v in checks.items() if v == "mismatch"]
    results["passed"] = len(failed) == 0
    if failed:
        raise RuntimeError(f"correctness pre-check failed: {failed}")
    return results


def _time_ms(fn, repeats: int = 3) -> float:
    samples: list[float] = []
    for _ in range(repeats):
        t0 = time.perf_counter()
        fn()
        samples.append((time.perf_counter() - t0) * 1000.0)
    return min(samples)


def run_comparisons(quick: bool) -> dict[str, Any]:
    rows = 100_000 if quick else 1_000_000
    df = generate_ohlcv(OhlcvConfig(rows=rows))
    close = df["close"].to_numpy()

    precheck = correctness_precheck(df.head(PREVIEW_ROWS))

    import pandas as pd

    pdf = df.select(["close"]).to_pandas()

    results: dict[str, Any] = {
        "rows": rows,
        "indicator": f"SMA({SMA_PERIOD})",
        "precheck": precheck,
        "libraries": _library_versions(),
        "timings_ms": {},
    }

    results["timings_ms"]["pandas_rolling"] = _time_ms(
        lambda: pdf["close"].rolling(SMA_PERIOD).mean()
    )
    results["timings_ms"]["polars_rolling_mean"] = _time_ms(
        lambda: df.select(pl.col("close").rolling_mean(SMA_PERIOD))
    )

    try:
        import quantwave  # noqa: F401

        results["timings_ms"]["quantwave_polars_ta"] = _time_ms(
            lambda: df.lazy()
            .with_columns(pl.col("close").ta.sma(period=SMA_PERIOD))
            .collect()
        )
    except Exception as exc:
        results["timings_ms"]["quantwave_polars_ta"] = None
        results["quantwave_skip"] = str(exc)

    try:
        import talib

        results["timings_ms"]["talib_batch"] = _time_ms(
            lambda: talib.SMA(close, timeperiod=SMA_PERIOD)
        )
    except ImportError:
        results["timings_ms"]["talib_batch"] = None

    return results