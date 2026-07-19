//! Point & Figure column construction (close-based, N-box reversal).
//!
//! A Point & Figure chart discards time and volume and reduces price to a grid
//! of boxes of height `box_size`. Price movement is drawn as columns: rising
//! columns of X's and falling columns of O's. A column keeps extending while
//! price advances by whole boxes, and a new (opposite) column starts only when
//! price reverses by at least `reversal` boxes from the current column's extreme
//! — the classic **N-box reversal** filter (`reversal = 3` is the traditional
//! default).
//!
//! This implements the **close-price** construction: each bar contributes its
//! close, quantised to the box grid `floor(price / box_size)`. On a reversal the
//! new column begins one box back from the prior extreme (the standard
//! `top − 1 … top − N` placement).
//!
//! Sources:
//! - Cohen, A. W. (1947), *How to Use the Three-Point Reversal Method of Point &
//!   Figure Stock Market Trading* — origin of the three-box (N-box) reversal.
//! - du Plessis, J. (2012), *The Definitive Guide to Point and Figure*, 2nd ed. —
//!   modern box-size / reversal construction; the close-price method.
//!
//! Two surfaces kept in parity (see the proptest):
//! - streaming: [`PointFigureBuilder::next`] pushes one close, returns a
//!   completed column when a reversal starts a new one (else `None`);
//! - batch: [`point_figure_batch`] folds the builder over a close slice.

/// One completed Point & Figure column. `direction` is +1 (X, rising) or -1
/// (O, falling); `top`/`bottom` are the column's high/low box levels
/// (`top >= bottom`); `boxes` is the span between them in `box_size` units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointFigureColumn {
    pub top: f64,
    pub bottom: f64,
    pub direction: i8,
    pub boxes: u32,
}

/// Stateful close-based Point & Figure builder.
#[derive(Debug, Clone)]
pub struct PointFigureBuilder {
    box_size: f64,
    reversal: u32,
    /// Current column's high and low box indices (`floor(price / box_size)`).
    hi_idx: i64,
    lo_idx: i64,
    /// Current column: +1 X, -1 O, 0 before the first box move establishes one.
    direction: i8,
    initialized: bool,
}

impl PointFigureBuilder {
    /// Create a builder. `box_size` must be positive and `reversal >= 1`.
    pub fn new(box_size: f64, reversal: u32) -> Self {
        Self {
            box_size,
            reversal: reversal.max(1),
            hi_idx: 0,
            lo_idx: 0,
            direction: 0,
            initialized: false,
        }
    }

    fn level(&self, idx: i64) -> f64 {
        idx as f64 * self.box_size
    }

    /// Push one close; return a completed column if this close reversed the
    /// current one, else `None`.
    pub fn next(&mut self, close: f64) -> Option<PointFigureColumn> {
        if self.box_size <= 0.0 || !close.is_finite() {
            return None;
        }
        let i = (close / self.box_size).floor() as i64;
        if !self.initialized {
            self.hi_idx = i;
            self.lo_idx = i;
            self.initialized = true;
            return None;
        }
        let rev = self.reversal as i64;
        match self.direction {
            0 => {
                // First whole-box move fixes the initial column direction.
                if i > self.hi_idx {
                    self.direction = 1;
                    self.hi_idx = i;
                } else if i < self.lo_idx {
                    self.direction = -1;
                    self.lo_idx = i;
                }
                None
            }
            1 => {
                // X column: extend up on a new high; reverse down `rev` boxes.
                if i > self.hi_idx {
                    self.hi_idx = i;
                    None
                } else if i <= self.hi_idx - rev {
                    let col = PointFigureColumn {
                        top: self.level(self.hi_idx),
                        bottom: self.level(self.lo_idx),
                        direction: 1,
                        boxes: (self.hi_idx - self.lo_idx) as u32,
                    };
                    // New O column starts one box below the prior top.
                    self.hi_idx -= 1;
                    self.lo_idx = i;
                    self.direction = -1;
                    Some(col)
                } else {
                    None
                }
            }
            _ => {
                // O column: extend down on a new low; reverse up `rev` boxes.
                if i < self.lo_idx {
                    self.lo_idx = i;
                    None
                } else if i >= self.lo_idx + rev {
                    let col = PointFigureColumn {
                        top: self.level(self.hi_idx),
                        bottom: self.level(self.lo_idx),
                        direction: -1,
                        boxes: (self.hi_idx - self.lo_idx) as u32,
                    };
                    // New X column starts one box above the prior bottom.
                    self.lo_idx += 1;
                    self.hi_idx = i;
                    self.direction = 1;
                    Some(col)
                } else {
                    None
                }
            }
        }
    }
}

