#!/usr/bin/env python3
"""Build a single quantwave PyPI wheel bundling core, backtest, and Polars plugins."""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
import zipfile
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


@dataclass(frozen=True)
class WheelTag:
    python: str
    abi: str
    platform: str

    @classmethod
    def parse(cls, tag: str) -> WheelTag:
        parts = tag.split("-", 2)
        if len(parts) != 3:
            raise ValueError(f"invalid wheel tag: {tag!r}")
        return cls(python=parts[0], abi=parts[1], platform=parts[2])

    def serialize(self) -> str:
        return f"{self.python}-{self.abi}-{self.platform}"

    @property
    def is_py3_none(self) -> bool:
        return self.python == "py3" and self.abi == "none"

    @property
    def is_abi3(self) -> bool:
        return self.abi == "abi3"

    @property
    def cp_version(self) -> int | None:
        m = re.fullmatch(r"cp(\d+)", self.python)
        return int(m.group(1)) if m else None


def workspace_version() -> str:
    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'^\s*version\s*=\s*"([^"]+)"', cargo, re.M)
    if not m:
        raise SystemExit("Could not read workspace version from Cargo.toml")
    return m.group(1)


def run(cmd: list[str], **kwargs) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.check_call(cmd, cwd=ROOT, **kwargs)


def read_wheel_tag(path: Path) -> WheelTag:
    with zipfile.ZipFile(path) as zf:
        wheel_entries = [n for n in zf.namelist() if n.endswith(".dist-info/WHEEL")]
        if not wheel_entries:
            raise ValueError(f"{path.name}: missing WHEEL metadata")
        raw = zf.read(wheel_entries[0]).decode("utf-8")
        for line in raw.splitlines():
            if line.startswith("Tag: "):
                return WheelTag.parse(line.removeprefix("Tag: ").strip())
    raise ValueError(f"{path.name}: Tag line missing")


def restrictive_tag(tags: list[WheelTag]) -> WheelTag:
    """Intersection of compatibility: never inherit the least restrictive tag."""
    if not tags:
        raise ValueError("no wheel tags to merge")

    platforms = {t.platform for t in tags}
    if len(platforms) != 1:
        raise ValueError(f"cannot merge wheels with different platforms: {platforms}")

    specific_cp: int | None = None
    abi3_min: int | None = None
    any_py3_none = False

    for tag in tags:
        if tag.is_py3_none:
            any_py3_none = True
            continue
        if tag.is_abi3:
            cp = tag.cp_version
            if cp is None:
                raise ValueError(f"abi3 tag without cp prefix: {tag.serialize()}")
            abi3_min = cp if abi3_min is None else max(abi3_min, cp)
            continue
        cp = tag.cp_version
        if cp is not None and tag.abi == f"cp{cp}":
            specific_cp = cp if specific_cp is None else max(specific_cp, cp)
            continue
        raise ValueError(f"unsupported wheel tag: {tag.serialize()}")

    if specific_cp is not None:
        return WheelTag(f"cp{specific_cp}", f"cp{specific_cp}", tags[0].platform)
    if abi3_min is not None:
        return WheelTag(f"cp{abi3_min}", "abi3", tags[0].platform)
    if any_py3_none:
        return WheelTag("py3", "none", tags[0].platform)
    raise ValueError("could not derive merged wheel tag")


def write_wheel_tag(dist_info: Path, tag: WheelTag) -> None:
    wheel_file = dist_info / "WHEEL"
    lines = wheel_file.read_text(encoding="utf-8").splitlines()
    out: list[str] = []
    replaced = False
    for line in lines:
        if line.startswith("Tag: "):
            out.append(f"Tag: {tag.serialize()}")
            replaced = True
        else:
            out.append(line)
    if not replaced:
        out.append(f"Tag: {tag.serialize()}")
    wheel_file.write_text("\n".join(out) + "\n", encoding="utf-8")


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

    input_tags = [read_wheel_tag(base)] + [read_wheel_tag(extra) for extra in extras]
    merged_tag = restrictive_tag(input_tags)
    print(f"Merged wheel tag: {merged_tag.serialize()} (from {[t.serialize() for t in input_tags]})")

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

        dist_info_dirs = list(merged.glob("*.dist-info"))
        if not dist_info_dirs:
            raise SystemExit("merged wheel missing .dist-info")
        write_wheel_tag(dist_info_dirs[0], merged_tag)

        out_dir.mkdir(parents=True, exist_ok=True)
        run([sys.executable, "-m", "wheel", "pack", str(merged), "-d", str(out_dir)])
        packed = sorted(out_dir.glob("quantwave-*.whl"))
        if not packed:
            raise SystemExit(f"wheel pack did not produce quantwave-*.whl in {out_dir}")
        final = out_dir / f"quantwave-{workspace_version()}-{merged_tag.serialize()}.whl"
        if packed[-1] != final:
            if final.exists():
                final.unlink()
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