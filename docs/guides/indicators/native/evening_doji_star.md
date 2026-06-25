# Evening Doji Star

<div class="indicator-meta"><span class="category-badge">Patterns</span> <span class="kw-badge">pattern</span> <span class="kw-badge">candlestick</span> <span class="kw-badge">classic</span></div>

A bearish reversal pattern involving a doji.

## Visual Example

![Evening Doji Star — annotated preview mapping to core implementation](../../../assets/candlestick-previews/evening_doji_star.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A bearish reversal pattern involving a doji.

A highly reliable signal of a market top.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Recognition Rules (TA-Lib-compatible, `CDLEVENINGDOJISTAR` in `quantwave-core/src/indicators/pattern.rs`)**:

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
use quantwave_core::indicators::CDLEVENINGDOJISTAR;
use quantwave_core::traits::Next;

let mut det = CDLEVENINGDOJISTAR::new();
for (o, h, l, c) in &ohlcv {
    let sig = det.next((o, h, l, c));
}
```

**Streaming (Python)**

```python
from quantwave import CDLEVENINGDOJISTAR

det = CDLEVENINGDOJISTAR()
for o, h, l, c in ohlcv:
    sig = det.next((o, h, l, c))
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_evening_doji_star(series: pl.Series) -> pl.Series:
    ind = qw.CDLEVENINGDOJISTAR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_evening_doji_star, return_dtype=pl.Float64).alias("evening_doji_star")
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

**Implementation**: `quantwave-core/src/indicators/pattern.rs` (`CDLEVENINGDOJISTAR` / `CDLEVENINGDOJISTAR_METADATA`).
**Pattern reference**: TA-Lib CDL family via `talib_cdl!` in `pattern.rs`. Nison (1991) cited for psychology only — no duplicated boilerplate.
**Parity**: `quantwave-core/tests/gold_standard/cdleveningdojistar.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
