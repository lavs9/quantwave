#!/usr/bin/env python3
"""DEPRECATED — use benchmarks/harness.py instead.

This script previously injected unmeasured latency figures into docs/benchmarks.md.
It is retained only as a guardrail: running it exits non-zero and points to the harness.
"""

from __future__ import annotations

import sys


def main() -> int:
    print(
        "update_benchmarks_metrics.py is deprecated (quantwave-9gek.2).\n"
        "Run: python benchmarks/harness.py\n"
        "Docs render from benchmarks/results/latest.json only.",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())