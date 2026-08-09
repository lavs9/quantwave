use crate::indicators::incremental::rolling_variance::RollingVariance;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::utils::RingBuffer as VecDeque;

pub use crate::indicators::incremental::statistics_ta::{
    TaBETA, TaCORREL, TaLINEARREG, TaLINEARREG_ANGLE, TaLINEARREG_INTERCEPT, TaLINEARREG_SLOPE,
    TaSTDDEV, TaTSF, TaVAR,
};
impl From<usize> for TaBETA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaCORREL {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaLINEARREG {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaLINEARREG_SLOPE {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaLINEARREG_INTERCEPT {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaLINEARREG_ANGLE {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for TaTSF {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}

/// Standard Deviation (Population)
///
/// Backed by [`RollingVariance`], a shifted-data accumulator with a
/// deterministic periodic exact refresh. The previous `E[X²] - E[X]²` form lost
/// ~1e-6 relative precision on slowly-varying series such as log-price
/// (quantwave-qkft); this one stays at reference precision while remaining
/// O(1) amortised per bar, so batch and streaming stay bit-identical.
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    period: usize,
    acc: RollingVariance,
}

impl StandardDeviation {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            acc: RollingVariance::new(period),
        }
    }
}

impl From<usize> for StandardDeviation {
    fn from(period: usize) -> Self {
        Self::new(period)
    }
}

impl Next<f64> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.period == 0 {
            // Preserved from the original: a zero period yields no window.
            return f64::NAN;
        }
        self.acc.push(input);
        // Population stddev over the partial-or-full window, matching the
        // original "emit from bar 0" warmup convention.
        self.acc.stddev()
    }
}

/// Linear Regression
/// Returns the value of the regression line at the current bar.
#[derive(Debug, Clone)]
pub struct LinearRegression {
    period: usize,
    window: VecDeque<f64>,
    // Precomputed x values and their sums
    sum_x: f64,
    sum_x2: f64,
}

impl LinearRegression {
    pub fn new(period: usize) -> Self {
        let _n = period as f64;
        let mut sum_x = 0.0;
        let mut sum_x2 = 0.0;
        for i in 0..period {
            let x = i as f64;
            sum_x += x;
            sum_x2 += x * x;
        }

        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum_x,
            sum_x2,
        }
    }
}

impl From<usize> for LinearRegression {
    fn from(period: usize) -> Self {
        Self::new(period)
    }
}

impl Next<f64> for LinearRegression {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.len() < self.period {
            // For partial windows, we could recalculate x sums,
            // but for TTM Squeeze, we'll wait for full window or return partial.
            // Standard approach: wait for full window or adjust n.
            let n = self.window.len() as f64;
            let mut sum_x = 0.0;
            let mut sum_x2 = 0.0;
            let mut sum_y = 0.0;
            let mut sum_xy = 0.0;
            for (i, &y) in self.window.iter().enumerate() {
                let x = i as f64;
                sum_x += x;
                sum_x2 += x * x;
                sum_y += y;
                sum_xy += x * y;
            }

            let denominator = n * sum_x2 - sum_x * sum_x;
            if denominator == 0.0 {
                return input;
            }

            let b = (n * sum_xy - sum_x * sum_y) / denominator;
            let a = (sum_y - b * sum_x) / n;
            return a + b * (n - 1.0);
        }

        let n = self.period as f64;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        for (i, &y) in self.window.iter().enumerate() {
            let x = i as f64;
            sum_y += y;
            sum_xy += x * y;
        }

        let denominator = n * self.sum_x2 - self.sum_x * self.sum_x;
        if denominator == 0.0 {
            return input;
        }

        let b = (n * sum_xy - self.sum_x * sum_y) / denominator;
        let a = (sum_y - b * self.sum_x) / n;

