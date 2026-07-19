//! Kagi line construction (price → alternating up/down lines).
//!
//! A Kagi chart discards time and draws a single connected line that keeps
//! extending in the current direction while price advances, and reverses only
//! when price retraces from the running extreme by at least the reversal amount
//! `H`. Each completed *line* runs between two consecutive turning points (true
//! price extrema — the tops/"shoulders" and bottoms/"waists").
//!
//! Reversal construction (threshold `H`, extrema as turning points) follows:
//!
//!   Bogomolov (2013), "Pairs trading based on statistical variability of the
//!   spread process", Quantitative Finance 13(9): 1411–1430 — which formalises
//!   the Kagi (and Renko) constructions as model-free, time-independent samplers
//!   of a price path driven by a single threshold `H`: an up-move reverses at the
//!   first price `<= running_max − H`, a down-move at the first `>= running_min + H`.
//!
//! The yin/yang `thickness` follows the classic charting convention (Nison,
//! "Beyond Candlesticks", 1994): the line turns **yang** (`+1`, thick/bullish)
//! when price rises above the prior shoulder, and **yin** (`-1`, thin/bearish)
//! when it falls below the prior waist; it holds its state in between. This is
//! the machine-readable Kagi signal, kept separate from any visualization.
//!
//! Two surfaces kept in parity (see the proptest):
//! - streaming: [`KagiBuilder::next`] pushes one price, returns a line when the
//!   current direction reverses (else `None`);
//! - batch: [`kagi_batch`] folds the builder over a price slice.

/// One completed Kagi line, running between two consecutive turning points.
/// `direction` is +1 (up) / -1 (down); `thickness` is +1 (yang), -1 (yin), or 0
/// (undetermined — no prior shoulder/waist has been broken yet).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KagiLine {
    /// Start turning point (the previous extreme the line departs from).
    pub open: f64,
    /// End turning point (the extreme where the reversal was confirmed).
    pub close: f64,
    pub direction: i8,
    pub thickness: i8,
}

/// Stateful Kagi builder with a fixed reversal amount `H`.
#[derive(Debug, Clone)]
pub struct KagiBuilder {
    reversal: f64,
    /// Start price of the current line (previous turning point, or the anchor).
    start: f64,
    /// Running extreme of the current line (max while up, min while down).
    extreme: f64,
    /// Current trend: +1 up, -1 down, 0 undetermined (before the first `H`-move).
    direction: i8,
    /// Most recent top turning point (set when an up-line completes).
    shoulder: Option<f64>,
    /// Most recent bottom turning point (set when a down-line completes).
    waist: Option<f64>,
    /// Current yin/yang state; carries over until a shoulder/waist break flips it.
    thickness: i8,
    initialized: bool,
}

impl KagiBuilder {
    /// Create a builder. `reversal` (`H`) must be strictly positive.
    pub fn new(reversal: f64) -> Self {
        Self {
            reversal,
            start: 0.0,
            extreme: 0.0,
            direction: 0,
            shoulder: None,
            waist: None,
            thickness: 0,
            initialized: false,
        }
    }

    /// Update the yin/yang state from a price crossing the prior shoulder/waist.
    fn update_thickness(&mut self, price: f64) {
        if let Some(s) = self.shoulder
            && price > s
        {
            self.thickness = 1;
        }
        if let Some(w) = self.waist
            && price < w
        {
            self.thickness = -1;
        }
    }

    /// Push one price; return a completed line if this price confirmed a reversal
    /// of the current direction, else `None`.
    pub fn next(&mut self, price: f64) -> Option<KagiLine> {
        if self.reversal <= 0.0 || !price.is_finite() {
            return None;
        }
        if !self.initialized {
            self.start = price;
            self.extreme = price;
            self.initialized = true;
            return None;
        }
        if self.direction == 0 {
            // Undetermined: the first `H`-move from the anchor fixes the trend.
            if price >= self.start + self.reversal {
                self.direction = 1;
                self.extreme = price;
            } else if price <= self.start - self.reversal {
                self.direction = -1;
                self.extreme = price;
            }
            self.update_thickness(price);
            return None;
        }
        if self.direction == 1 {
            if price > self.extreme {
                // Line still advancing: thickness tracks the price making the high.
                self.extreme = price;
                self.update_thickness(price);
            } else if price <= self.extreme - self.reversal {
                // Up-line completes at its top (a new shoulder); a down-line opens.
                // `thickness` is the state at the top, before the triggering price
                // (which belongs to the new down-line) touches it.
                let line = KagiLine {
                    open: self.start,
                    close: self.extreme,
                    direction: 1,
                    thickness: self.thickness,
                };
                self.shoulder = Some(self.extreme);
                self.start = self.extreme;
                self.direction = -1;
                self.extreme = price;
                self.update_thickness(price);
                return Some(line);
            }
        } else {
            if price < self.extreme {
                self.extreme = price;
                self.update_thickness(price);
            } else if price >= self.extreme + self.reversal {
                // Down-line completes at its bottom (a new waist); an up-line opens.
                let line = KagiLine {
                    open: self.start,
                    close: self.extreme,
                    direction: -1,
                    thickness: self.thickness,
                };
                self.waist = Some(self.extreme);
                self.start = self.extreme;
                self.direction = 1;
                self.extreme = price;
                self.update_thickness(price);
                return Some(line);
            }
        }
        None
    }
}

