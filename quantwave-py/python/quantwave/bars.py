"""Alternative bar construction (Renko, Kagi, range bars, Point & Figure).

Bar transforms discard time and produce a different number of rows than the
input, so they are frame-in / frame-out helpers rather than ``.ta`` expressions.

    import quantwave as qw
    bricks = qw.bars.renko(ohlcv_df, box_size=2.0)          # DataFrame[open, close, direction]
    bricks = qw.bars.renko(ohlcv_df, box_size="atr", atr_period=14, multiplier=2.0)
    lines = qw.bars.kagi(ohlcv_df, reversal=2.0)            # DataFrame[open, close, direction, thickness]
    bars = qw.bars.range_bars(ohlcv_df, range_size=2.0)     # DataFrame[open, high, low, close]
    cols = qw.bars.point_figure(ohlcv_df, box_size=1.0)     # DataFrame[top, bottom, direction, boxes]
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Union

if TYPE_CHECKING:
    import polars as pl


def _pl():
    import polars as pl

    return pl


def _prices(data: "Union[pl.DataFrame, pl.LazyFrame, list[float]]", price_col: str) -> list[float]:
    pl = _pl()
    if isinstance(data, pl.LazyFrame):
        data = data.collect()
    if isinstance(data, pl.DataFrame):
        if price_col not in data.columns:
            raise ValueError(f"price column {price_col!r} not in DataFrame")
        return data[price_col].cast(pl.Float64).to_list()
    return [float(x) for x in data]


def renko(
    data: "Union[pl.DataFrame, pl.LazyFrame, list[float]]",
    box_size: "Union[float, str]" = 2.0,
    *,
    price_col: str = "close",
    atr_period: int = 14,
    multiplier: float = 1.0,
) -> "pl.DataFrame":
    """Build Renko bricks from a price series.

    Args:
        data: OHLCV DataFrame/LazyFrame (uses ``price_col``) or a list of prices.
        box_size: a positive float for a fixed box, or ``"atr"`` to derive the
            box from ``multiplier * ATR(atr_period)`` over the series.
        price_col: price column when ``data`` is a frame.
        atr_period / multiplier: used only when ``box_size == "atr"``.

    Returns:
        DataFrame with columns ``open``, ``close``, ``direction`` (+1/-1), one
        row per brick.
    """
    from quantwave import _bars

    prices = _prices(data, price_col)
    if isinstance(box_size, str):
        if box_size != "atr":
            raise ValueError("box_size must be a positive float or 'atr'")
        atr = _atr_value(data, price_col, atr_period)
        return _bars.renko_atr(prices, atr, multiplier)
    return _bars.renko(prices, float(box_size))


def kagi(
    data: "Union[pl.DataFrame, pl.LazyFrame, list[float]]",
    reversal: "Union[float, str]" = 2.0,
    *,
    price_col: str = "close",
    atr_period: int = 14,
    multiplier: float = 1.0,
) -> "pl.DataFrame":
    """Build Kagi lines from a price series.

    A Kagi line keeps extending in its current direction while price advances and
    reverses only when price retraces from the running extreme by at least
    ``reversal`` (a positive float, or ``"atr"`` to derive it from
    ``multiplier * ATR(atr_period)``). Each row is one completed line between two
    turning points; ``thickness`` is the yin/yang state (+1 yang, -1 yin, 0
    undetermined).

    Returns:
        DataFrame with columns ``open``, ``close``, ``direction`` (+1/-1),
        ``thickness`` (+1/-1/0), one row per completed line.
    """
    from quantwave import _bars

    prices = _prices(data, price_col)
    if isinstance(reversal, str):
        if reversal != "atr":
            raise ValueError("reversal must be a positive float or 'atr'")
        atr = _atr_value(data, price_col, atr_period)
        return _bars.kagi_atr(prices, atr, multiplier)
    return _bars.kagi(prices, float(reversal))


def point_figure(
    data: "Union[pl.DataFrame, pl.LazyFrame, list[float]]",
    box_size: "Union[float, str]" = 1.0,
    reversal: int = 3,
    *,
    price_col: str = "close",
    atr_period: int = 14,
    multiplier: float = 1.0,
) -> "pl.DataFrame":
    """Build Point & Figure columns from a price series (close-based).

    Price is reduced to a grid of boxes of height ``box_size``; rising columns of
    X's and falling columns of O's alternate, and a new column starts only when
    price reverses by at least ``reversal`` boxes (the classic N-box reversal,
    default 3).

    Args:
        data: OHLCV DataFrame/LazyFrame (uses ``price_col``) or a list of prices.
        box_size: a positive float for a fixed box, or ``"atr"`` to derive it from
            ``multiplier * ATR(atr_period)``.
        reversal: number of boxes required to start a new (opposite) column.
        price_col: price column when ``data`` is a frame.
        atr_period / multiplier: used only when ``box_size == "atr"``.

    Returns:
        DataFrame with columns ``top``, ``bottom`` (the column's box-level range),
        ``direction`` (+1 X / -1 O), and ``boxes`` (the span in box units), one
        row per completed column.
    """
    from quantwave import _bars

    prices = _prices(data, price_col)
    if isinstance(box_size, str):
        if box_size != "atr":
            raise ValueError("box_size must be a positive float or 'atr'")
        atr = _atr_value(data, price_col, atr_period)
        return _bars.point_figure_atr(prices, atr, multiplier, reversal)
    return _bars.point_figure(prices, float(box_size), reversal)


def range_bars(
    data: "Union[pl.DataFrame, pl.LazyFrame, list[float]]",
    range_size: "Union[float, str]" = 2.0,
    *,
    price_col: str = "close",
    atr_period: int = 14,
    multiplier: float = 1.0,
) -> "pl.DataFrame":
    """Build constant-range OHLC bars from a price series.

    A bar closes as soon as its high-low span reaches ``range_size`` (a positive
    float, or ``"atr"`` to derive it from ``multiplier * ATR(atr_period)``); a new
    bar opens at the triggering price.

    Returns:
        DataFrame with columns ``open``, ``high``, ``low``, ``close`` (one row per
        completed bar).
    """
    from quantwave import _bars

    prices = _prices(data, price_col)
    if isinstance(range_size, str):
        if range_size != "atr":
            raise ValueError("range_size must be a positive float or 'atr'")
        atr = _atr_value(data, price_col, atr_period)
        return _bars.range_bars_atr(prices, atr, multiplier)
    return _bars.range_bars(prices, float(range_size))


def _atr_value(data, price_col: str, period: int) -> float:
    """A single representative ATR over the series (mean true range)."""
    pl = _pl()
    if not isinstance(data, (pl.DataFrame, pl.LazyFrame)):
        raise ValueError("box_size='atr' requires a DataFrame with high/low/close")
    if isinstance(data, pl.LazyFrame):
        data = data.collect()
    for c in ("high", "low", price_col):
        if c not in data.columns:
            raise ValueError(f"box_size='atr' requires column {c!r}")
    tr = data.select(
        pl.max_horizontal(
            pl.col("high") - pl.col("low"),
            (pl.col("high") - pl.col(price_col).shift(1)).abs(),
            (pl.col("low") - pl.col(price_col).shift(1)).abs(),
        ).alias("tr")
    )["tr"]
    atr = tr.tail(max(len(tr) - 1, 1)).mean()
    if atr is None or atr <= 0:
        raise ValueError("ATR is non-positive; provide a numeric box_size")
    return float(atr)