        a + b * (n - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdev_basic() {
        let mut sd = StandardDeviation::new(3);
        // [10] -> mean 10, var 0
        assert_eq!(sd.next(10.0), 0.0);
        // [10, 20] -> mean 15, var (100+400)/2 - 225 = 250 - 225 = 25 -> std 5
        assert_eq!(sd.next(20.0), 5.0);
        // [10, 20, 30] -> mean 20, var (100+400+900)/3 - 400 = 1400/3 - 400 = 466.66 - 400 = 66.66 -> std 8.1649
        approx::assert_relative_eq!(sd.next(30.0), 8.1649658092, epsilon = 1e-6);
    }

    #[test]
    fn test_linreg_basic() {
        let mut lr = LinearRegression::new(3);
        // Perfect line: 1, 2, 3
        lr.next(1.0);
        lr.next(2.0);
        let res = lr.next(3.0);
        approx::assert_relative_eq!(res, 3.0);

        // Line y = 2x + 5. x in [0, 1, 2]. y = [5, 7, 9]
        let mut lr2 = LinearRegression::new(3);
        lr2.next(5.0);
        lr2.next(7.0);
        let res2 = lr2.next(9.0);
        approx::assert_relative_eq!(res2, 9.0);
    }

    use crate::test_utils::{
        assert_bitwise_batch_streaming_parity, log_price_series, reference_stddev,
    };

    /// quantwave-qkft: `StandardDeviation` on log-price must agree with a
    /// compensated two-pass reference well inside the strict 1e-7 gate that the
    /// old one-pass accumulator failed (~1e-6 relative).
    #[test]
    fn stddev_matches_two_pass_reference_on_log_price() {
        let period = 20;
        let data = log_price_series(600);
        let mut sd = StandardDeviation::new(period);
        let mut worst: f64 = 0.0;
        for (i, &x) in data.iter().enumerate() {
            let got = sd.next(x);
            if i + 1 < period {
                continue; // partial-window warmup
            }
            let reference = reference_stddev(&data[i + 1 - period..=i]);
            assert!(reference > 0.0);
            worst = worst.max(((got - reference) / reference).abs());
        }
        assert!(
            worst < 1e-7,
            "StandardDeviation vs two-pass reference on log-price: {worst:e} (gate 1e-7)"
        );
        assert!(worst < 1e-12, "expected ~1e-15 precision, got {worst:e}");
    }

    /// Warmup bars (partial windows) must also be at reference precision.
    #[test]
    fn stddev_partial_window_matches_reference() {
        let period = 20;
        let data = log_price_series(30);
        let mut sd = StandardDeviation::new(period);
        for (i, &x) in data.iter().enumerate() {
            let got = sd.next(x);
            let lo = (i + 1).saturating_sub(period);
            let reference = reference_stddev(&data[lo..=i]);
            if reference > 0.0 {
                approx::assert_relative_eq!(got, reference, max_relative = 1e-12);
            } else {
                assert_eq!(got, 0.0);
            }
        }
    }

    #[test]
    fn stddev_batch_streaming_bitwise_parity() {
        assert_bitwise_batch_streaming_parity(&log_price_series(500), || {
            StandardDeviation::new(20)
        });
        let prices: Vec<f64> = (0..500)
            .map(|i| 2980.0 + (i as f64 * 0.31).sin() * 40.0)
            .collect();
        assert_bitwise_batch_streaming_parity(&prices, || StandardDeviation::new(30));
        assert_bitwise_batch_streaming_parity(&prices, || LinearRegression::new(14));
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_ta_stddev_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let nbdev = 1.0;
            let mut ta_stddev = TaSTDDEV::new(period, nbdev);
            let streaming_results: Vec<f64> = input.iter().map(|&x| ta_stddev.next(x)).collect();
            let batch_results = talib_rs::statistic::stddev(&input, period, nbdev).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ta_linearreg_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut ta_lr = TaLINEARREG::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| ta_lr.next(x)).collect();
            let batch_results = talib_rs::statistic::linearreg(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}

pub const STDDEV_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Standard Deviation",
    description: "Standard Deviation is a statistical measure of market volatility.",
    usage: "Use for statistical analysis of price series: linear regression, standard deviation, correlation coefficients, and other descriptive statistics used as indicator inputs.",
    keywords: &["statistics", "classic", "volatility", "trend"],
    ehlers_summary: "Standard statistical measures provide the mathematical foundation for many technical indicators. Linear regression finds the best-fit line through price, standard deviation quantifies dispersion, and correlation coefficients measure how closely two series move together — all are essential for quantitative strategy construction.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Period",
    }],
    formula_source: "https://www.investopedia.com/terms/s/standarddeviation.asp",
    formula_latex: r#"
\[
\sigma = \sqrt{ \frac{\sum (x_i - \mu)^2}{N} }
\]
"#,
    gold_standard_file: "stddev.json",
    category: "Classic",
};

pub const LINREG_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Linear Regression",
    description: "Linear Regression plots a straight line that best fits the data prices.",
    usage: "Use for statistical analysis of price series: linear regression, standard deviation, correlation coefficients, and other descriptive statistics used as indicator inputs.",
    keywords: &["statistics", "classic", "volatility", "trend"],
    ehlers_summary: "Standard statistical measures provide the mathematical foundation for many technical indicators. Linear regression finds the best-fit line through price, standard deviation quantifies dispersion, and correlation coefficients measure how closely two series move together — all are essential for quantitative strategy construction.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Period",
    }],
    formula_source: "https://www.investopedia.com/terms/l/linearregression.asp",
    formula_latex: r#"
\[
y = a + bx
\]
"#,
    gold_standard_file: "linreg.json",
    category: "Classic",
};

pub const CORREL_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Correlation Coefficient (CORREL)",
    description: "A statistical measure that determines the degree to which two securities move in relation to each other.",
    usage: "Use to measure the strength and direction of the linear relationship between two assets. Values range from -1.0 (inverse correlation) to +1.0 (perfect correlation).",
    keywords: &["statistics", "correlation", "classic"],
    ehlers_summary: "The Pearson Correlation Coefficient measures the strength and direction of a linear relationship between two price series. It is a fundamental tool for pair trading and portfolio diversification, allowing traders to quantify how much of a security's movement is explained by another. — StockCharts ChartSchool",
    params: &[ParamDef {
        name: "timeperiod",
        default: "30",
        description: "Lookback period",
    }],
    formula_source: "https://www.investopedia.com/terms/c/correlationcoefficient.asp",
    formula_latex: r#"
\[
\rho_{X,Y} = \frac{\text{cov}(X,Y)}{\sigma_X \sigma_Y}
\]
"#,
    gold_standard_file: "correl.json",
    category: "Classic",
};

pub const BETA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Beta (BETA)",
    description: "A measure of a security's volatility in relation to the overall market.",
    usage: "Use to understand the systematic risk of an asset. A beta of 1.0 indicates the asset moves with the market; >1.0 means it is more volatile, and <1.0 means it is less volatile.",
    keywords: &["statistics", "risk", "classic", "volatility"],
    ehlers_summary: "Beta is a measure of the volatility—or systematic risk—of a security or portfolio compared to the market as a whole. It is used in the Capital Asset Pricing Model (CAPM) to calculate the expected return of an asset based on its beta and expected market returns. — Investopedia",
    params: &[ParamDef {
        name: "timeperiod",
        default: "30",
        description: "Lookback period",
    }],
    formula_source: "https://www.investopedia.com/terms/b/beta.asp",
    formula_latex: r#"
\[
\beta = \frac{\text{Cov}(R_i, R_m)}{\text{Var}(R_m)}
\]
"#,
    gold_standard_file: "beta.json",
    category: "Classic",
};
