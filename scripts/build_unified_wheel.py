#!/usr/bin/env python3
"""Build a single quantwave PyPI wheel bundling core, backtest, and Polars plugins.

Since quantwave-5ipk.10 all three crates are PyO3 ``abi3-py39`` extensions, so every
per-crate wheel already carries the SAME ``cp39-abi3-<platform>`` tag. There is no
tag to reconcile — this script just drops the extra ``.so`` files into the base wheel
and repacks. (Before 5ipk.10, quantwave-python was a uniffi ``py3-none`` wheel and this
script hand-reconciled the mismatched tags — the source of the 9gek.1 wheel-tag bug.)
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def run(cmd: list[str], **kwargs) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.check_call(cmd, cwd=ROOT, **kwargs)


def read_wheel_tag(path: Path) -> str:
    with zipfile.ZipFile(path) as zf:
        wheel_entries = [n for n in zf.namelist() if n.endswith(".dist-info/WHEEL")]
        if not wheel_entries:
            raise ValueError(f"{path.name}: missing WHEEL metadata")
        for line in zf.read(wheel_entries[0]).decode("utf-8").splitlines():
            if line.startswith("Tag: "):
                return line.removeprefix("Tag: ").strip()
    raise ValueError(f"{path.name}: Tag line missing")


def assert_uniform_abi3(paths: list[Path]) -> str:
    """Every crate must ship the same cp3x-abi3-<platform> tag (5ipk.10 invariant)."""
    tags = {p.name: read_wheel_tag(p) for p in paths}
    distinct = set(tags.values())
    if len(distinct) != 1:
        raise SystemExit(f"wheels have divergent tags (expected one abi3 tag): {tags}")
    tag = distinct.pop()
    if not re.fullmatch(r"cp\d+-abi3-.+", tag):
        raise SystemExit(
            f"expected a cp3x-abi3-<platform> tag but got {tag!r}; is a crate still "
            "building a non-abi3 (uniffi py3-none or cpXY) wheel?"
        )
    return tag


def maturin_build(manifest: Path, out: Path, release: bool) -> None:
    out.mkdir(parents=True, exist_ok=True)
    cmd = [sys.executable, "-m", "maturin", "build", "--manifest-path", str(manifest), "--out", str(out)]
    if release:
        cmd.append("--release")
    run(cmd)


def pick_wheel(dist: Path, prefix: str) -> Path:
    wheels = sorted(dist.glob(f"{prefix}*.whl"))
    if not wheels:
        raise SystemExit(f"No wheel matching {prefix!r} in {dist}")
    return wheels[-1]


def bundle(base: Path, extras: list[Path], out_dir: Path) -> Path:
    """Unpack the base wheel, copy the extras' payload (their .so files) into it, repack.

    All wheels share one abi3 tag, so the base wheel's tag is kept verbatim.
    """
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

        for i, extra in enumerate(extras):
            extra_unpack = work / f"extra{i}"
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
                dst = merged / src.relative_to(extra_root)
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(src, dst)

        out_dir.mkdir(parents=True, exist_ok=True)
        run([sys.executable, "-m", "wheel", "pack", str(merged), "-d", str(out_dir)])
        packed = sorted(out_dir.glob("quantwave-*.whl"))
        if not packed:
            raise SystemExit(f"wheel pack did not produce quantwave-*.whl in {out_dir}")
        return packed[-1]


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

    tag = assert_uniform_abi3([core, backtest, plugins])
    print(f"All crates ship uniform tag: {tag}")

    final = bundle(core, [backtest, plugins], args.out)
    print(f"Unified wheel: {final}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
