#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
//! Risk overlay integration tests (quantwave-pvmr).
//!
//! - Default (`risk_model: None`) leaves batch results byte-identical to a
//!   backtest that never had the field at all.
//! - Each overlay (vol_target, inverse_vol, position_limit, pre_trade)
//!   measurably changes sizing in the expected direction end-to-end.
//! - A configured `risk_model` produces identical equity curves in the
//!   batch DataFrame path and the streaming path (the parity moat), since
//!   both funnel through the same shared `run_simulation` core.
//!
//! `cargo nextest run -p quantwave-backtest risk_model`

use polars::prelude::IntoLazy;
use quantwave_backtest::risk::{
    InverseVolConfig, PositionLimitConfig, PreTradeConfig, RiskModel, VolTargetConfig,
};
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, Bar, CostModel, ExecutionModel, StrategySignal,
    run_streaming_simulation,
};
use quantwave_core::traits::Next;

struct ConstReplay {
    exposure: f64,
}

impl Next<&Bar> for ConstReplay {
    type Output = StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        StrategySignal {
            exposure: self.exposure,
            metadata: None,
        }
    }
}

/// Alternating-shock series: gives a large, well-defined trailing realized vol.
fn shocky_closes(n: usize, start: f64, shock: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(n);
    let mut p = start;
    for i in 0..n {
        v.push(p);
        p *= 1.0 + if i % 2 == 0 { shock } else { -shock };
    }
    v
}

fn make_df(closes: &[f64], signal: f64) -> polars::prelude::DataFrame {
    let n = closes.len();
    let ts: Vec<i64> = (0..n as i64).map(|i| 1_700_000_000 + i * 3600).collect();
    let signals = vec![signal; n];
    polars::prelude::DataFrame::new(vec![
        polars::prelude::Column::new("timestamp".into(), ts),
        polars::prelude::Column::new("close".into(), closes.to_vec()),
        polars::prelude::Column::new("signal".into(), signals),
    ])
    .unwrap()
}

fn make_bars(closes: &[f64]) -> Vec<Bar> {
    (0..closes.len())
        .map(|i| Bar {
            ts: chrono::DateTime::<chrono::Utc>::from_timestamp(1_700_000_000 + i as i64 * 3600, 0)
                .unwrap(),
            close: closes[i],
            high: None,
            low: None,
        })
        .collect()
}

fn zero_cost_exec() -> ExecutionModel {
    ExecutionModel::Simple(CostModel {
        commission_bps: 0.0,
        slippage_bps: 0.0,
        initial_cash: 100_000.0,
    })
}

fn max_abs_position(df: &polars::prelude::DataFrame) -> f64 {
    df.column("position")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .flatten()
        .fold(0.0_f64, |acc, v| acc.max(v.abs()))
}

/// Build a DataFrame with a per-bar signal vector, so a position can be made to
/// open only *after* a warmup window. Risk overlays size a position at ENTRY
/// (the engine does not resize an already-open position — "no intra-trade
/// resizing"), so a meaningful vol_target/inverse_vol test must open post-warmup.
fn make_df_signals(closes: &[f64], signals: &[f64]) -> polars::prelude::DataFrame {
    let n = closes.len();
    let ts: Vec<i64> = (0..n as i64).map(|i| 1_700_000_000 + i * 3600).collect();
    polars::prelude::DataFrame::new(vec![
        polars::prelude::Column::new("timestamp".into(), ts),
        polars::prelude::Column::new("close".into(), closes.to_vec()),
        polars::prelude::Column::new("signal".into(), signals.to_vec()),
    ])
    .unwrap()
}

#[test]
fn default_risk_model_none_matches_no_field_baseline() {
    let closes = shocky_closes(30, 100.0, 0.01);
    let df = make_df(&closes, 1.0);

    let with_explicit_none = BacktestConfig {
        execution_model: zero_cost_exec(),
        risk_model: None,
        ..Default::default()
    };
    let baseline = BacktestConfig {
        execution_model: zero_cost_exec(),
        ..Default::default()
    };

    let a = BacktestEngine::new(with_explicit_none)
        .run(df.clone().lazy())
        .expect("a");
    let b = BacktestEngine::new(baseline).run(df.lazy()).expect("b");

    let eq_a: Vec<f64> = a
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let eq_b: Vec<f64> = b
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    assert_eq!(
        eq_a, eq_b,
        "risk_model: None must be byte-identical to today's default"
    );
}

