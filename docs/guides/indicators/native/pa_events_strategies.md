# Using Rich PA Events & Metadata for Strategies and ML

<div class="indicator-meta"><span class="category-badge">Price Action</span> <span class="kw-badge">paevent</span> <span class="kw-badge">events</span> <span class="kw-badge">ml-features</span> <span class="kw-badge">position-sizing</span> <span class="kw-badge">backtesting</span></div>

**Date**: 2026-05-31 IST  
**Sources**: Design principles drawn from the full MQL5 lynnchris Price Action series (especially Parts 66/67/69/70 lessons on separating signals from drawing). Core standardization lives in `quantwave-core/src/indicators/market_structure.rs` (`PAEvent`, `PAEventKind`, `extract_pa_events`) with rich structs from `geometric_patterns.rs` and `sr_monitor.rs`.

The single biggest advantage of QuantWave's PA toolkit is that **every detector emits rich, serializable, machine-readable events and structs** instead of chart objects or single scalars. These carry exactly the numbers strategies and ML models need:

- `pole_length_atr` / `height_atr` → dynamic risk and position sizing
- `score`, `symmetry`, `structure_strength` → quality filters
- `breakout_confirmed`, interaction type → clean event triggers
- `source` provenance, `distance_at_event`, feature bags → ML features + attribution

A unified `PAEvent` envelope (with extension points for regime, custom features, etc.) plus direct access to the typed structs (`FlagPattern`, `HsPattern`, `SRInteraction`, `FlipEvent`) gives you one consistent way to consume everything.

## Core Concepts

- **Streaming path** (`Next` impls) → `MarketStructureState`, `FlagPattern`/`HsPattern`, `SRMonitorOutput` (Vec of interactions)
- **Adapter layer** → `extract_pa_events(state)` + `PAEvent::from_...` helpers produce normalized `Vec<PAEvent>` for backtesters
- **Polars path** → Nested Struct columns (`market_structure`, `geometric_patterns`) that you filter/explode on the rich fields
- **Python surface** → Ergonomic wrappers returning dataclass-like results (full fidelity in Polars/Rust)

All paths maintain **exact numerical parity**.

## Rich Fields You Will Actually Use

**From Market Structure / Flips**
- `structure_strength` on `FlipEvent`
- `bias` + `has_current_flip`

**From Flags (most actionable)**
- `pole_length_atr` (primary sizing input)
- `max_retrace_pct`, `pullbacks` vs `pushes`, `consolidation_bars`

**From H&S**
- `height_atr`, `score`, `price_symmetry`, `time_symmetry`

**From S/R Interactions**
- `interaction` type, `strength`, `distance_at_event`, `source` (AutoSwing vs UserProvided)

**Standardized `PAEvent` wrapper** (extensible)
- `bar`, `kind` (MarketStructureFlip today; Flags/H&S/SR planned), `strength`, `size_atr`, `confidence`, `regime_at_event`, `feature_values: Vec<(String, f64)>`

## Practical Strategy Patterns

### 1. Dynamic Sizing from Pattern Metadata (Rust / Python)

```python
def size_from_pole(pole_atr: float, atr: float, account: float = 100_000, risk_pct: float = 0.01, k: float = 0.5):
    risk_distance = k * pole_atr * atr
    if risk_distance <= 0:
        return 0.0
    return (account * risk_pct) / risk_distance

# Usage after a confirmed flag on bullish structure
size = size_from_pole(flag.pole_length_atr, current_atr)
```

### 2. Polars Event Extraction + Feature Join (Research / Training)

```python
df = (
    pl.scan_parquet("bars.parquet")
    .ta.market_structure("high", "low", 3)
    .ta.geometric_patterns("high", "low", 3)
    # ... other indicators
    .collect()
)

events = (
    df.filter(
        pl.col("geometric_patterns").struct.field("flag")
          .struct.field("breakout_confirmed")
    )
    .select([
        "bar",
        pl.col("geometric_patterns").struct.field("flag").struct.field("pole_length_atr"),
        pl.col("market_structure").struct.field("bias"),
        # pull concurrent features at the exact event bar
        pl.col("trendflex_20"),
        pl.col("hurst_100"),
    ])
)
```

### 3. Streaming Event Loop with PAEvent Adapter (Live / Backtester)

```rust
let mut ms = MarketStructure::new(3);
let mut geo = GeometricPatternScanner::new(3);

for (i, (h, l)) in data.iter().enumerate() {
    let state = ms.next((*h, *l));
    let (_, flag, hs) = geo.next((*h, *l));  // or use geo only

    let mut events = extract_pa_events(&state);
    // Extend with geometric events using their rich fields (future PAEventKind variants or manual wrapping)
    if let Some(f) = flag {
        // wrap f into PAEvent or consume directly
    }

    for ev in events {
        // feed your BacktestEngine or live order router
        // ev.size_atr, ev.feature_values etc. already populated or ready for augmentation
    }
}
```

## ML Feature Engineering Recipes

Turn every PA event into a rich training row:

- Categorical: `ms_bias`, `flag_is_bull`, `sr_interaction_type`, `pattern_type`
- Numeric: `pole_length_atr`, `hs_score`, `structure_strength`, `sr_distance_atr`, `touches_last_n`
- Interaction: `flag_on_sr_level`, `bos_during_high_hurst`
- Target alignment: forward return (various horizons) or "did the pattern succeed?" label

Because the same `Next` engine powers both historical Polars feature generation **and** live inference, there is zero train/serve skew.

Example feature vector at a bull-flag breakout event:
`{"pole_atr": 2.7, "retrace": 0.41, "ms_bullish": 1, "hurst_50": 0.72, "regime_trending": 1, "label_1d_fwd": 0.018}`

## Backtester & Production Considerations

- Feed the full struct (or `PAEvent`) into your position sizer — the metadata travels with the signal.
- Use `has_current_flip` / `breakout_confirmed` booleans as sparse event masks.
- Store the originating `bar` for perfect alignment when you later join regime probabilities or other slow features.
- The design deliberately separates "detection" (these detectors) from "drawing/visualization" (your own charting layer or the backtester's plotter).

See the companion notebooks for complete runnable sketches, including a tiny equity curve using pole-based sizing and regime filtering.

## Recommended Reading Order

1. [Market Structure](market_structure.md) — the foundation everything else rests on
2. [Geometric Patterns](geometric_patterns.md) — Flags & H&S with the richest sizing fields
3. [S/R Interactions](sr_monitor.md) — confluence and mean-reversion events
4. This page — how to consume it all uniformly in strategies & ML pipelines
5. Runnable deep-dive: `docs/examples/notebooks/pa_flag_breakout_strategy.md` + `pa_foundation_strategy.py`

## MQL5 Lineage & Philosophy

These tools deliberately follow the spirit of the later articles in the series (Parts 66–70): produce clean, machine-consumable events with rich quantitative metadata first. Visualization and alerts are secondary. QuantWave takes this further by guaranteeing streaming/batch parity and exposing everything through both low-level Rust and high-level Polars/Python surfaces.

All components are covered by property tests. The `PAEvent` system is intentionally forward-compatible — new pattern kinds can be added without breaking existing consumers.

**Related docs**: [Native Indicators index](index.md) • [Gallery](../gallery.md) • [../../../DOCUMENTATION_DECISIONS.md](../../../DOCUMENTATION_DECISIONS.md)

These four pages (plus the strategy notebooks) give developers everything needed to build production-grade, metadata-rich price action systems on top of QuantWave.
