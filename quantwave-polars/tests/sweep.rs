//! Polars `.bt.sweep` integration tests (quantwave-cr6v.12).
//!
//! `cargo nextest run -p quantwave-polars sweep`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_polars::prelude::*;

fn sweep_base_df() -> DataFrame {
    DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..6)
                .map(|i| 1_700_000_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new(
            "close".into(),
            vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
        ),
        Column::new("signal_early".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
        Column::new("signal_late".into(), vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0]),
        Column::new("signal_flat".into(), vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ])
    .unwrap()
}

fn zero_cost_options() -> BtOptions {
    BtOptions {
        commission_bps: 0.0,
        slippage_bps: 0.0,
        ..Default::default()
    }
}

#[test]
fn test_bt_sweep_single_param_grid() {
    let df = sweep_base_df()
        .lazy()
        .bt()
        .sweep_single_param(
            "threshold",
            &[0.5, 1.0, 2.0],
            &["signal_early", "signal_late", "signal_flat"],
            zero_cost_options(),
        )
        .expect("sweep should succeed");

    assert_eq!(df.height(), 3);
    assert!(df.column("threshold").is_ok());
    assert!(df.column("num_trades").is_ok());

    let trades = df.column("num_trades").unwrap().f64().unwrap();
    assert_relative_eq!(trades.get(0).unwrap(), 1.0, epsilon = 1e-9);
    assert_relative_eq!(trades.get(1).unwrap(), 1.0, epsilon = 1e-9);
    assert_relative_eq!(trades.get(2).unwrap(), 0.0, epsilon = 1e-9);
}

#[test]
fn test_bt_sweep_explicit_variants() {
    use std::collections::HashMap;

    let variants = vec![
        SweepVariant {
            params: HashMap::from([("hurst_period".to_string(), 10.0)]),
            signal_col: "signal_early".into(),
        },
        SweepVariant {
            params: HashMap::from([("hurst_period".to_string(), 15.0)]),
            signal_col: "signal_late".into(),
        },
    ];

    let df = sweep_base_df()
        .lazy()
        .bt()
        .sweep(&variants, zero_cost_options())
        .expect("explicit sweep");

    assert_eq!(df.height(), 2);
    let periods = df.column("hurst_period").unwrap().f64().unwrap();
    assert_relative_eq!(periods.get(0).unwrap(), 10.0, epsilon = 1e-9);
    assert_relative_eq!(periods.get(1).unwrap(), 15.0, epsilon = 1e-9);

    let equity = df.column("final_equity").unwrap().f64().unwrap();
    assert!(equity.get(0).unwrap() != equity.get(1).unwrap());
}