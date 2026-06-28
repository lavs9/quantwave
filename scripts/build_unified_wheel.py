#!/usr/bin/env python3
"""Build a single quantwave PyPI wheel bundling core, backtest, and Polars plugins."""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def workspace_version() -> str:
    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'^\s*version\s*=\s*"([^"]+)"', cargo, re.M)
    if not m:
        raise SystemExit("Could not read workspace version from Cargo.toml")
    return m.group(1)


def run(cmd: list[str], **kwargs) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.check_call(cmd, cwd=ROOT, **kwargs)


def maturin_build(manifest: Path, out: Path, release: bool) -> None:
    out.mkdir(parents=True, exist_ok=True)
    cmd = [
        sys.executable,
        "-m",
        "maturin",
        "build",
        "--manifest-path",
        str(manifest),
        "--out",
        str(out),
    ]
    if release:
        cmd.append("--release")
    run(cmd)


def pick_wheel(dist: Path, prefix: str) -> Path:
    wheels = sorted(dist.glob(f"{prefix}*.whl"))
    if not wheels:
        raise SystemExit(f"No wheel matching {prefix!r} in {dist}")
    if len(wheels) > 1:
        # Prefer platform-specific wheels over py3-none-any when both exist.
        platform = [w for w in wheels if "py3-none" not in w.name]
        wheels = platform or wheels
    return wheels[-1]


def merge_wheels(base: Path, extras: list[Path], out_dir: Path) -> Path:
    try:
        import wheel  # noqa: F401
    except ImportError:
        subprocess.check_call([sys.executable, "-m", "pip", "install", "wheel"])
    with tempfile.TemporaryDirectory(prefix="quantwave-wheel-") as td:
        work = Path(td)
        unpack_root = work / "unpack"
        unpack_root.mkdir()
        run([sys.executable, "-m", "wheel", "unpack", str(base), "-d", str(unpack_root)])
        merged = next(unpack_root.iterdir())

        for extra in extras:
            extra_unpack = work / "extra"
            extra_unpack.mkdir()
            run([sys.executable, "-m", "wheel", "unpack", str(extra), "-d", str(extra_unpack)])
            extra_root = next(extra_unpack.iterdir())
            for src in extra_root.rglob("*"):
                if not src.is_file():
                    continue
                if "__pycache__" in src.parts or src.suffix == ".pyc":
                    continue
                if any(part.endswith(".dist-info") for part in src.parts):
                    continue
                rel = src.relative_to(extra_root)
                dst = merged / rel
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(src, dst)
            shutil.rmtree(extra_unpack)

        out_dir.mkdir(parents=True, exist_ok=True)
        run([sys.executable, "-m", "wheel", "pack", str(merged), "-d", str(out_dir)])
        packed = sorted(out_dir.glob("quantwave-*.whl"))
        if not packed:
            raise SystemExit(f"wheel pack did not produce quantwave-*.whl in {out_dir}")
        final = out_dir / f"quantwave-{workspace_version()}-{packed[-1].name.split('-', 2)[-1]}"
        if packed[-1] != final:
            packed[-1].rename(final)
        return final


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=ROOT / "dist")
    parser.add_argument("--debug", action="store_true", help="Build debug wheels (faster, local only)")
    args = parser.parse_args()

    staging = args.out / ".staging"
    if staging.exists():
        shutil.rmtree(staging)
    staging.mkdir(parents=True)

    release = not args.debug
    maturin_build(ROOT / "quantwave-python" / "Cargo.toml", staging, release)
    maturin_build(ROOT / "quantwave-backtest-py" / "Cargo.toml", staging, release)
    maturin_build(ROOT / "quantwave-plugins" / "Cargo.toml", staging, release)

    core = pick_wheel(staging, "quantwave-")
    backtest = pick_wheel(staging, "quantwave_backtest_native-")
    plugins = pick_wheel(staging, "quantwave_plugins-")

    args.out.mkdir(parents=True, exist_ok=True)
    final = merge_wheels(core, [backtest, plugins], args.out)
    print(f"Unified wheel: {final}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())