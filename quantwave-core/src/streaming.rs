//! Streaming readiness tracking for any `Next<T>` indicator.
//!
//! Live systems need to know when warmup is complete before acting on signals.
//! Rather than modifying every indicator struct, wrap any `Next` implementor in
//! `TrackedNext` to get `bars_consumed`, `warmup_bars`, and `is_ready`.
//!
//! Python DX: `quantwave.wrap_streaming()` mirrors this at the Python layer;
//! Rust consumers use `track()` or `TrackedNext::new()` directly.

use crate::traits::Next;

/// Readiness state for streaming indicators.
pub trait StreamingReadiness {
    /// Bars processed since construction (or last reset).
    fn bars_consumed(&self) -> usize;
    /// Bars required before the indicator is considered ready.
    fn warmup_bars(&self) -> usize;
    /// True when `bars_consumed >= warmup_bars` (or heuristic for warmup_bars=0).
    fn is_ready(&self) -> bool {
        let warmup = self.warmup_bars();
        if warmup == 0 {
            self.bars_consumed() > 0
        } else {
            self.bars_consumed() >= warmup
        }
    }
}

/// Wraps any `Next<Input>` indicator with bar-count readiness tracking.
#[derive(Debug, Clone)]
pub struct TrackedNext<I> {
    inner: I,
    bars_consumed: usize,
    warmup_bars: usize,
}

impl<I> TrackedNext<I> {
    pub fn new(inner: I, warmup_bars: usize) -> Self {
        Self {
            inner,
            bars_consumed: 0,
            warmup_bars,
        }
    }

    pub fn into_inner(self) -> I {
        self.inner
    }

    pub fn inner(&self) -> &I {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.inner
    }

    pub fn reset(&mut self) {
        self.bars_consumed = 0;
    }
}

impl<I, Input> Next<Input> for TrackedNext<I>
where
    I: Next<Input>,
{
    type Output = I::Output;

    fn next(&mut self, input: Input) -> Self::Output {
        self.bars_consumed += 1;
        self.inner.next(input)
    }
}

impl<I> StreamingReadiness for TrackedNext<I> {
    fn bars_consumed(&self) -> usize {
        self.bars_consumed
    }

    fn warmup_bars(&self) -> usize {
        self.warmup_bars
    }
}

/// Convenience: wrap an indicator with explicit warmup bar count.
pub fn track<I>(inner: I, warmup_bars: usize) -> TrackedNext<I> {
    TrackedNext::new(inner, warmup_bars)
}

/// Derive warmup from the largest numeric period-like parameter.
pub fn warmup_from_params(params: &[(&str, usize)]) -> usize {
    params.iter().map(|(_, v)| *v).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::smoothing::EMA;

    #[test]
    fn test_tracked_next_readiness() {
        let mut tracked = track(EMA::new(5), 5);
        assert!(!tracked.is_ready());
        assert_eq!(tracked.bars_consumed(), 0);

        for i in 1..=4 {
            tracked.next(i as f64);
            assert_eq!(tracked.bars_consumed(), i);
            assert!(!tracked.is_ready());
        }
        tracked.next(5.0);
        assert_eq!(tracked.bars_consumed(), 5);
        assert!(tracked.is_ready());
    }

    #[test]
    fn test_zero_warmup_uses_heuristic() {
        let mut tracked = track(EMA::new(3), 0);
        assert!(!tracked.is_ready());
        tracked.next(1.0);
        assert!(tracked.is_ready());
    }

    #[test]
    fn test_reset_clears_readiness() {
        let mut tracked = track(EMA::new(3), 2);
        tracked.next(1.0);
        tracked.next(2.0);
        assert!(tracked.is_ready());
        tracked.reset();
        assert!(!tracked.is_ready());
        assert_eq!(tracked.bars_consumed(), 0);
    }

    #[test]
    fn test_warmup_from_params() {
        assert_eq!(warmup_from_params(&[("fast", 12), ("slow", 26)]), 26);
        assert_eq!(warmup_from_params(&[]), 0);
    }
}