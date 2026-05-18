# Cyber Cycle (Ehlers)

The Cyber Cycle is a high-resolution oscillator introduced by John Ehlers that isolates the cyclical component of market price movement while minimizing lag.

## Formula

The calculation involves a symmetrical FIR filter followed by a recursive alpha-smoothing process:

1.  **FIR Smoothing**:
    $$\text{Smooth}_t = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}$$
2.  **Cycle Calculation**:
    $$CC_t = (1 - 0.5\alpha)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2}) + 2(1 - \alpha)CC_{t-1} - (1 - \alpha)^2 CC_{t-2}$$
    where $\alpha = \frac{2}{\text{Length} + 1}$.

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length`  | 14      | The alpha smoothing length parameter. |

## Polars Usage

```python
import polars as pl
import quantwave as qw

df = pl.read_csv("data.csv")
df = df.with_columns([
    pl.col("close").ta.cyber_cycle(length=14).alias("cc")
])
```

## Performance Note

Due to the recursive nature of the Cyber Cycle ($CC_t$ depends on $CC_{t-1}$ and $CC_{t-2}$), this indicator is notoriously slow in Python (Pandas/NumPy). QuantWave's native Rust implementation executes this recursive logic at the hardware level, processing **1 million rows in ~5ms**.

---

*See also: [Indicator Gallery](../gallery.md)*
