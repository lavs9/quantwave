//! PA Confluence Layer — combine structure, geometric, S/R events with ML features and regimes.
//!
//! Sources: MQL5 Part 50 confluence patterns + quantwave-4ps ML features + cu03 PA foundation.
//! Used by canonical notebooks and backtester filters (quantwave-8aht).

use crate::indicators::market_structure::{Bias, MarketStructureState, PAEvent, PAEventKind};
use crate::regimes::MarketRegime;

/// Runtime context for confluence scoring / filtering at event time.
#[derive(Debug, Clone, Default)]
pub struct ConfluenceContext {
    /// Hard regime label (1=Bull, 2=Bear, 0=Steady, etc. — matches regime_features Polars output).
    pub regime_label: Option<u32>,
    /// Hurst persistence at event bar (from HurstFeatureExtractor).
    pub hurst_persistence: Option<f64>,
    /// Minimum hurst required for trend-following setups (default 0.5).
    pub min_hurst: f64,
    /// Required regime label if set (e.g. 1 for bull-only flag breakouts).
    pub required_regime: Option<u32>,
    /// Required market structure bias (1=Bullish, 2=Bearish, 0=any).
    pub required_bias: Option<u32>,
}

impl ConfluenceContext {
    pub fn bull_flag_filter(regime_label: u32, hurst: f64) -> Self {
        Self {
            regime_label: Some(regime_label),
            hurst_persistence: Some(hurst),
            min_hurst: 0.5,
            required_regime: Some(1),
            required_bias: Some(1),
        }
    }
}

/// Score a PA event against ML/regime/structure context (0.0–1.0+).
pub fn score_pa_event(event: &PAEvent, ctx: &ConfluenceContext) -> f64 {
    let mut score = event.strength * event.confidence;

    if let Some(h) = ctx.hurst_persistence {
        if h >= ctx.min_hurst {
            score *= 1.0 + (h - ctx.min_hurst).min(0.5);
        } else {
            score *= 0.5;
        }
    }

    if let (Some(req), Some(cur)) = (ctx.required_regime, ctx.regime_label) {
        if cur == req {
            score *= 1.2;
        } else {
            score *= 0.3;
        }
    }

    match &event.kind {
        PAEventKind::GeometricFlag(f) if f.is_bull => score *= 1.1,
        PAEventKind::GeometricFlag(f) if !f.is_bull => score *= 1.05,
        PAEventKind::GeometricHs(h) if h.breakout_confirmed => score *= 1.15,
        PAEventKind::SrInteraction(sr) => {
            score *= 1.0 + (sr.strength / 10.0).min(0.2);
        }
        _ => {}
    }

    score
}

/// Returns true if the event passes all configured confluence filters.
pub fn passes_confluence_filter(
    event: &PAEvent,
    ctx: &ConfluenceContext,
    structure: Option<&MarketStructureState>,
) -> bool {
    if let Some(req_bias) = ctx.required_bias {
        if let Some(st) = structure {
            let bias = match st.bias {
                Bias::Bullish => 1u32,
                Bias::Bearish => 2,
                Bias::Neutral => 0,
            };
            if bias != req_bias && req_bias != 0 {
                return false;
            }
        }
    }

    if let Some(req) = ctx.required_regime {
        if ctx.regime_label != Some(req) {
            return false;
        }
    }

    if let Some(h) = ctx.hurst_persistence {
        if h < ctx.min_hurst {
            return false;
        }
    }

    match &event.kind {
        PAEventKind::GeometricFlag(f) => f.breakout_confirmed,
        PAEventKind::GeometricHs(h) => h.breakout_confirmed && h.score >= 60.0,
        PAEventKind::MarketStructureFlip(_) => true,
        PAEventKind::SrInteraction(_) => true,
    }
}

/// Enrich a PA event with regime + feature slots for backtester / ML consumption.
pub fn enrich_pa_event(event: &mut PAEvent, ctx: &ConfluenceContext) {
    if let Some(label) = ctx.regime_label {
        event.regime_at_event = Some(match label {
            1 => "Bull".into(),
            2 => "Bear".into(),
            3 => "Crisis".into(),
            _ => "Steady".into(),
        });
    }
    if let Some(h) = ctx.hurst_persistence {
        event.feature_values.push(("hurst_persistence".into(), h));
    }
    event
        .feature_values
        .push(("confluence_score".into(), score_pa_event(event, ctx)));
}

/// Filter a batch of events through confluence rules.
pub fn filter_confluent_events(
    events: Vec<PAEvent>,
    ctx: &ConfluenceContext,
    structure: Option<&MarketStructureState>,
) -> Vec<PAEvent> {
    events
        .into_iter()
        .filter(|e| passes_confluence_filter(e, ctx, structure))
        .map(|mut e| {
            enrich_pa_event(&mut e, ctx);
            e
        })
        .collect()
}

/// Map MarketRegime to the u32 label used in Polars regime_features.
pub fn regime_to_label(regime: MarketRegime) -> u32 {
    match regime {
        MarketRegime::Bull => 1,
        MarketRegime::Bear => 2,
        MarketRegime::Crisis => 3,
        MarketRegime::Steady => 0,
        MarketRegime::Cluster(c) => 4 + (c as u32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::geometric_patterns::FlagPattern;
    use crate::indicators::market_structure::FlipEvent;

    fn sample_flag() -> FlagPattern {
        FlagPattern {
            id: 1,
            is_bull: true,
            pole_start_bar: 10,
            pole_end_bar: 12,
            flag_start_bar: 13,
            flag_end_bar: 20,
            pole_length: 3.0,
            pole_length_atr: 2.5,
            max_retrace_pct: 40.0,
            pullbacks: 2,
            pushes: 1,
            breakout_confirmed: true,
            breakout_price: 105.0,
            consolidation_bars: 7,
            pole_strength: 2.5,
        }
    }

    #[test]
    fn test_bull_flag_passes_confluence() {
        let event = PAEvent::from_flag(sample_flag(), 20);
        let ctx = ConfluenceContext::bull_flag_filter(1, 0.6);
        let st = MarketStructureState {
            bias: Bias::Bullish,
            last_swing_high: None,
            last_swing_low: None,
            current_flip: None,
            swing_depth_used: 3,
            bar_index: 20,
        };
        assert!(passes_confluence_filter(&event, &ctx, Some(&st)));
        assert!(score_pa_event(&event, &ctx) > 0.5);
    }

    #[test]
    fn test_wrong_regime_fails_filter() {
        let event = PAEvent::from_flag(sample_flag(), 20);
        let ctx = ConfluenceContext::bull_flag_filter(2, 0.6);
        let st = MarketStructureState {
            bias: Bias::Bullish,
            last_swing_high: None,
            last_swing_low: None,
            current_flip: None,
            swing_depth_used: 3,
            bar_index: 20,
        };
        assert!(!passes_confluence_filter(&event, &ctx, Some(&st)));
    }

    #[test]
    fn test_enrich_adds_metadata() {
        let mut event = PAEvent::from_market_structure_flip(
            FlipEvent {
                is_bearish: false,
                price: 100.0,
                bar: 5,
                structure_strength: 3,
            },
            5,
        );
        let ctx = ConfluenceContext {
            regime_label: Some(1),
            hurst_persistence: Some(0.65),
            ..Default::default()
        };
        enrich_pa_event(&mut event, &ctx);
        assert_eq!(event.regime_at_event, Some("Bull".into()));
        assert!(event.feature_values.iter().any(|(k, _)| k == "confluence_score"));
    }
}