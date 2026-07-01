#!/usr/bin/env python3
"""Verify native doc pages and slug redirect stubs match the metadata registry."""

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from docs.upgrade_to_standards import SKIP_FILES, SLUG_ALIASES, slugify  # noqa: E402

NATIVE_DOCS = ROOT / "docs" / "guides" / "indicators" / "native"
METADATA_FILE = ROOT / "metadata_export.json"
DOCS_ROOT = ROOT / "docs"

# Hub pages whose internal links must resolve (no broken featured paths).
HUB_PAGES = [
    DOCS_ROOT / "index.md",
    DOCS_ROOT / "guides" / "indicators" / "gallery.md",
    DOCS_ROOT / "guides" / "indicators" / "index.md",
    DOCS_ROOT / "guides" / "backtest" / "index.md",
    DOCS_ROOT / "guides" / "backtest" / "quickstart.md",
    DOCS_ROOT / "guides" / "backtest" / "capability_matrix.md",
]

LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def is_redirect_stub(text: str) -> bool:
    return text.startswith("<!-- redirect-stub:")


def canonical_stems() -> set[str]:
    stems: set[str] = set()
    for path in NATIVE_DOCS.glob("*.md"):
        if path.name == "index.md":
            continue
        text = path.read_text(encoding="utf-8")
        if is_redirect_stub(text):
            continue
        stems.add(path.stem)
    return stems


def resolve_stem(slug: str, name: str, stems: set[str]) -> str | None:
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


def check_backtest_docs_exist() -> bool:
    mkdocs_file = ROOT / "mkdocs.yml"
    if not mkdocs_file.exists():
        return True
    
    text = mkdocs_file.read_text(encoding="utf-8")
    paths = re.findall(r':\s+([\w\-\/\.]+\.md)', text)
    
    failed = False
    for path_str in paths:
        path_str = path_str.strip()
        if path_str.startswith("guides/backtest/") or path_str.startswith("examples/notebooks/"):
            target = ROOT / "docs" / path_str
            if not target.exists():
                print(f"FAIL: mkdocs.yml references missing file: {path_str}", file=sys.stderr)
                failed = True
                
    return not failed


def resolve_doc_link(source: Path, href: str) -> Path | None:
    """Resolve a markdown href from *source* to a docs-root-relative path."""
    href = href.strip()
    if not href or href.startswith(("http://", "https://", "mailto:", "#")):
        return None
    if href.startswith("/"):
        return DOCS_ROOT / href.lstrip("/")
    return (source.parent / href).resolve()


def link_target_exists(path: Path) -> bool:
    if path.suffix == ".md":
        return path.exists()
    # MkDocs directory URLs: foo/ → foo.md or foo/index.md
    md = path.with_suffix(".md")
    if md.exists():
        return True
    return (path / "index.md").exists()


def check_hub_links() -> list[str]:
    """Return human-readable failures for broken links on hub pages."""
    failures: list[str] = []
    for hub in HUB_PAGES:
        if not hub.exists():
            failures.append(f"missing hub page: {hub.relative_to(ROOT)}")
            continue
        text = hub.read_text(encoding="utf-8")
        for href in LINK_RE.findall(text):
            target = resolve_doc_link(hub, href)
            if target is None:
                continue
            try:
                target.relative_to(DOCS_ROOT)
            except ValueError:
                continue
            if not link_target_exists(target):
                failures.append(
                    f"{hub.relative_to(ROOT)}: broken link `{href}`"
                )
            # Featured native paths should not use .md suffix (MkDocs directory URLs).
            if "native/" in href and href.endswith(".md"):
                failures.append(
                    f"{hub.relative_to(ROOT)}: use directory URL for `{href}` (drop .md)"
                )
    return failures


def export_metadata_json() -> None:
    result = subprocess.run(
        ["cargo", "run", "-p", "quantwave-core", "--bin", "export_metadata"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    METADATA_FILE.write_text(result.stdout, encoding="utf-8")


def main() -> None:
    export_metadata_json()

    with METADATA_FILE.open(encoding="utf-8") as f:
        data = json.load(f)

    stems = canonical_stems()
    missing_canonical: list[str] = []
    missing_redirects: list[str] = []

    for item in data:
        slug = item["slug"]
        name = item["name"]
        stem = resolve_stem(slug, name, stems)
        if not stem:
            missing_canonical.append(slug)
            continue
        if stem != slug:
            redirect = NATIVE_DOCS / f"{slug}.md"
            if not redirect.exists():
                missing_redirects.append(slug)
            else:
                text = redirect.read_text(encoding="utf-8")
                if not is_redirect_stub(text) or f"→{stem}" not in text.splitlines()[0]:
                    missing_redirects.append(slug)

    # Orphans: canonical pages with no metadata (excluding hand-crafted PA guides)
    metadata_stems: set[str] = set()
    for item in data:
        stem = resolve_stem(item["slug"], item["name"], stems)
        if stem:
            metadata_stems.add(stem)
    orphans = sorted(
        s
        for s in stems
        if s not in metadata_stems
        and f"{s}.md" not in SKIP_FILES
        and s not in SLUG_ALIASES
    )

    failed = False
    if missing_canonical:
        print("FAIL: Missing canonical documentation pages:", file=sys.stderr)
        for slug in sorted(missing_canonical):
            print(f"  - {slug}", file=sys.stderr)
        failed = True

    if missing_redirects:
        print("FAIL: Missing slug redirect stubs (run sync_indicator_docs.py):", file=sys.stderr)
        for slug in sorted(missing_redirects):
            print(f"  - {slug}", file=sys.stderr)
        failed = True

    if orphans:
        print("FAIL: Orphan documentation pages (no metadata entry):", file=sys.stderr)
        for stem in orphans:
            print(f"  - {stem}.md", file=sys.stderr)
        failed = True

    if not check_backtest_docs_exist():
        failed = True

    hub_failures = check_hub_links()
    if hub_failures:
        print("FAIL: Broken or non-canonical hub page links:", file=sys.stderr)
        for msg in hub_failures:
            print(f"  - {msg}", file=sys.stderr)
        failed = True

    if failed:
        sys.exit(1)

    print(
        f"Doc drift check passed: {len(data)} indicators, "
        f"{len(stems)} canonical pages, slug redirects verified."
    )


if __name__ == "__main__":
    main()