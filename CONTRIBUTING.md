# Contributing to QuantWave

Thanks for taking the time to contribute. QuantWave is a Polars-native TA/backtesting
library with a single Rust math core (`Next<T>`) that powers batch, expression-plugin,
and streaming surfaces — the biggest rule in this repo is that those three never drift
apart. Everything below exists to protect that invariant.

For the day-to-day dev workflow (build cache, style guidelines) and the full
"Adding a New Indicator" walkthrough, see the canonical doc that ships with the
docs site: **[`docs/contributing.md`](docs/contributing.md)** (published at
[lavs9.github.io/quantwave/reference/contributing](https://lavs9.github.io/quantwave/reference/contributing/)).
This file is the GitHub-surfaced entry point — read it first, then follow the link for
the details.

## Quick links

- Questions, ideas, "is this possible?" → **GitHub Discussions** (link added once
  enabled — see repo README).
- Bugs and numeric-parity mismatches → [open an issue](https://github.com/lavs9/quantwave/issues/new/choose).
- Feature requests → [open an issue](https://github.com/lavs9/quantwave/issues/new/choose).
- Silent footguns that are *not* bugs (documented, intentional behavior) → check
  [`.claude/skills/quantwave/PITFALLS.md`](.claude/skills/quantwave/PITFALLS.md) before filing.

## Dev setup

```bash
git clone https://github.com/lavs9/quantwave
cd quantwave
python -m venv .venv && source .venv/bin/activate
pip install -r requirements-docs.txt
cargo build
```

Install the pre-push quality gate — this runs the same checks CI runs, before you push:

```bash
./scripts/install-git-hooks.sh
```

This symlinks `scripts/git-hooks/pre-push` into `.git/hooks/pre-push`, which invokes
`./scripts/quantwave_verify.sh` (metadata/doc drift checks → `cargo fmt`/`clippy`/
`nextest` → wheel build + smoke test → `pytest`) before every `git push`. Useful knobs:

- `SKIP_PRE_PUSH_VERIFY=1 git push` — bypass once (emergency only)
- `VERIFY_NO_CACHE=1 git push` — force a full rebuild instead of the cached result
- `python3 scripts/verify_cache.py status` — check what's cached
- `./scripts/quantwave_verify.sh --skip-rust --skip-python --skip-wheel` — run a subset
  manually (see the script for all flags)

## Adding or changing an indicator

This is the highest-scrutiny change type in the repo because a wrong indicator is
**silently** wrong — it returns a plausible number, not an error. The checklist:

1. **Core logic** — implement the `Next<T>` trait in `quantwave-core/src/indicators/`.
   This is the single source of truth; batch and streaming both call it.
2. **Metadata** — register the indicator (name, params, category, `warmup_bars`) in
   `metadata.rs`, then regenerate the registry:
   ```bash
   python scripts/regenerate_metadata_registry.py
   ```
3. **Gold fixture** — add a reference-vector fixture under `gold_standard/` (see
   `quantwave-core/tests/gold_standard/` and `quantwave-backtest/tests/gold_standard/`
   for the existing layout) so the indicator has an independently-verified numeric
   answer, not just an internal self-consistency check.
4. **Parity proptest** — add a `proptest!` case asserting streaming output matches the
   TA-Lib oracle (`talib-rs`, test-only dependency) across randomized inputs. See
   `quantwave-core/tests/test_all_talib_parity.rs` for the pattern used for every
   TA-Lib-lineage indicator.
5. **Python parity test** — if the indicator ships a gold JSON fixture, add/extend a
   case in `tests/python/gold_parity_registry.py` so
   `tests/python/test_gold_parity.py::test_streaming_matches_gold_vector` covers it
   (or add it to `GOLD_PARITY_DEFERRED` with a reason — `test_gold_fixture_inventory`
   fails on any fixture that's neither covered nor explicitly deferred).
6. **Docs page** — generate the skeleton, then hand-enrich it:
   ```bash
   python scripts/generate_native_docs.py     # emits into docs/guides/indicators/native/
   python docs/generate_all_previews.py --sync-docs
   ```
   Follow [`docs/DOCUMENTATION_STANDARDS.md`](docs/DOCUMENTATION_STANDARDS.md) for the
   content template, then lint before landing:
   ```bash
   python docs/upgrade_to_standards.py --lint
   python docs/upgrade_to_standards.py --depth-lint
   ```

If the indicator has a footgun worth warning users about (units, ddof, sign
convention, a surface where the same slug means different math) — add it to
[`.claude/skills/quantwave/PITFALLS.md`](.claude/skills/quantwave/PITFALLS.md), not
just the docs page. That file is what the project's agent skill and PR reviewers both
read first.

## Commit conventions

This repo follows [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>
```

Types actually in use in this history: `feat`, `fix`, `docs`, `style`, `chore`, `test`,
`refactor`, `perf`, `ci`. Scope is usually a crate, module, or subsystem
(`indicators`, `backtest`, `ci`, `getting-started`, `clippy`, …). Examples straight
from `git log`:

```
fix(clippy): avoid drain-into-same-type in geometric_patterns::promote_pending_poles
style: fix rustfmt violations in recently-added backtest/hmm test files
docs(getting-started): linear zero-to-RSI path with verified output blocks
fix(indicators): hmm_bull_bear raises on price-scale input instead of silent constant Bull
feat(backtest): robustness tearsheet report (DSR, PSR, Monte Carlo MDD, slippage breakeven)
```

Keep the summary in the imperative mood and under ~72 characters; put "why", not just
"what", in the body when the change isn't self-explanatory.

## Style guidelines

- Rust: idiomatic, `cargo clippy -- -D warnings` clean, no unnecessary allocations on
  hot paths.
- Public functions carry docstrings.
- Match existing parameter-naming convention exactly — TA-Lib-lineage indicators use
  `timeperiod`, native ones use `period`. Don't guess; check `qw.metadata("<slug>")`
  or the sibling indicators in the same file.

## Release process

Before tagging, promote the changelog: run
`python3 scripts/release_changelog.py --apply X.Y.Z`, which renames
`## [Unreleased]` to `## [X.Y.Z] - <today>` and leaves a fresh empty
`[Unreleased]` above it, then commit `docs/changelog.md`. `docs/changelog.md`
follows [Keep a Changelog 1.1](https://keepachangelog.com/en/1.1.0/): only the
`Added` / `Changed` / `Deprecated` / `Removed` / `Fixed` / `Security`
subheadings, categorized by what the entry actually describes, and dates that
match the release tag exactly. `scripts/check_changelog.py` (run in CI's
"Doc & metadata sanity" job) enforces this structure and cross-checks each
version's date against its `vX.Y.Z` git tag.

Releases are cut by pushing a `v*` tag, which triggers
[`.github/workflows/release.yml`](.github/workflows/release.yml): a gate step
first runs `scripts/release_changelog.py --check`, which fails the release if
`docs/changelog.md` doesn't already have a `## [X.Y.Z] - date` section for the
pushed tag with `[Unreleased]` empty (i.e. you forgot the `--apply` step
above). Rust crates then publish to crates.io first (core → backtest/plugins
→ polars → quantwave), then Python wheels build and smoke-test across
platforms/versions, then publish to PyPI via OIDC. See
[`.github/workflows/README.md`](.github/workflows/README.md) for the full CI/release
pipeline diagram, and `scripts/check_release_invariants.py` for the pre-release
invariant checks. Release notes live under [`docs/releases/`](docs/releases/) and the
running [`docs/changelog.md`](docs/changelog.md).

## Reporting issues

Use the GitHub issue tracker — pick the **Bug report** template for a broken build/API,
the **Parity mismatch** section of that same template if QuantWave returns a
plausibly-wrong number (include the reproducing input series and expected vs. actual
output), or the **Feature request** template for anything else. See
[`.github/ISSUE_TEMPLATE/`](.github/ISSUE_TEMPLATE/).
