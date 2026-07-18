//! Renko bar construction (close-based, Drozda et al. 2024 Definition 1).
//!
//! A Renko brick spans a fixed height `ΔP` (the "box"); a new brick is created
//! only when the latest *close* leaves the current brick's `[min, max]` band, so
//! small moves are filtered out. This follows the formal definition in:
//!
//!   Drozda, Cavojsky, Sebes (2024), "On Suitability of Renko Charts for
//!   Algorithmic Trading". Def. 1: the first brick `R⁰` is anchored at the first
//!   close `p₀`; a close above `p_max` creates an up-brick `[p_max, p_max + ΔP]`
//!   and a close below `p_min` creates a down-brick `[p_min − ΔP, p_min]`
//!   (adjacent, non-retracing); several bricks may be created for one close.
//!   `ΔP` may vary per brick, so an ATR-derived box is the natural default.
//!
//! A high/low-based variant (symmetric threshold `|P(u) − P(τ)| = H`) is used for
//! pairs trading in Bogomolov (2013), "Pairs trading based on statistical
//! variability of the spread process", Quantitative Finance 13(9): 1411–1430;
//! this module implements the close-based Drozda definition.
//!
//! Two surfaces kept in parity (see the proptest):
//! - streaming: [`RenkoBuilder::next`] pushes one close, returns 0+ new bricks;
//! - batch: [`renko_batch`] folds the builder over a close slice.

/// One Renko brick. `direction` is +1 (up) or -1 (down); `open`/`close` are the
/// brick's price boundaries `box_size` apart, in the direction of travel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenkoBrick {
    pub open: f64,
    pub close: f64,
    pub direction: i8,
}

/// Stateful close-based Renko builder (Drozda et al. 2024, Def. 1) with a fixed
/// box size.
#[derive(Debug, Clone)]
pub struct RenkoBuilder {
    box_size: f64,
    /// First close, used to anchor the first brick (Def. 1: `R⁰` at `p₀`).
    anchor: f64,
    anchored: bool,
    /// Current brick band `[min, max]`; valid once `started`.
    min: f64,
    max: f64,
    started: bool,
}

impl RenkoBuilder {
    /// Create a builder. `box_size` (`ΔP`) must be strictly positive.
    pub fn new(box_size: f64) -> Self {
        Self {
            box_size,
            anchor: 0.0,
            anchored: false,
            min: 0.0,
            max: 0.0,
            started: false,
        }
    }

    /// Push one close; return any bricks created by it (0+).
    pub fn next(&mut self, close: f64) -> Vec<RenkoBrick> {
        let mut out = Vec::new();
        if self.box_size <= 0.0 || !close.is_finite() {
            return out;
        }
        if !self.anchored {
            self.anchor = close;
            self.anchored = true;
            return out;
        }
        if !self.started {
            // First brick forms after a full box move from p₀, either direction.
            if close >= self.anchor + self.box_size {
                let mut bottom = self.anchor;
                while close >= bottom + self.box_size {
                    out.push(RenkoBrick {
                        open: bottom,
                        close: bottom + self.box_size,
                        direction: 1,
                    });
                    bottom += self.box_size;
                }
                self.min = bottom - self.box_size;
                self.max = bottom;
                self.started = true;
            } else if close <= self.anchor - self.box_size {
                let mut top = self.anchor;
                while close <= top - self.box_size {
                    out.push(RenkoBrick {
                        open: top,
                        close: top - self.box_size,
                        direction: -1,
                    });
                    top -= self.box_size;
                }
                self.max = top + self.box_size;
                self.min = top;
                self.started = true;
            }
            return out;
        }
        // Continuation / reversal via Def. 1 formulas on the current band.
        while close > self.max {
            out.push(RenkoBrick {
                open: self.max,
                close: self.max + self.box_size,
                direction: 1,
            });
            self.min = self.max;
            self.max += self.box_size;
        }
        while close < self.min {
            out.push(RenkoBrick {
                open: self.min,
                close: self.min - self.box_size,
                direction: -1,
            });
            self.max = self.min;
            self.min -= self.box_size;
        }
        out
    }
}

