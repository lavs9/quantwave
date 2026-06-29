#!/usr/bin/env python3
"""Create backtest v0.7 child beads (skips titles that already exist)."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EPIC = "quantwave-qzpi"
E8 = "quantwave-8v4s"

# (title, type, priority, labels, description, acceptance, dep_titles)
SPECS: list[tuple] = [
    (
        "Backtest docs landing page (index.md)",
        "task", "2", "grunt,docs,backtest",
        "AFK/GRUNT. Backtest Engine landing: moat, .bt API table, notebook links.\n\nAgent tier: GRUNT.",
        "- [ ] Landing page in mkdocs nav\n- [ ] Links to quickstart, matrix, notebooks\n- [ ] mkdocs build passes",
        ["mkdocs: Backtest top-level nav + rename Guides to Indicators"],
    ),
    (
        "Refresh backtest capability_matrix.md for v0.6+ reality",
        "task", "2", "grunt,docs,backtest",
        "AFK/GRUNT. Update matrix: tear sheets ✅, unified wheel, portfolio ⏸.\n\nAgent tier: GRUNT.",
        "- [ ] Proof links valid\n- [ ] Tear sheets ✅\n- [ ] mkdocs build passes",
        ["mkdocs: Backtest top-level nav + rename Guides to Indicators"],
    ),
    (
        "Sync planning/BACKTEST_SOA.md with shipped v0.6 state",
        "task", "3", "grunt,docs,backtest",
        "AFK/GRUNT. Align BACKTEST_SOA with v0.6 shipped state.\n\nAgent tier: GRUNT.",
        "- [ ] No false ❌ for shipped features",
        [],
    ),
    (
        "Backtest doc drift gate in check_doc_drift.py",
        "task", "2", "grunt,docs,ci,backtest",
        "AFK/GRUNT. Validate backtest docs + notebook stubs in check_doc_drift.py.\n\nAgent tier: GRUNT.",
        "- [ ] check_doc_drift covers backtest section\n- [ ] In quantwave_verify.sh",
        ["mkdocs: Backtest top-level nav + rename Guides to Indicators"],
    ),
    (
        "ADR: shared-capital portfolio allocation semantics",
        "decision", "1", "important,architecture,backtest,hitl",
        "HITL/IMPORTANT. ADR: cash pool, allocators, API, parity. Sign-off before engine work.\n\nAgent tier: IMPORTANT.",
        "- [ ] ADR committed\n- [ ] Phase-1: 2-symbol long-format MVP",
        [],
    ),
    (
        "Rust: shared-capital 2-symbol portfolio simulation (MVP)",
        "feature", "1", "important,engine,rust,backtest",
        "AFK/IMPORTANT. Pooled-book sim. Proof: tests/portfolio_shared_capital.rs\n\nAgent tier: IMPORTANT.",
        "- [ ] cargo nextest green\n- [ ] Pooled equity not sum of independent books",
        ["ADR: shared-capital portfolio allocation semantics"],
    ),
    (
        "Rust: portfolio equity + metrics from pooled book",
        "feature", "1", "important,engine,rust,backtest",
        "AFK/IMPORTANT. metrics.rs for pooled mode. Proof: tests/portfolio_metrics.rs\n\nAgent tier: IMPORTANT.",
        "- [ ] Metrics on pooled equity\n- [ ] Fixture regression",
        ["Rust: shared-capital 2-symbol portfolio simulation (MVP)"],
    ),
    (
        "Python: .bt.portfolio_backtest() API + PyO3 bindings",
        "feature", "1", "important,python,backtest",
        "AFK/IMPORTANT. portfolio_backtest() in bt_polars. Proof: test_portfolio_backtest.py\n\nAgent tier: IMPORTANT.",
        "- [ ] pytest green\n- [ ] 2-symbol smoke",
        ["Rust: portfolio equity + metrics from pooled book"],
    ),
    (
        "Batch↔streaming parity for portfolio shared-capital mode",
        "feature", "1", "important,parity,backtest",
        "AFK/IMPORTANT. Portfolio batch/stream parity. Proof: portfolio_streaming_parity.rs\n\nAgent tier: IMPORTANT.",
        "- [ ] Equity match within tolerance\n- [ ] Moat documented",
        ["Rust: shared-capital 2-symbol portfolio simulation (MVP)"],
    ),
    (
        "Multi-symbol (3+) shared-capital stress tests",
        "task", "2", "grunt,tests,backtest",
        "AFK/GRUNT. 3-5 symbol edge cases. Rust + pytest.\n\nAgent tier: GRUNT.",
        "- [ ] ≥5 Rust scenarios\n- [ ] ≥3 pytest mirrors",
        ["Python: .bt.portfolio_backtest() API + PyO3 bindings"],
    ),
    (
        "Python cross_sectional transform=winsorize parity",
        "task", "2", "grunt,python,backtest",
        "AFK/GRUNT. transform='winsorize' on cross_sectional_backtest.\n\nAgent tier: GRUNT.",
        "- [ ] E2E works\n- [ ] capability_matrix ✅",
        [],
    ),
    (
        "Proptest batch↔streaming backtest parity (core engine)",
        "task", "2", "important,tests,parity,backtest",
        "AFK/IMPORTANT. proptest_parity.rs\n\nAgent tier: IMPORTANT.",
        "- [ ] cargo nextest stable",
        [],
    ),
    (
        "Deduplicate WFO: Python bt_polars delegates to Rust walk_forward.rs",
        "task", "2", "important,python,rust,backtest",
        "AFK/IMPORTANT. Thin Python WFO wrapper.\n\nAgent tier: IMPORTANT.",
        "- [ ] WFO pytest green\n- [ ] Single Rust source",
        [],
    ),
    (
        "Regenerate backtest Marimo notebook artifacts (.md exports)",
        "task", "3", "grunt,docs,artifacts,backtest",
        "AFK/GRUNT. export_notebooks.py for 5 backtest notebooks.\n\nAgent tier: GRUNT.",
        "- [ ] All .md exports current\n- [ ] CI export green",
        ["mkdocs: Backtest top-level nav + rename Guides to Indicators"],
    ),
    (
        "Portfolio shared-capital showcase notebook + export",
        "task", "2", "grunt,docs,artifacts,backtest",
        "AFK/GRUNT. portfolio_shared_capital_backtest.py notebook.\n\nAgent tier: GRUNT.",
        "- [ ] Notebook + .md export\n- [ ] Linked from backtest index",
        ["Python: .bt.portfolio_backtest() API + PyO3 bindings"],
    ),
    (
        "Tear sheet usage guide under Backtest Engine docs",
        "task", "3", "grunt,docs,backtest",
        "AFK/GRUNT. guides/backtest/tear_sheets.md\n\nAgent tier: GRUNT.",
        "- [ ] In Backtest nav\n- [ ] Code sample works",
        ["mkdocs: Backtest top-level nav + rename Guides to Indicators"],
    ),
    (
        "Wire portfolio + backtest tests into quantwave_verify.sh",
        "task", "2", "grunt,ci,backtest",
        "AFK/GRUNT. Portfolio tests in verify.sh.\n\nAgent tier: GRUNT.",
        "- [ ] verify.sh runs portfolio tests",
        ["Multi-symbol (3+) shared-capital stress tests"],
    ),
]


def run(*args: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(list(args), cwd=ROOT, capture_output=True, text=True, check=check)


def list_issues() -> list[dict]:
    proc = run("bd", "list", "--json")
    return json.loads(proc.stdout)


def title_to_id(items: list[dict]) -> dict[str, str]:
    return {i["title"]: i["id"] for i in items}


def create(title: str, typ: str, priority: str, labels: str, desc: str, acceptance: str) -> str:
    proc = run(
        "bd", "create", title,
        "-d", desc,
        "-t", typ,
        "-p", priority,
        "--parent", EPIC,
        "--labels", labels,
        "--acceptance", acceptance,
        "--json",
    )
    return json.loads(proc.stdout)["id"]


def add_dep(issue_id: str, dep_id: str, dep_type: str = "blocks") -> None:
    run("bd", "update", issue_id, "--deps", f"{dep_type}:{dep_id}", "--json", check=False)


def main() -> int:
    items = list_issues()
    ids = title_to_id(items)
    created = 0
    for title, typ, pri, labels, desc, acceptance, dep_titles in SPECS:
        if title in ids:
            print(f"skip: {title}")
            continue
        iid = create(title, typ, pri, labels, desc, acceptance)
        ids[title] = iid
        created += 1
        print(f"created {iid}: {title[:55]}")

    # Refresh and wire deps
    items = list_issues()
    ids = title_to_id(items)
    nav_id = ids.get("mkdocs: Backtest top-level nav + rename Guides to Indicators")
    adr_id = ids.get("ADR: shared-capital portfolio allocation semantics")

    if nav_id:
        for title, *_rest, dep_titles in SPECS:
            if "mkdocs: Backtest top-level nav" in dep_titles and title in ids:
                add_dep(ids[title], nav_id)

    if adr_id:
        add_dep(E8, adr_id, "blocks")
        add_dep(adr_id, E8, "discovered-from")

    for title, *_rest, dep_titles in SPECS:
        if title not in ids:
            continue
        for dep_title in dep_titles:
            if dep_title in ids and ids[dep_title] != ids[title]:
                add_dep(ids[title], ids[dep_title])

    add_dep(E8, EPIC, "discovered-from")

    print(f"\nDone. Created {created} new issues under {EPIC}.")
    print("Run: bd ready --json | jq '.[].title'")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())