"""Warmup helpers: NaN-vs-null semantics and frame trimming.

QuantWave indicators emit their warmup region as **NaN**, not null. That is a
deliberate convention (it keeps the Rust/Polars/streaming outputs identical and
avoids a per-indicator validity mask), but it has one sharp edge: the usual
pandas/Polars reflex for cleaning warmup is a silent no-op.

    df = df.with_columns(pl.col("close").ta.rsi(14).alias("rsi"))
    df.drop_nulls()          # <- no-op: null_count is 0, warmup rows survive
    df.drop_nans()           # <- this is the one that actually drops warmup

Worse, NaN comparisons are always ``false``, so ``(pl.col("rsi") < 30)`` yields
``False`` (and ``0.0`` after a cast) across the entire warmup — indistinguishable
from a genuine no-signal period.

This module gives you the alignment-preserving fix:

    import quantwave as qw

    df = df.pipe(qw.trim_warmup, "rsi", ("ema", {"period": 50}))

:func:`trim_warmup` slices off ``max(warmup)`` leading rows across *all* named
indicators, so columns with different warmups stay row-aligned — unlike
``drop_nans()``, which drops rows per-column-set and silently changes what
"row 0" means depending on which columns you happen to be holding.
"""

from __future__ import annotations

import warnings
from typing import Any, Mapping, Sequence, Union

from quantwave._metadata import metadata, warmup_bars

__all__ = [
    "WarmupWarning",
    "warmup_rows",
    "trim_warmup",
    "leading_nan_count",
]


class WarmupWarning(UserWarning):
    """Raised (as a warning) when indicator warmup rows look like they leaked
    into a downstream consumer such as a backtest.

    Silence with ``warnings.filterwarnings("ignore", category=qw.WarmupWarning)``.
    """


# A spec is either an indicator name, a (name, params) pair, an explicit
# integer bar count, or a mapping of {name: params}.
IndicatorSpec = Union[str, int, "Sequence[Any]", "Mapping[str, Any]"]


def _iter_specs(specs):
    """Normalize the accepted spec forms into ``(name_or_None, params_or_bars)``.

    Yields ``(None, int)`` for an explicit bar count and ``(name, params)`` for
    a named indicator.
    """
    for spec in specs:
        if spec is None:
            continue
        if isinstance(spec, bool):
            raise TypeError(f"invalid warmup spec: {spec!r}")
        if isinstance(spec, int):
            if spec < 0:
                raise ValueError(f"explicit warmup bar count must be >= 0, got {spec}")
            yield (None, spec)
            continue
        if isinstance(spec, str):
            yield (spec, None)
            continue
        if isinstance(spec, Mapping):
            for name, params in spec.items():
                yield (name, params)
            continue
        if isinstance(spec, Sequence):
            items = list(spec)
            if len(items) == 2 and isinstance(items[0], str) and (
                items[1] is None or isinstance(items[1], Mapping)
            ):
                yield (items[0], items[1])
                continue
            # Otherwise treat it as a plain collection of specs.
            for sub in _iter_specs(items):
                yield sub
            continue
        raise TypeError(
            f"invalid warmup spec: {spec!r}. Expected an indicator name, "
            '("name", {params}), {"name": {params}}, or an int bar count.'
        )


