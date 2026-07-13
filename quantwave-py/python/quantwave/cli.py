"""QuantWave CLI — indicator discovery and install diagnostics.

Installed as the ``quantwave`` console script (``pip install quantwave``).

Commands:

* ``quantwave list`` — indicators by category
* ``quantwave info <slug>`` — metadata, warmup, doc URL
* ``quantwave doctor`` — verify core, Polars, plugins, and backtest extensions
* ``quantwave version`` — package version
"""

from __future__ import annotations

import argparse
import json
import sys
from typing import Any


def _cmd_list(args: argparse.Namespace) -> int:
    import quantwave as qw

    if args.category:
        slugs = qw.category(args.category)
        if not slugs:
            print(f"No indicators in category {args.category!r}", file=sys.stderr)
            return 1
        for slug in slugs:
            print(slug)
        return 0

    by_cat = qw.indicators_by_category()
    if args.json:
        print(json.dumps(by_cat, indent=2))
        return 0
    for cat in sorted(by_cat):
        print(f"\n{cat} ({len(by_cat[cat])})")
        for slug in by_cat[cat]:
            print(f"  {slug}")
    return 0


def _cmd_info(args: argparse.Namespace) -> int:
    import quantwave as qw

    slug = args.indicator
    if not qw.is_indicator(slug):
        print(f"Unknown indicator: {slug}", file=sys.stderr)
        return 1
    meta = qw.metadata(slug)
    boundary = qw.boundary_info(slug)
    sig = qw.get_indicator_signature(slug)

    if args.json:
        payload: dict[str, Any] = {
            "slug": meta.name,
            "display_name": meta.display_name,
            "category": meta.category,
            "description": meta.description,
            "usage": meta.usage,
            "params": [p.__dict__ if hasattr(p, "__dict__") else p for p in meta.params],
            "data_inputs": meta.data_inputs,
            "outputs": meta.outputs,
            "warmup_bars": meta.warmup_bars,
            "boundary": boundary.__dict__ if boundary else None,
            "signature": sig,
            "docs": f"https://lavs9.github.io/quantwave/guides/indicators/native/{slug}/",
        }
        print(json.dumps(payload, indent=2))
        return 0

    print(f"{meta.display_name} (`{meta.name}`)")
    print(f"Category: {meta.category}")
    print(meta.description)
    if meta.usage:
        print(f"\nUsage: {meta.usage}")
    if meta.params:
        print("\nParameters:")
        for p in meta.params:
            print(f"  - {p.name} (default={p.default}): {p.description}")
    if meta.warmup_bars is not None:
        print(f"\nWarmup bars: {meta.warmup_bars}")
    if boundary:
        print(f"Boundary: {boundary.warmup_behavior}")
    if sig:
        print(f"\nSignature hint: {sig}")
    print(f"\nDocs: https://lavs9.github.io/quantwave/guides/indicators/native/{slug}/")
    return 0


def _cmd_doctor(_args: argparse.Namespace) -> int:
    import quantwave as qw

    ok = True
    print(f"quantwave {qw.__version__}")

    def check(label: str, fn) -> None:
        nonlocal ok
        try:
            fn()
            print(f"  ✓ {label}")
        except Exception as exc:
            ok = False
            print(f"  ✗ {label}: {exc}")

    check("core extension (_quantwave)", lambda: __import__("quantwave._quantwave"))
    check("metadata registry", lambda: qw.metadata("rsi"))
    check("streaming (RSI)", lambda: qw.streaming_class("rsi")(14))

    try:
        import polars as pl  # noqa: F401
        print("  ✓ polars installed")
    except ImportError:
        print("  ✗ polars not installed — run: pip install \"quantwave[polars]\"")
        ok = False
        pl = None

    if pl is not None:
        check("backtest native (_backtest)", lambda: __import__("quantwave._backtest"))
        check("Polars .bt namespace", lambda: getattr(pl.LazyFrame({"a": [1]}).lazy(), "bt"))
        try:
            from quantwave import _ta_namespace  # noqa: F401
            print("  ✓ Polars expression plugins (pl.col().ta)")
        except ImportError:
            print("  ✗ .ta namespace missing — reinstall quantwave wheel (unified build)")
            ok = False

    if ok:
        print("\nAll checks passed.")
        return 0
    print("\nSome checks failed. See https://lavs9.github.io/quantwave/getting-started/python/")
    return 1


def _cmd_version(_args: argparse.Namespace) -> int:
    import quantwave as qw

    print(qw.__version__)
    return 0


def main(argv: list[str] | None = None) -> int:
    """CLI entry point used by the ``quantwave`` console script.

    Args:
        argv: Optional argument list (defaults to ``sys.argv[1:]``).

    Returns:
        Process exit code (0 success, non-zero on user or environment errors).
    """
    parser = argparse.ArgumentParser(
        prog="quantwave",
        description="QuantWave — indicator discovery and environment diagnostics",
    )
    sub = parser.add_subparsers(dest="command", required=True)

    p_list = sub.add_parser("list", help="List indicators (optionally by category)")
    p_list.add_argument("--category", "-c", help="Filter by category name")
    p_list.add_argument("--json", action="store_true", help="Emit JSON")
    p_list.set_defaults(func=_cmd_list)

    p_info = sub.add_parser("info", help="Show metadata for one indicator")
    p_info.add_argument("indicator", help="Indicator slug (e.g. rsi, sdo, supertrend)")
    p_info.add_argument("--json", action="store_true", help="Emit JSON")
    p_info.set_defaults(func=_cmd_info)

    p_doctor = sub.add_parser("doctor", help="Verify install: core, polars, plugins, backtest")
    p_doctor.set_defaults(func=_cmd_doctor)

    p_ver = sub.add_parser("version", help="Print package version")
    p_ver.set_defaults(func=_cmd_version)

    args = parser.parse_args(argv)
    return int(args.func(args))


if __name__ == "__main__":
    raise SystemExit(main())