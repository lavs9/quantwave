//! Range bar construction (price → constant-range OHLC bars).
//!
//! A range bar closes as soon as its high-low span reaches `range_size`, then a
//! new bar opens at the price that triggered the close. Like Renko this discards
//! time; unlike Renko it keeps full OHLC per bar.
//!
//! Two surfaces kept in parity (see the proptest):
//! - streaming: [`RangeBarBuilder::next`] pushes one price, returns a bar when
//!   the current bar's span completes (else `None`);
//! - batch: [`range_bars_batch`] folds the builder over a price slice.

/// One completed range bar. Its `high - low` is `>= range_size`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// Stateful constant-range bar builder.
#[derive(Debug, Clone)]
pub struct RangeBarBuilder {
    range_size: f64,
    open: f64,
    high: f64,
    low: f64,
    initialized: bool,
}

impl RangeBarBuilder {
    /// Create a builder. `range_size` must be strictly positive.
    pub fn new(range_size: f64) -> Self {
        Self {
            range_size,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            initialized: false,
        }
    }

    /// Push one price; return a completed bar if this price closed the current
    /// bar's span, else `None`.
    pub fn next(&mut self, price: f64) -> Option<RangeBar> {
        if self.range_size <= 0.0 || !price.is_finite() {
            return None;
        }
        if !self.initialized {
            self.open = price;
            self.high = price;
            self.low = price;
            self.initialized = true;
            return None;
        }
        if price > self.high {
            self.high = price;
        }
        if price < self.low {
            self.low = price;
        }
        if self.high - self.low >= self.range_size {
            let bar = RangeBar {
                open: self.open,
                high: self.high,
                low: self.low,
                close: price,
            };
            // New bar opens at the triggering price.
            self.open = price;
            self.high = price;
            self.low = price;
            return Some(bar);
        }
        None
    }
}

/// Batch range bars: fold [`RangeBarBuilder`] over a price slice.
pub fn range_bars_batch(prices: &[f64], range_size: f64) -> Vec<RangeBar> {
    let mut b = RangeBarBuilder::new(range_size);
    let mut out = Vec::new();
    for &p in prices {
        if let Some(bar) = b.next(p) {
            out.push(bar);
        }
    }
    out
}

/// ATR-range bars: range size is `multiplier * atr` (a single representative ATR).
pub fn range_bars_atr_batch(prices: &[f64], atr: f64, multiplier: f64) -> Vec<RangeBar> {
    range_bars_batch(prices, atr * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn gold_single_bar() {
        // Open 10, climbs to 12 (span 2 >= range 2) → one bar closing at 12.
        let bars = range_bars_batch(&[10.0, 10.5, 11.0, 12.0], 2.0);
        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].open, 10.0);
        assert_eq!(bars[0].close, 12.0);
        assert_eq!(bars[0].high, 12.0);
        assert_eq!(bars[0].low, 10.0);
    }

    #[test]
    fn gold_span_includes_both_directions() {
        // 10 up to 11, down to 9 → span 11-9=2 completes at 9.
        let bars = range_bars_batch(&[10.0, 11.0, 9.0], 2.0);
        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].high, 11.0);
        assert_eq!(bars[0].low, 9.0);
        assert_eq!(bars[0].close, 9.0);
    }

    #[test]
    fn no_bar_within_range() {
        assert!(range_bars_batch(&[10.0, 10.9, 10.1, 10.5], 2.0).is_empty());
    }

    proptest! {
        // Streaming (one at a time) must equal batch (fold).
        #[test]
        fn streaming_equals_batch(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            range_size in 0.5f64..50.0,
        ) {
            let batch = range_bars_batch(&prices, range_size);
            let mut b = RangeBarBuilder::new(range_size);
            let mut streamed = Vec::new();
            for &p in &prices {
                if let Some(bar) = b.next(p) {
                    streamed.push(bar);
                }
            }
            prop_assert_eq!(streamed, batch);
        }

        // Every completed bar's span is at least range_size, with consistent OHLC.
        #[test]
        fn bar_span_at_least_range(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            range_size in 0.5f64..50.0,
        ) {
            for bar in range_bars_batch(&prices, range_size) {
                prop_assert!(bar.high - bar.low >= range_size - 1e-9);
                prop_assert!(bar.high >= bar.open && bar.high >= bar.close);
                prop_assert!(bar.low <= bar.open && bar.low <= bar.close);
            }
        }
    }
}
