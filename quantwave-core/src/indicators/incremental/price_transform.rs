//! Native O(1) price transforms — TA-Lib parity.

use crate::traits::Next;

#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct AVGPRICE;

impl AVGPRICE {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64, f64, f64)> for AVGPRICE {
    type Output = f64;
    fn next(&mut self, (o, h, l, c): (f64, f64, f64, f64)) -> Self::Output {
        (o + h + l + c) / 4.0
    }
}

#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct MEDPRICE;

impl MEDPRICE {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64)> for MEDPRICE {
    type Output = f64;
    fn next(&mut self, (h, l): (f64, f64)) -> Self::Output {
        (h + l) / 2.0
    }
}

#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct TYPPRICE;

impl TYPPRICE {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64, f64)> for TYPPRICE {
    type Output = f64;
    fn next(&mut self, (h, l, c): (f64, f64, f64)) -> Self::Output {
        (h + l + c) / 3.0
    }
}

#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct WCLPRICE;

impl WCLPRICE {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64, f64)> for WCLPRICE {
    type Output = f64;
    fn next(&mut self, (h, l, c): (f64, f64, f64)) -> Self::Output {
        (h + l + 2.0 * c) / 4.0
    }
}
