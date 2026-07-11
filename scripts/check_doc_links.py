#!/usr/bin/env python3
"""Fail on broken relative links across all docs pages (quantwave-5ipk.5).

A fast, build-free gate that mirrors how MkDocs resolves links, so it agrees with
`mkdocs build` without the brittleness of `--strict` (which also promotes benign
griffe/autorefs/nav warnings to errors):

* ``.md`` links and relative assets (``.png``, ``.txt`` …) → file-path resolution
  (relative to the source file's directory), the way MkDocs rewrites them.
* bare directory links (``../native/foo/``) → directory-URL resolution (each page
  is served as ``/dir/``), the way the browser resolves the left-as-is link.

Absolute URLs, in-page anchors, and ``api/`` targets (mkdocstrings-generated at
build time) are skipped.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"

# Meta docs that intentionally contain illustrative "bad example" links inside
# heavily-inlined code fences (which defeat simple fence tracking). Their links
# are templates, not site navigation.
SKIP_FILES = {DOCS / "DOCUMENTATION_STANDARDS.md", DOCS / "DOCUMENTATION_DECISIONS.md"}

# [text](target) / ![alt](target) and raw/inline HTML href/src.
_MD_LINK = re.compile(r"!?\]\(\s*(<[^>]+>|[^)\s]+)")
_HTML_ATTR = re.compile(r'(?:href|src)\s*=\s*"([^"]+)"')

SKIP_PREFIXES = ("http://", "https://", "mailto:", "tel:", "data:", "#", "/")


def mkdocs_page_dir(md_path: Path) -> Path:
    """Docs-root-relative directory URL for a page (use_directory_urls)."""
    rel = md_path.relative_to(DOCS)
    if rel.name == "index.md":
        return rel.parent
    return rel.with_suffix("")


def _normalize(base: Path, href: str) -> Path:
    stack = list(base.parts) if str(base) != "." else []
    for part in Path(href).parts:
        if part == "..":
            if stack:
                stack.pop()
        elif part != ".":
            stack.append(part)
    return Path(*stack) if stack else Path(".")


def _target_exists(path: Path) -> bool:
    if path.exists():
        return True
    if path.suffix == "":  # directory URL → foo.md or foo/index.md
        return path.with_suffix(".md").exists() or (path / "index.md").exists()
    return False


_INLINE_CODE = re.compile(r"`[^`]*`")


def _strip_code(text: str) -> str:
    """Blank out fenced code blocks and inline-code spans so example link-like
    syntax inside them isn't checked."""
    out_lines = []
    in_fence = False
    for line in text.split("\n"):
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            out_lines.append("")
            continue
        out_lines.append("" if in_fence else _INLINE_CODE.sub("", line))
    return "\n".join(out_lines)


def _iter_links(text: str):
    text = _strip_code(text)
    for m in _MD_LINK.finditer(text):
        yield m.group(1).strip("<>")
    for m in _HTML_ATTR.finditer(text):
        yield m.group(1).strip()


def _broken(md_file: Path, raw: str) -> bool:
    path = raw.split("#", 1)[0].split("?", 1)[0]
    if not path:
        return False
    if path.endswith("/"):
        # directory link → directory-URL resolution
        target = DOCS / _normalize(mkdocs_page_dir(md_file), path)
    else:
        # .md link or relative asset → file-path resolution
        target = (md_file.parent / path).resolve()
    return not _target_exists(target)


def main() -> int:
    broken: list[str] = []
    checked = 0
    for md_file in sorted(DOCS.rglob("*.md")):
        if md_file in SKIP_FILES:
            continue
        try:
            text = md_file.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for raw in _iter_links(text):
            if raw.startswith(SKIP_PREFIXES) or "api/" in raw:
                continue
            path = raw.split("#", 1)[0].split("?", 1)[0]
            # Skip build-time generated notebook HTML (docs/**/rendered/*.html),
            # materialized by scripts/export_notebooks.py, absent in a fresh checkout.
            if "rendered/" in path and path.endswith(".html"):
                continue
            # skip protocol-relative/odd schemes and non-path example text
            # (real relative links never contain raw spaces)
            if not path or ":" in path or " " in path:
                continue
            checked += 1
            if _broken(md_file, raw):
                broken.append(f"{md_file.relative_to(ROOT)}: -> {raw}")

    if broken:
        print(f"check_doc_links: {len(broken)} broken relative link(s):", file=sys.stderr)
        for item in broken:
            print(f"  - {item}", file=sys.stderr)
        return 1
    print(f"check_doc_links: OK ({checked} relative links resolve)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
