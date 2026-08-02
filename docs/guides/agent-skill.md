# Agent Skill

!!! tip "Short answer"
    QuantWave ships an **agent skill** — a packaged set of instructions that teaches coding
    agents (Claude Code, and any tool that reads the agent-skill format) how to use
    QuantWave correctly *and* which mistakes produce wrong-but-plausible numbers.
    Install it by copying `.claude/skills/quantwave/` into your own project.

## Why this exists

Most QuantWave mistakes are silent. `roc` returns percent while `rocp` returns a fraction;
`stddev` is population while pandas is sample; a `signal` of `1` opens **one unit**, not one
unit of capital. None of these raise. An agent writing a strategy from general TA knowledge
will hit them, produce a backtest, and report a Sharpe ratio that means nothing.

The skill front-loads those conventions so the agent gets them right the first time.

## Install

Copy the skill directory from the repository into your project:

```bash
git clone --depth 1 https://github.com/lavs9/quantwave /tmp/quantwave
mkdir -p .claude/skills
cp -r /tmp/quantwave/.claude/skills/quantwave .claude/skills/
```

For availability across all your projects, copy it to `~/.claude/skills/` instead.
Agents discover it automatically — no configuration needed.

## What's inside

| File | Contents |
|------|----------|
| `SKILL.md` | Happy path: install, `.ta` vs plugins vs streaming, discovery, warmup, parity |
| `PITFALLS.md` | Twelve verified silent-wrongness cases, ordered by damage |
| `BACKTEST.md` | `.bt` input requirements, sizing model, and the full output contract |
| `scripts/check_usage.py` | Static linter for the anti-patterns — no dependencies |

## Linting your own code

The linter runs standalone, with no QuantWave import and no third-party dependencies:

```bash
python .claude/skills/quantwave/scripts/check_usage.py strategy.py
```

```text
strategy.py:12: [PITFALLS §1] No `size_multiplier_col`: signal=1 opens 1 unit, not 1 unit of capital.
    fix: Add a Float64 sizing column, e.g. (initial_cash * 0.95 / close).
strategy.py:12: [PITFALLS §6] `execution_delay` defaults to "same_bar" — fills on the signal bar's own close.
    fix: Pass execution_delay="next_bar" for any strategy claiming execution realism.
```

Exit code is `1` when there are findings, `0` when clean — so it drops into CI or a
pre-commit hook directly.

## The pitfalls it covers

1. `signal=1` means one unit, not one unit of capital
2. Ratio metrics (`sharpe`, `sortino`, `profit_factor`) go to `inf` on thin trade counts
3. `roc` is ×100; `rocp` is the fraction
4. `stddev` is ddof=0; pandas `.std()` is ddof=1
5. `ta_*` plugins take `(high, low, close)` — the receiver must be `high`, not `close`
6. `execution_delay` defaults to the optimistic `"same_bar"`
7. `hmm_bull_bear` batch-fits the series — look-ahead
8. Multi-symbol frames must be sorted `["timestamp", "symbol"]`, in that order
9. `drop_nulls()` does not remove warmup — warmup is `NaN`, not `null`
10. Metrics are fractions, not percents; `max_drawdown_pct` is positive
11. Plugins are stateless; streaming instances are stateful and per-symbol
12. Parameter names are not uniform — `timeperiod` vs `period`

Each is reproduced against the current release rather than inferred from the source.

## Related

- [Plugin vs `.ta`](plugin_vs_ta.md) — choosing an integration surface
- [Backtest Output Contract](backtest/output-contract.md) — the authoritative schema
- [llms.txt](../llms.txt) — canonical page index for AI crawlers
