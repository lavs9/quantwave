//! Renko bar construction (price → fixed/ATR-box bricks).
//!
//! Renko discards time and volume: a new brick is emitted only when price moves
//! a full `box_size` from the last brick boundary. This implementation is
//! close-based with a 1-box reversal (a direction change needs one full box past
//! the last brick close), and emits multiple bricks when price jumps several
//! boxes in one step.
//!
//! Two surfaces, kept in parity (see the proptest below):
//! - streaming: [`RenkoBuilder::next`] pushes one price, returns 0+ new bricks;
//! - batch: [`renko_batch`] folds the builder over a price slice.

/// One Renko brick. `direction` is +1 (up) or -1 (down); `open`/`close` are the
/// brick's price boundaries `box_size` apart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenkoBrick {
    pub open: f64,
    pub close: f64,
    pub direction: i8,
}

/// Stateful close-based Renko builder with a fixed box size.
#[derive(Debug, Clone)]
pub struct RenkoBuilder {
    box_size: f64,
    /// Boundary the next brick is measured from (close of the last brick).
    anchor: f64,
    direction: i8,
    initialized: bool,
}

impl RenkoBuilder {
    /// Create a builder. `box_size` must be strictly positive.
    pub fn new(box_size: f64) -> Self {
        Self {
            box_size,
            anchor: 0.0,
            direction: 0,
            initialized: false,
        }
    }

    /// Push one price; return any bricks that completed at this price (0+).
    pub fn next(&mut self, price: f64) -> Vec<RenkoBrick> {
        let mut out = Vec::new();
        if self.box_size <= 0.0 || !price.is_finite() {
            return out;
        }
        if !self.initialized {
            // Anchor the grid to the first price (floor to a box multiple so
            // brick boundaries are stable regardless of the starting value).
            self.anchor = (price / self.box_size).floor() * self.box_size;
            self.initialized = true;
            return out;
        }
        while price >= self.anchor + self.box_size {
            let open = self.anchor;
            let close = self.anchor + self.box_size;
            out.push(RenkoBrick { open, close, direction: 1 });
            self.anchor = close;
            self.direction = 1;
        }
        while price <= self.anchor - self.box_size {
            let open = self.anchor;
            let close = self.anchor - self.box_size;
            out.push(RenkoBrick { open, close, direction: -1 });
            self.anchor = close;
            self.direction = -1;
        }
        out
    }
}

/// Batch Renko: fold [`RenkoBuilder`] over a price slice, returning all bricks.
pub fn renko_batch(prices: &[f64], box_size: f64) -> Vec<RenkoBrick> {
    let mut b = RenkoBuilder::new(box_size);
    let mut out = Vec::new();
    for &p in prices {
        out.extend(b.next(p));
    }
    out
}

/// ATR-box Renko: box size is `multiplier * atr`, where `atr` is a single
/// representative volatility value (e.g. the ATR over the whole series). Callers
/// that want a per-bar adaptive box should segment and re-run; a fixed box per
/// run keeps brick boundaries well-defined and reproducible.
pub fn renko_atr_batch(prices: &[f64], atr: f64, multiplier: f64) -> Vec<RenkoBrick> {
    renko_batch(prices, atr * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn gold_simple_uptrend() {
        // box=1, anchored at floor(10)=10. Prices climbing to 13 → 3 up bricks.
        let bricks = renko_batch(&[10.0, 10.5, 11.0, 12.4, 13.0], 1.0);
        assert_eq!(bricks.len(), 3);
        for (i, b) in bricks.iter().enumerate() {
            assert_eq!(b.direction, 1);
            assert_eq!(b.open, 10.0 + i as f64);
            assert_eq!(b.close, 11.0 + i as f64);
        }
    }

    #[test]
    fn gold_reversal() {
        // Up to 12 (2 bricks), then down to 9 → down bricks from anchor 12.
        let bricks = renko_batch(&[10.0, 12.0, 11.5, 10.0, 9.0], 1.0);
        let dirs: Vec<i8> = bricks.iter().map(|b| b.direction).collect();
        assert_eq!(dirs, vec![1, 1, -1, -1, -1]);
        assert_eq!(bricks.last().unwrap().close, 9.0);
    }

    #[test]
    fn no_brick_within_box() {
        // Movement smaller than the box → no bricks.
        assert!(renko_batch(&[10.0, 10.9, 10.1, 10.99], 1.0).is_empty());
    }

    proptest! {
        // Streaming (push one at a time) must equal batch (fold over the slice).
        #[test]
        fn streaming_equals_batch(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            let batch = renko_batch(&prices, box_size);
            let mut b = RenkoBuilder::new(box_size);
            let mut streamed = Vec::new();
            for &p in &prices {
                streamed.extend(b.next(p));
            }
            prop_assert_eq!(streamed, batch);
        }

        // Every brick spans exactly one box, in its stated direction.
        #[test]
        fn brick_spans_one_box(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            for brick in renko_batch(&prices, box_size) {
                let span = brick.close - brick.open;
                prop_assert!((span.abs() - box_size).abs() < 1e-9);
                prop_assert_eq!(span.signum() as i8, brick.direction);
            }
        }

        // Consecutive brick boundaries are contiguous (each open == previous close).
        #[test]
        fn bricks_are_contiguous(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            let bricks = renko_batch(&prices, box_size);
            for pair in bricks.windows(2) {
                prop_assert!((pair[1].open - pair[0].close).abs() < 1e-9);
            }
        }
    }
}
