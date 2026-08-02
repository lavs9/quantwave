#!/usr/bin/env python3
"""Static-lint a Python file for known QuantWave anti-patterns.

Every check here corresponds to a numbered section in PITFALLS.md and to a
failure mode that QuantWave does *not* raise on. Findings are heuristic: this
reads source text, not runtime values, so it reports suspicion, not proof.

Usage:
    python check_usage.py strategy.py [more.py ...]

Exit codes:
    0  no findings
    1  one or more findings
    2  bad invocation / unreadable input
"""

from __future__ import annotations

import ast
import sys
from dataclasses import dataclass

BACKTEST_METHODS = {
    "backtest",
    "backtest_with_report",
    "backtest_metrics",
    "portfolio_backtest",
}

RATIO_METRICS = {"sharpe_ratio", "sortino_ratio", "profit_factor", "calmar_ratio"}

FRACTION_KWARGS = {"stop_loss_pct", "take_profit_pct", "trailing_stop_pct"}


@dataclass
class Finding:
    line: int
    pitfall: str
    message: str
    fix: str


def _attr_chain(node: ast.AST) -> str:
    """Render a dotted attribute chain, e.g. `df.lazy().bt.backtest`."""
    parts: list[str] = []
    while True:
        if isinstance(node, ast.Attribute):
            parts.append(node.attr)
            node = node.value
        elif isinstance(node, ast.Call):
            node = node.func
        elif isinstance(node, ast.Name):
            parts.append(node.id)
            break
        else:
            break
    return ".".join(reversed(parts))


def _receiver_column(func: ast.AST) -> str | None:
    """Column name in the receiver of a `.ta.<method>` call.

    For `pl.col("close").ta.ta_atr(...)` this walks back to the `pl.col("close")`
    call and returns `"close"`. Returns None when the receiver is not a literal
    `pl.col("...")` — a variable, an expression, anything we cannot read
    statically — so callers can stay silent rather than guess.
    """
    node = func
    while isinstance(node, ast.Attribute):
        node = node.value
    if (
        isinstance(node, ast.Call)
        and isinstance(node.func, ast.Attribute)
        and node.func.attr == "col"
        and node.args
    ):
        return _const(node.args[0]) if isinstance(node.args[0], ast.Constant) else None
    return None


def _kwargs(call: ast.Call) -> dict[str, ast.AST]:
    return {kw.arg: kw.value for kw in call.keywords if kw.arg}


def _const(node: ast.AST):
    return node.value if isinstance(node, ast.Constant) else None


