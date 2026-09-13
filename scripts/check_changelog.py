#!/usr/bin/env python3
"""Enforce Keep a Changelog 1.1 structure on docs/changelog.md (quantwave-zr74.3).

Validates:
  - An `## [Unreleased]` section exists as the very first version heading.
  - Every other version heading is `## [X.Y.Z] - YYYY-MM-DD` with a valid semver
    and an ISO-8601 date.
  - That date matches the corresponding `vX.Y.Z` git tag's creation date, when
    the tag is available locally (best-effort `git fetch --tags` first).
  - Only the six standard Keep a Changelog subheadings are used
    (Added / Changed / Deprecated / Removed / Fixed / Security), each at most
    once per version section.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHANGELOG = ROOT / "docs" / "changelog.md"

SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
VERSION_HEADING_RE = re.compile(r"^## \[(?P<version>[^\]]+)\](?: - (?P<date>.+))?\s*$")
SUBHEADING_RE = re.compile(r"^### (?P<name>.+?)\s*$")

STANDARD_SUBHEADINGS = {"Added", "Changed", "Deprecated", "Removed", "Fixed", "Security"}


def _ensure_tags_fetched() -> None:
    """Best-effort: pull tags if this is a shallow/tagless checkout."""
    try:
        existing = subprocess.run(
            ["git", "tag", "-l"], cwd=ROOT, capture_output=True, text=True, timeout=10
        )
        if existing.returncode == 0 and existing.stdout.strip():
            return
        subprocess.run(
            ["git", "fetch", "--tags", "--quiet"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
        )
    except (OSError, subprocess.SubprocessError):
        pass


def _tag_date(version: str) -> str | None:
    """Return the ISO date (YYYY-MM-DD) of tag vX.Y.Z, or None if unavailable."""
    try:
        result = subprocess.run(
            ["git", "tag", "--list", f"v{version}", "--format=%(creatordate:iso-strict)"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=10,
        )
    except (OSError, subprocess.SubprocessError):
        return None
    out = result.stdout.strip()
    if not out:
        return None
    return out[:10]


def main() -> int:
    if not CHANGELOG.exists():
        print(f"check_changelog: missing {CHANGELOG}", file=sys.stderr)
        return 1

    lines = CHANGELOG.read_text(encoding="utf-8").splitlines()

    # Collect version headings with their line numbers.
    headings: list[tuple[int, str, str | None]] = []
    for i, line in enumerate(lines, start=1):
        m = VERSION_HEADING_RE.match(line)
        if m:
            headings.append((i, m.group("version"), m.group("date")))

    errors: list[str] = []
    warnings: list[str] = []

    if not headings:
        errors.append("no version headings (`## [...]`) found at all")
    else:
        first_line, first_version, first_date = headings[0]
        if first_version != "Unreleased":
            errors.append(
                f"line {first_line}: expected `## [Unreleased]` as the first version "
                f"heading, found `## [{first_version}]`"
            )
        elif first_date is not None:
            errors.append(f"line {first_line}: `[Unreleased]` must not carry a date")

    _ensure_tags_fetched()

    for line_no, version, date in headings:
        if version == "Unreleased":
            continue

        if not SEMVER_RE.match(version):
            errors.append(f"line {line_no}: `{version}` is not a valid semver (X.Y.Z)")
            continue

        if date is None:
            errors.append(f"line {line_no}: `[{version}]` heading is missing a date")
            continue

        if not DATE_RE.match(date):
            errors.append(
                f"line {line_no}: date `{date}` for `[{version}]` is not ISO-8601 (YYYY-MM-DD)"
            )
            continue

        tag_date = _tag_date(version)
        if tag_date is None:
            warnings.append(
                f"line {line_no}: no `v{version}` git tag available to verify date "
                f"`{date}` against (skipping — tags may not be fetched in this checkout)"
            )
        elif tag_date != date:
            errors.append(
                f"line {line_no}: `[{version}]` date `{date}` does not match "
                f"`v{version}` tag date `{tag_date}`"
            )

    # Walk sections (heading line -> next heading line) and check subheadings.
    boundaries = [ln for ln, _, _ in headings] + [len(lines) + 1]
    for idx, (line_no, version, _date) in enumerate(headings):
        section_end = boundaries[idx + 1]
        seen: dict[str, int] = {}
        for j in range(line_no, section_end - 1):
            sub = SUBHEADING_RE.match(lines[j])
            if not sub:
                continue
            name = sub.group("name")
            if name not in STANDARD_SUBHEADINGS:
                errors.append(
                    f"line {j + 1}: non-standard subheading `### {name}` under "
                    f"`[{version}]` (expected one of {sorted(STANDARD_SUBHEADINGS)})"
                )
                continue
            if name in seen:
                errors.append(
                    f"line {j + 1}: duplicate `### {name}` under `[{version}]` "
                    f"(first seen at line {seen[name]}) — merge into one section"
                )
                continue
            seen[name] = j + 1

    for w in warnings:
        print(f"check_changelog: WARNING: {w}", file=sys.stderr)

    if errors:
        print("check_changelog: FAILED", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1

    print("check_changelog: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
