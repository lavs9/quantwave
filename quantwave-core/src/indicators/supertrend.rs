use crate::indicators::incremental::ta_atr::TaATR;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::volatility::ATR;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// Initial directional bias for SuperTrend before trend confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SuperTrendInitialDirection {
    /// Assumes an uptrend from bar 0 (direction = 1, quantwave legacy default).
    #[default]
    Up,
    /// Assumes a downtrend initially (direction = -1, TradingView convention).
    /// The first close above the upper band represents an explicit flip to uptrend (e.g. IPO breakout).
    Down,
    /// Sets the initial direction from close vs bar midpoint `(high + low) / 2` on the first valid bar.
    FromClose,
}

impl std::str::FromStr for SuperTrendInitialDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "from_close" | "fromclose" => Ok(Self::FromClose),
            other => Err(format!(
                "unknown initial_direction: '{other}' (expected 'up', 'down', or 'from_close')"
            )),
        }
    }
}

/// ATR smoothing algorithm used for SuperTrend volatility bands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SuperTrendAtrSmoothing {
    /// Standard EMA smoothing (alpha = 2/(period+1), seeded from bar 0, quantwave legacy default).
    #[default]
    Ema,
    /// Wilder's RMA smoothing (alpha = 1/period, seeded with SMA of first `period` true ranges,
    /// emits NaN during warm-up, TA-Lib and TradingView Pine `ta.atr` convention).
    Wilder,
}

impl std::str::FromStr for SuperTrendAtrSmoothing {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ema" => Ok(Self::Ema),
            "wilder" | "rma" => Ok(Self::Wilder),
            other => Err(format!(
                "unknown smoothing: '{other}' (expected 'ema' or 'wilder')"
            )),
        }
    }
}

#[derive(Debug, Clone)]
enum AtrBackend {
    Ema(ATR),
    Wilder(TaATR),
}

impl AtrBackend {
    fn next(&mut self, hlc: (f64, f64, f64)) -> f64 {
        match self {
            Self::Ema(atr) => atr.next(hlc),
            Self::Wilder(ta_atr) => ta_atr.next(hlc),
        }
    }
}

pub const SUPERTREND_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "SuperTrend",
    description: "Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.",
    usage: "Use as a primary trend-following indicator and dynamic stop-loss. A SuperTrend flip from bearish to bullish (or vice versa) provides a clear, rule-based entry and exit signal.",
    keywords: &["trend", "atr", "stop-loss", "classic", "breakout"],
    ehlers_summary: "SuperTrend computes upper and lower ATR-based bands around the midpoint of each bar. The active line flips from upper to lower (and vice versa) only when price closes beyond the band, providing a clean directional bias and a trailing stop level in one indicator. — TradingView Community",
    params: &[
        ParamDef {
            name: "period",
            default: "10",
            description: "ATR length",
        },
        ParamDef {
            name: "multiplier",
            default: "3.0",
            description: "ATR multiplier",
        },
        ParamDef {
            name: "initial_direction",
            default: "up",
            description: "Initial trend direction before reversal ('up', 'down', or 'from_close')",
        },
        ParamDef {
            name: "smoothing",
            default: "ema",
            description: "ATR smoothing algorithm ('ema' default, or 'wilder'). Note: SuperTrend defaults to EMA, whereas standalone atr() defaults to Wilder.",
        },
    ],
    formula_source: "https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/",
    formula_latex: r#"
\[
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
\]
"#,
    gold_standard_file: "supertrend_10_3.json",
    category: "Classic",
};

/// SuperTrend Indicator with configurable initial direction and ATR smoothing.
///
/// Note on smoothing:
///   SuperTrend's internal ATR defaults to EMA smoothing (`SuperTrendAtrSmoothing::Ema`).
///   In contrast, standalone `ATR` / `TaATR` defaults to Wilder's RMA.
///   Use `with_options(..., SuperTrendAtrSmoothing::Wilder)` if Wilder RMA smoothing is desired.
#[derive(Debug, Clone)]
pub struct SuperTrend {
    atr: AtrBackend,
    multiplier: f64,
    prev_close: Option<f64>,
    prev_upper_band: Option<f64>,
    prev_lower_band: Option<f64>,
    direction: i8, // 1 for up, -1 for down
    initial_direction: SuperTrendInitialDirection,
    initialized_direction: bool,
}

impl SuperTrend {
    pub fn new(period: usize, multiplier: f64) -> Self {
        Self::with_options(
            period,
            multiplier,
            SuperTrendInitialDirection::Up,
            SuperTrendAtrSmoothing::Ema,
        )
    }

    pub fn with_options(
        period: usize,
        multiplier: f64,
        initial_direction: SuperTrendInitialDirection,
        atr_smoothing: SuperTrendAtrSmoothing,
    ) -> Self {
        let initial_dir_val = match initial_direction {
            SuperTrendInitialDirection::Up => 1,
            SuperTrendInitialDirection::Down => -1,
            SuperTrendInitialDirection::FromClose => 1,
        };
        let atr = match atr_smoothing {
            SuperTrendAtrSmoothing::Ema => AtrBackend::Ema(ATR::new(period)),
            SuperTrendAtrSmoothing::Wilder => AtrBackend::Wilder(TaATR::new(period)),
        };
        Self {
            atr,
            multiplier,
            prev_close: None,
            prev_upper_band: None,
            prev_lower_band: None,
            direction: initial_dir_val,
            initial_direction,
            initialized_direction: false,
        }
    }
}

