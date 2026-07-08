# QuantWave: The Honest, Brutal Review

**Date:** 2026-07-07 (IST)
**Scope:** Full repo audit (Rust workspace, Python packaging/API, docs source), live site review (https://lavs9.github.io/quantwave/), GitHub landing page, and a competitive landscape scan (July 2026).

---

**One-paragraph verdict:** The Rust core is genuinely good — well-designed traits, 674 tests, 157 streaming-vs-batch proptests, zero `unsafe`, real gold-standard parity against TA-Lib. But the project's public face is writing checks the repo can't cash: the benchmarks page is partly fabricated (and the generator script's own comments admit it), the headline indicator count is a different number on nearly every page, the shipped PyPI wheel has a packaging bug that likely breaks every Python version except 3.12, and CI runs zero Rust builds or tests. You've built an institutional-grade engine and wrapped it in marketing that a skeptical quant will falsify in about ten minutes. The good news: almost all of the credibility damage is cheap to fix.

---

## ✅ The Good

- **The `Next<T>` trait architecture is the real deal.** Minimal, idiomatic, with a clean blanket impl — and the batch/streaming parity story is actually *enforced*: ~140 indicator files each carry a streaming-vs-batch proptest. This is the moat, and it's real.
- **Test discipline in Rust is excellent.** 674 `#[test]` functions, 27 gold-standard JSON fixtures, dedicated `test_all_talib_parity.rs` (136KB of parity validation), 22 proptest regression files, plus cross-language Python↔Rust parity tests. Very few hobby projects do this.
- **Code quality in the core is high.** ~1:1 doc-comment-to-public-item ratio, zero `unsafe` in 66K LOC, zero `panic!` in core, proper `thiserror` error enums. `gaussian_hmm.rs` (835 lines with Hamilton 1989 / Zucchini citations) is standout work.
- **Release engineering is mature.** Ordered crates.io publishing, genuine 4-platform wheel builds (Linux x64/arm64, macOS, Windows), conventional commits with ticket traceability across 283 commits.
- **The flagship docs pages (RSI, MACD) are among the best indicator reference pages in the entire OSS TA space** — formula, citation, three runnable code paths, edge cases, gold-vector provenance. The docs *information architecture* (learning paths, gallery vs. catalog, llms.txt) is more sophisticated than most established projects.
- **You have a genuine market position nobody else occupies:** the only MIT-licensed, Polars-native, full-breadth TA + backtest stack. Yvictor's polars-backtest is PolyForm Noncommercial; vectorbt is Commons-Clause; the other Polars backtesters have 0–11 stars. The Options India stack is unique, full stop.
- The **memory benchmarks are real** (actual `estimated_size()` vs pandas `memory_usage(deep=True)` measurements) — the one performance claim that survives scrutiny.

## ⚠️ The Bad

- **CI has no Rust quality gate.** `ci.yml` runs only doc/metadata Python scripts on PRs; `cargo test`/`clippy`/`fmt` live entirely in a local, opt-in, bypassable pre-push hook (commit `8a5a2d35a` did this deliberately). A fork PR that doesn't compile could merge to main. Also missing: cargo-audit/deny, MSRV pin, benchmark regression tracking.
- **The Python layer is smoke-tested, not validated.** Across `tests/python/` and `quantwave-plugins/tests/`, exactly *one* test asserts numeric values (SMA). Everything else is "did it throw?" with `print()` statements. All numerical rigor is Rust-side — fine internally, but Python users are trusting a binding layer nobody numerically checks.
- **The Python API surface is undesigned.** No `.pyi` stubs anywhere (zero IDE autocomplete for the `.ta` accessor), the namespace is built by runtime reflection with broad `try/except: pass`, string-casing heuristics, and a hardcoded list of ~20 "recently-wired" names. Internal bead IDs (`quantwave-1x2z`, etc.) leak into shipped docstrings.
- **Two parallel FFI stacks** (uniffi in `quantwave-python`, PyO3 in plugins/backtest-py) merged by a custom wheel-merging script — double the interop surface, and the direct cause of the packaging bug below. `quantwave-backtest-py`'s lib path points into a *sibling crate's* src tree.
- **The docs long tail is templated filler.** Flagship pages are great; pages like `dmh.md` and `s_r_interaction_monitor_part_67.md` repeat their header blurb as the "Description", carry copy-pasted boilerplate bullets, and at least one has broken LaTeX delimiters so the math doesn't render. Your own `upgrade_to_standards.py` names "Description duplication" and "Low word count" as lint categories — the problem is known and half-patched.
- **Broken links, confirmed in the live site:** `faq`, `comparison`, and the native catalog ship literal unresolved `href="../getting-started/index.md"`-style links (the extra-`../` bug), with ~150+ more candidates in the bulk-generated cross-links. `/reference/` and `/backtest/` 404.
- **`quantwave-backtest` has panics in production paths** — ~20 unwraps in `walk_forward.rs` alone outside test code, on the FFI-adjacent crate where a panic kills the host Python process.
- **The options Polars wrappers betray the pitch:** `quantwave/polars.py` runs Python list comprehensions inside `map_batches` (not vectorized Rust) with zero docstrings on ~15 methods. `talib.py` maps only 35 of the 221 indicators to TA-Lib names despite "TA-Lib replacement" positioning.
- One gold-standard test (`griffiths_spectrum`) was **commented out** when its fixture went missing, rather than regenerated.
- Getting-started funnel has **no sample dataset and never shows expected output** — a new user has no "it worked" checkpoint. Realistic time-to-first-success exceeds the promised 10 minutes for anyone not already fluent in Polars.

## 🔥 The Ugly

These are the credibility killers, ranked by severity:

1. **The shipping wheel is broken for most Python versions.** The published 0.6.0 wheel is tagged `py3-none-<platform>` (claims universal Python) but bundles `*.cpython-312-darwin.so` extensions. pip will happily install it on 3.9/3.10/3.11/3.13 — and imports will then fail everywhere except CPython 3.12. The uniffi wheel's universal tag survives `build_unified_wheel.py`'s merge while PyO3's 3.12-only `.so`s get bolted on. The classifiers claim 3.9–3.12 support; in practice one version works.

2. **The benchmarks page is substantially fabricated.** Hardware specs are literal `[Placeholder: e.g., Apple M2 Pro]` text, live right now. The page claims numbers come from `docs/gen_benchmarks.py` "for transparency" — that script never imports quantwave and benchmarks `pl.rolling_mean` as a proxy. The pandas comparisons (`>200ms*`, `>500ms*`) are asterisked guesses, not measurements. Worst: the "Streaming Latency" table is the batch throughput numbers **relabeled from milliseconds to nanoseconds** — and `scripts/update_benchmarks_metrics.py`'s own comments admit it ("*we can't run precision nano-benchmarks… we use the previous Rust results as a base*"). Meanwhile the one honest benchmark in the repo (`benchmark_results.md`) shows quantwave streaming SMA **4× slower** than talib-rs batch. Every "27×/100× faster" claim on the homepage, gallery, and comparison pages rests on this.

3. **The indicator count is a different number almost everywhere it appears:** 217 (docs home, gallery, comparison), 218 (README line 39, FAQ, mkdocs description), 221 (README line 5, catalog, llms.txt, GitHub description, metadata_export.json), **"150+" on the actual PyPI landing page** (a stale README most first-time users see first), "~500+" in getting-started, and `quantwave.indicators()` at runtime returns **563** — polluted with 47 `*Result` types that the code's own comments say should be filtered. For a library whose entire pitch is numerical correctness, the most basic number about it isn't under version control.

4. **`GMM::fit()` is a silent no-op.** A public, exported method with underscore-ignored params and a `// TODO: Implement EM algorithm` body. No error, no `unimplemented!()` — callers get an unfitted model that reports success. "Full Regime Detection Suite (HMM, GMM, …)" is on the homepage.

5. **Repo hygiene contradicts the engineering discipline.** Tracked in git: 9 scratch/fix Python scripts, 3 empty `clippy_errors*.txt`, captured `nextest_output.txt`, a **5MB Tesseract OCR model (`eng.traineddata`)** referenced by nothing, 31MB of compiled wheels in `dist/`, and 33MB of reference PDFs — `.git` is 541MB. Any experienced reviewer who clones the repo sees this before they see the good code.

---

## 🏆 Major Wins (all of them, ranked by leverage)

### Tier 0 — credibility wins

These aren't features, but they're the highest-ROI wins available because right now they actively negate everything else:

1. **Fix the wheel tag / go abi3.** Either adopt PyO3's `abi3-py39` stable ABI (one wheel, all versions — the clean fix) or build per-version wheels with correct `cp3X` tags. This is a currently-shipping defect affecting most users.
2. **Real benchmark harness as a product.** Delete every unmeasured number today. Then build a committed, CI-run benchmark suite (criterion + Python side) comparing quantwave vs TA-Lib, polars-talib, pandas-ta, talipp — and now **Wickra** — publishing auto-generated results with hardware metadata into the docs. The streaming-parity claim is being commoditized by Wickra (514 indicators, O(1)/tick, WASM, 7 language bindings, launched May 2026, claiming the exact same positioning); benchmarked proof is how you defend it.
3. **Single source of truth for the indicator count.** Generate it from `metadata_export.json` in CI, template it into README/docs/pyproject/llms.txt, fail the build on drift (extend the drift-check scripts that already exist), and push the corrected README to PyPI. Also fix `quantwave.indicators()` to apply the `*Result` filter.
4. **Restore the CI quality gate.** `cargo nextest + clippy + fmt` on every PR (path-filtered per crate to stay fast), plus cargo-audit and an MSRV pin. Keep the pre-push hook as a bonus, not the gate.
5. **Implement or delete `GMM::fit()`.** Either ship real EM (the machinery already exists in gaussian_hmm) or make it return `Err`/`unimplemented!()` and pull GMM from the marketing until it exists.

### Tier 1 — feature wins (from the competitive scan)

6. **TA-Lib Abstract-API-equivalent introspection registry** — `get_functions()`, `get_function_groups()`, per-indicator input/output schema and parameter defaults, exposed in Python and Rust. `IndicatorMetadata` consts and a 221-entry metadata export already exist; this is mostly plumbing, and it's what lets screeners, no-code UIs, and migration tools adopt QuantWave as a *true* TA-Lib replacement.
7. **Bulk "compute everything" API** (`df.ta.all()` / pandas-ta's `strategy("all")` equivalent), built on #6 with sane defaults — the single biggest ergonomic gap vs pandas-ta and the fastest path to ML feature-matrix generation. Pairs naturally with generated `.pyi` stubs from the same metadata (fixes the zero-autocomplete problem in one stroke).
8. **Multi-timeframe resampling helpers** — compute daily regime/indicators, broadcast onto intraday bars. There is zero resample/timeframe code in the repo today; it's a `group_by_dynamic` wrapper, cheap to ship, and it's in every practitioner's workflow.
9. **Data layer: bundled sample dataset + connectors.** A packaged sample OHLCV parquet (instantly fixes the getting-started funnel) plus fetch/cache helpers — and for the unique Options India stack, NSE bhavcopy/option-chain loaders. This is the #1 friction in a new user's first 10 minutes; vectorbt PRO treats it as core.
10. **Portfolio optimization engine** (mean-variance, risk parity, HRP) — already on the capability matrix as "not implemented," and it completes the shared-capital portfolio story the backtester already half-tells.
11. **functime-style bulk ML-feature namespace** — a `.ts`/`.features` accessor doing panel-scale feature extraction, differentiated by what only QuantWave has: regime probabilities and Ehlers DSP outputs as first-class features. functime's 1.2k stars prove the demand.
12. **QuantStats-compatible tear-sheet output** — HTML tear sheets already exist; emitting QuantStats-compatible returns series plugs into the existing ecosystem instead of competing with it.
13. **WASM target** — the Rust core makes this nearly free, Wickra already ships it, and it enables the killer marketing asset: a live in-browser indicator playground embedded in the docs.
14. **Alternative bar types + advanced patterns** — Renko/Kagi/Point-&-Figure construction and harmonic patterns (Gartley/Bat/Butterfly). Wickra lists these as families; the geometric-patterns foundation to match already exists.
15. **India broker execution bridge** (Kite/Upstox/Fyers) or the deferred Nautilus bridge — the natural completion of the Options India analytics story, and the ADR already exists. Biggest lift on this list; do it last.

## 🔩 Minor Wins (top 10)

1. **Show expected output in every getting-started code block** and use the bundled sample dataset — give users a "green checkmark" moment.
2. **CI link checker + mkdocs strict mode** — fixes the ~150+ extra-`../` broken links class permanently; swap meta-refresh stubs for the mkdocs redirects plugin; fix the `/reference/` and `/backtest/` 404s.
3. **Repo cleanup:** delete scratch/fix scripts (or move under gitignored `scratch/`), remove `eng.traineddata`, `dist/` wheels, clippy/nextest logs; consider `git filter-repo` on the 541MB history (coordinate before rewriting history).
4. **Expand `talib.py`'s `_TALIB_MAP` from 35 names to full coverage** — generate it from the metadata registry.
5. **Docstrings + real vectorization for `quantwave/polars.py`** options wrappers (native expressions instead of Python `map_batches`).
6. **De-panic the backtest crate's production paths** — convert `walk_forward.rs`/`sweep.rs` unwraps to `BacktestError` propagation, especially anything reachable from the Python FFI.
7. **Strip internal bead IDs from shipped docstrings/comments** and regenerate the fabricated-feeling boilerplate bullets on long-tail indicator pages (or honestly mark them as auto-generated).
8. **Restore the disabled `griffiths_spectrum` gold-standard test** by regenerating the fixture.
9. **Community surface:** enable GitHub Discussions, add CONTRIBUTING.md, link both from docs nav — 5 stars and 0 issues reads as "unused"; give visitors somewhere to land.
10. **Unify on one FFI stack** (realistically: PyO3 everywhere, dropping uniffi) — eliminates the wheel-merging script that caused the tag bug and halves the binding maintenance surface. (Borderline major; scoped as a refactor rather than a feature.)

---

## Appendix: Evidence Index

| Finding | Evidence |
|---|---|
| Wheel tag bug | PyPI 0.6.0 wheel: `Tag: py3-none-macosx_11_0_arm64` containing `quantwave/_backtest.cpython-312-darwin.so`; no `abi3` feature in any PyO3 Cargo.toml; CI builds only with Python 3.12 |
| Fabricated benchmarks | `docs/benchmarks.md` `[Placeholder]` hardware; `docs/gen_benchmarks.py` (14 lines, never imports quantwave); `scripts/update_benchmarks_metrics.py` ms→ns relabeling with confessing comments |
| Honest benchmark that loses | `benchmark_results.md`: SMA(14) talib-rs batch 1519µs vs quantwave streaming 6016µs on 1M rows |
| Indicator count chaos | 217 / 218 / 221 / "150+" (PyPI) / "~500+" (docs) / 563 (runtime `quantwave.indicators()`) |
| GMM no-op | `quantwave-core/src/regimes/gmm.rs` — `pub fn fit(&mut self, _data: …)` with empty TODO body |
| No CI quality gate | `.github/workflows/ci.yml` (doc scripts only); commit `8a5a2d35a ci: move full quality gate to local pre-push hook` |
| Repo cruft tracked in git | `git ls-files` confirms scratch_*.py, fix_*.py, clippy_errors*.txt, nextest_output.txt, `eng.traineddata` (5MB), `dist/*.whl` (31MB); `.git` = 541MB |
| Python test thinness | `tests/python/` = 100 lines, 1 value assertion (SMA); plugins tests are throw-checks only |
| Broken live links | `site/faq/index.html` ships `href="../getting-started/index.md"`; same class in comparison + native catalog; `/reference/`, `/backtest/` 404 |
| Backtest panics | `quantwave-backtest/src/walk_forward.rs` ~20 production-path unwraps (lines 88–306); 111 unwraps crate-wide |
| Docs boilerplate | `docs/guides/indicators/native/dmh.md`, `s_r_interaction_monitor_part_67.md` (duplicated descriptions, broken LaTeX); `docs/upgrade_to_standards.py` lints for "Description duplication" |
| Wickra threat | github.com/wickra-lib/wickra — May 2026, 514 indicators, O(1)/tick, WASM + 7 language bindings, "drop-in TA-Lib replacement" |
| Market moat | Only MIT-licensed Polars-native backtester (Yvictor's is PolyForm Noncommercial; vectorbt is Commons-Clause) |

---

## Tracking (beads epics, created 2026-07-08)

Every finding and win above is now a detailed beads issue (problem evidence + solution design + TDD acceptance criteria), ready for handover:

| Epic | ID | Children |
|---|---|---|
| [UGLY] Credibility killers | `quantwave-9gek` | .1 wheel tag bug (P0) · .2 fabricated benchmarks (P0) · .3 indicator count (P1) · .4 GMM no-op (P1) · .5 repo hygiene (P1) |
| [BAD] Engineering debt | `quantwave-5ipk` | .1 CI quality gate · .2 Python numerical suite · .3 API surface + .pyi stubs · .4 de-panic backtest · .5 docs links/boilerplate · .6 options vectorization · .7 TA-Lib map · .8 griffiths fixture · .9 bead-ID sweep · .10 FFI unification |
| [GOOD] Guard the strengths | `quantwave-ruh0` | .1 parity meta-check · .2 validation docs page · .3 workspace lints · .4 release invariants |
| [WINS-MAJOR] New capabilities | `quantwave-p2k0` | .1 introspection registry · .2 df.ta.all() · .3 MTF helpers · .4 data layer · .5 portfolio optimization · .6 ML features · .7 QuantStats interop · .8 WASM playground · .9 bar types + harmonics · .10 broker bridge |
| [WINS-MINOR] Quick wins | `quantwave-zr74` | .1 getting-started UX · .2 community surface · .3 changelog hygiene · .4 manifest nits · .5 GitHub landing polish |

Key dependencies: `p2k0.6 → p2k0.2 → p2k0.1` (ML features ← bulk API ← registry); `5ipk.10 → {5ipk.2, 9gek.1}` (FFI unification after Python test suite + wheel hotfix); `ruh0.3 → {5ipk.1, 5ipk.4}` (lints after CI gate + de-panic); `ruh0.4 → 9gek.1`; `ruh0.2 → {ruh0.1, 9gek.3}`.

Suggested attack order: `9gek.*` (credibility, ~2 weeks) → `5ipk.1/.2` (CI + Python tests) → `p2k0.1/.2/.3` (registry, bulk API, MTF) → rest by priority.

---

**The uncomfortable summary:** the engineering is better than the credibility, and that's backwards from most projects. A quant evaluating QuantWave will find the placeholder benchmarks, the shifting indicator count, and the broken wheel before they ever find the 157 parity proptests. Tier 0 is maybe two weeks of work and it changes the entire first impression; the feature wins only compound after that foundation is honest.
