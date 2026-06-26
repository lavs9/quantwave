//! ML Feature Engineering Polars layer (ta.features.*)
//!
//! Wires the rich Rust feature extractors from `quantwave_core::features` into
//! the .ta. namespace on LazyFrame, following the exact patterns from
//! quantwave-polars/src/lib.rs (UDF map closures + StructChunked::from_series
//! for rich multi-outputs + with_columns for lazy exprs).
//!
//! This delivers the **minimal locked surface** required for the cross-epic
//! deliverable (ML Features → Realistic Backtest with Rich Metadata) that
//! closes quantwave-4ps + quantwave-gwx.
//!
//! The canonical executable demonstration + parity verification is the notebook:
//! docs/examples/notebooks/ml_feature_backtest_parity.py
//! (uses this surface in documented Rust batch path + equivalent Python streaming generators
//!  + FeatureToSignal adapter + full rich metadata preservation in trades).
//!
//! LOCKED SURFACE (per quantwave-4ps notes, "DETAILED WLX SURFACE REQUIRED..." section, 2026-05-31 IST):
//! 1. .ta.features.hurst(period) -> column "hurst_{period}" (f64 persistence)
//! 2. .ta.features.cyber_cycle(length) -> Struct column "cyber_cycle" with fields [cycle, trigger, momentum, signal]
//! 3. .ta.features.griffiths_dominant_cycle(lower, upper, length) -> column "griffiths_dc" (f64)
//! 4. .ta.features.regime_features() -> column "regime_label" (u32, from HMM bull_bear for MVP usability)
//! 5. .ta.features.instantaneous_trendline() -> Struct "itl" {trend, strength}
//! 6. .ta.features.regime_probs() -> Struct "regime_probs" {prob_bull, prob_bear, prob_steady, prob_crisis, prob_other}
//! 7. .ta.features.trendflex(length) -> column "trendflex_{length}"
//! 8. .ta.features.ehlers_autocorrelation(length, num_lags) -> Struct {dominant_lag, max_correlation}
//!
//! All are lazy (exprs built with with_columns + map UDFs; execution deferred to collect).
//! All delegate directly to the Next<T> wrappers in quantwave-core (zero lookahead by construction).
//! No build_matrix yet (per instructions; kept minimal).
//!
//! Sources recorded (per AGENTS.md + 4ps spec):
//! - quantwave-core/src/features/hurst.rs (HurstFeatureExtractor + HurstFeatures; wraps indicators/hurst.rs)
//! - quantwave-core/src/features/cyber_cycle.rs (CyberCycleFeatureExtractor + CyberCycleFeatures; primary source indicators/cyber_cycle.rs:35 per Ehlers "Cybernetic Analysis...")
//! - quantwave-core/src/features/griffiths_dominant_cycle.rs (GriffithsDominantCycleFeatureExtractor + ...Features; wraps indicators/griffiths_dominant_cycle.rs)
//! - quantwave-core/src/features/regime.rs + regimes/hmm.rs (regime_to_features + HMM::bull_bear for label; MarketRegime)
//! - quantwave-core/src/features/mod.rs (wlx prep note 2026-05-30 + AsFeatures skeleton + proptest parity contract)
//! - quantwave-4ps epic (parent) + wlx child design notes (this surface is the exact contract for the "smoking gun" notebook)
//! - Existing .ta. patterns in quantwave-polars/src/lib.rs (macd/bbands/supertrend/gap_momentum struct returns, adosc etc. stateful maps, regimes_conditioned_metrics)
//! - gw7s notebook (docs/examples/notebooks/ml_feature_stability.py) + quantwave-4ub research (P0 feature list)
//! - quantwave-backtest (future consumer of the metadata columns from these exprs)
//!
//! Decision: CyberCycle uses Struct (matches all rich outputs in this crate on Polars 0.46; users .unnest("cyber_cycle") if needed). Regime uses simple but real HMM label (usable in MVP notebook/backtester filters) rather than pure placeholder.

use polars::prelude::*;
use quantwave_core::features::{self as rust_features};
use quantwave_core::traits::Next;

// Bring parent crate type into scope for the inherent impl that extends the .ta. namespace.
use crate::QuantWaveNamespace;

