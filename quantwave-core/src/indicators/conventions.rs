//! Machine-readable convention notes for indicators whose name does not fully
//! determine their formula (quantwave-xnaf / quantwave-l2n4).
//!
//! Some indicator slugs are ambiguous in the wild: "ATR" can mean Wilder's RMA
//! (TA-Lib, TradingView, Wilder 1978) or a plain EMA of true range; "stddev" can
//! be population (`ddof=0`) or sample (`ddof=1`); "roc" can be a percentage or a
//! plain ratio. Prose in a docs page is easy to miss, so the convention actually
//! implemented is recorded here as data and surfaced through the Python
//! `metadata()` / `boundary_info()` discovery API.
//!
//! PROCESS RULE (AGENTS.md): the *source* of every calculation must be recorded
//! and never assumed. `source` below is the recorded provenance for the
//! convention that is actually implemented. Where no authoritative source has
//! been found for the implemented variant, `source` says so explicitly rather
//! than naming a plausible-looking one.

/// One convention that a caller could plausibly get wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct ConventionNote {
    /// Indicator slug this note applies to (matches `metadata_registry`).
    pub slug: &'static str,
    /// What aspect of the calculation is ambiguous, e.g. "smoothing", "scaling".
    pub aspect: &'static str,
    /// The convention this implementation actually follows.
    pub convention: &'static str,
    /// Widely used conventions this implementation does *not* follow.
    pub differs_from: &'static str,
    /// Recorded provenance for the implemented convention, or an explicit
    /// statement that no source has been established.
    pub source: &'static str,
    /// What a caller should do about it.
    pub guidance: &'static str,
}

/// Slugs whose implemented convention differs from what the bare name implies.
///
/// Keep this sorted by slug. Mirrored on the Python side in
/// `quantwave/_metadata.py::_CONVENTION_NOTES`.
pub const CONVENTION_NOTES: &[ConventionNote] = &[
    ConventionNote {
        slug: "atr",
        aspect: "smoothing",
        convention: "Exponential moving average of true range, alpha = 2/(period+1), \
                     seeded from the first bar's true range (high - low, no prior close) \
                     with no NaN warmup. Applies to the `Atr` streaming class and the \
                     `atr(period, high, low, close)` native batch function.",
        differs_from: "Wilder's RMA (alpha = 1/period, SMA-seeded over the first `period` \
                       true ranges, NaN during warmup) as used by TA-Lib, TradingView's \
                       Pine `ta.atr`, and pandas `ewm(alpha=1/period, adjust=False)`. \
                       Also differs from a simple rolling mean of true range. At period=14 \
                       the EMA variant typically runs a few tenths of a percent to several \
                       percent away from Wilder on the same input.",
        source: "NONE RECORDED for the EMA-smoothed variant. The metadata `formula_source` \
                 for this indicator (Investopedia ATR) and its LaTeX formula both describe \
                 Wilder's RMA, which is what `ta_atr` implements — not this one. Treated as \
                 an undocumented divergence pending review (quantwave-xnaf).",
        guidance: "For TA-Lib/TradingView-identical ATR use `ta_atr` (Rust `TaATR`, Polars \
                   `.ta.ta_atr()` / `lf.ta().ta_atr()`). Note that the Polars plugin \
                   `pl.col(close).ta.atr(high, low)` and `quantwave.talib.ATR` are ALREADY \
                   Wilder — only the streaming class and native batch function are not.",
    },
    ConventionNote {
        slug: "atr_ts",
        aspect: "smoothing",
        convention: "ATR trailing stop built on the EMA-smoothed `Atr` (alpha = 2/(period+1)).",
        differs_from: "TradingView ATR trailing stops, which use Wilder's RMA.",
        source: "NONE RECORDED for the EMA smoothing. Metadata `formula_source` points at \
                 TradingView's ATR page, which specifies Wilder. Inherited from `atr`.",
        guidance: "Stop distances differ from a TradingView-equivalent ATR trailing stop. \
                   See quantwave-xnaf before using for live risk sizing.",
    },
    ConventionNote {
        slug: "keltner",
        aspect: "smoothing",
        convention: "Channel width uses the EMA-smoothed `Atr` (alpha = 2/(period+1)); \
                     the basis is a plain EMA of close.",
        differs_from: "The Investopedia / Chester Keltner formulation and most charting \
                       platforms, which use Wilder-smoothed ATR for the band width.",
        source: "NONE RECORDED for the EMA smoothing. Inherited from `atr`.",
        guidance: "Band widths differ from a Wilder-ATR Keltner channel.",
    },
    ConventionNote {
        slug: "roc",
        aspect: "scaling",
        convention: "Percentage: (price / price[-period] - 1) * 100, matching TA-Lib `ROC`.",
        differs_from: "pandas `pct_change(period)` and TA-Lib `ROCP`, which return the \
                       plain ratio (no x100).",
        source: "TA-Lib ROC/ROCP definitions.",
        guidance: "Use `rocp` for the unscaled ratio. Mixing the two is a silent 100x error.",
    },
    ConventionNote {
        slug: "stddev",
        aspect: "degrees of freedom",
        convention: "Population standard deviation, ddof = 0, matching TA-Lib `STDDEV`.",
        differs_from: "pandas `Series.std()` and numpy `ndarray.std(ddof=1)` defaults, \
                       which use the sample estimator ddof = 1.",
        source: "TA-Lib STDDEV definition.",
        guidance: "Values differ by a factor of sqrt(N / (N - 1)); rescale if you are \
                   reconciling against pandas.",
    },
    ConventionNote {
        slug: "supertrend",
        aspect: "smoothing",
        convention: "Bands are built on the EMA-smoothed `Atr` (alpha = 2/(period+1)).",
        differs_from: "TradingView's SuperTrend, which uses Pine `ta.atr` — Wilder's RMA.",
        source: "NONE RECORDED for the EMA smoothing. Metadata `formula_source` points at \
                 a TradingView SuperTrend script, which uses Wilder. Inherited from `atr`.",
        guidance: "Flip points can differ from a TradingView SuperTrend on the same inputs.",
    },
    ConventionNote {
        slug: "ttm_squeeze",
        aspect: "smoothing",
        convention: "The Keltner leg of the squeeze test uses the EMA-smoothed `Atr`.",
        differs_from: "Implementations that use Wilder-smoothed ATR for the Keltner leg.",
        source: "NONE RECORDED for the EMA smoothing. Inherited from `atr`.",
        guidance: "Squeeze on/off transitions can differ near the threshold.",
    },
    ConventionNote {
        slug: "vpn",
        aspect: "smoothing",
        convention: "The volume-positive-negative threshold uses the EMA-smoothed `Atr`.",
        differs_from: "Implementations that use Wilder-smoothed ATR for the threshold.",
        source: "NONE RECORDED for the EMA smoothing. Inherited from `atr`.",
        guidance: "Threshold crossings can differ from a Wilder-ATR VPN.",
    },
];

