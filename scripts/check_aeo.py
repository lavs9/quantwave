#!/usr/bin/env python3
"""AEO governance checks — llms.txt URL hygiene and required AI-discovery pages."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs"
LLMS = DOCS / "llms.txt"
ROBOTS = DOCS / "robots.txt"
MKDOCS = ROOT / "mkdocs.yml"
SITE_PREFIX = "https://lavs9.github.io/quantwave"

# Pages that must be cited in llms.txt for AI discovery.
REQUIRED_LLMS_PATHS = {
    "/getting-started/",
    "/getting-started/python/",
    "/comparison/",
    "/faq/",
    "/guides/backtest/quickstart/",
    "/api/",
    "/guides/rust/",
}

# MkDocs sources that must exist when cited from llms.txt (path → docs file).
PATH_TO_SOURCE: dict[str, Path] = {
    "/": DOCS / "index.md",
    "/getting-started/": DOCS / "getting-started" / "index.md",
    "/getting-started/python/": DOCS / "getting-started" / "python.md",
    "/getting-started/rust/": DOCS / "getting-started" / "rust.md",
    "/comparison/": DOCS / "comparison.md",
    "/faq/": DOCS / "faq.md",
    "/guides/indicators/": DOCS / "guides" / "indicators" / "index.md",
    "/guides/indicators/native/": DOCS / "guides" / "indicators" / "native" / "index.md",
    "/guides/indicators/gallery/": DOCS / "guides" / "indicators" / "gallery.md",
    "/guides/indicators/native/supertrend/": DOCS / "guides" / "indicators" / "native" / "supertrend.md",
    "/guides/backtest/quickstart/": DOCS / "guides" / "backtest" / "quickstart.md",
    "/guides/plugin_vs_ta/": DOCS / "guides" / "plugin_vs_ta.md",
    "/guides/rust/": DOCS / "guides" / "rust" / "index.md",
    "/guides/rust/crate-map/": DOCS / "guides" / "rust" / "crate-map.md",
    "/guides/rust/next-trait/": DOCS / "guides" / "rust" / "next-trait.md",
    "/guides/rust/backtest/": DOCS / "guides" / "rust" / "backtest.md",
    "/benchmarks/": DOCS / "benchmarks.md",
}

# Generated at build time — existence not required in repo.
GENERATED_PREFIXES = ("/api/",)

URL_RE = re.compile(r"https://lavs9\.github\.io/quantwave[^\s\)>\"]*")


def normalize_path(url: str) -> str:
    parsed = urlparse(url)
    path = parsed.path or "/"
    # GitHub Pages project site: paths are /quantwave/... on the host.
    prefix = "/quantwave"
    if path.startswith(prefix):
        path = path[len(prefix) :] or "/"
    if not path.endswith("/") and "." not in Path(path).name:
        path = f"{path}/"
    return path


def url_to_doc_path(url_path: str) -> Path | None:
    if any(url_path.startswith(p) for p in GENERATED_PREFIXES):
        return None
    if url_path in PATH_TO_SOURCE:
        return PATH_TO_SOURCE[url_path]
    # Fallback: /foo/bar/ → docs/foo/bar.md or docs/foo/bar/index.md
    rel = url_path.strip("/")
    if not rel:
        return DOCS / "index.md"
    direct = DOCS / f"{rel}.md"
    if direct.exists():
        return direct
    index = DOCS / rel / "index.md"
    if index.exists():
        return index
    return None


def extract_llms_urls(text: str) -> list[str]:
    return URL_RE.findall(text)


def check_mkdocs_rust_nav() -> list[str]:
    failures: list[str] = []
    if not MKDOCS.exists():
        return ["missing mkdocs.yml"]
    text = MKDOCS.read_text(encoding="utf-8")
    for fragment in ("guides/rust/index.md", "guides/rust/crate-map.md"):
        if fragment not in text:
            failures.append(f"mkdocs.yml nav missing {fragment}")
    return failures


def main() -> None:
    failures: list[str] = []

    if not LLMS.exists():
        failures.append("missing docs/llms.txt")
    else:
        llms_text = LLMS.read_text(encoding="utf-8")
        urls = extract_llms_urls(llms_text)
        if not urls:
            failures.append("docs/llms.txt contains no canonical site URLs")

        paths_found = {normalize_path(u) for u in urls}
        for required in REQUIRED_LLMS_PATHS:
            if required not in paths_found:
                failures.append(f"docs/llms.txt missing required URL path: {required}")

        for url in urls:
            path = normalize_path(url)
            src = url_to_doc_path(path)
            if src is None:
                continue  # generated or unknown — skip
            if not src.exists():
                failures.append(f"llms.txt URL {url} → missing source {src.relative_to(ROOT)}")

    if not ROBOTS.exists():
        failures.append("missing docs/robots.txt")
    elif SITE_PREFIX not in ROBOTS.read_text(encoding="utf-8"):
        failures.append("docs/robots.txt should reference site URL")

    failures.extend(check_mkdocs_rust_nav())

    rust_index = DOCS / "guides" / "rust" / "index.md"
    if not rust_index.exists():
        failures.append("missing docs/guides/rust/index.md (Phase 4 mirror)")

    if failures:
        print("FAIL: AEO checks:", file=sys.stderr)
        for msg in failures:
            print(f"  - {msg}", file=sys.stderr)
        sys.exit(1)

    print("AEO check passed (llms.txt URLs, rust nav, robots.txt).")


if __name__ == "__main__":
    main()