/// Sub-namespace returned by .ta().features().
/// Methods here implement the exact locked surface for the 4ps/gwx cross-epic deliverable.
pub struct TaFeaturesNamespace<'a>(pub(crate) &'a LazyFrame);

impl<'a> QuantWaveNamespace<'a> {
    /// Entry point for the ML features namespace.
    /// Usage: df.lazy().ta().features().hurst(20) etc.
    pub fn features(self) -> TaFeaturesNamespace<'a> {
        TaFeaturesNamespace(self.0)
    }
}

impl<'a> TaFeaturesNamespace<'a> {
    /// Hurst persistence feature (plus internal regime label in the core extractor).
    /// Output column: "hurst_{period}" (f64).
    ///
    /// Delegates to quantwave_core::features::HurstFeatureExtractor (Next<f64, Output=HurstFeatures>).
    pub fn hurst(self, period: usize) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor = rust_features::HurstFeatureExtractor::new(period);
                    let ca: &Float64Chunked = s.f64()?;
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(extractor.next(val).persistence);
                    }
                    Ok(Some(Column::from(Series::new(
                        format!("hurst_{}", period).into(),
                        values,
                    ))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias(&format!("hurst_{}", period))])
    }

    /// Cyber Cycle rich features (cycle + trigger + derived momentum + signal).
    /// Returns Struct column named "cyber_cycle" with fields:
    ///   cycle, trigger, momentum, signal (all f64).
    ///
    /// Delegates to quantwave_core::features::CyberCycleFeatureExtractor.
    /// Struct return matches project convention for multi-output (see macd, bbands, supertrend etc in lib.rs).
    pub fn cyber_cycle(self, length: usize) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor = rust_features::CyberCycleFeatureExtractor::new(length);
                    let ca: &Float64Chunked = s.f64()?;
                    let mut cycles = Vec::with_capacity(s.len());
                    let mut triggers = Vec::with_capacity(s.len());
                    let mut momenta = Vec::with_capacity(s.len());
                    let mut signals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let f = extractor.next(val);
                        cycles.push(f.cycle);
                        triggers.push(f.trigger);
                        momenta.push(f.cycle_momentum);
                        signals.push(f.trigger_signal);
                    }

                    let s_cycle = Series::new("cycle".into(), cycles);
                    let s_trigger = Series::new("trigger".into(), triggers);
                    let s_mom = Series::new("momentum".into(), momenta);
                    let s_sig = Series::new("signal".into(), signals);

                    let struct_series = StructChunked::from_series(
                        "cyber_cycle_result".into(),
                        s.len(),
                        [s_cycle, s_trigger, s_mom, s_sig].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("cycle".into(), DataType::Float64),
                    Field::new("trigger".into(), DataType::Float64),
                    Field::new("momentum".into(), DataType::Float64),
                    Field::new("signal".into(), DataType::Float64),
                ])),
            )
            .alias("cyber_cycle")])
    }

    /// Griffiths Dominant Cycle estimate (high-value stationary cycle feature).
    /// Output column: "griffiths_dc" (f64) — name fixed per locked 4ps deliverable spec (params not encoded in col name).
    ///
    /// Delegates to quantwave_core::features::GriffithsDominantCycleFeatureExtractor.
    pub fn griffiths_dominant_cycle(self, lower: usize, upper: usize, length: usize) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor =
                        rust_features::GriffithsDominantCycleFeatureExtractor::new(lower, upper, length);
                    let ca: &Float64Chunked = s.f64()?;
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(extractor.next(val).dominant_cycle);
                    }
                    Ok(Some(Column::from(Series::new("griffiths_dc".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("griffiths_dc")])
    }

    /// Basic regime label feature (usable for filters/sizing in backtester + MVP notebook).
    /// Output column: "regime_label" (u32).
    ///
    /// For this minimal surface we compute a real label using the HMM bull_bear detector
    /// on close (consistent with existing regime exprs in lib.rs). Simple label satisfies
    /// the locked 4ps deliverable spec; richer probs/one-hot can layer on later.
    ///
    /// Delegates to quantwave_core::regimes::hmm::HMM + MarketRegime (see also regime.rs helpers).
    pub fn regime_features(self) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut hmm = quantwave_core::regimes::hmm::HMM::bull_bear();
                    let ca = s.f64()?;
                    let mut labels = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let regime = if val.is_nan() {
                            quantwave_core::regimes::MarketRegime::Steady
                        } else {
                            hmm.next(val)
                        };
                        let label: u32 = match regime {
                            quantwave_core::regimes::MarketRegime::Bull => 1,
                            quantwave_core::regimes::MarketRegime::Bear => 2,
                            quantwave_core::regimes::MarketRegime::Crisis => 3,
                            quantwave_core::regimes::MarketRegime::Steady => 0,
                            quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + (c as u32),
                        };
                        labels.push(label);
                    }
                    Ok(Some(Column::from(Series::new("regime_label".into(), labels))))
                },
                GetOutput::from_type(DataType::UInt32),
            )
            .alias("regime_label")])
    }

    /// Instantaneous Trendline (Ehlers) with derived trend-strength feature.
    /// Returns Struct column "itl" with fields: trend, strength (f64).
    pub fn instantaneous_trendline(self) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor = rust_features::InstantaneousTrendlineFeatureExtractor::new();
                    let ca: &Float64Chunked = s.f64()?;
                    let mut trends = Vec::with_capacity(s.len());
                    let mut strengths = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let f = extractor.next(val);
                        trends.push(f.trend);
                        strengths.push(f.strength);
                    }
                    let struct_series = StructChunked::from_series(
                        "itl_result".into(),
                        s.len(),
                        [
                            Series::new("trend".into(), trends),
                            Series::new("strength".into(), strengths),
                        ]
                        .iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("trend".into(), DataType::Float64),
                    Field::new("strength".into(), DataType::Float64),
                ])),
            )
            .alias("itl")])
    }

    /// HMM soft regime probabilities (bull/bear forward probs from Viterbi deltas).
    /// Returns Struct column "regime_probs" with prob_bull, prob_bear, prob_steady, prob_crisis, prob_other.
    pub fn regime_probs(self) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor = rust_features::RegimeProbFeatureExtractor::bull_bear();
                    let ca = s.f64()?;
                    let mut bull = Vec::with_capacity(s.len());
                    let mut bear = Vec::with_capacity(s.len());
                    let mut steady = Vec::with_capacity(s.len());
                    let mut crisis = Vec::with_capacity(s.len());
                    let mut other = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let f = extractor.next(val);
                        bull.push(f.probs[0]);
                        bear.push(f.probs[1]);
                        crisis.push(f.probs[2]);
                        steady.push(f.probs[3]);
                        other.push(f.probs[4]);
                    }
                    let struct_series = StructChunked::from_series(
                        "regime_probs_result".into(),
                        s.len(),
                        [
                            Series::new("prob_bull".into(), bull),
                            Series::new("prob_bear".into(), bear),
                            Series::new("prob_steady".into(), steady),
                            Series::new("prob_crisis".into(), crisis),
                            Series::new("prob_other".into(), other),
                        ]
                        .iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("prob_bull".into(), DataType::Float64),
                    Field::new("prob_bear".into(), DataType::Float64),
                    Field::new("prob_steady".into(), DataType::Float64),
                    Field::new("prob_crisis".into(), DataType::Float64),
                    Field::new("prob_other".into(), DataType::Float64),
                ])),
            )
            .alias("regime_probs")])
    }

    /// Trendflex zero-lag trend component.
    /// Output column: "trendflex_{length}" (f64).
    pub fn trendflex(self, length: usize) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor = rust_features::TrendflexFeatureExtractor::new(length);
                    let ca: &Float64Chunked = s.f64()?;
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(extractor.next(val).trendflex);
                    }
                    Ok(Some(Column::from(Series::new(
                        format!("trendflex_{}", length).into(),
                        values,
                    ))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias(&format!("trendflex_{}", length))])
    }

    /// Ehlers Autocorrelation summary features (dominant lag + max correlation).
    /// Returns Struct column "ehlers_autocorr" with dominant_lag (u32), max_correlation (f64).
    pub fn ehlers_autocorrelation(self, length: usize, num_lags: usize) -> LazyFrame {
        self.0.clone().with_columns([col("close")
            .map(
                move |s| {
                    let mut extractor =
                        rust_features::EhlersAutocorrelationFeatureExtractor::new(length, num_lags);
                    let ca: &Float64Chunked = s.f64()?;
                    let mut lags = Vec::with_capacity(s.len());
                    let mut max_corrs = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let f = extractor.next(val);
                        lags.push(f.dominant_lag as u32);
                        max_corrs.push(f.max_correlation);
                    }
                    let struct_series = StructChunked::from_series(
                        "ehlers_autocorr_result".into(),
                        s.len(),
                        [
                            Series::new("dominant_lag".into(), lags),
                            Series::new("max_correlation".into(), max_corrs),
                        ]
                        .iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("dominant_lag".into(), DataType::UInt32),
                    Field::new("max_correlation".into(), DataType::Float64),
                ])),
            )
            .alias("ehlers_autocorr")])
    }
}

