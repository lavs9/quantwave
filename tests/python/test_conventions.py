"""Calculation-convention discovery surface (quantwave-xnaf / quantwave-l2n4).

Some indicator names do not determine their formula. These tests pin the
convention notes to the numbers actually produced, and keep the Python table in
sync with the Rust source of truth in
``quantwave-core/src/indicators/conventions.rs``.
"""

from __future__ import annotations

import re
from pathlib import Path

import numpy as np
import polars as pl
import pytest

import quantwave as qw

RUST_CONVENTIONS = (
    Path(__file__).resolve().parents[2]
    / "quantwave-core"
    / "src"
    / "indicators"
    / "conventions.rs"
)


@pytest.fixture(scope="module")
def ohlc():
    rng = np.random.default_rng(7)
    n = 300
    close = 100 + np.cumsum(rng.standard_normal(n))
    high = close + np.abs(rng.standard_normal(n)) * 2
    low = close - np.abs(rng.standard_normal(n)) * 2
    return high, low, close


# --- discovery surface -------------------------------------------------------


def test_conventions_empty_for_unambiguous_indicator():
    assert qw.conventions("rsi") == ()
    assert qw.conventions("not_a_real_indicator_xyz") == ()


def test_atr_convention_is_discoverable():
    notes = qw.conventions("atr")
    assert len(notes) == 1
    note = notes[0]
    assert note.aspect == "smoothing"
    assert "2/(period+1)" in note.convention
    assert "Wilder" in note.differs_from
    assert "ta_atr" in note.guidance
    # AGENTS.md: a source is recorded or its absence is stated -- never assumed.
    assert note.source.strip()
    assert "NONE RECORDED" in note.source


def test_conventions_reachable_from_metadata_and_boundary_info():
    assert qw.metadata("atr").conventions == qw.conventions("atr")
    assert qw.boundary_info("atr").conventions == qw.conventions("atr")
    # Indicators without a divergence keep an empty tuple, not None.
    assert qw.metadata("rsi").conventions == ()
    assert qw.boundary_info("rsi").conventions == ()


def test_convention_slugs_lists_every_affected_indicator():
    slugs = qw.convention_slugs()
    assert slugs == sorted(slugs)
    for expected in ("atr", "atr_ts", "keltner", "roc", "stddev", "supertrend"):
        assert expected in slugs
    for slug in slugs:
        assert qw.metadata(slug) is not None, f"{slug} has a note but no metadata"


def test_alias_resolves_to_canonical_conventions():
    assert qw.conventions("atr_trailing_stop") == qw.conventions("atr_ts")


def test_every_note_is_fully_populated():
    for slug in qw.convention_slugs():
        for note in qw.conventions(slug):
            for fieldname in ("aspect", "convention", "differs_from", "source", "guidance"):
                value = getattr(note, fieldname)
                assert value and value.strip(), f"{slug}.{fieldname} is empty"


# --- the notes must describe reality ----------------------------------------


def test_atr_streaming_class_is_ema_not_wilder(ohlc):
    """The `Atr` class follows the documented EMA convention, not Wilder."""
    high, low, close = ohlc
    period = 14
    streaming = qw.Atr(period)
    got = [streaming.next(h, l, c) for h, l, c in zip(high, low, close)][-1]

    tr = np.maximum.reduce(
        [
            high[1:] - low[1:],
            np.abs(high[1:] - close[:-1]),
            np.abs(low[1:] - close[:-1]),
        ]
    )
    tr = np.concatenate([[high[0] - low[0]], tr])

    def ewm(alpha, seed_first=True):
        out = tr[0]
        for x in tr[1:]:
            out = alpha * x + (1 - alpha) * out
        return out

    ema = ewm(2.0 / (period + 1))
    assert got == pytest.approx(ema, rel=1e-9), "Atr is no longer the EMA variant"

    wilder = ewm(1.0 / period)
    assert got != pytest.approx(wilder, rel=1e-6), "Atr now matches Wilder -- update the note"


def test_ta_atr_is_wilder_and_diverges_from_atr(ohlc):
    """`ta_atr` is Wilder; the gap against `atr` is the thing the note warns about."""
    high, low, close = ohlc
    period = 14
    df = pl.DataFrame({"high": high, "low": low, "close": close})
    wilder = df.select(
        pl.col("high").ta.ta_atr("low", "close", timeperiod=period).alias("v")
    )["v"][-1]

    streaming = qw.Atr(period)
    ema = [streaming.next(h, l, c) for h, l, c in zip(high, low, close)][-1]

    assert wilder != pytest.approx(ema, rel=1e-6)


def test_polars_atr_plugin_and_talib_shim_are_wilder(ohlc):
    """Documented in the note: these two surfaces are ALREADY Wilder."""
    high, low, close = ohlc
    period = 14
    df = pl.DataFrame({"high": high, "low": low, "close": close})
    plugin = df.select(
        pl.col("close").ta.atr("high", "low", timeperiod=period).alias("v")
    )["v"][-1]
    # ta_atr now takes close in the receiver, matching its sibling above
    # (quantwave-sww3); it previously required high in the receiver.
    wilder = df.select(
        pl.col("close").ta.ta_atr("high", "low", timeperiod=period).alias("v")
    )["v"][-1]
    assert plugin == pytest.approx(wilder, rel=1e-12)


def test_stddev_is_population_ddof_zero():
    rng = np.random.default_rng(3)
    close = 100 + np.cumsum(rng.standard_normal(60))
    period = 20
    got = pl.DataFrame({"close": close}).select(
        pl.col("close").ta.stddev(period).alias("v")
    )["v"][-1]
    window = close[-period:]
    assert got == pytest.approx(window.std(ddof=0), rel=1e-9)
    assert got != pytest.approx(window.std(ddof=1), rel=1e-6)


def test_roc_is_scaled_by_100_and_rocp_is_not():
    rng = np.random.default_rng(3)
    close = 100 + np.cumsum(rng.standard_normal(60))
    period = 10
    df = pl.DataFrame({"close": close})
    roc = df.select(pl.col("close").ta.roc(timeperiod=period).alias("v"))["v"][-1]
    rocp = df.select(pl.col("close").ta.rocp(timeperiod=period).alias("v"))["v"][-1]
    assert roc == pytest.approx(rocp * 100.0, rel=1e-9)
    assert rocp == pytest.approx(close[-1] / close[-1 - period] - 1.0, rel=1e-9)


# --- Rust <-> Python drift ---------------------------------------------------


def test_python_table_matches_rust_source_of_truth():
    """The Rust `CONVENTION_NOTES` table is authoritative; Python mirrors it."""
    text = RUST_CONVENTIONS.read_text(encoding="utf-8")
    body = text.split("pub const CONVENTION_NOTES", 1)[1].split("\n];", 1)[0]
    rust_slugs = re.findall(r'slug:\s*"([^"]+)"', body)
    assert rust_slugs, "could not parse CONVENTION_NOTES from conventions.rs"
    assert sorted(rust_slugs) == qw.convention_slugs(), (
        "conventions.rs and _metadata.py::_CONVENTION_NOTES have drifted"
    )
