# Instantaneous Trendline (Ehlers)

The Instantaneous Trendline is an adaptive trend-following tool that automatically adjusts to the current dominant cycle period of the market. It provides a "zero-lag" trend estimate by removing the cyclical component identified via Hilbert Transform phasors.

## Formula

The Trendline is a complex multi-stage calculation:
1.  **Hilbert Transform**: Identifies the market's instantaneous period ($DCPeriod$).
2.  **SMA Smoothing**: A Simple Moving Average with a dynamic period equal to $DCPeriod$.
3.  **WMA Smoothing**: A 4-bar Weighted Moving Average applied to the output.

$$\text{Trendline} = \text{WMA}(\text{SMA}(Price, DCPeriod), 4)$$

## Parameters

This indicator is fully adaptive and typically requires no user-defined lookback parameters, as it self-tunes to the market's internal rhythm.

## Polars Usage

```python
import polars as pl
import quantwave as qw

df = pl.read_csv("data.csv")
df = df.with_columns([
    pl.col("close").ta.instantaneous_trendline().alias("itrend")
])
```

## Performance Note

The Instantaneous Trendline is one of the most computationally expensive indicators in the suite, involving trigonometric functions and dynamic windowing. While a Python implementation would struggle to maintain real-time performance on large datasets, QuantWave processes **1 million rows in ~74ms**.

---

*See also: [Indicator Gallery](../gallery.md)*