// The struct is pub so it is reachable as quantwave_polars::features::TaFeaturesNamespace if needed for turbofish/docs.
// No additional re-export required here; the .ta().features() chaining works via the impl on QuantWaveNamespace
// (the mod features; declaration in lib.rs ensures the impl is linked).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QuantWaveExt; // brings .ta() extension method into scope for the smoke test

    /// Smoke test for the exact minimal locked .ta.features.* surface (quantwave-4ps wlx slice).
    /// Exercises all four methods on a tiny close series; verifies column names, dtypes, and basic collect.
    /// (Full numeric parity + proptests live in quantwave-core/tests/ per project rules.)
    #[test]
    fn smoke_ta_features_surface() -> PolarsResult<()> {
        // Small oscillatory + trending price series (enough to warm extractors with period ~5-14)
        let prices: Vec<f64> = (0..40)
            .map(|i| 100.0 + 3.0 * (i as f64 * 0.4).sin() + (i as f64) * 0.1)
            .collect();

        let df = df!["close" => prices]?;
        let lf = df.lazy();

        // 1. hurst
        let out = lf
            .clone()
            .ta()
            .features()
            .hurst(8)
            .collect()?;
        assert!(out.column("hurst_8").is_ok());
        assert_eq!(out.column("hurst_8")?.dtype(), &DataType::Float64);

        // 2. cyber_cycle -> struct
        let out = out
            .lazy()
            .ta()
            .features()
            .cyber_cycle(12)
            .collect()?;
        let cc = out.column("cyber_cycle")?;
        assert_eq!(cc.dtype().clone(), DataType::Struct(vec![
            Field::new("cycle".into(), DataType::Float64),
            Field::new("trigger".into(), DataType::Float64),
            Field::new("momentum".into(), DataType::Float64),
            Field::new("signal".into(), DataType::Float64),
        ]));
        let ca = cc.struct_()?;
        assert!(ca.field_by_name("cycle".into())?.f64()?.get(39).is_some());

        // 3. griffiths_dominant_cycle -> "griffiths_dc"
        let out = out
            .lazy()
            .ta()
            .features()
            .griffiths_dominant_cycle(6, 40, 25)
            .collect()?;
        assert!(out.column("griffiths_dc").is_ok());
        assert_eq!(out.column("griffiths_dc")?.dtype(), &DataType::Float64);

        // 4. regime_features -> "regime_label"
        let out = out
            .lazy()
            .ta()
            .features()
            .regime_features()
            .collect()?;
        assert!(out.column("regime_label").is_ok());
        assert_eq!(out.column("regime_label")?.dtype(), &DataType::UInt32);

        let out = out.lazy().ta().features().instantaneous_trendline().collect()?;
        assert!(out.column("itl").is_ok());

        let out = out.lazy().ta().features().regime_probs().collect()?;
        assert!(out.column("regime_probs").is_ok());

        let out = out.lazy().ta().features().trendflex(20).collect()?;
        assert!(out.column("trendflex_20").is_ok());

        let out = out
            .lazy()
            .ta()
            .features()
            .ehlers_autocorrelation(30, 10)
            .collect()?;
        assert!(out.column("ehlers_autocorr").is_ok());

        Ok(())
    }
}
