#!/usr/bin/env python3
"""Guard public metadata consistency for AI SEO and package discovery."""

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
METADATA_FILE = ROOT / "metadata_export.json"

# User-facing files that must not advertise stale indicator counts.
PUBLIC_FILES = [
    ROOT / "README.md",
    ROOT / "quantwave-python" / "README.md",
    ROOT / "quantwave-python" / "pyproject.toml",
    ROOT / "docs" / "llms.txt",
]

COUNT_DRIFT_FILES = [
    ROOT / "README.md",
    ROOT / "mkdocs.yml",
    ROOT / "docs" / "index.md",
    ROOT / "docs" / "faq.md",
    ROOT / "docs" / "comparison.md",
    ROOT / "docs" / "guides" / "indicators" / "gallery.md",
    ROOT / "docs" / "guides" / "indicators" / "index.md",
    ROOT / "docs" / "getting-started" / "python.md",
]

INDICATOR_COUNT_RE = re.compile(
    r"(?i)(?<![\w/])(?:~?\s*)?(1\d{2}|2\d{2}|500)\+?\s*(?:native\s+)?indicators?"
)

STALE_COUNT_RE = re.compile(r"150\+?\s*(native\s+)?indicators?", re.IGNORECASE)
# Must match a line in llms.txt; validated against live metadata export count.
EXPECTED_IN_LLMS = None  # set at runtime from export


def export_metadata() -> int:
    result = subprocess.run(
        ["cargo", "run", "-p", "quantwave-core", "--bin", "export_metadata"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    METADATA_FILE.write_text(result.stdout, encoding="utf-8")
    data = json.loads(result.stdout)
    return len(data)


def main() -> None:
    count = export_metadata()
    failures: list[str] = []
    count_str = str(count)

    llms = ROOT / "docs" / "llms.txt"
    if not llms.exists():
        failures.append("missing docs/llms.txt (hand-written AI crawler index)")
    else:
        text = llms.read_text(encoding="utf-8")
        if count_str not in text:
            failures.append(f"docs/llms.txt must mention {count} indicators")
        if "lavs9.github.io/quantwave" not in text:
            failures.append("docs/llms.txt must include official docs URL")

    robots = ROOT / "docs" / "robots.txt"
    if not robots.exists():
        failures.append("missing docs/robots.txt")

    for path in PUBLIC_FILES:
        if not path.exists():
            failures.append(f"missing public file: {path.relative_to(ROOT)}")
            continue
        text = path.read_text(encoding="utf-8")
        if STALE_COUNT_RE.search(text):
            failures.append(
                f"{path.relative_to(ROOT)}: stale '150+ indicators' copy — use {count}"
            )
        if path.suffix == ".toml":
            if str(count) not in text and f"{count} " not in text:
                failures.append(
                    f"{path.relative_to(ROOT)}: description should mention {count} indicators"
                )
        elif path.name == "README.md" and str(count) not in text:
            failures.append(
                f"{path.relative_to(ROOT)}: README should mention {count} indicators"
            )

    docs_rs_fields = [
        ROOT / "quantwave-core" / "Cargo.toml",
        ROOT / "quantwave-polars" / "Cargo.toml",
        ROOT / "quantwave-plugins" / "Cargo.toml",
        ROOT / "quantwave-backtest" / "Cargo.toml",
        ROOT / "quantwave" / "Cargo.toml",
    ]
    for cargo in docs_rs_fields:
        text = cargo.read_text(encoding="utf-8")
        if "documentation" not in text:
            failures.append(
                f"{cargo.relative_to(ROOT)}: missing documentation = https://docs.rs/..."
            )

    for path in COUNT_DRIFT_FILES:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        for match in INDICATOR_COUNT_RE.finditer(text):
            snippet = match.group(0)
            if str(count) in snippet:
                continue
            if "500" in snippet and "names" in text[match.start() : match.end() + 20]:
                failures.append(
                    f"{path.relative_to(ROOT)}: stale indicator count {snippet!r} (want {count})"
                )
            elif "indicator" in snippet.lower():
                failures.append(
                    f"{path.relative_to(ROOT)}: stale indicator count {snippet!r} (want {count})"
                )

    if failures:
        print("FAIL: public metadata / AEO checks:", file=sys.stderr)
        for msg in failures:
            print(f"  - {msg}", file=sys.stderr)
        sys.exit(1)

    print(f"Public metadata check passed ({count} indicators, llms.txt + robots.txt OK).")


if __name__ == "__main__":
    main()