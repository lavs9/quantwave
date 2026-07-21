# ADR: Order Types on the Existing Convention Engine (quantwave-hs8j / quantwave-xfdr)

**Status:** Accepted
**Date:** 2026-07-21

## Context

`quantwave-backtest` already simulates exits realistically: stop-loss,
take-profit and trailing stops are evaluated per bar via
`StopEvaluationMode::OhlcTouched`, which checks the bar's high/low to see whether
a level was touched. When a stop and a target are both touched in the same bar,
the engine already applies the **conservative (pessimistic) convention** — see
`stops.rs`, `evaluate_stops_long`: *"Conservative intrabar ordering: SL /
trailing before TP."* All of this preserves the batch ↔ streaming parity moat,
because every fill is a **deterministic function of completed-bar OHLC + a fixed
rule.**

A comparison vs **QuantJourney-bt** flagged "order-mode execution" as our biggest
gap. On inspection, the gap is **not** that we can't model fills — we already do.
It is that we don't yet expose a few **order *types* as first-class objects**:
**limit entry** orders, **bracket** and **OCO**. QuantJourney also models the
intrabar path more finely; but with only OHLC that path is fundamentally
unknowable, so serious engines (vectorbt, backtrader) resolve it exactly the way
we already do — a documented pessimistic convention. Truly resolving it requires
**lower-timeframe / tick data**, which is a separate, optional capability.

This ADR (superseding an earlier over-engineered two-tier draft) records the
**simple** decision: add the missing order types **on the engine we already have,
under the conventions we already trust** — no second engine, no parity "tiers,"
no parity guard.

**Sources (per AGENTS.md):** `quantjourney-bt/backtester/execution/*` (clean-room
inspiration only); existing `quantwave-backtest/src/stops.rs` (`evaluate_stops`,
`OhlcTouched`, conservative SL-before-TP ordering).

## Decision

### 1. One engine, unchanged in spirit
Keep the existing convention-based, OHLC-touched engine. It is already correct
and already parity-safe. We do **not** build a parallel "high-fidelity" engine
and we do **not** split execution into parity tiers.

### 2. Add order types as first-class objects
Introduce an `Order` abstraction with these types, all evaluated under the same
deterministic OHLC conventions:

| Order type | Fill convention (deterministic, from completed-bar OHLC) |
|---|---|
| **Market** | Fill at next bar's open (respecting `execution_delay`). |
| **Limit** | Fills at the limit price **if** the bar's range touches it (buy: `low ≤ limit`; sell: `high ≥ limit`). |
| **Stop / Stop-limit** | Existing touched-level logic; stop-limit fills at the limit once the stop is touched. |
| **Take-profit** | Existing touched-level logic. |
| **Bracket** (entry + TP + SL) | Legs evaluated with the existing **pessimistic** ordering (SL/trailing before TP) when both touch in one bar. |
| **OCO** (one-cancels-other) | Same pessimistic convention decides which leg wins on a same-bar double-touch; the other is cancelled. |
| **Trailing stop** | Existing once-per-bar ratchet against the bar extreme. |

### 3. Parity is preserved automatically
Because every fill is a pure function of completed-bar OHLC plus a fixed rule,
batch and streaming produce identical results. The existing
`test_batch_vs_streaming_parity_*` discipline is **extended to cover the new
order types** — the single most important test in this epic.

### 4. Explicitly deferred (documented, not built now)
- **Tick / lower-timeframe path replay** — the *only* way to resolve a same-bar
  target-and-stop double-touch *accurately* instead of by convention. Optional,
  data-dependent, future.
- **Futures / contract specs** — `tick_size`, `multiplier`, `margin`. Separate
  capability, future.

### 5. Rust surface (sketch)
```rust
pub enum OrderType {
    Market,
    Limit { price: f64 },
    Stop { level: f64 },
    StopLimit { stop: f64, limit: f64 },
    TakeProfit { level: f64 },
    TrailingStop { offset: f64 },
    Bracket { tp: f64, sl: f64 },
    Oco { a: Box<OrderType>, b: Box<OrderType> },
}
```
Fills reuse the existing `evaluate_stops`-style OHLC-touched machinery and its
conservative ordering — one code path, no new engine.

## Consequences

**Gain:** closes the real QuantJourney gap (first-class limit/bracket/OCO
orders); zero risk to the parity moat (everything stays deterministic); small,
honest scope built on trusted code; natural substrate for a future live bridge.

**Lose / cost:** same-bar target-and-stop double-touches are resolved by
convention, not by the true path (documented, pessimistic — the industry norm);
tick-accurate fidelity and futures remain out of scope until later.

## Risks — what can break or confuse (and mitigations)

1. **Two-engine drift.** The vectorized and streaming paths could compute the new
   order fills differently. → *Mitigation (non-negotiable):* extend the existing
   both-paths parity proptest to every new order type.
2. **Convention mistaken for truth.** Users may think a bracket "knew" which leg
   filled first. → *Mitigation:* document the pessimistic convention prominently;
   surface which convention resolved an ambiguous bar in the trade record.
3. **Intrabar look-ahead.** New order fills must use only completed-bar OHLC,
   never the next bar. → *Mitigation:* property tests that no fill reads `t+1`.
4. **False expectation of tick accuracy.** → *Mitigation:* the docs state plainly
   that same-bar double-touches use a convention and that tick fidelity is a
   deferred, data-dependent option.

## References

- Existing `quantwave-backtest/src/stops.rs` — `evaluate_stops`, `OhlcTouched`, conservative SL-before-TP ordering (the pattern we extend)
- `quantjourney-bt/backtester/execution/*` — clean-room inspiration only
- Epic `quantwave-xfdr`; children `quantwave-9gk7` (order types), `quantwave-pvmr` (risk overlays), `quantwave-bbhb` (Python surface)
