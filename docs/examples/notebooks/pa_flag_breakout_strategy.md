# Price Action Patterns: Flag Breakout on Confirmed Market Structure (Runnable Notebook)

**Date**: 2026-06-01 IST  
**Sources**: MQL5 Price Action Analysis Toolkit (Part 21 "Market Structure Flip Detector", Part 66 H&S, Part 69 Flags) by lynnchris. Archived `.mq5`: `references/MQL5/lynnchris/implemented/`. Core Rust: `quantwave-core/src/indicators/{market_structure,geometric_patterns}.rs` + synthetic generators in `test_utils.rs`. Visuals via `docs/gen_pa_visuals.py`.

This runnable Marimo notebook is the **canonical executable reference** for QuantWave's Price Action suite. It demonstrates the exact production pattern recommended in the four dedicated guides:

- Build only on **confirmed** MarketStructure bias (structure_count ≥ 2 before any BOS flip or pattern).
- Filter further with regime (HMM) + ML features (e.g. Hurst, Trendflex).
- Size dynamically using `pole_length_atr` (or `height_atr`) from the rich event structs.
- Consume `PAEvent` / `FlagPattern` / `HsPattern` / `SRInteraction` for event-driven logic.

## What the Notebook Covers
- Synthetic data generators that exactly reproduce the invariants tested in Rust proptests (clean bull flag after confirmed bullish structure; perfect bear H&S; violation cases for robustness).
- Step-by-step: Market Structure bias + flip detection → Geometric (Flag / H&S) detection on the same swings → rich metadata extraction → position sizing math → simple trade log with R-multiples.
- Fallback pure-Python implementation (for browser / no-quantwave runs) that mirrors the Rust `Next<T>` logic.
- Notes on wiring the real `quantwave` Polars + streaming surfaces (identical results by design).

## Run It
```bash
python -m marimo edit docs/examples/notebooks/pa_flag_breakout_strategy.py
# or: marimo run ... (after `pip install "quantwave[all]" marimo polars numpy`)
```

## Companion Documentation (Read These First)
The notebook is intentionally **code-first**. For full explanations, field semantics, when-to-use, ML ideas, and annotated visuals, start with the dedicated pages:

- [Market Structure (Swings + Confirmed BOS)](../../guides/indicators/native/market_structure.md)
- [Geometric Patterns (Flags + Head & Shoulders)](../../guides/indicators/native/geometric_patterns.md)
- [S/R Interactions](../../guides/indicators/native/sr_monitor.md)
- [Using Rich PA Events & Metadata for Strategies and ML](../../guides/indicators/native/pa_events_strategies.md)

All four + this notebook + the visual assets in `docs/assets/pa-visuals/` were built to the documentation standards.

## Parity & Fidelity
Every example here is designed to produce identical output to the Rust `Next` implementations and Polars `.ta.*` expressions (validated by the property tests in `quantwave-core/tests/` and the generators in `test_utils.rs`). The synthetic vectors are the single source of truth for the documented behaviors.

## Visuals
The notebook text describes the charts; the high-quality annotated PNGs (bos_flip.png, bull_flag.png, bear_head_shoulders.png, sr_interactions.png) live in the dedicated guides and were generated under the visual strategy in DOCUMENTATION_DECISIONS.md.

---

*This notebook + the four dedicated guides together deliver the complete, professional, leak-free documentation for the PA / geometric features. No internal planning references remain in public content.*