def warmup_rows(*specs: IndicatorSpec, extra: int = 0, strict: bool = True) -> int:
    """Return the number of leading rows that are warmup for *all* the specs.

    This is ``max(qw.warmup_bars(name, params))`` over every spec, plus ``extra``.
    Taking the max (rather than trimming each column independently) is what
    preserves row alignment across indicators with different warmups.

    Args:
        *specs: One or more indicator specs. Accepted forms, freely mixed:

            * ``"rsi"`` — indicator name, metadata defaults for params
            * ``("rsi", {"period": 21})`` — name plus the params you called it with
            * ``{"rsi": {"period": 21}, "ema": {"period": 50}}`` — mapping form
            * ``30`` — an explicit bar count, for custom/derived columns whose
              warmup QuantWave cannot know
            * a list/tuple of any of the above

        extra: Additional bars to drop on top of the computed maximum. Useful
            for chained transforms (e.g. a diff or shift applied after the
            indicator) that add their own bar of warmup.
        strict: When ``True`` (default), an unrecognized indicator name raises
            ``ValueError`` instead of silently contributing ``0`` bars — a typo
            would otherwise trim nothing and leave warmup in the frame, which is
            exactly the failure mode this helper exists to prevent. Pass
            ``strict=False`` to treat unknown names as ``0``.

    Returns:
        Non-negative row count.

    Example:
        >>> warmup_rows("rsi")
        14
        >>> warmup_rows("rsi", ("ema", {"period": 50}))
        50
        >>> warmup_rows({"rsi": {"period": 21}}, extra=1)
        22
    """
    if extra < 0:
        raise ValueError(f"extra must be >= 0, got {extra}")

    largest = 0
    for name, params in _iter_specs(specs):
        if name is None:
            bars = int(params)
        else:
            if strict and metadata(name) is None:
                raise ValueError(
                    f"unknown indicator {name!r} — qw.warmup_bars() would return 0 "
                    "and nothing would be trimmed. Check the spelling, use an "
                    "explicit integer bar count, or pass strict=False."
                )
            bars = warmup_bars(name, dict(params) if params else None)
        largest = max(largest, int(bars))

    return largest + int(extra)


def trim_warmup(frame, *specs: IndicatorSpec, extra: int = 0, strict: bool = True):
    """Drop the leading warmup rows from a Polars frame, preserving alignment.

    Use this instead of ``drop_nulls()`` (a no-op on QuantWave output — warmup is
    NaN, not null) and instead of ``drop_nans()`` (which trims per-column and
    therefore depends on which columns happen to be present).

    Args:
        frame: A Polars ``DataFrame``, ``LazyFrame`` or ``Series`` — anything with
            a ``.slice()`` method.
        *specs: Indicator specs; see :func:`warmup_rows` for the accepted forms.
        extra: Extra leading rows to drop on top of the computed maximum.
        strict: See :func:`warmup_rows`.

    Returns:
        The same frame type, sliced from ``warmup_rows(*specs, extra=extra)``.

    Example:
        >>> import polars as pl, quantwave as qw
        >>> df = df.with_columns(                        # doctest: +SKIP
        ...     pl.col("close").ta.rsi(14).alias("rsi"),
        ...     pl.col("close").ta.ema(50).alias("ema"),
        ... )
        >>> clean = df.pipe(qw.trim_warmup, "rsi", ("ema", {"period": 50}))  # doctest: +SKIP

        ``clean`` has 50 fewer rows and both ``rsi`` and ``ema`` are finite from
        row 0 onward, still aligned to the same bars.
    """
    n = warmup_rows(*specs, extra=extra, strict=strict)
    if n == 0:
        return frame
    if not hasattr(frame, "slice"):
        raise TypeError(
            f"trim_warmup expects a Polars DataFrame/LazyFrame/Series, got {type(frame).__name__}"
        )

    height = getattr(frame, "height", None)
    if height is None and hasattr(frame, "len") and not hasattr(frame, "collect"):
        try:
            height = frame.len()
        except Exception:  # pragma: no cover - defensive
            height = None
    if height is not None and n >= height:
        warnings.warn(
            f"trim_warmup would drop all {height} row(s): warmup is {n} bars. "
            "The frame is shorter than the indicator warmup — the result is empty.",
            WarmupWarning,
            stacklevel=2,
        )

    return frame.slice(n)


def leading_nan_count(series) -> int:
    """Count leading rows of a Polars Series that are NaN or null.

    Returns ``0`` for a non-float series or an empty series. Used by the ``.bt``
    namespace to detect warmup rows that leaked into a backtest.
    """
    try:
        import polars as pl
    except ImportError:  # pragma: no cover
        return 0

    n = series.len()
    if n == 0:
        return 0

    mask = series.is_null()
    if series.dtype in (pl.Float32, pl.Float64):
        mask = mask | series.is_nan().fill_null(True)

    first = mask[0]
    if first is None or not bool(first):
        return 0

    valid = ~mask
    if not bool(valid.any()):
        return n
    return int(valid.arg_max())
