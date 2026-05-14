#!/usr/bin/env python3
"""
insert_metadata_stubs.py
========================
Inserts stub values for the three new IndicatorMetadata fields
(usage, keywords, ehlers_summary) into every indicator .rs file
that still has the OLD struct format (missing those fields).

Run from workspace root:
    python3 scripts/insert_metadata_stubs.py

After this runs, `cargo check -p quantwave-core` should produce 0 errors.
Subagents then replace stubs with real curated content.
"""

import re
import sys
from pathlib import Path

INDICATORS_DIR = Path("quantwave-core/src/indicators")

# Pattern: find `description: "..."` line inside an IndicatorMetadata block
# and insert the three new fields after it, if not already present.
DESC_PATTERN = re.compile(
    r'([ \t]+description:\s*r?\"\"\"?.*?\"\"\"?|[ \t]+description:\s*r?#?".*?"#?,)',
    re.DOTALL
)

# Simpler line-by-line approach: find "description:" line, check if "usage:" already follows,
# if not insert the stub block after the description line.

STUB_BLOCK = '''\
    usage: "TODO: Add practical usage description.",
    keywords: &["TODO"],
    ehlers_summary: "TODO: Add authoritative summary from Ehlers\\'s papers or StockCharts.",'''

def already_has_fields(content: str) -> bool:
    return "usage:" in content and "keywords:" in content and "ehlers_summary:" in content

def insert_stubs(content: str) -> str:
    """Insert stub fields after the `description:` line in IndicatorMetadata structs."""
    lines = content.splitlines(keepends=True)
    result = []
    inside_metadata = False
    inserted = False

    for i, line in enumerate(lines):
        result.append(line)

        # Detect start of IndicatorMetadata struct
        if "IndicatorMetadata {" in line or "IndicatorMetadata{" in line:
            inside_metadata = True
            inserted = False

        if inside_metadata and not inserted:
            stripped = line.strip()
            # Match the description field (handles both regular and raw strings)
            if stripped.startswith("description:") and ("," in stripped or stripped.endswith('"')):
                # Make sure the line ends with comma (raw strings may span lines — skip those)
                if stripped.endswith(",") or stripped.endswith('#,'):
                    result.append(STUB_BLOCK + "\n")
                    inserted = True

        # Detect end of struct
        if inside_metadata and stripped_ends_struct(line):
            inside_metadata = False

    return "".join(result)

def stripped_ends_struct(line: str) -> bool:
    s = line.strip()
    return s in ("};", "} ;")

def process_file(path: Path) -> bool:
    content = path.read_text(encoding="utf-8")
    if already_has_fields(content):
        return False
    if "IndicatorMetadata" not in content:
        return False
    new_content = insert_stubs(content)
    if new_content != content:
        path.write_text(new_content, encoding="utf-8")
        return True
    return False

def main():
    if not INDICATORS_DIR.exists():
        print(f"ERROR: {INDICATORS_DIR} not found. Run from workspace root.", file=sys.stderr)
        sys.exit(1)

    updated = []
    skipped = []

    for rs_file in sorted(INDICATORS_DIR.glob("*.rs")):
        if rs_file.name in ("metadata.rs", "mod.rs"):
            continue
        changed = process_file(rs_file)
        if changed:
            updated.append(rs_file.name)
        else:
            skipped.append(rs_file.name)

    print(f"Updated {len(updated)} files:")
    for f in updated:
        print(f"  ✓ {f}")
    if skipped:
        print(f"\nSkipped {len(skipped)} files (already have fields or no metadata).")

if __name__ == "__main__":
    main()
