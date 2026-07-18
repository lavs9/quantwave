"""Generator script for the bundled sample dataset (quantwave/data/sample.parquet).

Not imported by the package at runtime — this is a one-shot script kept for
reproducibility/documentation of how ``quantwave/data/sample.parquet`` was
produced. Regenerate with:

    python -m quantwave._generate_sample

The sample is entirely synthetic (see ``quantwave.datasets.synthetic`` and
``quantwave.datasets.load_sample`` docstrings) — it is NOT real NSE or any
other exchange's data, and is safe to redistribute.

Recipe: one NIFTY-like index ("NIFTY") plus two stock-like instruments
("STOCK_A", "STOCK_B"), daily bars, ~10 years (2015-01-01 .. ~2024-12-31),
seed=1337, 5 volatility regimes, generated via ``quantwave.datasets.synthetic``
and written to parquet with zstd compression to keep the committed asset
small (~1-2MB).
"""

from __future__ import annotations

from datetime import datetime
from pathlib import Path

from .datasets import synthetic

SAMPLE_SEED = 1337
SAMPLE_ROWS = 10 * 252  # ~10 trading years of daily bars
SAMPLE_SYMBOLS = ["NIFTY", "STOCK_A", "STOCK_B"]
SAMPLE_START = datetime(2015, 1, 1, 9, 15)
SAMPLE_N_REGIMES = 5

OUT_PATH = Path(__file__).resolve().parent / "data" / "sample.parquet"


def generate() -> None:
    df = synthetic(
        seed=SAMPLE_SEED,
        rows=SAMPLE_ROWS,
        start=SAMPLE_START,
        freq="1d",
        symbols=SAMPLE_SYMBOLS,
        n_regimes=SAMPLE_N_REGIMES,
        base_price=20000.0,
    )
    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    df.write_parquet(OUT_PATH, compression="zstd")
    size_kb = OUT_PATH.stat().st_size / 1024.0
    print(f"Wrote {len(df)} rows for {len(SAMPLE_SYMBOLS)} symbols to {OUT_PATH} ({size_kb:.1f} KB)")


if __name__ == "__main__":
    generate()
