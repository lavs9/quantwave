use crate::traits::Next;
use crate::indicators::smoothing::WMA;

#[derive(Debug, Clone)]
pub struct HMA {
    wma_half: WMA,
    wma_full: WMA,
    wma_sqrt: WMA,
}

impl HMA {
    pub fn new(period: usize) -> Self {
        Self {
            wma_half: WMA::new(period / 2),
            wma_full: WMA::new(period),
            wma_sqrt: WMA::new((period as f64).sqrt() as usize),
        }
    }
}

impl Next<f64> for HMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let wma_half = self.wma_half.next(input);
        let wma_full = self.wma_full.next(input);
        let raw = 2.0 * wma_half - wma_full;
        self.wma_sqrt.next(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hma_basic() {
        let mut hma = HMA::new(20);
        // HMA is complex to verify manually, but we can check if it returns values
        for i in 0..100 {
            let val = hma.next(i as f64);
            if i > 20 {
                assert!(val > 0.0);
            }
        }
    }
}
