import polars as pl

def build_pa_flag_signals(df: pl.LazyFrame) -> pl.LazyFrame:
    return df.with_columns(
        pole_len=(pl.col("high") - pl.col("high").shift(3))
    ).with_columns(
        recent_pole=pl.col("pole_len").rolling_max(window_size=8) > 2.0,
        pole_length_atr=pl.col("pole_len") / 1.5, # mock atr proxy for sizing
        regime_ok=pl.lit(True) # mock regime for cycle 3
    ).with_columns(
        entry=(pl.col("recent_pole") & (pl.col("close") > pl.col("high").shift(1)))
    ).with_columns(
        signal=pl.when(
            pl.col("entry") | pl.col("entry").shift(1) | pl.col("entry").shift(2)
        ).then(1.0).otherwise(0.0)
    )
