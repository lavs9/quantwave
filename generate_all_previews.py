#!/usr/bin/env python3
"""Generate documentation preview PNGs for every native indicator page.

Produces indicator sparklines/charts in docs/assets/indicator-previews/ and
candlestick schematics in docs/assets/candlestick-previews/. Skips slugs that
already have hand-crafted PNGs unless --force is passed.

Run:
    python docs/generate_all_previews.py
    python docs/generate_all_previews.py --force
    python docs/generate_all_previews.py --sync-docs   # also embed in .md files
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR))

import upgrade_to_standards as _upgrade  # noqa: E402

from visual_utils import (  # noqa: E402
    CANDLE_OUT,
    INDICATOR_OUT,
    generate_candle_preview,
    generate_indicator_preview,
)

SKIP_SLUGS = {
    "market_structure",
    "geometric_patterns",
    "sr_monitor",
    "pa_events_strategies",
}

# Hand-crafted candle generators in gen_candle_previews.py — do not overwrite unless --force
HAND_CRAFTED_CANDLES = {
    "engulfing",
    "morning_star",
    "hammer",
    "doji",
    "gravestone_doji",
    "dragonfly_doji",
    "harami",
    "harami_cross",
    "three_black_crows",
    "three_white_soldiers",
    "abandoned_baby",
}

HAND_CRAFTED_INDICATORS = {
    "supertrend",
    "cyber_cycle",
    "rsi",
    "ehlers_filter",
    "reflex",
    "ehlers_stochastic",
    "ehlers_loops",
    "ultimatesmoother",
    "supersmoother",
    "roofing_filter",
    "fisher_transform",
    "instantaneous_trendline",
    "trendflex",
}


def resolve_slug(md_stem: str, metadata: dict) -> str | None:
    alias = _upgrade.SLUG_ALIASES.get(md_stem, md_stem)
    if alias in metadata:
        return alias
    return md_stem if md_stem in metadata else None


def generate_all(force: bool = False) -> tuple[int, int, int]:
    metadata = _upgrade.parse_metadata_files()
    md_files = sorted(_upgrade.NATIVE_DOCS.glob("*.md"))
    generated = skipped = errors = 0

    for md_path in md_files:
        if md_path.name in _upgrade.SKIP_FILES:
            continue
        stem = md_path.stem
        if stem in SKIP_SLUGS:
            continue
        meta_slug = resolve_slug(stem, metadata)
        if not meta_slug:
            print(f"  skip (no metadata): {stem}", file=sys.stderr)
            skipped += 1
            continue
        rec = metadata[meta_slug]
        # PNG filename matches the .md stem (site path), not always metadata slug.
        file_slug = stem

        if rec.is_pattern:
            out = CANDLE_OUT / f"{file_slug}.png"
            if out.exists() and file_slug in HAND_CRAFTED_CANDLES and not force:
                skipped += 1
                continue
            if out.exists() and not force:
                skipped += 1
                continue
            try:
                generate_candle_preview(file_slug, rec.name, rec.keywords, rec.struct_name)
                generated += 1
            except Exception as exc:
                print(f"  error candle {file_slug}: {exc}", file=sys.stderr)
                errors += 1
        else:
            out = INDICATOR_OUT / f"{file_slug}.png"
            if out.exists() and file_slug in HAND_CRAFTED_INDICATORS and not force:
                skipped += 1
                continue
            if out.exists() and not force:
                skipped += 1
                continue
            try:
                generate_indicator_preview(
                    file_slug,
                    rec.name,
                    rec.category,
                    rec.keywords,
                    is_pattern=False,
                    is_ehlers=rec.is_ehlers,
                    struct_name=rec.struct_name,
                )
                generated += 1
            except Exception as exc:
                print(f"  error indicator {file_slug}: {exc}", file=sys.stderr)
                errors += 1

    return generated, skipped, errors


def sync_doc_visuals() -> int:
    metadata = _upgrade.parse_metadata_files()
    updated = 0
    placeholder_re = re.compile(
        r"(## Visual Example\n\n)(.*?)(\n## Description)",
        re.S,
    )

    for md_path in sorted(_upgrade.NATIVE_DOCS.glob("*.md")):
        if md_path.name in _upgrade.SKIP_FILES:
            continue
        stem = md_path.stem
        meta_slug = resolve_slug(stem, metadata)
        if not meta_slug or meta_slug not in metadata:
            continue
        rec = metadata[meta_slug]
        content = md_path.read_text(encoding="utf-8")
        if "Visual placeholder" not in content and "> **Chart**:" not in content:
            continue
        asset, _ = _upgrade.rel_asset(stem, rec.is_pattern)
        if not asset:
            continue
        new_visual = _upgrade.render_visual(rec).rstrip() + "\n"
        new_content, n = placeholder_re.subn(
            lambda m: new_visual + "\n## Description",
            content,
            count=1,
        )
        if n:
            md_path.write_text(new_content, encoding="utf-8")
            updated += 1
            print(f"  synced visual: {md_path.name}")

    return updated


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--force", action="store_true", help="Regenerate existing PNGs")
    parser.add_argument("--sync-docs", action="store_true", help="Embed PNGs into .md files")
    parser.add_argument("--sync-only", action="store_true", help="Only sync docs, no generation")
    args = parser.parse_args()

    if not args.sync_only:
        print("Generating preview PNGs for all native indicator pages...")
        generated, skipped, errors = generate_all(force=args.force)
        print(f"Done: generated={generated}, skipped={skipped}, errors={errors}")
        if errors:
            return 1

    if args.sync_docs or args.sync_only:
        print("Syncing Visual Example sections in documentation...")
        updated = sync_doc_visuals()
        print(f"Synced {updated} pages")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())