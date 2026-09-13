#!/usr/bin/env python3
"""Release-time changelog automation (quantwave-zr74.3).

Two modes:

  --apply VERSION [--date YYYY-MM-DD]
      Maintainer-run, best-effort transform: renames `## [Unreleased]` to
      `## [VERSION] - DATE` (date defaults to today, UTC) and inserts a
      fresh, empty `## [Unreleased]` section above it. Run this, commit the
      result, and *then* tag `vVERSION` — this script does not create tags
      or commits itself.

  --check [VERSION]
      Gate used by the release workflow (runs after a `vX.Y.Z` tag is
      pushed). Fails unless docs/changelog.md already has a
      `## [VERSION] - <date>` section (i.e. someone ran --apply and
      committed it before tagging) and `[Unreleased]` is now empty. VERSION
      defaults to the pushed tag, read from $GITHUB_REF.

This is intentionally simple: it moves the whole Unreleased section as-is
rather than trying to recategorize entries (that judgment call belongs to
whoever writes the entries), and it never mutates the changelog on its own
in the release workflow — it only verifies a human already did.
"""

from __future__ import annotations

import argparse
import datetime
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHANGELOG = ROOT / "docs" / "changelog.md"

UNRELEASED_RE = re.compile(r"^## \[Unreleased\]\s*$")
VERSION_HEADING_RE = re.compile(r"^## \[(?P<version>[^\]]+)\](?: - (?P<date>\S+))?\s*$")
SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def _sections(lines: list[str]) -> list[tuple[int, int, str, str | None]]:
    """Return (start, end, version, date) for every `## [...]` section,
    start/end as 0-based line indices with end exclusive of the next
    heading (or EOF)."""
    heads = []
    for i, line in enumerate(lines):
        m = VERSION_HEADING_RE.match(line)
        if m:
            heads.append((i, m.group("version"), m.group("date")))
    out = []
    for idx, (start, version, date) in enumerate(heads):
        end = heads[idx + 1][0] if idx + 1 < len(heads) else len(lines)
        out.append((start, end, version, date))
    return out


def _infer_version_from_env() -> str | None:
    ref = os.environ.get("GITHUB_REF", "")
    if ref.startswith("refs/tags/v"):
        return ref[len("refs/tags/v") :]
    return None


def cmd_apply(version: str, date: str | None) -> int:
    if not SEMVER_RE.match(version):
        print(f"release_changelog: `{version}` is not a valid semver (X.Y.Z)", file=sys.stderr)
        return 1
    if date is None:
        date = datetime.datetime.now(datetime.timezone.utc).date().isoformat()
    elif not DATE_RE.match(date):
        print(f"release_changelog: `{date}` is not ISO-8601 (YYYY-MM-DD)", file=sys.stderr)
        return 1

    if not CHANGELOG.exists():
        print(f"release_changelog: missing {CHANGELOG}", file=sys.stderr)
        return 1

    lines = CHANGELOG.read_text(encoding="utf-8").splitlines()
    sections = _sections(lines)
    unreleased = next((s for s in sections if s[2] == "Unreleased"), None)
    if unreleased is None:
        print("release_changelog: no `## [Unreleased]` heading found", file=sys.stderr)
        return 1
    start, end, _, _ = unreleased

    body = "\n".join(lines[start + 1 : end]).strip()
    if not body:
        print("release_changelog: `[Unreleased]` is empty — refusing to apply", file=sys.stderr)
        return 1

    new_lines = lines[: start + 1] + [""] + [f"## [{version}] - {date}"] + lines[start + 1 : end]
    if end < len(lines):
        new_lines += lines[end:]

    CHANGELOG.write_text("\n".join(new_lines) + "\n", encoding="utf-8")
    print(f"release_changelog: moved [Unreleased] content into [{version}] - {date}")
    print("release_changelog: commit docs/changelog.md before tagging v" + version)
    return 0


def cmd_check(version: str | None) -> int:
    if version is None:
        version = _infer_version_from_env()
    if version is None:
        print(
            "release_changelog: no VERSION given and $GITHUB_REF is not a tag ref "
            "(refs/tags/vX.Y.Z) — pass one explicitly: --check X.Y.Z",
            file=sys.stderr,
        )
        return 1
    if not SEMVER_RE.match(version):
        print(f"release_changelog: `{version}` is not a valid semver (X.Y.Z)", file=sys.stderr)
        return 1

    if not CHANGELOG.exists():
        print(f"release_changelog: missing {CHANGELOG}", file=sys.stderr)
        return 1

    lines = CHANGELOG.read_text(encoding="utf-8").splitlines()
    sections = _sections(lines)

    unreleased = next((s for s in sections if s[2] == "Unreleased"), None)
    if unreleased is not None:
        start, end, _, _ = unreleased
        if "\n".join(lines[start + 1 : end]).strip():
            print(
                "release_changelog: `[Unreleased]` still has content at release time. "
                f"Run `scripts/release_changelog.py --apply {version}`, commit "
                "docs/changelog.md, and re-tag.",
                file=sys.stderr,
            )
            return 1

    released = next((s for s in sections if s[2] == version), None)
    if released is None:
        print(
            f"release_changelog: no `## [{version}]` section in docs/changelog.md. "
            f"Run `scripts/release_changelog.py --apply {version}` and commit before tagging.",
            file=sys.stderr,
        )
        return 1
    _, _, _, date = released
    if date is None or not DATE_RE.match(date):
        print(
            f"release_changelog: `[{version}]` heading is missing a valid ISO-8601 date",
            file=sys.stderr,
        )
        return 1

    print(f"release_changelog: [{version}] - {date} is present and [Unreleased] is empty — OK")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", metavar="VERSION", help="promote [Unreleased] to [VERSION]")
    parser.add_argument(
        "--check",
        nargs="?",
        const="",
        metavar="VERSION",
        help="verify [VERSION] is released and [Unreleased] is empty "
        "(VERSION defaults to $GITHUB_REF's tag)",
    )
    parser.add_argument("--date", metavar="YYYY-MM-DD", help="release date (default: today UTC)")
    args = parser.parse_args()

    if args.apply and args.check is not None:
        parser.error("pass exactly one of --apply / --check")
    if args.apply:
        return cmd_apply(args.apply, args.date)
    if args.check is not None:
        return cmd_check(args.check or None)
    parser.error("pass exactly one of --apply / --check")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
