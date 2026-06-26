# QuantWave vs Alternatives

One-page comparison for evaluators integrating a technical-analysis + backtest stack on Polars.

| Dimension | **QuantWave** | **TA-Lib** | **pandas-ta** | **vectorbt** | **polars-backtest** |
|-----------|---------------|------------|---------------|--------------|---------------------|
| **Core language** | Rust | C | Python | Python | Rust |
| **Polars native** | Yes (`.ta`, plugins) | No | No (pandas) | Optional | Yes |
| **Indicator count** | 216+ registered, 150+ TA-Lib-class | ~150 | ~130 | Via other libs | Limited TA |
| **Ehlers DSP suite** | 30+ (deep) | Minimal | Some ports | No | No |
| **Price action** | MS, S/R, flags/H&S, confluence | Candle patterns only | Limited | No | No |
| **Regime detection** | HMM, GMM, PELT, vol clustering | No | No | No | No |
| **Batch ↔ streaming parity** | **Guaranteed** (`Next<T>`) | N/A | No | N/A | N/A |
| **Backtest engine** | Built-in (sweep, WFO, cross-sectional) | No | No | **Core product** | Long-format sim |
| **License** | MIT | BSD-like | MIT | **AGPL-3.0** / commercial | Check repo |
| **Live trading bridge** | Planned (Nautilus, deferred) | N/A | N/A | Partial | No |

---

## When QuantWave wins

- You need **one math core** for batch Polars research **and** live streaming without drift.
- You want **Ehlers cycle tools** or **structured price-action** (not just scalar oscillators).
- You are building on **Polars** and want `.ta` + `.bt` in one MIT-licensed stack.
- You care about **validated parity** (`qw.assert_parity`, gold-standard vectors, proptest).

## When alternatives win

- **vectorbt** — portfolio-level vectorized backtest at scale is the only goal; AGPL acceptable.
- **TA-Lib** — minimal dependency, C bindings only, classic indicator set is enough.
- **pandas-ta** — quick pandas notebooks, no streaming/parity requirements.
- **polars-backtest** — simple long-format backtest only, no indicator breadth.

---

## QuantWave surfaces (pick your path)

| Surface | Use case |
|---------|----------|
| `lf.ta().rsi(14)` | Research LazyFrame pipelines |
| `pl.col("close").ta.rsi(14)` | Expression-plugin hot paths |
| `qw.streaming_class("rsi")` | Live bar-by-bar |
| `lf.bt.backtest_with_report()` | Strategy evaluation |

See [Plugin vs `.ta`](guides/plugin_vs_ta.md) and the [capability matrix](guides/backtest/capability_matrix.md).

---

## Verify claims yourself

```bash
./scripts/quantwave_verify.sh
```

```python
import quantwave as qw
qw.assert_parity("rsi", {"period": 14}, your_closes)
```