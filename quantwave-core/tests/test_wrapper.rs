use quantwave_core::traits::Next;

pub struct DEMA {
    period: usize,
    history: Vec<f64>,
}

impl DEMA {
    pub fn new(period: usize) -> Self {
        Self { period, history: Vec::new() }
    }
}

impl Next<f64> for DEMA {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.history.push(input);
        let res = talib_rs::overlap::dema(&self.history, self.period).unwrap_or_default();
        *res.last().unwrap_or(&f64::NAN)
    }
}

#[test]
fn test_dema() {
    let mut dema = DEMA::new(3);
    for i in 1..=5 {
        println!("i: {}, dema: {}", i, dema.next(i as f64));
    }
}
