# QuantWave

**High-performance quantitative finance library**  
Built in Rust · Native Polars support · 150+ indicators · Full Ehlers DSP suite

**Python** `pip install quantwave`  
**Rust** `cargo add quantwave`

[📖 Full Documentation](https://lavs9.github.io/quantwave/)  
[⭐ GitHub](https://github.com/lavs9/quantwave)

---

## Purpose of Our Work

Most quant libraries force you to choose between **speed** and **ease of use**.  
We built QuantWave to give you both.

- **150+ technical indicators** with perfect TA-Lib parity  
- **Complete Ehlers Digital Signal Processing suite** (the most advanced open-source cycle tools)  
- **Zero-copy Polars expressions** that run at Rust speed  
- **Seamless batch + streaming modes**  
- **Future-proof architecture** (Options Greeks, risk metrics, etc. coming soon)

**One library. Research to production. No compromises.**

---

## Quickstart (Python)

```bash
pip install quantwave
```

```python
import polars as pl
from quantwave import ta

df = pl.read_parquet("ohlcv.parquet")

df = df.with_columns(
    ta.rsi("close", 14).alias("rsi"),
    ta.mama("close").alias("mama"),
)
```

[Full examples → Documentation](https://lavs9.github.io/quantwave/examples/batch-streaming/)

## Features

- **Lightning fast** – Rust core with Polars native expressions.
- **Battle-tested** – Every indicator validated against reference implementations.
- **Modern** – Works perfectly in Jupyter, scripts, and live trading systems.
- **MIT licensed** – Free for commercial and personal use.

## Next Steps

- [Full Python Guide](https://lavs9.github.io/quantwave/getting-started/python/)
- [Rust Guide](https://lavs9.github.io/quantwave/getting-started/rust/)
- [Options Greeks & Pricing (roadmap)](https://lavs9.github.io/quantwave/purpose/)

Made with ❤️ for the quant community.
