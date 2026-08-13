"""Deterministic synthetic OHLCV data for benchmark harness.

The frame produced here is hashed into ``benchmarks/results/latest.json`` as
``dataset.frame_hash``, whose entire job is to prove that two benchmark runs
measured *the same data*. That only works if the generator is reproducible
across environments, so this module deliberately does not use ``numpy.random``.

``np.random.default_rng`` returns a ``Generator``, and NumPy's random-number
policy (NEP 19) explicitly reserves the right to change ``Generator`` streams in
feature releases -- only the legacy ``RandomState`` is frozen. Depending on it
meant the same seed produced different data on different numpy builds, so
``frame_hash`` moved whenever CI's pip resolution shifted and every nightly run
committed a spurious diff to main (quantwave-5yjg).

Instead, values are drawn from SplitMix64 implemented here in explicit uint64
arithmetic. It is counter-based (value ``i`` depends only on the seed, the
stream id and ``i``), so it is fully specified by this file, independent of any
library version, and identical on every platform. ``test_benchmark_harness.py``
pins the output with golden digests; if anything perturbs the stream, that test
fails loudly instead of quietly rewriting the benchmark baseline.
"""

from __future__ import annotations

import hashlib
from dataclasses import dataclass

import numpy as np
import polars as pl

DEFAULT_SEED = 0x5157_0001
DEFAULT_ROWS = 1_000_000

# Format tag for frame_hash. Bump when the hashed byte layout changes, so a
# changed digest is attributable rather than mysterious.
FRAME_HASH_VERSION = b"quantwave-frame-hash-v2"

# SplitMix64 constants (Steele, Lea & Flood 2014). Typed as np.uint64 on
# purpose: mixing a uint64 array with a plain Python int promotes to float64 on
# numpy 1.x and silently destroys the bit pattern.
_GAMMA = np.uint64(0x9E3779B97F4A7C15)
_MIX_A = np.uint64(0xBF58476D1CE4E5B9)
_MIX_B = np.uint64(0x94D049BB133111EB)
_SHIFT_30 = np.uint64(30)
_SHIFT_27 = np.uint64(27)
_SHIFT_31 = np.uint64(31)
_SHIFT_11 = np.uint64(11)
_U64_MASK = 0xFFFF_FFFF_FFFF_FFFF

# Separates the per-column substreams. Odd, so successive stream ids never
# collide with each other's counter space.
_STREAM_STRIDE = 0x1000_0000_0000_0001

# 2**-53: scales a 53-bit mantissa into [0, 1) exactly, the standard
# construction for a double from random bits.
_TWO_POW_NEG53 = 1.0 / (1 << 53)


@dataclass(frozen=True)
class OhlcvConfig:
    rows: int = DEFAULT_ROWS
    seed: int = DEFAULT_SEED
    symbols: int = 1_000


def _splitmix64(seed: int, count: int, stream: int) -> np.ndarray:
    """``count`` uint64 draws from the ``stream``-th substream of ``seed``.

    Counter-based: element ``i`` is a pure function of (seed, stream, i), so
    slicing or reordering never changes a value. Unsigned overflow wraps in
    numpy, which is exactly the mod-2^64 arithmetic SplitMix64 specifies.
    """
    base = np.uint64((seed + stream * _STREAM_STRIDE) & _U64_MASK)
    z = base + np.arange(count, dtype=np.uint64) * _GAMMA
    z ^= z >> _SHIFT_30
    z *= _MIX_A
    z ^= z >> _SHIFT_27
    z *= _MIX_B
    z ^= z >> _SHIFT_31
    return z


def _uniform(seed: int, count: int, stream: int, lo: float, hi: float) -> np.ndarray:
    """``count`` float64 draws uniform on ``[lo, hi)``."""
    bits = _splitmix64(seed, count, stream) >> _SHIFT_11
    return lo + bits.astype(np.float64) * _TWO_POW_NEG53 * (hi - lo)


def _integers(seed: int, count: int, stream: int, lo: int, hi: int) -> np.ndarray:
    """``count`` int64 draws uniform on ``[lo, hi)``.

    Uses modulo reduction. The bias is ~2^-64 per draw and, more to the point,
    it is *specified here* rather than delegated to a library that may change
    its rejection strategy between releases.
    """
    span = np.uint64(hi - lo)
    return (_splitmix64(seed, count, stream) % span).astype(np.int64) + lo


def generate_ohlcv(cfg: OhlcvConfig | None = None) -> pl.DataFrame:
    """Return a deterministic OHLCV frame (close-only path uses close column)."""
    cfg = cfg or OhlcvConfig()
    n = cfg.rows
    seed = cfg.seed

    # One substream per column, so adding or resizing a column never shifts the
    # values of any other -- a single shared stream would renumber everything.
    close = _uniform(seed, n, 0, 100.0, 200.0)
    spread = _uniform(seed, n, 1, 0.5, 2.0)
    high = close + spread
    low = close - spread
    open_ = close + _uniform(seed, n, 2, -1.0, 1.0)
    volume = _integers(seed, n, 3, 1_000, 50_000)
    sym_ids = _integers(seed, n, 4, 0, cfg.symbols).astype(np.int32)

    symbol = pl.Series("symbol", np.char.add("SYM", np.char.zfill(sym_ids.astype("U"), 4)))

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
    """Stable hash of the numeric columns, for cross-run dataset identity.

    Column name, dtype and length are folded in alongside the bytes so that a
    reordered or retyped frame cannot collide with the original. Arrays are
    normalised to little-endian first, so the digest is comparable between a
    big-endian and a little-endian machine rather than merely between two runs
    on the same one.
    """
    cols = [c for c in ("open", "high", "low", "close", "volume") if c in df.columns]
    digest = hashlib.sha256()
    digest.update(FRAME_HASH_VERSION + b"\n")
    for name in cols:
        arr = df[name].to_numpy()
        arr = arr.astype(arr.dtype.newbyteorder("<"), copy=False)
        digest.update(f"{name}:{arr.dtype.str}:{arr.size}\n".encode())
        digest.update(arr.tobytes())
    return digest.hexdigest()
