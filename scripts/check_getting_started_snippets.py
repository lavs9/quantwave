#!/usr/bin/env python3
"""Execute every Python snippet on the Getting Started pages and diff its
documented output against what it actually prints.

Docs credibility gap this closes: `docs/getting-started/index.md` and
`docs/getting-started/python.md` claim every code block's printed output is
verified against a real run. This script is how that claim stays true after
the next edit — it is the enforcement mechanism, not just documentation of
intent.

Convention encoded here (see those two files): a fenced ```python block is
immediately followed by a fenced ```text block holding its exact expected
stdout. Blocks may sit inside an indented admonition (`!!! danger`, 4-space
indented) — indentation is stripped before execution/comparison. All ```python
blocks on one page execute in a single shared namespace, in document order
(later snippets may rely on names — e.g. `df` — bound by earlier ones on the
same page), mirroring how a reader would actually copy-paste through the
page top to bottom. ```bash blocks (install, `quantwave doctor`, ...) are
intentionally not executed here — they are CLI/shell, not Python, and are
out of scope for this checker; their output was still verified by hand when
the docs were written.

Usage:
    python3 scripts/check_getting_started_snippets.py
"""

from __future__ import annotations

import contextlib
import io
import re
import sys
import textwrap
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

TARGET_FILES = [
    ROOT / "docs" / "getting-started" / "index.md",
    ROOT / "docs" / "getting-started" / "python.md",
]

# Matches a fenced code block, capturing any leading indentation (so blocks
# nested inside a 4-space-indented `!!! danger` admonition body still match),
# the language tag, and the raw (still-indented) body.
FENCE_RE = re.compile(
    r"(?P<indent>[ \t]*)```(?P<lang>[a-zA-Z0-9_+-]*)\n"
    r"(?P<body>.*?)\n"
    r"(?P=indent)```",
    re.DOTALL,
)


@dataclass
class Block:
    lang: str
    body: str  # dedented
    line_no: int


def _line_of(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1


def extract_blocks(text: str) -> list[Block]:
    blocks: list[Block] = []
    for m in FENCE_RE.finditer(text):
        indent = m.group("indent")
        raw_body = m.group("body")
        if indent:
            # Strip the admonition's indentation from every line.
            lines = raw_body.split("\n")
            dedented = "\n".join(
                line[len(indent):] if line.startswith(indent) else line
                for line in lines
            )
        else:
            dedented = raw_body
        blocks.append(
            Block(lang=m.group("lang"), body=dedented, line_no=_line_of(text, m.start()))
        )
    return blocks


def pair_python_blocks(blocks: list[Block]) -> list[tuple[Block, Block | None]]:
    """Pair each python block with the immediately-following fenced block.

    Only a ``text``-tagged follower counts as a documented-output pair; any
    other adjacency (e.g. python block followed by another python block)
    means "no expected output was documented for this snippet", which is
    itself a failure this checker reports.
    """
    pairs: list[tuple[Block, Block | None]] = []
    for i, block in enumerate(blocks):
        if block.lang != "python":
            continue
        nxt = blocks[i + 1] if i + 1 < len(blocks) else None
        if nxt is not None and nxt.lang == "text":
            pairs.append((block, nxt))
        else:
            pairs.append((block, None))
    return pairs


def run_snippet(source: str, namespace: dict) -> tuple[str, str | None]:
    """Execute *source* in *namespace*, returning (stdout, error_or_None)."""
    buf = io.StringIO()
    try:
        with contextlib.redirect_stdout(buf):
            exec(compile(source, "<snippet>", "exec"), namespace)
    except Exception as exc:  # noqa: BLE001 - we want to report any failure
        return buf.getvalue(), f"{type(exc).__name__}: {exc}"
    return buf.getvalue(), None


def check_file(path: Path) -> list[str]:
    failures: list[str] = []
    text = path.read_text(encoding="utf-8")
    blocks = extract_blocks(text)
    pairs = pair_python_blocks(blocks)

    if not pairs:
        return failures

    namespace: dict = {"__name__": "__main__"}
    for block, expected_block in pairs:
        if expected_block is None:
            failures.append(
                f"{path.relative_to(ROOT)}:{block.line_no}: python snippet has no "
                "paired ```text expected-output block immediately after it"
            )
            continue

        actual, error = run_snippet(block.body, namespace)
        if error is not None:
            failures.append(
                f"{path.relative_to(ROOT)}:{block.line_no}: snippet raised {error}\n"
                f"--- snippet ---\n{block.body}"
            )
            continue

        expected = expected_block.body
        # Normalize a single trailing newline difference (fence formatting);
        # everything else — including internal whitespace and table
        # formatting — must match exactly, since that is the whole point.
        if actual.rstrip("\n") != expected.rstrip("\n"):
            failures.append(
                f"{path.relative_to(ROOT)}:{block.line_no}: documented output does not "
                f"match actual output\n"
                f"--- expected (docs) ---\n{expected}\n"
                f"--- actual (real run) ---\n{actual}"
            )

    return failures


def main() -> int:
    all_failures: list[str] = []
    checked = 0
    for path in TARGET_FILES:
        if not path.exists():
            all_failures.append(f"missing target file: {path.relative_to(ROOT)}")
            continue
        blocks = extract_blocks(path.read_text(encoding="utf-8"))
        checked += sum(1 for b in blocks if b.lang == "python")
        all_failures.extend(check_file(path))

    if all_failures:
        print("FAIL: Getting Started snippet drift:", file=sys.stderr)
        for msg in all_failures:
            print(textwrap.indent(msg, "  "), file=sys.stderr)
        return 1

    print(
        f"Getting Started snippet check passed: {checked} python snippets "
        f"verified across {len(TARGET_FILES)} files."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
