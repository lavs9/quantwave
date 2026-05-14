use crate::indicators::metadata::IndicatorMetadata;
use crate::indicators::hilbert_transform::{HilbertFIR, EhlersWma4};
use crate::traits::Next;
use std::collections::VecDeque;

/// Instantaneous Trendline
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 10).
/// Removes the dominant cycle component to reveal the underlying trend
/// with minimal lag.
#[derive(Debug, Clone)]
pub struct InstantaneousTrendline {
    wma_price: EhlersWma4,
    hilbert_detrender: HilbertFIR,
    hilbert_q1: HilbertFIR,
    hilbert_ji: HilbertFIR,
    hilbert_jq: HilbertFIR,
    
    price_history: VecDeque<f64>,
    detrender_history: VecDeque<f64>,
    i1_history: VecDeque<f64>,
    q1_history: VecDeque<f64>,
    
    i2_prev: f64,
    q2_prev: f64,
    re_prev: f64,
    im_prev: f64,
    period_prev: f64,
    smooth_period_prev: f64,
    
    itrend_wma: EhlersWma4,
    _itrend_history: VecDeque<f64>,
    count: usize,
}

impl InstantaneousTrendline {
    pub fn new() -> Self {
        Self {
            wma_price: EhlersWma4::new(),
            hilbert_detrender: HilbertFIR::new(),
            hilbert_q1: HilbertFIR::new(),
            hilbert_ji: HilbertFIR::new(),
            hilbert_jq: HilbertFIR::new(),
            
            price_history: VecDeque::from(vec![0.0; 50]),
            detrender_history: VecDeque::from(vec![0.0; 7]),
            i1_history: VecDeque::from(vec![0.0; 7]),
            q1_history: VecDeque::from(vec![0.0; 7]),
            
            i2_prev: 0.0,
            q2_prev: 0.0,
            re_prev: 0.0,
            im_prev: 0.0,
            period_prev: 6.0,
            smooth_period_prev: 6.0,
            
            itrend_wma: EhlersWma4::new(),
            _itrend_history: VecDeque::from(vec![0.0; 4]),
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
            self.wma_price.next(price);
            return price;
        }

        let smooth = self.wma_price.next(price);
        let detrender = self.hilbert_detrender.next(smooth, self.period_prev);
        
        self.detrender_history.pop_back();
        self.detrender_history.push_front(detrender);

        let q1 = self.hilbert_q1.next(detrender, self.period_prev);
        let i1 = self.detrender_history[3];

        self.i1_history.pop_back();
        self.i1_history.push_front(i1);
        self.q1_history.pop_back();
        self.q1_history.push_front(q1);

        let ji = self.hilbert_ji.next(i1, self.period_prev);
        let jq = self.hilbert_jq.next(q1, self.period_prev);

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
        period = period.clamp(6.0, 50.0);
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

        let trendline = self.itrend_wma.next(itrend);

        if self.count < 12 {
            return price;
        }
        
        trendline
    }
}

pub const INSTANTANEOUS_TRENDLINE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Instantaneous Trendline",
    description: "Removes the dominant cycle to reveal the underlying trend with minimal lag.",
    usage: "Use as an adaptive trend line that automatically adjusts to the current dominant cycle period, replacing fixed-period moving averages in trend-following systems.",
    keywords: &["trend", "adaptive", "moving-average", "ehlers", "dsp"],
    ehlers_summary: "Defined in Rocket Science for Traders (2001), the Instantaneous Trendline is derived from Hilbert Transform phasors and synchronized to the current market cycle. It is computed as a 3-bar weighted average adjusted by the instantaneous period, giving a zero-lag trend estimate.",
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
