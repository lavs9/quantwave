use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;
use std::collections::VecDeque;

/// Instantaneous Trendline
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 10).
/// Removes the dominant cycle component to reveal the underlying trend
/// with minimal lag.
#[derive(Debug, Clone)]
pub struct InstantaneousTrendline {
    price_history: VecDeque<f64>,
    smooth_history: VecDeque<f64>,
    detrender_history: VecDeque<f64>,
    i1_history: VecDeque<f64>,
    q1_history: VecDeque<f64>,
    i2_prev: f64,
    q2_prev: f64,
    re_prev: f64,
    im_prev: f64,
    period_prev: f64,
    smooth_period_prev: f64,
    itrend_history: VecDeque<f64>,
    count: usize,
}

impl InstantaneousTrendline {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::from(vec![0.0; 50]), // Enough for DCPeriod
            smooth_history: VecDeque::from(vec![0.0; 7]),
            detrender_history: VecDeque::from(vec![0.0; 7]),
            i1_history: VecDeque::from(vec![0.0; 7]),
            q1_history: VecDeque::from(vec![0.0; 7]),
            i2_prev: 0.0,
            q2_prev: 0.0,
            re_prev: 0.0,
            im_prev: 0.0,
            period_prev: 6.0, // Initial seed
            smooth_period_prev: 6.0,
            itrend_history: VecDeque::from(vec![0.0; 4]),
            count: 0,
        }
    }
}

impl Default for InstantaneousTrendline {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<f64> for InstantaneousTrendline {
    type Output = f64;

    fn next(&mut self, price: f64) -> Self::Output {
        self.count += 1;

        self.price_history.pop_back();
        self.price_history.push_front(price);

        if self.count < 7 {
            return price;
        }

        // Smooth = (4*Price + 3*Price[1] + 2*Price[2] + Price[3]) / 10;
        let smooth = (4.0 * self.price_history[0]
            + 3.0 * self.price_history[1]
            + 2.0 * self.price_history[2]
            + self.price_history[3])
            / 10.0;

        self.smooth_history.pop_back();
        self.smooth_history.push_front(smooth);

        // Detrender = (.0962*Smooth + .5769*Smooth[2] - .5769*Smooth[4] - .0962*Smooth[6])*(.075*Period[1] + .54);
        let detrender = (0.0962 * self.smooth_history[0] + 0.5769 * self.smooth_history[2]
            - 0.5769 * self.smooth_history[4]
            - 0.0962 * self.smooth_history[6])
            * (0.075 * self.period_prev + 0.54);

        self.detrender_history.pop_back();
        self.detrender_history.push_front(detrender);

        // Q1 = (.0962*Detrender + .5769*Detrender[2] - .5769*Detrender[4] - .0962*Detrender[6])*(.075*Period[1] + .54);
        let q1 = (0.0962 * self.detrender_history[0] + 0.5769 * self.detrender_history[2]
            - 0.5769 * self.detrender_history[4]
            - 0.0962 * self.detrender_history[6])
            * (0.075 * self.period_prev + 0.54);

        // I1 = Detrender[3];
        let i1 = self.detrender_history[3];

        self.i1_history.pop_back();
        self.i1_history.push_front(i1);
        self.q1_history.pop_back();
        self.q1_history.push_front(q1);

        // jI = (.0962*I1 + .5769*I1[2] - .5769*I1[4] - .0962*I1[6])*(.075*Period[1] + .54);
        let ji = (0.0962 * self.i1_history[0] + 0.5769 * self.i1_history[2]
            - 0.5769 * self.i1_history[4]
            - 0.0962 * self.i1_history[6])
            * (0.075 * self.period_prev + 0.54);

        // jQ = (.0962*Q1 + .5769*Q1[2] - .5769*Q1[4] - .0962*Q1[6])*(.075*Period[1] + .54);
        let jq = (0.0962 * self.q1_history[0] + 0.5769 * self.q1_history[2]
            - 0.5769 * self.q1_history[4]
            - 0.0962 * self.q1_history[6])
            * (0.075 * self.period_prev + 0.54);

        // I2 = I1 - jQ;
        // Q2 = Q1 + jI;
        let mut i2 = i1 - jq;
        let mut q2 = q1 + ji;

        // Smooth I and Q components
        i2 = 0.2 * i2 + 0.8 * self.i2_prev;
        q2 = 0.2 * q2 + 0.8 * self.q2_prev;
        
        // Homodyne Discriminator
        let mut re = i2 * self.i2_prev + q2 * self.q2_prev;
        let mut im = i2 * self.q2_prev - q2 * self.i2_prev;

        self.i2_prev = i2;
        self.q2_prev = q2;

        re = 0.2 * re + 0.8 * self.re_prev;
        im = 0.2 * im + 0.8 * self.im_prev;
        self.re_prev = re;
        self.im_prev = im;

        let mut period = self.period_prev;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / (im / re).atan().to_degrees();
        }
        if period > 1.5 * self.period_prev {
            period = 1.5 * self.period_prev;
        }
        if period < 0.67 * self.period_prev {
            period = 0.67 * self.period_prev;
        }
        if period < 6.0 {
            period = 6.0;
        }
        if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * self.period_prev;
        self.period_prev = period;

        let smooth_period = 0.33 * period + 0.67 * self.smooth_period_prev;
        self.smooth_period_prev = smooth_period;

        // DCPeriod = IntPortion(SmoothPeriod + .5);
        let dc_period = (smooth_period + 0.5) as usize;
        
        let mut itrend = 0.0;
        for i in 0..dc_period {
            if i < self.price_history.len() {
                itrend += self.price_history[i];
            }
        }
        if dc_period > 0 {
            itrend /= dc_period as f64;
        }

        self.itrend_history.pop_back();
        self.itrend_history.push_front(itrend);

        // Trendline = (4*ITrend + 3*ITrend[1] + 2*ITrend[2] + ITrend[3]) / 10;
        let trendline = (4.0 * self.itrend_history[0]
            + 3.0 * self.itrend_history[1]
            + 2.0 * self.itrend_history[2]
            + self.itrend_history[3])
            / 10.0;

        if self.count < 12 {
            return price;
        }
        
        trendline
    }
}

pub const INSTANTANEOUS_TRENDLINE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Instantaneous Trendline",
    description: "Removes the dominant cycle to reveal the underlying trend with minimal lag.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf",
    formula_latex: r#"
\[
Trendline = \text{WMA}(\text{SMA}(Price, DCPeriod), 4)
\]
"#,
    gold_standard_file: "instantaneous_trendline.json",
    category: "Rocket Science",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_instantaneous_trendline_basic() {
        let mut it = InstantaneousTrendline::new();
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0];
        for p in prices {
            let res = it.next(p);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_instantaneous_trendline_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut it = InstantaneousTrendline::new();
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| it.next(x)).collect();

            // Reference implementation (batch)
            let mut it_batch = InstantaneousTrendline::new();
            let batch_results: Vec<f64> = inputs.iter().map(|&x| it_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