impl Next<(f64, f64, f64)> for SuperTrend {
    type Output = (f64, i8);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        if high.is_nan() || low.is_nan() || close.is_nan() {
            return (f64::NAN, self.direction);
        }

        let atr = self.atr.next((high, low, close));
        if atr.is_nan() {
            return (f64::NAN, self.direction);
        }

        let mid = (high + low) / 2.0;

        if !self.initialized_direction {
            self.direction = match self.initial_direction {
                SuperTrendInitialDirection::Up => 1,
                SuperTrendInitialDirection::Down => -1,
                SuperTrendInitialDirection::FromClose => {
                    if close >= mid {
                        1
                    } else {
                        -1
                    }
                }
            };
            self.initialized_direction = true;
        }

        let basic_upper = mid + self.multiplier * atr;
        let basic_lower = mid - self.multiplier * atr;

        let upper_band = match self.prev_upper_band {
            Some(prev_upper) => {
                if basic_upper < prev_upper || self.prev_close.unwrap_or(0.0) > prev_upper {
                    basic_upper
                } else {
                    prev_upper
                }
            }
            None => basic_upper,
        };

        let lower_band = match self.prev_lower_band {
            Some(prev_lower) => {
                if basic_lower > prev_lower || self.prev_close.unwrap_or(0.0) < prev_lower {
                    basic_lower
                } else {
                    prev_lower
                }
            }
            None => basic_lower,
        };

        if self.direction == -1 && close > upper_band {
            self.direction = 1;
        } else if self.direction == 1 && close < lower_band {
            self.direction = -1;
        }

        let supertrend = if self.direction == 1 {
            lower_band
        } else {
            upper_band
        };

        self.prev_close = Some(close);
        self.prev_upper_band = Some(upper_band);
        self.prev_lower_band = Some(lower_band);

        (supertrend, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct SuperTrendCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_st: Vec<f64>,
        expected_dir: Vec<i8>,
    }

    #[test]
    fn test_supertrend_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/supertrend_10_3.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path
                .parent()
                .unwrap()
                .join("tests/gold_standard/supertrend_10_3.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: SuperTrendCase = serde_json::from_str(&content).unwrap();

        let mut st = SuperTrend::new(10, 3.0);
        for i in 0..case.high.len() {
            let (val, dir) = st.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(val, case.expected_st[i], epsilon = 1e-6);
            assert_eq!(dir, case.expected_dir[i]);
        }
    }

    #[test]
    fn test_supertrend_nan_inputs_emit_nan() {
        let mut st = SuperTrend::new(3, 2.0);
        let c = [1.0, 2.0, 3.0, f64::NAN, 5.0, 6.0, 7.0];
        let mut results = Vec::new();
        for &val in &c {
            results.push(st.next((val + 1.0, val - 1.0, val)));
        }
        // At index 3 (NaN), result must be NaN, NOT stale finite number!
        assert!(
            results[3].0.is_nan(),
            "SuperTrend should emit NaN on NaN input, got {}",
            results[3].0
        );
    }

    #[test]
    fn test_supertrend_wilder_down_startup_and_breakout() {
        // Test IPO-base stock: listings starting in downtrend (Down) with Wilder ATR
        let period = 3;
        let mult = 2.0;
        let mut st = SuperTrend::with_options(
            period,
            mult,
            SuperTrendInitialDirection::Down,
            SuperTrendAtrSmoothing::Wilder,
        );

        // Bars 0..period (indices 0, 1, 2): Wilder ATR needs period (3) true ranges.
        // Bar 0 sets prev_close; bars 1, 2 collect 2 true ranges (< 3) -> emits NaN.
        let r0 = st.next((10.0, 8.0, 9.0));
        assert!(r0.0.is_nan());
        let r1 = st.next((11.0, 9.0, 10.0));
        assert!(r1.0.is_nan());
        let r2 = st.next((11.0, 9.0, 10.0));
        assert!(r2.0.is_nan());

        // Bar 3: 3rd true range collected -> Wilder ATR emits SMA of the 3 true ranges.
        // TRs:
        // bar1: max(2, |11-9|, |9-9|) = 2.0
        // bar2: max(2, |11-10|, |9-10|) = 2.0
        // bar3: max(2, |10-10|, |8-10|) = 2.0
        // ATR = (2 + 2 + 2)/3 = 2.0.
        // Mid = (10 + 8)/2 = 9.0. basic_upper = 9 + 2*2 = 13.0.
        // Close = 9.5 < 13.0 -> remains Down (-1), supertrend line = 13.0.
        let r3 = st.next((10.0, 8.0, 9.5));
        assert!(!r3.0.is_nan());
        approx::assert_relative_eq!(r3.0, 13.0);
        assert_eq!(r3.1, -1);

        // Bar 4: Price breaks out above upper band (IPO-base breakout)!
        // Close = 15.0 > 13.0 -> flips to Up (1)!
        let r4 = st.next((16.0, 14.0, 15.0));
        assert_eq!(
            r4.1, 1,
            "Should flip to uptrend upon breakout above upper band"
        );
    }

    fn supertrend_batch(
        data: Vec<(f64, f64, f64)>,
        period: usize,
        multiplier: f64,
    ) -> Vec<(f64, i8)> {
        let mut st = SuperTrend::new(period, multiplier);
        data.into_iter().map(|x| st.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_supertrend_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 10;
            let multiplier = 3.0;
            let mut st = SuperTrend::new(period, multiplier);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(st.next(val));
            }

            let batch_results = supertrend_batch(adj_input, period, multiplier);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                assert_eq!(s.1, b.1);
            }
        }
    }
}
