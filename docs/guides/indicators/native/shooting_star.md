# Shooting Star

<div class="indicator-meta"><span class="category-badge">Patterns</span> <span class="kw-badge">pattern</span> <span class="kw-badge">candlestick</span> <span class="kw-badge">classic</span></div>

A bearish reversal pattern at the top of an uptrend.

## Visual Example

![Shooting Star — annotated preview mapping to core implementation](../../../assets/candlestick-previews/shooting_star.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A bearish reversal pattern at the top of an uptrend.

Signals a potential move to the downside.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Recognition Rules (TA-Lib-compatible, `CDLSHOOTINGSTAR` in `quantwave-core/src/indicators/pattern.rs`)**:

1. Stateless candlestick pattern evaluated on OHLC windows.
2. Returns a signed signal (+100 bullish, −100 bearish, 0 none) on the completion bar.
3. Exact threshold geometry (body ratios, gap requirements, shadow lengths) matches the TA-Lib reference implementation wrapped via `talib_cdl!` in `quantwave-core/src/indicators/pattern.rs`.
4. Validate against `quantwave-core/tests/gold_standard/` vectors where present.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CDLSHOOTINGSTAR;
use quantwave_core::traits::Next;

let mut det = CDLSHOOTINGSTAR::new();
for (o, h, l, c) in &ohlcv {
    let sig = det.next((o, h, l, c));
}
```

**Streaming (Python)**

```python
from quantwave import CDLSHOOTINGSTAR

det = CDLSHOOTINGSTAR()
for o, h, l, c in ohlcv:
    sig = det.next((o, h, l, c))
```

**Polars Batch (Python)**

```python
import polars as pl

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("open").ta.cdl_shootingstar("open", "high", "low", "close").alias("shooting_star")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Requires sufficient complete OHLC bars; early bars yield no signal.
- False positives are common in sideways markets — gate with trend or structure filters.
- Pattern semantics follow TA-Lib body/shadow rules; literature variants may differ.
- Signed output (+/−/0) should be consumed as events, not continuous features without encoding.
- Combine with volume expansion or higher-timeframe confirmation for production use.
- No look-ahead bias; signal is known only after the pattern window closes.

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Engulfing](engulfing.md)
- [Market Structure](../price_action/market_structure.md)
- [PA Flag Breakout notebook](../../../examples/notebooks/pa_flag_breakout_strategy.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp

**Implementation**: `quantwave-core/src/indicators/pattern.rs` (`CDLSHOOTINGSTAR` / `CDLSHOOTINGSTAR_METADATA`).
**Pattern reference**: TA-Lib CDL family via `talib_cdl!` in `pattern.rs`. Nison (1991) cited for psychology only — no duplicated boilerplate.
**Parity**: `quantwave-core/tests/gold_standard/cdlshootingstar.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
