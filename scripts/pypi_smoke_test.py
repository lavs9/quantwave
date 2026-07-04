#!/usr/bin/env python3
"""Install a quantwave wheel in a fresh venv and verify the public Python surface."""

from __future__ import annotations

import argparse
import subprocess
import sys
import tempfile
import venv
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

SMOKE_CORE = r'''
import quantwave as qw

assert qw.__version__, "missing __version__"
names = qw.indicators()
assert len(names) >= 200, f"expected 200+ indicators, got {len(names)}"
meta = qw.metadata("rsi")
assert meta.name, "metadata failed"
cls = qw.streaming_class("rsi")
assert cls is not None, "streaming_class rsi"
inst = cls(14)
vals = [inst.next(float(x)) for x in range(1, 30)]
assert len(vals) == 29, "streaming length"

batch = qw.ta.rsi(14, [float(x) for x in range(1, 30)])
assert len(batch) == 29, "batch rsi length"

print("PYPI_SMOKE_CORE_OK")
'''

SMOKE_POLARS = r'''
import quantwave as qw
import polars as pl

df = pl.DataFrame({
    "timestamp": list(range(20)),
    "open": [10.0 + i * 0.1 for i in range(20)],
    "high": [10.5 + i * 0.1 for i in range(20)],
    "low": [9.5 + i * 0.1 for i in range(20)],
    "close": [10.0 + i * 0.1 for i in range(20)],
    "volume": [1000.0] * 20,
    "signal": [1.0] * 20,
})

out = df.lazy().with_columns(
    pl.col("close").ta.rsi(timeperiod=14).alias("rsi")
).collect()
assert "rsi" in out.columns, "pl.col().ta.rsi missing"
assert out["rsi"].null_count() < len(out), "rsi all null"

from quantwave._backtest import BacktestEngine
report = df.lazy().bt.backtest_with_report(
    signal="signal", commission_bps=0.0, slippage_bps=0.0
)
assert report.metrics is not None, "backtest metrics missing"

from quantwave import build_feature_matrix
fm = build_feature_matrix(df.lazy(), close_col="close")
assert fm.width >= 2, "feature matrix empty"

print("PYPI_SMOKE_POLARS_OK")
'''


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("wheel", type=Path, help="Path to unified quantwave-*.whl")
    args = parser.parse_args()
    if not args.wheel.is_file():
        print(f"Wheel not found: {args.wheel}", file=sys.stderr)
        return 1

    with tempfile.TemporaryDirectory(prefix="quantwave-pypi-smoke-") as td:
        td_path = Path(td)
        venv_dir = td_path / "venv"
        venv.create(venv_dir, with_pip=True)
        py = venv_dir / "bin" / "python"
        if not py.exists():
            py = venv_dir / "Scripts" / "python.exe"

        subprocess.check_call([str(py), "-m", "pip", "install", "-q", "--upgrade", "pip"])
        subprocess.check_call([str(py), "-m", "pip", "install", "-q", str(args.wheel)])

        proc_core = subprocess.run(
            [str(py), "-c", SMOKE_CORE],
            capture_output=True,
            text=True,
        )
        if proc_core.returncode != 0:
            print(proc_core.stdout, file=sys.stderr)
            print(proc_core.stderr, file=sys.stderr)
            return proc_core.returncode
        if "PYPI_SMOKE_CORE_OK" not in proc_core.stdout:
            print(
                "Core smoke (no polars) did not complete:",
                proc_core.stdout,
                proc_core.stderr,
                file=sys.stderr,
            )
            return 1

        subprocess.check_call(
            [str(py), "-m", "pip", "install", "-q", "polars>=1.20.0,<2.0.0"]
        )
        proc = subprocess.run(
            [str(py), "-c", SMOKE_POLARS],
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            print(proc.stdout, file=sys.stderr)
            print(proc.stderr, file=sys.stderr)
            return proc.returncode
        if "PYPI_SMOKE_POLARS_OK" not in proc.stdout:
            print("Polars smoke did not complete:", proc.stdout, proc.stderr, file=sys.stderr)
            return 1

    print("PyPI smoke test passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())