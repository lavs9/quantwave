# SuperTrend

The SuperTrend indicator is a trend-following indicator based on Average True Range (ATR). It is highly effective for identifying trend reversals and setting trailing stop-losses.

## Formula

The SuperTrend is calculated using the following steps:

1.  **Basic Upper Band**:
    $$\text{Basic Upper Band} = \frac{\text{High} + \text{Low}}{2} + (\text{Multiplier} \times \text{ATR})$$
2.  **Basic Lower Band**:
    $$\text{Basic Lower Band} = \frac{\text{High} + \text{Low}}{2} - (\text{Multiplier} \times \text{ATR})$$
3.  **Final Upper Band**:
    $$\text{Final Upper Band} = \begin{cases} \text{Basic Upper Band} & \text{if } \text{Basic Upper Band} < \text{Prev Final Upper Band} \text{ or } \text{Prev Close} > \text{Prev Final Upper Band} \\ \text{Prev Final Upper Band} & \text{otherwise} \end{cases}$$
4.  **Final Lower Band**:
    $$\text{Final Lower Band} = \begin{cases} \text{Basic Lower Band} & \text{if } \text{Basic Lower Band} > \text{Prev Final Lower Band} \text{ or } \text{Prev Close} < \text{Prev Final Lower Band} \\ \text{Prev Final Lower Band} & \text{otherwise} \end{cases}$$

## Parameters

| Parameter  | Default | Description |
|------------|---------|-------------|
| `period`   | 10      | The lookback period for ATR calculation. |
| `multiplier`| 3.0     | The coefficient applied to the ATR. |

## Polars Usage

```python
import polars as pl
import quantwave as qw

df = pl.read_csv("data.csv")
df = df.with_columns([
    pl.col("close").ta.supertrend(period=10, multiplier=3.0).alias("supertrend")
])
```

## Performance Note

QuantWave's SuperTrend implementation is written in Rust and uses SIMD instructions where possible. On a dataset of 1M rows, it typically calculates in under 50ms.

---

*See also: [Indicator Gallery](../gallery.md)*
