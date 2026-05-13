use std::collections::VecDeque;

/// Hilbert Transform 7-tap FIR Filter
///
/// Based on John Ehlers' "Rocket Science for Traders".
/// This filter provides a 90-degree phase shift for the input signal.
#[derive(Debug, Clone)]
pub struct HilbertFIR {
    window: VecDeque<f64>,
}

impl HilbertFIR {
    pub fn new() -> Self {
        Self {
            window: VecDeque::from(vec![0.0; 7]),
        }
    }

    pub fn next(&mut self, input: f64, period: f64) -> f64 {
        self.window.pop_back();
        self.window.push_front(input);

        // FIR coefficients for 90-degree phase shift
        (0.0962 * self.window[0] 
            + 0.5769 * self.window[2] 
            - 0.5769 * self.window[4] 
            - 0.0962 * self.window[6]) 
            * (0.075 * period + 0.54)
    }
}

impl Default for HilbertFIR {
    fn default() -> Self {
        Self::new()
    }
}

/// A simpler 4-tap WMA often used in Ehlers' smoothing
#[derive(Debug, Clone)]
pub struct EhlersWma4 {
    window: VecDeque<f64>,
}

impl EhlersWma4 {
    pub fn new() -> Self {
        Self {
            window: VecDeque::from(vec![0.0; 4]),
        }
    }

    pub fn next(&mut self, input: f64) -> f64 {
        self.window.pop_back();
        self.window.push_front(input);
        
        (4.0 * self.window[0] 
            + 3.0 * self.window[1] 
            + 2.0 * self.window[2] 
            + self.window[3]) / 10.0
    }
}

impl Default for EhlersWma4 {
    fn default() -> Self {
        Self::new()
    }
}