#[test]
fn vol_target_scales_exposure_toward_target_end_to_end() {
    // High realized vol -> vol_target should size DOWN the position opened after
    // warmup vs. an unmodulated run. Signal is flat during the 20-bar vol lookback
    // then turns long, so the position OPENS post-warmup and its entry size
    // reflects the vol_target scale (the engine sizes at entry, no intra-trade
    // resizing).
    let closes = shocky_closes(40, 100.0, 0.02);
    let mut signals = vec![0.0; 40];
    for s in signals.iter_mut().skip(25) {
        *s = 1.0;
    }
    let df = make_df_signals(&closes, &signals);

    let plain = BacktestConfig {
        execution_model: zero_cost_exec(),
        signal_col: "signal".to_string(),
        ..Default::default()
    };
    let with_vt = BacktestConfig {
        execution_model: zero_cost_exec(),
        signal_col: "signal".to_string(),
        risk_model: Some(RiskModel {
            vol_target: Some(VolTargetConfig {
                target_annual_vol: 0.05,
                lookback: 20,
                bars_per_year: 252.0,
                min_scale: 0.0,
                max_scale: 10.0,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let plain_res = BacktestEngine::new(plain)
        .run(df.clone().lazy())
        .expect("plain");
    let vt_res = BacktestEngine::new(with_vt).run(df.lazy()).expect("vt");

    let plain_max = max_abs_position(&plain_res.equity_curve);
    let vt_max = max_abs_position(&vt_res.equity_curve);
    assert!(
        vt_max < plain_max,
        "vol_target should size down the entry under high realized vol: {vt_max} vs {plain_max}"
    );
    assert!(
        vt_max > 0.0,
        "vol_target should not fully zero the position"
    );
}

#[test]
fn inverse_vol_sizes_position_from_target_over_realized_vol() {
    // Open post-warmup so inverse-vol absolute sizing (units = leverage * equity /
    // price) actually applies at entry rather than passing through during warmup.
    let closes = shocky_closes(40, 100.0, 0.01);
    let mut signals = vec![0.0; 40];
    for s in signals.iter_mut().skip(25) {
        *s = 1.0;
    }
    let df = make_df_signals(&closes, &signals);

    let cfg = InverseVolConfig {
        target_annual_vol: 0.20,
        lookback: 20,
        bars_per_year: 252.0,
        min_scale: 0.0,
        max_scale: 10.0,
    };
    let config = BacktestConfig {
        execution_model: zero_cost_exec(),
        signal_col: "signal".to_string(),
        risk_model: Some(RiskModel {
            inverse_vol: Some(cfg),
            ..Default::default()
        }),
        ..Default::default()
    };

    let res = BacktestEngine::new(config).run(df.lazy()).expect("run");
    // Absolute vol sizing (~target/realized * equity/price) is far larger than the
    // raw exposure of 1.0 unit — proving the overlay set the size, not a passthrough.
    assert!(max_abs_position(&res.equity_curve) > 100.0);
}

#[test]
fn position_limit_caps_absolute_exposure_end_to_end() {
    let closes = vec![100.0; 20];
    let df = make_df(&closes, 1.0);

    let config = BacktestConfig {
        execution_model: zero_cost_exec(),
        signal_col: "signal".to_string(),
        risk_model: Some(RiskModel {
            position_limit: Some(PositionLimitConfig {
                max_abs_exposure: Some(50.0),
                max_leverage: None,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let res = BacktestEngine::new(config).run(df.lazy()).expect("run");
    let max_pos = max_abs_position(&res.equity_curve);
    assert!(
        max_pos <= 50.0 + 1e-9,
        "position_limit must cap exposure at 50 units, got {max_pos}"
    );
    assert!(max_pos > 0.0, "position should still open, just capped");
}

#[test]
fn pre_trade_veto_zeroes_out_breaching_positions() {
    let closes = vec![100.0; 20];
    let df = make_df(&closes, 1.0);

    let config = BacktestConfig {
        execution_model: zero_cost_exec(),
        risk_model: Some(RiskModel {
            pre_trade: Some(PreTradeConfig {
                max_notional: Some(1.0), // absurdly small -> every entry is vetoed
                max_leverage: None,
                veto_on_breach: true,
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let res = BacktestEngine::new(config).run(df.lazy()).expect("run");
    assert_eq!(
        res.trades.height(),
        0,
        "pre_trade veto should suppress all entries"
    );
}

#[test]
fn risk_model_batch_streaming_parity() {
    let closes = shocky_closes(50, 100.0, 0.015);
    let df = make_df(&closes, 1.0);
    let bars = make_bars(&closes);

    let risk_model = Some(RiskModel {
        vol_target: Some(VolTargetConfig {
            target_annual_vol: 0.10,
            lookback: 15,
            bars_per_year: 252.0,
            min_scale: 0.0,
            max_scale: 8.0,
        }),
        position_limit: Some(PositionLimitConfig {
            max_abs_exposure: Some(20.0),
            max_leverage: None,
        }),
        ..Default::default()
    });

    let config = BacktestConfig {
        execution_model: zero_cost_exec(),
        risk_model,
        ..Default::default()
    };

    let batch = BacktestEngine::new(config.clone())
        .run(df.lazy())
        .expect("batch");
    let stream =
        run_streaming_simulation(&bars, ConstReplay { exposure: 1.0 }, config).expect("stream");

    let batch_eq: Vec<f64> = batch
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let stream_eq: Vec<f64> = stream
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    assert_eq!(batch_eq.len(), stream_eq.len());
    for (i, (b, s)) in batch_eq.iter().zip(stream_eq.iter()).enumerate() {
        assert!(
            (b - s).abs() < 1e-6,
            "equity mismatch at bar {i}: {b} vs {s}"
        );
    }
    assert_eq!(batch.trades.height(), stream.trades.height());
    assert_eq!(
        *batch.stats.get("final_equity").unwrap(),
        *stream.stats.get("final_equity").unwrap()
    );
}