/// Convention notes recorded for `slug`, if any.
///
/// ```
/// use quantwave_core::indicators::conventions::convention_notes;
/// let notes = convention_notes("atr");
/// assert_eq!(notes.len(), 1);
/// assert_eq!(notes[0].aspect, "smoothing");
/// ```
pub fn convention_notes(slug: &str) -> Vec<&'static ConventionNote> {
    CONVENTION_NOTES.iter().filter(|n| n.slug == slug).collect()
}

/// Every slug that carries at least one convention note, sorted and deduplicated.
pub fn slugs_with_conventions() -> Vec<&'static str> {
    let mut slugs: Vec<&'static str> = CONVENTION_NOTES.iter().map(|n| n.slug).collect();
    slugs.sort_unstable();
    slugs.dedup();
    slugs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::metadata_registry::ALL_REGISTERED;

    #[test]
    fn notes_are_sorted_by_slug() {
        let slugs: Vec<&str> = CONVENTION_NOTES.iter().map(|n| n.slug).collect();
        let mut sorted = slugs.clone();
        sorted.sort_unstable();
        assert_eq!(slugs, sorted, "CONVENTION_NOTES must be sorted by slug");
    }

    #[test]
    fn every_note_targets_a_registered_slug() {
        for note in CONVENTION_NOTES {
            assert!(
                ALL_REGISTERED.iter().any(|r| r.slug == note.slug),
                "convention note for unregistered slug {:?}",
                note.slug
            );
        }
    }

    #[test]
    fn every_note_is_fully_populated() {
        for note in CONVENTION_NOTES {
            for (field, value) in [
                ("aspect", note.aspect),
                ("convention", note.convention),
                ("differs_from", note.differs_from),
                ("source", note.source),
                ("guidance", note.guidance),
            ] {
                assert!(
                    !value.trim().is_empty(),
                    "convention note {:?} has empty {field}",
                    note.slug
                );
            }
        }
    }

    #[test]
    fn atr_family_divergence_is_recorded() {
        // The ATR-composed indicators all inherit the EMA smoothing; if one of
        // them is ever switched to Wilder, this test forces the note to be updated.
        for slug in [
            "atr",
            "atr_ts",
            "keltner",
            "supertrend",
            "ttm_squeeze",
            "vpn",
        ] {
            let notes = convention_notes(slug);
            assert_eq!(notes.len(), 1, "expected exactly one note for {slug}");
            assert_eq!(notes[0].aspect, "smoothing");
        }
    }

    #[test]
    fn lookup_of_unknown_slug_is_empty() {
        assert!(convention_notes("rsi").is_empty());
        assert!(convention_notes("not_a_real_slug").is_empty());
    }
}
