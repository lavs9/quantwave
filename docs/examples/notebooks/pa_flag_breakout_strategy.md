# Canonical PA Strategy Notebook: Flag Breakout on Confirmed Market Structure

**Bead**: quantwave-5mfc (PA Validation Harness + Canonical Notebook)  
**Date**: 2026-05-30 IST  
**Sources**: MQL5 Price Action series (Part 21 Market Structure / art17891 + Flip_Detector.mq5; Part 66 H&S / art22194 + HS_Indicator.mq5; Part 69 Flags / art22503 + Flag_Pattern_Detector.mq5); closed research quantwave-bfg, quantwave-r46a, quantwave-wtz; implementation in quantwave-iuzv (market_structure.rs) + ej8b skeleton (geometric_patterns.rs) + 5mfc harness (test_utils.rs generators + property tests).

## Strategy Definition (Reference Implementation)
"Enter bull flag breakout **only** after confirmed bullish MarketStructure bias (HH/HL established, flip only after count>=2 per Part 21). Further filter by regime (e.g. HMM trending) + ML feature proxy (e.g. high trendflex). Size position using `pole_length_atr` from the rich FlagPattern event (risk = 0.5 * pole_atr * ATR)."

This matches the exact success criteria in the cu03/5mfc spec: "A developer can build a 'Flag breakout only on confirmed bullish structure, filtered by regime + ML features, sized by pattern metadata' strategy using clean, documented APIs."

## Harness & Validation
- Synthetic generators (in `quantwave-core/src/test_utils.rs`): `generate_clean_bull_flag`, `generate_perfect_bear_hs`, structure cases with ground-truth flips, violation cases (retrace>61.8%, weak pole, etc.).
- Invariants exercised: confirmed flips only post-bias; flag pullbacks > pushes + retrace <=61.8; H&S head dominance + score >=60 via ported ComputePatternScore.
- Parity: streaming Next vs batch replay (proptest + dedicated harness test passes).
- All under `cargo nextest -p quantwave-core market_structure` (new test green).

## Notebook Cells (see .py)
- Load / synthesize data using vectors from the Rust generators (for exact reproducibility + parity with Rust).
- Run simplified Python equivalent of MarketStructure + Flag logic on the synthetic (demonstrates "confirmed bias first").
- Apply filters + compute size from pole_length_atr.
- Show trade log + R:R using rich metadata.
- Real data path (yfinance or CSV) + note on full Polars `.ta.market_structure()` + `.ta.geometric_patterns()` surface (in progress by 5thj).
- When full Rust Python bindings + Polars plugins active: identical results to the streaming Rust (Universal Indicator pattern).

## Deliverables & Links (in workspace)
- Generators + harness: `quantwave-core/src/test_utils.rs` (pa_synthetics section)
- Exercised in: `quantwave-core/src/indicators/market_structure.rs` (new test + rich PAEvent demo)
- Geometric rich structs: `quantwave-core/src/indicators/geometric_patterns.rs` (FlagPattern / HsPattern with pole_length_atr, score, etc.)
- MQL5 sources: `references/MQL5/lynnchris/implemented/Part{21,66,69}/`
- Bead tracking: ONLY via `bd` (see quantwave-5mfc notes for all design decisions, sources, discovered children: ImpulseDetector primitive, ATR-adaptive depth in swings).
- Full verification: `cargo nextest` + `cargo clippy` green on touched code (pre-existing unrelated issues in sr_monitor etc. noted).

This notebook + harness is the reference for all future PA work (backtester gwx, confluence, geometric completion in ej8b, S/R in fb17).

Run the .py with `python -m marimo edit docs/examples/notebooks/pa_flag_breakout_strategy.py` (or view exported HTML).

## Coordination Notes for Parallel Agents
- bmkn (rich standardized PA events): The PAEvent/PAEventKind + extract adapters in market_structure.rs are ready; your final event format can extend it. Harness tests cover serialization.
- 5thj (Polars surface + notebook): Use the synth vectors here for your demo cells; expose .ta.market_structure() and geometric on LazyFrame returning Structs with the rich fields. Polish this notebook.
- fb17 (S/R): Reuse/extend the S/R interaction synthetic stub idea from 5mfc generators for touch/breakout/retest cases.
- ej8b (geometric): Complete the matcher in geometric_patterns using the bfg/r46a logic; then the ground-truth cases in harness will assert the exact invariants (head dominance, pullback>push, ComputePatternScore, etc.). Current skeleton + toy detection is exercised but not yet full-fidelity.
- New child beads spawned (or to be): quantwave- (ATR depth), ImpulseDetector.

**Status of 5mfc**: Major progress — harness core + generators + property tests + passing nextest complete. Notebook started (this + .py). Ready as the verification reference. Pre-existing test noise in module unrelated. No markdown TODOs used; all tracking in bd.