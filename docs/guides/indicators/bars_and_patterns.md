# Bar Types & Patterns

Beyond per-bar indicators, QuantWave ships **frame-in / frame-out** transforms
that reshape a price series into alternative *bar types* or detect multi-pivot
*chart patterns*. Because they change the row count, they are plain functions
returning a fresh Polars `DataFrame` — not `.ta` expressions.

Both families share the project's design rules: every result is a **rich,
machine-readable struct** (kept separate from any charting), and every builder
has a stateful streaming form that is **bit-identical** to the batch form.

---

## Alternative bar types (`qw.bars`)

These discard time and re-sample price by movement. Each accepts an OHLCV
`DataFrame`/`LazyFrame` (using a `price_col`, default `close`) or a raw list of
prices, and each supports an ATR-derived threshold (`"atr"`).

| Function | Bar rule | Columns | Source |
|---|---|---|---|
| `qw.bars.renko` | Fixed box `ΔP`; a new brick when close leaves the band | `open, close, direction` | Drozda, Cavojsky, Sebes (2024), *On Suitability of Renko Charts for Algorithmic Trading*, Def. 1 |
| `qw.bars.range_bars` | Bar closes when high−low span reaches `range_size` | `open, high, low, close` | Constant-range (constant-range bar) construction |
| `qw.bars.kagi` | Line reverses when price retraces `reversal` from the extreme | `open, close, direction, thickness` | Bogomolov (2013) reversal construction + Nison yin/yang |

```python
import quantwave as qw

df = qw.datasets.synthetic(seed=3, rows=300)

bricks = qw.bars.renko(df, box_size=2.0)              # or box_size="atr"
bars   = qw.bars.range_bars(df, range_size=3.0)
lines  = qw.bars.kagi(df, reversal=2.0)               # thickness: +1 yang / -1 yin / 0
```

The Kagi `thickness` column is the classic **yin/yang** signal: `+1` (yang) once
price rises above the prior shoulder, `-1` (yin) once it falls below the prior
waist — the machine-readable form of Kagi's line-weight convention.

---

## Harmonic patterns (`qw.patterns.harmonic`)

Harmonic patterns are Fibonacci-ratio-constrained price structures. The detector
is built on the shared **MarketStructure** swing foundation: confirmed swing
pivots are tested against each pattern's ratio gates, and a pattern is reported
only once its completion pivot `D` is a *confirmed* swing. Detection therefore
uses no information from beyond `D` — it is **anti-lookahead** by construction
(the reported `d_bar` never sits after the bar at which the pattern is emitted).

### Attribution

> Harmonic Trading is the work of **Scott M. Carney** (HarmonicTrader.com).
> Carney named and defined these patterns; **"Harmonic Trading" and several
> pattern names (including the 5-0) are trademarks of Scott M. Carney /
> HarmonicTrader.com.** QuantWave implements his published Fibonacci-ratio
> definitions for interoperability, with attribution, and reproduces no source
> text. The string is also available at `qw.patterns.HARMONIC_ATTRIBUTION`.

Ratio definitions follow:

- **AB=CD** and **Alternate AB=CD** — Carney, *Harmonic Trading: Volume One*
  (2010), Ch. 4 "The AB=CD Pattern", and [harmonictrader.com › AB=CD](https://harmonictrader.com/harmonic-patterns/abcd-pattern/)
  / [Alternate ABCD](https://harmonictrader.com/harmonic-patterns/alternate-abcd-pattern/).
- **Gartley, Bat, Butterfly, Crab** — Carney, *Harmonic Trading: Volume One*
  (2010), Ch. 5–8.
- **5-0** and **Alternate Bat** — Carney, *Harmonic Trading: Volume Two* (2010),
  Ch. 3 "New Harmonic Patterns" (5-0 at pp. 78–79), and [harmonictrader.com › 5-0](https://harmonictrader.com/harmonic-patterns/5-0/).

### Patterns and ratios

**Four-point patterns** — C retraces AB by 0.382–0.886:

| Pattern (`kind`) | Ratio gate |
|---|---|
| `abcd` | **CD = AB** (equal length) |
| `alternate_abcd` | **CD = 1.27 × AB or 1.618 × AB** |

**Five-point patterns** (X, A, B, C, D) — the AB=CD reciprocal table (C
retracement `{0.382, 0.50, 0.618, 0.707, 0.786, 0.886}` pairs with the BC
projection `{2.618/2.24, 2.0, 1.618, 1.41, 1.27, 1.13}`) underlies the
completion. The **XABCD "Gartley family"** is distinguished by `d_xa` — D's
retracement/extension of the XA leg, each pattern's *defining number*:

| Pattern (`kind`) | B (of XA) | BC projection | **D (of XA) = `d_xa`** |
|---|---|---|---|
| `gartley` | 0.618 | ≤ 1.618 | **0.786** |
| `bat` | < 0.618 (~0.50) | 1.618–2.618 | **0.886** |
| `butterfly` | 0.786 | ≥ 1.618 | **1.27** |
| `crab` | ≤ 0.618 | 2.618–3.618 | **1.618** |
| `alternate_bat` | 0.382 | extreme | **1.13** |
| `5-0` | 1.13–1.618 *(extension)* | — | 50% of BC + reciprocal AB=CD |

All XABCD patterns take C as a 0.382–0.886 retracement of AB. Enable/disable the
family with `detect_xabcd`, or filter the output by `kind`.

### Usage

```python
import polars as pl
import quantwave as qw

df = qw.datasets.synthetic(seed=7, rows=500)          # needs high/low columns

pats = qw.patterns.harmonic(
    df,
    swing_strength=5,        # swing-pivot window (larger → fewer, bigger pivots)
    ratio_tolerance=0.10,    # ±10% on the Fibonacci ratios
    min_score=0.5,           # ratio-fit quality threshold (0-1)
)

# Only bullish 5-0 setups, most textbook-exact first
setups = pats.filter(
    (pl.col("kind") == "5-0") & pl.col("is_bull")
).sort("score", descending=True)
```

### Output schema

One row per detected pattern:

| Column | Meaning |
|---|---|
| `id`, `kind`, `is_bull` | sequence id, pattern type, buy (`true`) vs sell setup |
| `x_bar` … `d_bar`, `x_price` … `d_price` | pivot bars and prices (`x_*` is null for AB=CD) |
| `score` | ratio-fit quality in `[0, 1]` (1.0 = textbook-exact) |
| `xa_ext`, `bc_ab`, `cd_ab`, `cd_bc` | the measured leg ratios |
| `prz_low`, `prz_high` | Potential Reversal Zone (projected completion band) |
| `size_atr` | pattern vertical extent in ATR units (for sizing / filtering) |

Pass a `(highs, lows)` tuple instead of a frame if you already have the arrays.
Toggle families with `detect_abcd` / `detect_alternate_abcd` / `detect_5_0`.

---

## See also

- [Indicator Gallery](gallery.md) · [Full Catalog](native/index.md)
- Market Structure and Geometric Patterns (Flags, Head & Shoulders) build on the
  same swing foundation.
