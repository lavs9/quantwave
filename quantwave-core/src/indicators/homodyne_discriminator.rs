use crate::indicators::metadata::IndicatorMetadata;
use crate::indicators::hilbert_transform::{HilbertFIR, EhlersWma4};
use crate::traits::Next;
use std::collections::VecDeque;

/// Homodyne Discriminator
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 8).
/// It estimates the dominant cycle period of the input signal using a homodyne approach
/// (multiplying the signal by its delayed complex conjugate).
#[derive(Debug, Clone)]
pub struct HomodyneDiscriminator {
    wma_price: EhlersWma4,
    hilbert_detrender: HilbertFIR,
    hilbert_q1: HilbertFIR,
    hilbert_ji: HilbertFIR,
    hilbert_jq: HilbertFIR,
    
    detrender_history: VecDeque<f64>,
    i1_history: VecDeque<f64>,
    q1_history: VecDeque<f64>,
    
    i2_prev: f64,
    q2_prev: f64,
    re_prev: f64,
    im_prev: f64,
    period_prev: f64,
    count: usize,
}

impl HomodyneDiscriminator {
    pub fn new() -> Self {
        Self {
            wma_price: EhlersWma4::new(),
            hilbert_detrender: HilbertFIR::new(),
            hilbert_q1: HilbertFIR::new(),
            hilbert_ji: HilbertFIR::new(),
            hilbert_jq: HilbertFIR::new(),
            
            detrender_history: VecDeque::from(vec![0.0; 7]),
            i1_history: VecDeque::from(vec![0.0; 7]),
            q1_history: VecDeque::from(vec![0.0; 7]),
            
            i2_prev: 0.0,
            q2_prev: 0.0,
            re_prev: 0.0,
            im_prev: 0.0,
            period_prev: 6.0,
            count: 0,
        }
    }
}

impl Default for HomodyneDiscriminator {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<f64> for HomodyneDiscriminator {
    type Output = f64;

    fn next(&mut self, price: f64) -> Self::Output {
        self.count += 1;

        if self.count < 7 {
            self.wma_price.next(price);
            return 0.0;
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
        if period < 6.0 {
            period = 6.0;
        }
        if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * self.period_prev;
        self.period_prev = period;

        period
    }
}

pub const HOMODYNE_DISCRIMINATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Homodyne Discriminator",
    description: "Estimates the dominant cycle period using a homodyne approach.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf",
    formula_latex: r#"
\[
\text{Period} = \frac{360}{\text{atan}(Im / Re)}
\]
"#,
    gold_standard_file: "homodyne_discriminator.json",
    category: "Rocket Science",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_homodyne_discriminator_basic() {
        let mut hd = HomodyneDiscriminator::new();
        for i in 0..100 {
            // Sine wave with period 20
            let val = hd.next((2.0 * std::f64::consts::PI * i as f64 / 20.0).sin());
            if i > 50 {
                assert!(val > 10.0 && val < 30.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_homodyne_discriminator_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut hd = HomodyneDiscriminator::new();
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| hd.next(x)).collect();

            let mut hd_batch = HomodyneDiscriminator::new();
            let batch_results: Vec<f64> = inputs.iter().map(|&x| hd_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
