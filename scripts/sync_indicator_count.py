#!/usr/bin/env python3
"""Template canonical indicator count from metadata_export.json into user-facing files."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
METADATA_FILE = ROOT / "metadata_export.json"

# (path, pattern, replacement template with {count})
REPLACEMENTS: list[tuple[Path, re.Pattern[str], str]] = [
    (
        ROOT / "README.md",
        re.compile(r"(\*\*)?\d{2,3}(\*\*)?\s+[Nn]ative [Ii]ndicators"),
        "{count} Native Indicators",
    ),
    (
        ROOT / "mkdocs.yml",
        re.compile(r"with \d{2,3} native indicators"),
        "with {count} native indicators",
    ),
    (
        ROOT / "docs" / "faq.md",
        re.compile(r"\*\*\d{2,3} Rust indicators\*\*"),
        "**{count} Rust indicators**",
    ),
    (
        ROOT / "docs" / "faq.md",
        re.compile(r"\*\*\d{2,3}\*\* registered native indicators"),
        "**{count}** registered native indicators",
    ),
    (
        ROOT / "docs" / "comparison.md",
        re.compile(r"\|\s+\*\*\d{2,3}\*\* native \+ metadata"),
        "| **{count}** native + metadata",
    ),
    (
        ROOT / "docs" / "index.md",
        re.compile(r"<strong>\d{2,3} native indicators</strong>"),
        "<strong>{count} native indicators</strong>",
    ),
    (
        ROOT / "docs" / "index.md",
        re.compile(r"<strong>\d{2,3}</strong><span>Native indicators</span>"),
        "<strong>{count}</strong><span>Native indicators</span>",
    ),
    (
        ROOT / "docs" / "index.md",
        re.compile(r"\d{2,3} Rust-native indicators"),
        "{count} Rust-native indicators",
    ),
    (
        ROOT / "docs" / "index.md",
        re.compile(r"\d{2,3} indicators, full Ehlers"),
        "{count} indicators, full Ehlers",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "gallery.md",
        re.compile(r"\*\*\d{2,3} production-grade native indicators\*\*"),
        "**{count} production-grade native indicators**",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "index.md",
        re.compile(r"\*\*\d{2,3} native Rust indicators\*\*"),
        "**{count} native Rust indicators**",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "index.md",
        re.compile(r"ships \*\*\d{2,3} native Rust indicators\*\*"),
        "ships **{count} native Rust indicators**",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "index.md",
        re.compile(r"all \d{2,3} indicators by category"),
        "all {count} indicators by category",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "README.md",
        re.compile(r"\*\*\d{2,3} native indicators\*\*"),
        "**{count} native indicators**",
    ),
    (
        ROOT / "docs" / "guides" / "indicators" / "native" / "README.md",
        re.compile(r"overview of \d{2,3} native indicators"),
        "overview of {count} native indicators",
    ),
    (
        ROOT / "docs" / "guides" / "rust" / "index.md",
        re.compile(r"powers \*\*\d{2,3}\*\* native indicators"),
        "powers **{count}** native indicators",
    ),
    (
        ROOT / "docs" / "guides" / "rust" / "crate-map.md",
        re.compile(r"\|\s+\d{2,3} indicator catalog"),
        "| {count} indicator catalog",
    ),
    (
        ROOT / "docs" / "getting-started" / "index.md",
        re.compile(r"^\d{2,3} native tools"),
        "{count} native tools",
    ),
    (
        ROOT / "docs" / "getting-started" / "python.md",
        re.compile(r"# sorted list of ~500\+ names"),
        "# sorted list of {count} names",
    ),
    (
        ROOT / "docs" / "llms.txt",
        re.compile(r": \d{2,3} native Rust"),
        ": {count} native Rust",
    ),
    (
        ROOT / "quantwave-core" / "src" / "lib.rs",
        re.compile(r"\*\*\d{2,3}\*\* native indicators"),
        "**{count}** native indicators",
    ),
    (
        ROOT / "quantwave-polars" / "src" / "lib.rs",
        re.compile(r"\*\*\d{2,3}\*\* native indicators"),
        "**{count}** native indicators",
    ),
]


def canonical_count() -> int:
    if not METADATA_FILE.exists():
        subprocess.run(
            ["cargo", "run", "-p", "quantwave-core", "--bin", "export_metadata"],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    data = json.loads(METADATA_FILE.read_text(encoding="utf-8"))
    return len(data)


def main() -> int:
    count = canonical_count()
    changed = 0
    for path, pattern, template in REPLACEMENTS:
        if not path.exists():
            print(f"skip missing {path.relative_to(ROOT)}", file=sys.stderr)
            continue
        text = path.read_text(encoding="utf-8")
        new_text, n = pattern.subn(template.format(count=count), text)
        if n:
            path.write_text(new_text, encoding="utf-8")
            print(f"updated {path.relative_to(ROOT)} ({n} replacements)")
            changed += n
    print(f"sync_indicator_count: canonical={count}, replacements={changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())