# ADR: Nautilus Trader Live Bridge (quantwave-cr6v.16 / quantwave-53tj)

**Status:** Accepted (stub only — no runtime integration)  
**Date:** 2026-06-13

## Context

QuantWave's `quantwave-backtest` crate targets **vectorized research** with batch/streaming parity. [Nautilus Trader](https://github.com/nautechsystems/nautilus_trader) is a production event-driven engine (LGPL-3.0).

`planning/BACKTEST_ENGINE_RESEARCH.md` §2.5 rules Nautilus **out of scope** for the permissively-licensed core due to copyleft.

## Decision

1. **Do not embed** Nautilus Trader inside `quantwave-backtest` or `quantwave-python`.
2. Define a **clean-room `LiveBridge` trait** and `LiveSignalEvent` export type in `quantwave-backtest/src/live_bridge.rs`.
3. Implement **`RecordingLiveBridge`** for tests/notebooks (in-memory event log).
4. A future **`quantwave-nautilus`** adapter crate (separate repo or workspace member) may implement `LiveBridge` by translating `LiveSignalEvent` → Nautilus order commands. That crate would be **optional** and **LGPL-isolated**.

## Consequences

- Research users stay on MIT `quantwave-backtest` without copyleft contamination.
- Live trading requires an explicit opt-in adapter package and license review.
- `StrategySignal` / `Bar` from the streaming path are the canonical export shapes.

## References

- `quantwave-backtest/src/live_bridge.rs`
- Nautilus Trader: https://github.com/nautechsystems/nautilus_trader (LGPL-3.0)
- sigc complement (MIT): cross-sectional panels via `cross_sectional.rs`, not Nautilus