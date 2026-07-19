"""Price-action pattern detection (harmonic patterns).

Pattern detectors return a variable number of detected-pattern rows rather than
a per-bar series, so they are frame-in / frame-out helpers rather than ``.ta``
expressions.

    import quantwave as qw
    pats = qw.patterns.harmonic(ohlc_df)   # DataFrame, one row per detected pattern

Attribution
-----------
Harmonic patterns (AB=CD, Alternate AB=CD, 5-0) are the work of **Scott M.
Carney** (HarmonicTrader.com). Carney named and defined these patterns and holds
trademarks on "Harmonic Trading" and several pattern names. This detector
implements his published Fibonacci-ratio definitions for interoperability, with
attribution; it reproduces no source text. See ``qw.patterns.HARMONIC_ATTRIBUTION``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Union

if TYPE_CHECKING:
    import polars as pl

HARMONIC_ATTRIBUTION = (
    "Harmonic patterns (AB=CD, Alternate AB=CD, 5-0) are defined by Scott M. "
    "Carney, Harmonic Trading Vols. 1-2 (2010) and HarmonicTrader.com. "
    "'Harmonic Trading' and several pattern names are trademarks of Scott M. "
    "Carney / HarmonicTrader.com."
)


def _pl():
    import polars as pl

    return pl


def _high_low(
    data: "Union[pl.DataFrame, pl.LazyFrame, tuple, list]",
    high_col: str,
    low_col: str,
) -> "tuple[list[float], list[float]]":
    pl = _pl()
    if isinstance(data, pl.LazyFrame):
        data = data.collect()
    if isinstance(data, pl.DataFrame):
        for c in (high_col, low_col):
            if c not in data.columns:
                raise ValueError(f"column {c!r} not in DataFrame")
        return (
            data[high_col].cast(pl.Float64).to_list(),
            data[low_col].cast(pl.Float64).to_list(),
        )
    # (highs, lows) pair of sequences
    if isinstance(data, (tuple, list)) and len(data) == 2:
        highs, lows = data
        return ([float(x) for x in highs], [float(x) for x in lows])
    raise ValueError(
        "data must be an OHLC DataFrame/LazyFrame or a (highs, lows) pair"
    )


def harmonic(
    data: "Union[pl.DataFrame, pl.LazyFrame, tuple, list]",
    *,
    high_col: str = "high",
    low_col: str = "low",
    swing_strength: int = 5,
    ratio_tolerance: float = 0.10,
    min_score: float = 0.5,
    min_size_atr: float = 0.0,
    atr_period: int = 14,
    detect_abcd: bool = True,
    detect_alternate_abcd: bool = True,
    detect_5_0: bool = True,
    detect_xabcd: bool = True,
) -> "pl.DataFrame":
    """Detect harmonic patterns (AB=CD, 5-0, and the XABCD Gartley family).

    Built on the shared MarketStructure swing foundation: confirmed swing pivots
    are tested against Carney's Fibonacci-ratio gates, and a pattern is emitted
    only once its completion pivot ``D`` is a *confirmed* swing (so detection is
    anti-lookahead).

    Args:
        data: OHLC DataFrame/LazyFrame (uses ``high_col``/``low_col``) or a
            ``(highs, lows)`` pair of sequences.
        swing_strength: bar-window radius for swing detection (larger = fewer,
            more significant pivots).
        ratio_tolerance: relative tolerance on the Fibonacci ratios (0.10 = ±10%).
        min_score: minimum ratio-fit score (0-1) to report a pattern.
        min_size_atr: minimum pattern extent in ATR units (0 disables the filter).
        atr_period: ATR period used for ``size_atr`` normalization.
        detect_abcd / detect_alternate_abcd / detect_5_0: enable each family.
        detect_xabcd: enable the XABCD family (Gartley, Bat, Butterfly, Crab,
            Alternate Bat); filter the output by ``kind`` for finer control.

    Returns:
        DataFrame with one row per detected pattern: ``id``, ``kind``
        (``abcd``/``alternate_abcd``/``5-0``/``gartley``/``bat``/``butterfly``/
        ``crab``/``alternate_bat``), ``is_bull``, the pivot bars/prices (``x_*``
        null for AB=CD), ``score``, the measured ratios (``xa_ext``, ``bc_ab``,
        ``cd_ab``, ``cd_bc``, and ``d_xa`` — D's XA ratio, the XABCD defining
        number), the ``prz_low``/``prz_high`` reversal zone, and ``size_atr``.
    """
    from quantwave import _patterns

    highs, lows = _high_low(data, high_col, low_col)
    return _patterns.harmonic(
        highs,
        lows,
        swing_strength,
        ratio_tolerance,
        min_score,
        min_size_atr,
        atr_period,
        detect_abcd,
        detect_alternate_abcd,
        detect_5_0,
        detect_xabcd,
    )