class Checker(ast.NodeVisitor):
    def __init__(self, source: str) -> None:
        self.findings: list[Finding] = []
        self.source = source
        self.uses_ta = False
        self.has_sort = False
        self.metric_lines: list[int] = []
        self.num_trades_checked = False

    # -- pass 1 collects file-wide facts, visit() below is pass 2 ---------

    def scan_context(self, tree: ast.AST) -> None:
        for node in ast.walk(tree):
            if isinstance(node, ast.Attribute):
                if node.attr == "sort":
                    self.has_sort = True
                chain = _attr_chain(node)
                if ".ta." in f".{chain}." or chain.startswith("ta."):
                    self.uses_ta = True
            if isinstance(node, ast.Constant) and node.value == "num_trades":
                self.num_trades_checked = True
            if isinstance(node, ast.Attribute) and node.attr == "num_trades":
                self.num_trades_checked = True

    def add(self, node: ast.AST, pitfall: str, message: str, fix: str) -> None:
        self.findings.append(
            Finding(getattr(node, "lineno", 0), pitfall, message, fix)
        )

    def visit_Call(self, node: ast.Call) -> None:  # noqa: N802
        chain = _attr_chain(node.func)
        method = chain.rsplit(".", 1)[-1] if chain else ""
        kw = _kwargs(node)

        if method in BACKTEST_METHODS and ".bt." in f".{chain}":
            self._check_backtest(node, method, kw)

        if method in {"roc"} and ".ta." in f".{chain}":
            self.add(
                node,
                "PITFALLS §3",
                "`roc` returns percent (x100), not a fraction.",
                "Use `rocp` for (p / p_n) - 1, or divide by 100. A 100x error here is silent.",
            )

        if method.startswith("ta_") and ".ta." in f".{chain}" and len(node.args) >= 2:
            base = _receiver_column(node.func)
            # Only flag when we can actually read the receiver column and it is
            # not `high` — otherwise we would fire on correct calls.
            if base is not None and base != "high":
                self.add(
                    node,
                    "PITFALLS §5",
                    f"`{method}` expects TA-Lib order (high, low, close), but the receiver column is '{base}'.",
                    f'Write pl.col("high").ta.{method}("low", "close", ...). Putting close in the receiver '
                    "silently permutes the inputs and returns a wrong number with no error.",
                )

        if method == "stddev" and ".ta." in f".{chain}":
            self.add(
                node,
                "PITFALLS §4",
                "`stddev` is population std (ddof=0); pandas `.std()` defaults to ddof=1.",
                "Rescale by sqrt(N/(N-1)) if you are porting a pandas formula.",
            )

        if method in {"drop_nulls", "dropna"} and self.uses_ta:
            self.add(
                node,
                "PITFALLS §9",
                "`drop_nulls()` does not remove indicator warmup — warmup is NaN, not null.",
                "Use `drop_nans()`, or slice by `qw.warmup_bars(...)` (deterministic, keeps alignment).",
            )

        for name, value in kw.items():
            if name in FRACTION_KWARGS:
                literal = _const(value)
                if isinstance(literal, (int, float)) and literal >= 1.0:
                    self.add(
                        node,
                        "PITFALLS §10",
                        f"`{name}={literal}` is a fraction, so this is {literal * 100:.0f}%.",
                        f"Pass {literal / 100:g} for {literal:g}%.",
                    )

        self.generic_visit(node)

    def _check_backtest(self, node: ast.Call, method: str, kw: dict) -> None:
        if "size_multiplier_col" not in kw:
            self.add(
                node,
                "PITFALLS §1",
                "No `size_multiplier_col`: signal=1 opens **1 unit**, not 1 unit of capital.",
                "Add a Float64 sizing column, e.g. (initial_cash * 0.95 / close).",
            )

        delay = _const(kw.get("execution_delay"))
        if "execution_delay" not in kw:
            self.add(
                node,
                "PITFALLS §6",
                "`execution_delay` defaults to \"same_bar\" — fills on the signal bar's own close.",
                'Pass execution_delay="next_bar" for any strategy claiming execution realism.',
            )
        elif delay == "same_bar":
            self.add(
                node,
                "PITFALLS §6",
                '"same_bar" fills at the signal bar\'s close — optimistic if the signal uses that close.',
                'Use "next_bar" unless the signal is built purely from bar t-1 data.',
            )

        if kw.get("touched_exit") is None and any(
            k in kw for k in ("stop_loss_pct", "trailing_stop_pct")
        ):
            self.add(
                node,
                "PITFALLS §10",
                "Stops without `touched_exit=True` are evaluated on closes only.",
                "Pass touched_exit=True with high_col/low_col, or expect understated drawdowns.",
            )

        if method == "portfolio_backtest" and not self.has_sort:
            self.add(
                node,
                "PITFALLS §8",
                "`portfolio_backtest` requires the frame sorted by [\"timestamp\", \"symbol\"].",
                'Call .sort(["timestamp", "symbol"]) first — that column order, not ["symbol", "timestamp"].',
            )

    def visit_Subscript(self, node: ast.Subscript) -> None:  # noqa: N802
        key = _const(node.slice)
        if key in RATIO_METRICS and not self.num_trades_checked:
            self.add(
                node,
                "PITFALLS §2",
                f"`{key}` read without checking `num_trades`.",
                "These go to inf on a single trade. Report num_trades alongside; below ~30 trades the ratio is noise.",
            )
        self.generic_visit(node)


def check_source(source: str, path: str) -> list[Finding]:
    try:
        tree = ast.parse(source, filename=path)
    except SyntaxError as exc:
        print(f"{path}: could not parse ({exc})", file=sys.stderr)
        return []

    checker = Checker(source)
    checker.scan_context(tree)
    checker.visit(tree)

    findings = list(checker.findings)
    if "hmm_bull_bear" in source:
        line = next(
            (i for i, ln in enumerate(source.splitlines(), 1) if "hmm_bull_bear" in ln),
            0,
        )
        findings.append(
            Finding(
                line,
                "PITFALLS §7",
                "`hmm_bull_bear` batch-fits the whole series — look-ahead. Any backtest using it is fiction.",
                "Post-hoc regime description only. Also: feed returns not price, and states are {1,2} with 2 = bear.",
            )
        )

    return sorted(findings, key=lambda f: (f.line, f.pitfall))


def main(argv: list[str]) -> int:
    paths = argv[1:]
    if not paths:
        print(__doc__.strip(), file=sys.stderr)
        return 2

    total = 0
    for path in paths:
        try:
            with open(path, encoding="utf-8") as handle:
                source = handle.read()
        except OSError as exc:
            print(f"cannot read {path}: {exc}", file=sys.stderr)
            return 2

        findings = check_source(source, path)
        total += len(findings)
        for f in findings:
            print(f"{path}:{f.line}: [{f.pitfall}] {f.message}")
            print(f"    fix: {f.fix}")

    if total == 0:
        print("No known QuantWave anti-patterns found.")
        return 0

    print(f"\n{total} finding(s). See PITFALLS.md — none of these raise at runtime.")
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
