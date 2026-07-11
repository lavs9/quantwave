# Inverted Hammer

<div class="indicator-meta"><span class="category-badge">Patterns</span> <span class="kw-badge">pattern</span> <span class="kw-badge">candlestick</span> <span class="kw-badge">classic</span></div>

A bullish reversal pattern at the bottom of a downtrend.

## Visual Example

![Inverted Hammer — annotated preview mapping to core implementation](../../../assets/candlestick-previews/inverted_hammer.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A bullish reversal pattern at the bottom of a downtrend.

Signals a potential move to the upside.

QuantWave evaluates this pattern on completed OHLC windows using TA-Lib-aligned geometry rules. Output is an event signal (+100 bullish, −100 bearish, 0 none) — ideal for rule-based strategies and encoded ML features.

**Typical applications:**

- Scan for completed pattern windows — never act on partial formations
- Combine with [Market Structure](../market_structure/) or trend filters in production
- Encode signed output (+/−/0) before ML training
- Expect false positives in choppy ranges; require volume or HTF confirmation

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Recognition Rules (TA-Lib-compatible, `CDLINVERTEDHAMMER` in `quantwave-core/src/indicators/pattern.rs`)**:

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
use quantwave_core::indicators::CDLINVERTEDHAMMER;
use quantwave_core::traits::Next;

let mut det = CDLINVERTEDHAMMER::new();
for (o, h, l, c) in &ohlcv {
    let sig = det.next((o, h, l, c));
}
```

**Streaming (Python)**

```python
from quantwave import CDLINVERTEDHAMMER

det = CDLINVERTEDHAMMER()
for o, h, l, c in ohlcv:
    sig = det.next((o, h, l, c))
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_inverted_hammer(series: pl.Series) -> pl.Series:
    ind = qw.CDLINVERTEDHAMMER(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_inverted_hammer, return_dtype=pl.Float64).alias("inverted_hammer")
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

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Engulfing](engulfing.md)
- [Market Structure](market_structure.md)
- [PA Flag Breakout notebook](../../../examples/notebooks/pa_flag_breakout_strategy.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp

**Implementation**: `quantwave-core/src/indicators/pattern.rs` (`CDLINVERTEDHAMMER` / `CDLINVERTEDHAMMER_METADATA`).
**Pattern reference**: TA-Lib CDL family via `talib_cdl!` in `pattern.rs`. Nison (1991) cited for psychology only — no duplicated boilerplate.
**Parity**: `quantwave-core/tests/gold_standard/cdlinvertedhammer.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
