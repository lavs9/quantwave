#!/usr/bin/env python3
"""Verify wheel tags match their native extension modules (release gate).

Rules:
- py3-none-* wheels must not contain CPython-version-specific extensions.
- abi3-tagged wheels must contain only stable-ABI extensions (no cpython-XY in names).
- cpXY-cpXY wheels must contain extensions for that exact interpreter version.
"""

from __future__ import annotations

import argparse
import re
import sys
import zipfile
from dataclasses import dataclass
from pathlib import Path

CPYTHON_EXT_RE = re.compile(r"\.cpython-(\d+)(?:-|\.)", re.IGNORECASE)
ABI3_EXT_RE = re.compile(r"\.abi3\.(?:so|pyd)$", re.IGNORECASE)
NATIVE_EXT_RE = re.compile(r"\.(?:so|pyd|dylib)$", re.IGNORECASE)
# UniFFI/maturin core cdylib (py3-none tag) — not PyO3 abi3-suffixed, but stable across 3.9+.
UNIFFI_CORE_LIB_RE = re.compile(
    r"^libquantwave_python\.(?:so|dylib)$",
    re.IGNORECASE,
)


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

    @property
    def is_py3_none(self) -> bool:
        return self.python == "py3" and self.abi == "none"

    @property
    def is_abi3(self) -> bool:
        return self.abi == "abi3"

    @property
    def cp_version(self) -> int | None:
        m = re.fullmatch(r"cp(\d+)", self.python)
        if not m:
            return None
        return int(m.group(1))


def read_wheel_tag(zf: zipfile.ZipFile) -> str:
    wheel_entries = [n for n in zf.namelist() if n.endswith(".dist-info/WHEEL")]
    if not wheel_entries:
        raise ValueError("missing .dist-info/WHEEL")
    raw = zf.read(wheel_entries[0]).decode("utf-8")
    for line in raw.splitlines():
        if line.startswith("Tag: "):
            return line.removeprefix("Tag: ").strip()
    raise ValueError("Tag: line missing from WHEEL metadata")


def native_members(zf: zipfile.ZipFile) -> list[str]:
    return [n for n in zf.namelist() if NATIVE_EXT_RE.search(n)]


def verify_wheel(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        with zipfile.ZipFile(path) as zf:
            tag = WheelTag.parse(read_wheel_tag(zf))
            natives = native_members(zf)
    except (zipfile.BadZipFile, ValueError, KeyError) as exc:
        return [f"{path.name}: cannot read wheel ({exc})"]

    if not natives and not tag.is_py3_none:
        errors.append(f"{path.name}: platform wheel has no native extensions")

    for member in natives:
        base = member.rsplit("/", 1)[-1]
        cpython = CPYTHON_EXT_RE.search(base)
        abi3 = bool(ABI3_EXT_RE.search(base))

        if tag.is_py3_none:
            if cpython:
                errors.append(
                    f"{path.name}: py3-none tag but contains version-specific "
                    f"extension {base!r}"
                )
            continue

        if tag.is_abi3:
            if cpython:
                errors.append(
                    f"{path.name}: abi3 tag but contains cpython-specific "
                    f"extension {base!r}"
                )
            elif not abi3 and NATIVE_EXT_RE.search(base):
                # Windows abi3 wheels may use .pyd without abi3 in the name when
                # built correctly; allow .pyd only when no cpython marker present.
                # UniFFI core (libquantwave_python.{so,dylib}) uses maturin py3-none
                # cdylib naming, not PyO3's *.abi3.so suffix.
                if not (
                    base.lower().endswith(".pyd")
                    or UNIFFI_CORE_LIB_RE.fullmatch(base)
                ):
                    errors.append(
                        f"{path.name}: abi3 tag but extension {base!r} is not "
                        "marked abi3"
                    )
            continue

        cp = tag.cp_version
        if cp is not None and tag.abi == f"cp{cp}":
            if not cpython:
                errors.append(
                    f"{path.name}: cp{cp}-cp{cp} tag but extension {base!r} "
                    "is not version-specific"
                )
            elif int(cpython.group(1)) != cp:
                errors.append(
                    f"{path.name}: wheel requires cp{cp} but extension "
                    f"{base!r} targets cp{cpython.group(1)}"
                )

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "wheels",
        nargs="*",
        type=Path,
        help="Wheel files or directories to scan (default: dist/)",
    )
    args = parser.parse_args()

    paths: list[Path] = []
    if args.wheels:
        for item in args.wheels:
            if item.is_dir():
                paths.extend(sorted(item.glob("*.whl")))
            else:
                paths.append(item)
    else:
        dist = Path(__file__).resolve().parent.parent / "dist"
        paths = sorted(dist.glob("*.whl"))

    if not paths:
        print("verify_wheel_tags: no wheels found", file=sys.stderr)
        return 1

    all_errors: list[str] = []
    for path in paths:
        all_errors.extend(verify_wheel(path))

    if all_errors:
        print("Wheel tag verification FAILED:", file=sys.stderr)
        for err in all_errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print(f"verify_wheel_tags: OK ({len(paths)} wheel(s))")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())