/// Batch Point & Figure: fold [`PointFigureBuilder`] over a close slice.
pub fn point_figure_batch(closes: &[f64], box_size: f64, reversal: u32) -> Vec<PointFigureColumn> {
    let mut b = PointFigureBuilder::new(box_size, reversal);
    let mut out = Vec::new();
    for &c in closes {
        if let Some(col) = b.next(c) {
            out.push(col);
        }
    }
    out
}

/// ATR-box Point & Figure: box size is `multiplier * atr` (a single
/// representative ATR), matching the ATR-box convention of the other bar types.
pub fn point_figure_atr_batch(
    closes: &[f64],
    atr: f64,
    multiplier: f64,
    reversal: u32,
) -> Vec<PointFigureColumn> {
    point_figure_batch(closes, atr * multiplier, reversal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn gold_single_x_column_on_reversal() {
        // box=1, rev=3. Rise 10→13 (X column [10,13]); fall to 10 (= 13-3) starts
        // an O column and completes the X column.
        let cols = point_figure_batch(&[10.0, 13.0, 10.0], 1.0, 3);
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].direction, 1);
        assert_eq!(cols[0].bottom, 10.0);
        assert_eq!(cols[0].top, 13.0);
        assert_eq!(cols[0].boxes, 3);
    }

    #[test]
    fn gold_alternating_columns() {
        // X [10,13], then O completes when price turns back up 3 boxes.
        let cols = point_figure_batch(&[10.0, 13.0, 10.0, 7.0, 10.0], 1.0, 3);
        let dirs: Vec<i8> = cols.iter().map(|c| c.direction).collect();
        assert_eq!(dirs, vec![1, -1]);
        // O column ran from the reversal top (12) down to 7.
        assert_eq!(cols[1].top, 12.0);
        assert_eq!(cols[1].bottom, 7.0);
    }

    #[test]
    fn no_reversal_within_threshold() {
        // A 2-box pullback (< reversal 3) never starts a new column.
        assert!(point_figure_batch(&[10.0, 13.0, 11.0, 13.5], 1.0, 3).is_empty());
    }

    #[test]
    fn atr_box_reversal_param() {
        let cols = point_figure_atr_batch(&[10.0, 13.0, 10.0], 1.0, 1.0, 3);
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].boxes, 3);
    }

    proptest! {
        // Streaming (one at a time) must equal batch (fold).
        #[test]
        fn streaming_equals_batch(
            closes in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
            reversal in 1u32..5,
        ) {
            let batch = point_figure_batch(&closes, box_size, reversal);
            let mut b = PointFigureBuilder::new(box_size, reversal);
            let mut streamed = Vec::new();
            for &c in &closes {
                if let Some(col) = b.next(c) {
                    streamed.push(col);
                }
            }
            prop_assert_eq!(streamed, batch);
        }

        // Columns strictly alternate direction, and each column's open→close sign
        // matches its stated direction with a positive box span.
        #[test]
        fn columns_alternate_and_consistent(
            closes in prop::collection::vec(0.0f64..1000.0, 1..200),
            box_size in 0.5f64..50.0,
            reversal in 1u32..5,
        ) {
            let cols = point_figure_batch(&closes, box_size, reversal);
            for w in cols.windows(2) {
                prop_assert_eq!(w[0].direction, -w[1].direction);
            }
            for c in &cols {
                prop_assert!(c.direction == 1 || c.direction == -1);
                prop_assert!(c.top >= c.bottom);
                let span = ((c.top - c.bottom) / box_size).round() as u32;
                prop_assert_eq!(span, c.boxes);
            }
        }
    }
}