/// Batch Renko: fold [`RenkoBuilder`] over a close slice, returning all bricks.
pub fn renko_batch(closes: &[f64], box_size: f64) -> Vec<RenkoBrick> {
    let mut b = RenkoBuilder::new(box_size);
    let mut out = Vec::new();
    for &c in closes {
        out.extend(b.next(c));
    }
    out
}

/// ATR-box Renko: box size is `multiplier * atr`, a single representative
/// volatility value (Drozda et al. use ATR as the default box). Per Def. 1 the
/// box may vary per brick; a fixed box per run keeps boundaries reproducible.
pub fn renko_atr_batch(closes: &[f64], atr: f64, multiplier: f64) -> Vec<RenkoBrick> {
    renko_batch(closes, atr * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn gold_uptrend() {
        // box=1, anchor p₀=10. Climb to 13 → up bricks [10,11],[11,12],[12,13].
        let bricks = renko_batch(&[10.0, 10.5, 11.0, 12.4, 13.0], 1.0);
        assert_eq!(bricks.len(), 3);
        for (i, b) in bricks.iter().enumerate() {
            assert_eq!(b.direction, 1);
            assert_eq!(b.open, 10.0 + i as f64);
            assert_eq!(b.close, 11.0 + i as f64);
        }
    }

    #[test]
    fn gold_reversal_is_non_retracing() {
        // Up to 12 → bricks [10,11],[11,12] (band min=11). Reversal: close 10 < 11
        // → down brick [10,11]; close 9 < 10 → down brick [9,10]. Def. 1 places
        // reversal bricks adjacent below the band — it does NOT retrace [11,12].
        let bricks = renko_batch(&[10.0, 12.0, 11.5, 10.0, 9.0], 1.0);
        let dirs: Vec<i8> = bricks.iter().map(|b| b.direction).collect();
        assert_eq!(dirs, vec![1, 1, -1, -1]);
        assert_eq!(bricks[2].open, 11.0);
        assert_eq!(bricks[2].close, 10.0);
        assert_eq!(bricks.last().unwrap().close, 9.0);
    }

    #[test]
    fn no_brick_within_box() {
        assert!(renko_batch(&[10.0, 10.9, 10.1, 10.99], 1.0).is_empty());
    }

    #[test]
    fn multi_brick_on_gap() {
        // A single close far above p₀ creates several bricks at once.
        let bricks = renko_batch(&[10.0, 13.0], 1.0);
        assert_eq!(bricks.len(), 3);
        assert!(bricks.iter().all(|b| b.direction == 1));
    }

    proptest! {
        // Streaming (one at a time) must equal batch (fold).
        #[test]
        fn streaming_equals_batch(
            closes in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            let batch = renko_batch(&closes, box_size);
            let mut b = RenkoBuilder::new(box_size);
            let mut streamed = Vec::new();
            for &c in &closes {
                streamed.extend(b.next(c));
            }
            prop_assert_eq!(streamed, batch);
        }

        // Every brick spans exactly one box in its stated direction.
        #[test]
        fn brick_spans_one_box(
            closes in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            for brick in renko_batch(&closes, box_size) {
                let span = brick.close - brick.open;
                prop_assert!((span.abs() - box_size).abs() < 1e-9);
                prop_assert_eq!(span.signum() as i8, brick.direction);
            }
        }

        // Def. 1 anchors to p₀: every boundary lies on the p₀ + k·box grid.
        #[test]
        fn boundaries_on_p0_grid(
            closes in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
        ) {
            let p0 = closes[0];
            for brick in renko_batch(&closes, box_size) {
                for edge in [brick.open, brick.close] {
                    let k = (edge - p0) / box_size;
                    prop_assert!((k - k.round()).abs() < 1e-6);
                }
            }
        }
    }
}