/// Batch Kagi: fold [`KagiBuilder`] over a price slice, returning completed lines.
pub fn kagi_batch(prices: &[f64], reversal: f64) -> Vec<KagiLine> {
    let mut b = KagiBuilder::new(reversal);
    let mut out = Vec::new();
    for &p in prices {
        if let Some(line) = b.next(p) {
            out.push(line);
        }
    }
    out
}

/// ATR-reversal Kagi: reversal amount is `multiplier * atr` (a single
/// representative ATR), matching the ATR box convention of the other bar types.
pub fn kagi_atr_batch(prices: &[f64], atr: f64, multiplier: f64) -> Vec<KagiLine> {
    kagi_batch(prices, atr * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn gold_single_reversal() {
        // Anchor 10, H=2. Rise to 14 (up-line), retrace to 11 (<= 14-2) → the
        // up-line [10,14] completes; nothing else confirmed yet.
        let lines = kagi_batch(&[10.0, 12.0, 14.0, 11.0], 2.0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].open, 10.0);
        assert_eq!(lines[0].close, 14.0);
        assert_eq!(lines[0].direction, 1);
    }

    #[test]
    fn gold_alternating_directions() {
        // Up to 14, down to 10 (<=14-2 reverses; new bottom 10), up to 14 again
        // (>=10+2 reverses). Lines: up [10,14], down [14,10]. Directions alternate.
        let lines = kagi_batch(&[10.0, 14.0, 10.0, 14.0], 2.0);
        let dirs: Vec<i8> = lines.iter().map(|l| l.direction).collect();
        assert_eq!(dirs, vec![1, -1]);
        assert_eq!(lines[0].open, 10.0);
        assert_eq!(lines[0].close, 14.0);
        assert_eq!(lines[1].open, 14.0);
        assert_eq!(lines[1].close, 10.0);
    }

    #[test]
    fn gold_no_reversal_within_threshold() {
        // Small wiggles never retrace a full H from the extreme → no completed line.
        assert!(kagi_batch(&[10.0, 11.0, 10.2, 11.5, 10.8], 2.0).is_empty());
    }

    #[test]
    fn gold_yang_on_higher_high() {
        // First up-line [10,14] sets shoulder=14 (thickness 0, no prior shoulder).
        // Reverse down to 11 → down-line [14,11], waist=11. Rise: crossing 14
        // (prior shoulder) turns the line yang; peak 17 then retrace to 14 (<=17-2)
        // completes an up-line with thickness +1 (yang).
        let lines = kagi_batch(&[10.0, 14.0, 11.0, 17.0, 14.0], 2.0);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].thickness, 0); // no prior shoulder to break
        assert_eq!(lines[1].direction, -1);
        assert_eq!(lines[2].direction, 1);
        assert_eq!(lines[2].thickness, 1); // broke above prior shoulder 14 → yang
    }

    proptest! {
        // Streaming (one at a time) must equal batch (fold).
        #[test]
        fn streaming_equals_batch(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            reversal in 0.5f64..50.0,
        ) {
            let batch = kagi_batch(&prices, reversal);
            let mut b = KagiBuilder::new(reversal);
            let mut streamed = Vec::new();
            for &p in &prices {
                if let Some(line) = b.next(p) {
                    streamed.push(line);
                }
            }
            prop_assert_eq!(streamed, batch);
        }

        // Consecutive lines strictly alternate direction, each connected end-to-
        // start, and each spans at least the reversal amount.
        #[test]
        fn lines_alternate_and_connect(
            prices in prop::collection::vec(0.0f64..1000.0, 1..200),
            reversal in 0.5f64..50.0,
        ) {
            let lines = kagi_batch(&prices, reversal);
            for w in lines.windows(2) {
                prop_assert_eq!(w[0].direction, -w[1].direction);
                prop_assert_eq!(w[0].close, w[1].open); // connected turning point
            }
            for l in &lines {
                let span = l.close - l.open;
                prop_assert!((span.abs()) >= reversal - 1e-9);
                prop_assert_eq!(span.signum() as i8, l.direction);
                prop_assert!(l.thickness >= -1 && l.thickness <= 1);
            }
        }
    }
}
