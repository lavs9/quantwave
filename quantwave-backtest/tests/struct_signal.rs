//! Struct signal_col auto-parse (quantwave-cr6v.11).
//!
//! `cargo nextest run -p quantwave-backtest --test struct_signal`

use approx::assert_relative_eq;
use chrono::{TimeZone, Utc};
use polars::prelude::*;
use quantwave_backtest::{
    parse_struct_signal_row, pole_height_to_exposure, run_streaming_simulation, BacktestConfig,
    BacktestEngine, Bar, CostModel, ExecutionModel, InitialRiskPositionSizer, StrategySignal,
};

fn zero_cost_config() -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        ..Default::default()
    }
}

fn signal_struct_col(
    exposure: Vec<f64>,
    pole_height: Option<Vec<f64>>,
    long: Option<Vec<bool>>,
) -> Column {
    let n = exposure.len();
    let exp_s = Series::new("exposure".into(), exposure);
    let mut series: Vec<Series> = vec![exp_s];
    if let Some(p) = pole_height {
        series.push(Series::new("pole_height".into(), p));
    }
    if let Some(l) = long {
        series.push(Series::new("long".into(), l));
    }
    let ca = StructChunked::from_series("signal".into(), n, series.iter()).unwrap();
    Column::from(ca.into_series())
}

#[test]
fn test_struct_signal_exposure_field() {
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_900_100_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 105.0, 104.0]),
        signal_struct_col(vec![0.0, 1.0, 1.0, 0.0], None, None),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("struct exposure run");

    assert_eq!(result.trades.height(), 1);
    let pnl = result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    // Exit bar 3 @104 when signal flips flat.
    assert_relative_eq!(pnl, 4.0, epsilon = 1e-9);
}

#[test]
fn test_struct_signal_pole_height_sizing() {
    // long + pole_height=8 → exposure 2.0 (8/4 clamped).
    assert_relative_eq!(pole_height_to_exposure(8.0), 2.0, epsilon = 1e-9);

    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_900_200_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 102.0, 101.0]),
        signal_struct_col(
            vec![0.0; 4],
            Some(vec![0.0, 8.0, 8.0, 0.0]),
            Some(vec![false, true, true, false]),
        ),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("pole height struct run");

    assert_eq!(result.trades.height(), 1);
    let qty = result
        .trades
        .column("quantity")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    assert_relative_eq!(qty, 2.0, epsilon = 1e-9);
    let pnl = result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    // 2 units * (101 - 100) = 2 (exit bar 3 when long flips false)
    assert_relative_eq!(pnl, 2.0, epsilon = 1e-9);
}

#[test]
fn test_struct_signal_metadata_with_position_sizer() {
    let mut config = zero_cost_config();
    config.position_sizer = Some(InitialRiskPositionSizer {
        initial_risk: 0.01,
        max_target_pct: 0.5,
    });

    let exposure = Series::new("exposure".into(), vec![0.0, 1.0, 1.0, 0.0]);
    let pole_atr = Series::new("pole_height_atr".into(), vec![0.0, 2.0, 2.0, 0.0]);
    let series = vec![exposure, pole_atr];
    let ca = StructChunked::from_series("signal".into(), 4, series.iter()).unwrap();

    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_900_300_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 101.0, 100.0]),
        Column::from(ca.into_series()),
    ])
    .unwrap();

    let result = BacktestEngine::new(config)
        .run(df.lazy())
        .expect("sizer struct run");

    assert_eq!(result.trades.height(), 1);
    let qty = result
        .trades
        .column("quantity")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    // pole_height_atr=2, equity 100k → sized ~500 units (0.5 * 100k / 100)
    assert!((qty - 500.0).abs() < 1.0);
}

struct SignalReplay {
    exposures: Vec<f64>,
    metas: Vec<Option<std::collections::HashMap<String, f64>>>,
    idx: usize,
}

impl quantwave_core::traits::Next<&Bar> for SignalReplay {
    type Output = StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        let i = self.idx.min(self.exposures.len().saturating_sub(1));
        let exposure = self.exposures[i];
        let metadata = self.metas[i].clone();
        self.idx += 1;
        StrategySignal {
            exposure,
            metadata,
        }
    }
}

#[test]
fn test_struct_batch_streaming_parity() {
    let ts: Vec<i64> = (0..4).map(|i| 1_900_400_000 + i).collect();
    let closes = vec![100.0, 100.0, 105.0, 104.0];

    let exposure = Series::new("exposure".into(), vec![0.0, 1.0, 1.0, 0.0]);
    let pole = Series::new("pole_height".into(), vec![0.0, 4.0, 4.0, 0.0]);
    let series = vec![exposure, pole];
    let ca = StructChunked::from_series("signal".into(), 4, series.iter()).unwrap();

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts.clone()),
        Column::new("close".into(), closes.clone()),
        Column::from(ca.into_series()),
    ])
    .unwrap();

    // Derive streaming replay from same struct rows
    let s = df.column("signal").unwrap().as_series().unwrap();
    let struct_ca = s.struct_().unwrap();
    let mut exposures = Vec::new();
    let mut metas = Vec::new();
    for i in 0..struct_ca.len() {
        let (e, m) = parse_struct_signal_row(struct_ca, i).unwrap();
        exposures.push(e);
        metas.push(m);
    }

    let batch = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("batch struct");

    let bars: Vec<Bar> = ts
        .iter()
        .zip(closes.iter())
        .map(|(&t, &close)| Bar {
            ts: Utc.timestamp_opt(t, 0).unwrap(),
            close,
            high: None,
            low: None,
        })
        .collect();

    let stream = run_streaming_simulation(
        &bars,
        SignalReplay {
            exposures,
            metas,
            idx: 0,
        },
        zero_cost_config(),
    )
    .expect("streaming struct replay");

    let b_eq: Vec<f64> = batch
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let s_eq: Vec<f64> = stream
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    assert_eq!(b_eq.len(), s_eq.len());
    for (i, (b, s)) in b_eq.iter().zip(s_eq.iter()).enumerate() {
        assert_relative_eq!(*b, *s, epsilon = 1e-8, max_relative = 1e-8);
        if (b - s).abs() > 1e-7 {
            panic!("equity diverged at bar {i}: {b} vs {s}");
        }
    }
    assert_eq!(batch.trades.height(), stream.trades.height());
}