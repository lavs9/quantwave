"""Deterministic synthetic OHLCV data for benchmark harness."""

from __future__ import annotations

import hashlib
from dataclasses import dataclass

import numpy as np
import polars as pl

DEFAULT_SEED = 0x5157_0001
DEFAULT_ROWS = 1_000_000


@dataclass(frozen=True)
class OhlcvConfig:
    rows: int = DEFAULT_ROWS
    seed: int = DEFAULT_SEED
    symbols: int = 1_000


def generate_ohlcv(cfg: OhlcvConfig | None = None) -> pl.DataFrame:
    """Return a deterministic OHLCV frame (close-only path uses close column)."""
    cfg = cfg or OhlcvConfig()
    rng = np.random.default_rng(cfg.seed)

    n = cfg.rows
    close = rng.uniform(100.0, 200.0, n).astype(np.float64)
    spread = rng.uniform(0.5, 2.0, n)
    high = close + spread
    low = close - spread
    open_ = close + rng.uniform(-1.0, 1.0, n)
    volume = rng.integers(1_000, 50_000, n, dtype=np.int64)
    sym_ids = rng.integers(0, cfg.symbols, n, dtype=np.int32)
    symbol = pl.Series("symbol", [f"SYM{i:04d}" for i in sym_ids])

    return pl.DataFrame(
        {
            "open": open_,
            "high": high,
            "low": low,
            "close": close,
            "volume": volume,
            "symbol": symbol,
        }
    )


def frame_hash(df: pl.DataFrame) -> str:
    """Stable hash of numeric columns for harness self-tests."""
    cols = [c for c in ("open", "high", "low", "close", "volume") if c in df.columns]
    parts = [df[c].to_numpy().tobytes() for c in cols]
    return hashlib.sha256(b"".join(parts)).hexdigest()