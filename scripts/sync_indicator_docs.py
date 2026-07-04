#!/usr/bin/env python3
"""Sync indicator navigation, catalog, and slug redirect stubs from metadata_export.json."""

from __future__ import annotations

import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from docs.upgrade_to_standards import SLUG_ALIASES, SKIP_FILES, slugify  # noqa: E402

NATIVE = ROOT / "docs" / "guides" / "indicators" / "native"
INDICATORS = ROOT / "docs" / "guides" / "indicators"
SUMMARY = INDICATORS / "SUMMARY.md"
CATALOG = NATIVE / "index.md"

CATEGORY_ORDER = [
    "Price Action",
    "Price Action / Patterns",
    "Classic",
    "Ehlers DSP",
    "Patterns",
    "Moving Averages",
    "Volatility",
    "Volume",
    "Volume Indicators",
    "Momentum",
    "Modern",
    "ML Features",
    "Rocket Science",
    "Statistics",
    "Wilder",
    "Regime",
]


def load_metadata() -> list[dict]:
    return json.load((ROOT / "metadata_export.json").open(encoding="utf-8"))


def canonical_stems() -> set[str]:
    """Doc page stems (hand-crafted PA pages included; redirect stubs excluded)."""
    stems: set[str] = set()
    for path in NATIVE.glob("*.md"):
        if path.name == "index.md":
            continue
        text = path.read_text(encoding="utf-8")
        if text.startswith("<!-- redirect-stub:"):
            continue
        stems.add(path.stem)
    return stems


def resolve_stem(slug: str, name: str, stems: set[str]) -> str | None:
    # Prefer hand-crafted pages registered at the metadata slug (e.g. market_structure.md).
    if slug in stems:
        return slug
    candidates = [slugify(name)]
    for stem, aliased in SLUG_ALIASES.items():
        if aliased == slug:
            candidates.append(stem)
    for candidate in candidates:
        if candidate in stems:
            return candidate
    return None


def write_redirect_stub(slug: str, stem: str, name: str) -> None:
    if slug == stem or f"{slug}.md" in SKIP_FILES:
        return
    path = NATIVE / f"{slug}.md"
    if path.exists():
        text = path.read_text(encoding="utf-8")
        if text.startswith("<!-- redirect-stub:"):
            if f"→{stem}" in text.splitlines()[0]:
                return
    content = (
        f"<!-- redirect-stub:{slug}→{stem} -->\n"
        f'<meta http-equiv="refresh" content="0; url={stem}/">\n\n'
        f"Redirecting to [{name}]({stem}.md)…\n"
    )
    path.write_text(content, encoding="utf-8")


def remove_stale_redirects(valid_slugs: set[str]) -> int:
    """Drop redirect stubs whose slug is no longer registered."""
    removed = 0
    for path in NATIVE.glob("*.md"):
        if path.name == "index.md":
            continue
        text = path.read_text(encoding="utf-8")
        if not text.startswith("<!-- redirect-stub:"):
            continue
        slug = text.splitlines()[0].removeprefix("<!-- redirect-stub:").split("→", 1)[0]
        if slug not in valid_slugs:
            path.unlink()
            removed += 1
    return removed


def build_summary(by_category: dict[str, list[tuple[str, str, str]]]) -> str:
    lines = [
        "# Indicators",
        "",
        "- [Overview](index.md)",
        "- [Indicator Gallery](gallery.md)",
        "- [Native Indicators](native/index.md)",
    ]
    for category in CATEGORY_ORDER:
        items = by_category.get(category)
        if not items:
            continue
        lines.append(f"    - {category}")
        for name, stem, _slug in sorted(items, key=lambda x: x[0].lower()):
            lines.append(f"        - [{name}](native/{stem}.md)")
    for category in sorted(by_category.keys()):
        if category in CATEGORY_ORDER:
            continue
        items = by_category[category]
        lines.append(f"    - {category or 'General'}")
        for name, stem, _slug in sorted(items, key=lambda x: x[0].lower()):
            lines.append(f"        - [{name}](native/{stem}.md)")
    lines.append("- [Regime Detection](regimes/index.md)")
    lines.append("- [Ehlers DSP Guide](ehlers/index.md)")
    return "\n".join(lines) + "\n"


def build_catalog(by_category: dict[str, list[tuple[str, str, str]]], total: int) -> str:
    lines = [
        "# Native Indicators",
        "",
        f"QuantWave ships **{total} production-grade native indicators** in Rust with "
        "bit-identical batch (Polars `.ta()`) and streaming (`Next<T>`) parity.",
        "",
        "Every page follows [Documentation Standards](../../DOCUMENTATION_STANDARDS.md).",
        "",
        "## Quick links",
        "",
        "- [Indicator Gallery](../gallery.md) — curated starting points",
        "- [Ehlers DSP Suite](../ehlers/index.md)",
        "- [Regime Detection](../regimes/index.md)",
        "- [ML Features](../../ml_features.md)",
        "- [Price Action notebook](../../examples/notebooks/pa_flag_breakout_strategy.md)",
        "",
        "## Complete indicator catalog",
        "",
        f"**{total} indicators** across {len(by_category)} categories. "
        "Click any name for formulas, parameters, usage examples, edge cases, and sources.",
        "",
    ]
    for category in CATEGORY_ORDER:
        items = by_category.get(category)
        if not items:
            continue
        lines.append(f"### {category} ({len(items)})")
        lines.append("")
        lines.append("| Indicator | Slug |")
        lines.append("|-----------|------|")
        for name, stem, slug in sorted(items, key=lambda x: x[0].lower()):
            lines.append(f"| [{name}]({stem}.md) | `{slug}` |")
        lines.append("")
    for category in sorted(by_category.keys()):
        if category in CATEGORY_ORDER:
            continue
        items = by_category[category]
        label = category or "General"
        lines.append(f"### {label} ({len(items)})")
        lines.append("")
        lines.append("| Indicator | Slug |")
        lines.append("|-----------|------|")
        for name, stem, slug in sorted(items, key=lambda x: x[0].lower()):
            lines.append(f"| [{name}]({stem}.md) | `{slug}` |")
        lines.append("")
    return "\n".join(lines)


def main() -> int:
    metadata = load_metadata()
    stems = canonical_stems()
    by_category: dict[str, list[tuple[str, str, str]]] = defaultdict(list)
    redirects = 0
    unresolved: list[str] = []

    for item in metadata:
        slug = item["slug"]
        name = item["name"]
        category = item.get("category") or "General"
        stem = resolve_stem(slug, name, stems)
        if not stem:
            unresolved.append(slug)
            continue
        by_category[category].append((name, stem, slug))
        if stem != slug:
            write_redirect_stub(slug, stem, name)
            redirects += 1

    if unresolved:
        print("Unresolved indicators (no doc page):", unresolved, file=sys.stderr)
        return 1

    valid_slugs = {item["slug"] for item in metadata}
    removed = remove_stale_redirects(valid_slugs)
    SUMMARY.write_text(build_summary(by_category), encoding="utf-8")
    CATALOG.write_text(build_catalog(by_category, len(metadata)), encoding="utf-8")

    print(
        f"sync_indicator_docs: {len(metadata)} indicators, "
        f"{redirects} redirect stubs, {removed} stale stubs removed"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())