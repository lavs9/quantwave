#!/usr/bin/env python3
"""Incremental cache for quantwave verify / pre-push hook.

Stores content fingerprints in .cache/verify/ so unchanged steps are skipped.
Disable with VERIFY_NO_CACHE=1.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CACHE_DIR = ROOT / ".cache" / "verify"
STATE_FILE = CACHE_DIR / "state.json"

STEP_PATHS: dict[str, list[str]] = {
    "metadata": [
        "scripts/check_metadata_drift.py",
        "scripts/check_doc_drift.py",
        "scripts/check_public_metadata.py",
        "scripts/check_aeo.py",
        "scripts/check_benchmark_claims.py",
        "benchmarks",
        "docs/guides/rust",
        "scripts/sync_indicator_docs.py",
        "scripts/regenerate_metadata_registry.py",
        "docs/upgrade_to_standards.py",
        "docs",
        "quantwave-core/src/metadata.rs",
        "quantwave-python/python/quantwave/metadata.py",
    ],
    "rust": [
        "quantwave-core",
        "quantwave-polars",
        "quantwave-backtest",
        "Cargo.toml",
        "Cargo.lock",
    ],
    "maturin-python": [
        "quantwave-python",
        "quantwave-core",
        "Cargo.toml",
        "Cargo.lock",
    ],
    "maturin-backtest": [
        "quantwave-backtest-py",
        "quantwave-backtest",
        "quantwave-python/src/backtest.rs",
        "Cargo.toml",
        "Cargo.lock",
    ],
    "maturin-plugins": [
        "quantwave-plugins",
        "quantwave-core",
        "Cargo.toml",
        "Cargo.lock",
    ],
    "wheel": [
        "quantwave-python",
        "quantwave-backtest-py",
        "quantwave-plugins",
        "quantwave-core",
        "quantwave-backtest",
        "scripts/build_unified_wheel.py",
        "scripts/pypi_smoke_test.py",
        "Cargo.toml",
        "Cargo.lock",
    ],
    "pytest": [
        "quantwave-python/tests",
        "quantwave-python/python",
        "quantwave-backtest-py",
        "quantwave-plugins",
    ],
}

IMPORT_CHECKS: dict[str, str] = {
    "maturin-python": "import quantwave",
    "maturin-backtest": "import quantwave._backtest",
    "maturin-plugins": "import quantwave_plugins",
}

PIP_DEPS_KEY = "pip-deps"
PIP_DEPS_SPEC = "pytest polars maturin wheel"


def cache_enabled() -> bool:
    return os.environ.get("VERIFY_NO_CACHE", "").strip() not in ("1", "true", "yes")


def load_state() -> dict[str, str]:
    if not STATE_FILE.exists():
        return {}
    try:
        return json.loads(STATE_FILE.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {}


def save_state(state: dict[str, str]) -> None:
    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    STATE_FILE.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def git_fingerprint(paths: list[str]) -> str:
    listed = subprocess.run(
        ["git", "ls-files", *paths],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    files = sorted({line.strip() for line in listed.stdout.splitlines() if line.strip()})
    digest = hashlib.sha256()
    for rel in files:
        path = ROOT / rel
        if not path.is_file():
            continue
        digest.update(rel.encode())
        digest.update(path.read_bytes())
    return digest.hexdigest()


def text_fingerprint(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()


def fingerprint(step: str) -> str:
    if step == PIP_DEPS_KEY:
        return text_fingerprint(PIP_DEPS_SPEC)
    if step not in STEP_PATHS:
        raise SystemExit(f"unknown cache step: {step}")
    return git_fingerprint(STEP_PATHS[step])


def import_ok(step: str) -> bool:
    probe = IMPORT_CHECKS.get(step)
    if not probe:
        return True
    try:
        subprocess.run(
            [sys.executable, "-c", probe],
            cwd=ROOT,
            check=True,
            capture_output=True,
        )
        return True
    except subprocess.CalledProcessError:
        return False


def is_fresh(step: str, state: dict[str, str] | None = None) -> bool:
    if not cache_enabled():
        return False
    state = state if state is not None else load_state()
    current = fingerprint(step)
    if state.get(step) != current:
        return False
    if step in IMPORT_CHECKS and not import_ok(step):
        return False
    return True


def mark(step: str) -> None:
    state = load_state()
    state[step] = fingerprint(step)
    save_state(state)


def cmd_fresh(args: argparse.Namespace) -> int:
    print("fresh" if is_fresh(args.step) else "stale")
    return 0 if is_fresh(args.step) else 1


def cmd_mark(args: argparse.Namespace) -> int:
    mark(args.step)
    print(f"cached: {args.step}")
    return 0


def cmd_ensure(args: argparse.Namespace) -> int:
    if is_fresh(args.step):
        print(f"-- {args.step} (cached, skipping)")
        return 0
    if not args.cmd:
        print(f"-- {args.step} (stale, no command provided)", file=sys.stderr)
        return 2
    print(f"-- {args.step}")
    subprocess.run(args.cmd, cwd=ROOT, check=True)
    mark(args.step)
    return 0


def cmd_status(_: argparse.Namespace) -> int:
    state = load_state()
    print(f"cache: {STATE_FILE}")
    print(f"enabled: {cache_enabled()}")
    for step in [
        PIP_DEPS_KEY,
        "metadata",
        "rust",
        "maturin-python",
        "maturin-backtest",
        "maturin-plugins",
        "wheel",
        "pytest",
    ]:
        current = fingerprint(step)
        stored = state.get(step)
        if is_fresh(step, state):
            print(f"  {step}: hit")
        elif stored:
            print(f"  {step}: stale ({stored[:8]} -> {current[:8]})")
        else:
            print(f"  {step}: miss")
    return 0


def cmd_clear(_: argparse.Namespace) -> int:
    if STATE_FILE.exists():
        STATE_FILE.unlink()
    print("verify cache cleared")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    fresh = sub.add_parser("fresh", help="print fresh|stale and exit 0/1")
    fresh.add_argument("step")
    fresh.set_defaults(func=cmd_fresh)

    mark_p = sub.add_parser("mark", help="record step as successfully completed")
    mark_p.add_argument("step")
    mark_p.set_defaults(func=cmd_mark)

    ensure = sub.add_parser("ensure", help="run command only when step is stale")
    ensure.add_argument("step")
    ensure.add_argument("cmd", nargs=argparse.REMAINDER, help="command after --")
    ensure.set_defaults(func=cmd_ensure)

    status = sub.add_parser("status", help="show cache hit/miss per step")
    status.set_defaults(func=cmd_status)

    clear = sub.add_parser("clear", help="drop all cached fingerprints")
    clear.set_defaults(func=cmd_clear)

    args = parser.parse_args()
    if args.command == "ensure" and args.cmd and args.cmd[0] == "--":
        args.cmd = args.cmd[1:]
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())