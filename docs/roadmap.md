# Roadmap

QuantWave is evolving rapidly. Our goal is to provide the most comprehensive and high-performance technical analysis library for the Polars ecosystem.

## Current Status (v0.4.0)

- Regime Detection Suite (v0.2–v0.3): HMM, GMM, PELT change-point detection, volatility clustering, conditioned risk metrics — all with native Polars support
- Options India Suite (v0.4.0): Full Black-Scholes Greeks, implied volatility solvers, chain analytics (Max Pain, PCR, GEX, OI Zones), NSE utilities, and Polars expressions
- 150+ technical indicators, including the complete Ehlers Digital Signal Processing suite
- High-performance Polars expressions + Python bindings (`pip install quantwave`)
- Streaming and batch parity via the universal `Next<T>` trait
- Extensive documentation (200+ indicator guides)
- Gold-standard testing and batch/streaming parity validation

## In Progress

- 2D Kinematic Kalman Filter (position + velocity state for lower lag)
- Expanded native Polars expression plugin coverage
- Price Action Structure Foundation (MQL5 lynnchris Part 21): adaptive swings + confirmed BOS flips (`market_structure.rs` delivered with rich `PAEvent` system).
- Geometric PA library (MQL5 Parts 66+69): Flags + H&S detectors (`geometric_patterns.rs`) built on the foundation with rich `FlagPattern`/`HsPattern` (pole_length_atr, score, symmetry, breakout_confirmed). Full streaming + batch parity. S/R monitoring (Part 67) also available via `sr_monitor.rs`.
- Polars + Python surface + notebooks + PA docs (n6e7 closed 2026-06-01 IST): `.ta.market_structure()` / `.ta.geometric_patterns()`, Python streaming wrappers, 4 dedicated professional PA guides (Market Structure, Geometric Patterns, S/R, PA Events), cleaned flagship notebooks, visuals, and gallery/native updates. All major PA documentation professionalization complete (leaks removed, practical usage + rich metadata added, visuals delivered). Remaining bulk indicator page rollout tracked under p1k6.
- Dedicated high-quality user guide pages for the rich PA tools (Market Structure, Geometric Patterns/Flags+H&S, S/R Interactions, PA Events hub): professional pages with rich metadata (pole_length_atr etc. for sizing), 3-surface code, annotated visuals, ML/strategy recipes, and direct MQL5 Part 21/66/67/69 citations. Completed 2026-05-31 IST under documentation epic p1k6 (Phase 1 PA professionalization).

## Next Priorities

- GitHub Releases, topics, README hero, and overall visibility (this P0 epic)
- Example quality, streaming documentation, and visual gallery strategy (DOCUMENTATION_DECISIONS.md): full visual examples strategy + generators (gen_pa_visuals.py, gen_candle_previews.py) + 11+ professional annotated assets for PA + candles. **Indicator Gallery page completely overhauled** (2026-05-31 IST) into a high-value, scannable entry point with strong PA/Ehlers emphasis, correct cross-links, no placeholders, and clear expectations tied to DOCUMENTATION_STANDARDS.md. Related docs (roadmap, native index, contributing) updated together.
- **Candle Pattern Standards Proof batch (p1k6 child, 2026-05-31 IST)**: 8 worst-duplication pages (Doji variants, Harami family, Three Black Crows/White Soldiers, Abandoned Baby) + Engulfing enhancement fully rewritten to DOCUMENTATION_STANDARDS.md template. `gen_candle_previews.py` extended (portable + 8+ new professional generators); 11 annotated PNGs produced. All Nison boilerplate eliminated from batch. Cross-refs + decision record updated. Proves template + visuals tooling scales. See DOCUMENTATION_DECISIONS.md for full record.
- **Ehlers DSP Thin Pages STANDARDS batch (p1k6 Phase 1 batch 2, 2026-05-31 IST)**: 5 high-value thin Ehlers pages (ehlers_filter.md, reflex.md, ehlers_stochastic.md, ehlers_loops.md, ultimatesmoother.md) fully rewritten to Ehlers/scalar "Good" STANDARDS template. `gen_indicator_previews.py` extended (portable OUT, pure-numpy ports of the 5 core logics, CLI, Ehlers DSP styling); 5 new professional PNGs generated with 2026-05-31 IST captions mapping to exact core .rs Next implementations. 3-surface code + parity, Edge Cases, authoritative sources (core paths + Ehlers papers). Cross-refs + full decision record appended. Worktree left clean for merge. See DOCUMENTATION_DECISIONS.md.
- **Python API Reference landing page (quantwave-rbz4, 2026-06-01 IST)**: Fixed the previously empty https://lavs9.github.io/quantwave/api/ page. Rewrote `docs/gen_python_api.py` to emit a professional landing page explaining the three Python surfaces (Polars `.ta`, streaming wrappers, supporting modules) with strong cross-links to the authoritative manual Guides + Gallery. The auto-generated reference remains intentionally lightweight (the rich per-indicator content lives in the hand-written docs). Updated workflow comment, mkdocs nav, and decision record. Claimed + landed under p1k6.
- **Native Indicators landing page (quantwave-shtm, started 2026-06-01 IST)**: Major upgrade to `docs/guides/indicators/native/index.md` into a high-quality, scannable professional landing page (strong intro with parity guarantee, featured high-value areas including PA + Ehlers, clear expectations, links to Standards + Gallery). Matches the quality level of the redesigned gallery.md. Sidebar (SUMMARY.md) structure cleanup remains as follow-up. Task claimed and in progress under p1k6.
- Stronger positioning and competitive differentiation

## Future Horizons

- Machine Learning feature engineering toolkit
- Portfolio-level vectorized backtesting engine
- WASM / browser runtime support
- Additional data provider integrations and real-time bridges
