use crate::indicators::incremental::utils::RingBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeType {
    RealBody,
    HighLow,
    Shadows,
}

#[derive(Debug, Clone, Copy)]
pub struct CandleSetting {
    pub range_type: RangeType,
    pub avg_period: usize,
    pub factor: f64,
}

pub const BODY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 1.0,
};
pub const BODY_VERY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 3.0,
};
pub const BODY_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 1.0,
};
pub const BODY_DOJI: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 10,
    factor: 0.1,
};
pub const SHADOW_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 0,
    factor: 1.0,
};
pub const SHADOW_VERY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 0,
    factor: 2.0,
};
pub const SHADOW_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::Shadows,
    avg_period: 10,
    factor: 1.0,
};
pub const SHADOW_VERY_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 10,
    factor: 0.1,
};
pub const NEAR: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.2,
};
pub const FAR: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.6,
};
pub const EQUAL: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.05,
};

#[inline(always)]
pub fn real_body(open: f64, close: f64) -> f64 {
    (close - open).abs()
}

#[inline(always)]
pub fn upper_shadow(open: f64, high: f64, close: f64) -> f64 {
    high - open.max(close)
}

#[inline(always)]
pub fn lower_shadow(open: f64, low: f64, close: f64) -> f64 {
    open.min(close) - low
}

#[inline(always)]
pub fn candle_color(open: f64, close: f64) -> i32 {
    if close >= open { 1 } else { -1 }
}

#[inline(always)]
pub fn candle_range(setting: CandleSetting, open: f64, high: f64, low: f64, close: f64) -> f64 {
    match setting.range_type {
        RangeType::RealBody => real_body(open, close),
        RangeType::HighLow => high - low,
        RangeType::Shadows => upper_shadow(open, high, close) + lower_shadow(open, low, close),
    }
}

#[inline(always)]
pub fn candle_average(
    setting: CandleSetting,
    sum: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
) -> f64 {
    let divisor = match setting.range_type {
        RangeType::Shadows => 2.0,
        _ => 1.0,
    };
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64) / divisor
    } else {
        setting.factor * candle_range(setting, open, high, low, close) / divisor
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug, Clone)]
pub struct CandleWindow {
    window: RingBuffer<Candle>,
    capacity: usize,
}

impl CandleWindow {
    pub fn new(capacity: usize) -> Self {
        Self {
            window: RingBuffer::with_capacity(capacity),
            capacity,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, open: f64, high: f64, low: f64, close: f64) {
        if self.window.len() == self.capacity {
            self.window.pop_front();
        }
        self.window.push_back(Candle {
            open,
            high,
            low,
            close,
        });
    }

    /// Access bars backwards: 0 is current, 1 is previous, etc.
    ///
    /// Total by construction: callers gate on `len()` before indexing, and an
    /// out-of-range index yields a zeroed candle rather than panicking (the
    /// workspace forbids panics in core).
    #[inline(always)]
    pub fn bar(&self, idx: usize) -> Candle {
        let len = self.window.len();
        if idx >= len {
            return Candle::default();
        }
        self.window.get(len - 1 - idx).copied().unwrap_or_default()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.window.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.window.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct RollingCandleAvg {
    pub setting: CandleSetting,
    range_window: RingBuffer<f64>,
    sum: f64,
    val_window: RingBuffer<f64>,
}

impl RollingCandleAvg {
    pub fn new(setting: CandleSetting) -> Self {
        Self {
            setting,
            range_window: RingBuffer::with_capacity(if setting.avg_period > 0 {
                setting.avg_period
            } else {
                1
            }),
            sum: 0.0,
            val_window: RingBuffer::with_capacity(15), // Covers max lookback of 14 + current
        }
    }

    /// Calculate the average value for the current bar WITHOUT updating the sum.
    /// This uses the sum of the `avg_period` bars strictly before this one.
    #[inline(always)]
    fn calc_val(&self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        candle_average(self.setting, self.sum, open, high, low, close)
    }

    #[inline(always)]
    pub fn push(&mut self, open: f64, high: f64, low: f64, close: f64) {
        // Calculate the val for the current bar and save it in val_window
        let val = self.calc_val(open, high, low, close);
        if self.val_window.len() == 15 {
            self.val_window.pop_front();
        }
        self.val_window.push_back(val);

        // Update the sum using the current bar's range, so it's ready for the NEXT bar
        let cr = candle_range(self.setting, open, high, low, close);
        if self.setting.avg_period > 0 {
            if self.range_window.len() == self.setting.avg_period
                && let Some(old) = self.range_window.pop_front()
            {
                self.sum -= old;
            }
            self.range_window.push_back(cr);
            self.sum += cr;
        } else {
            self.sum = cr;
        }
    }

    /// Access previous calculated values backwards: 0 is current bar, 1 is previous bar, etc.
    #[inline(always)]
    pub fn val(&self, idx: usize) -> f64 {
        let len = self.val_window.len();
        if idx >= len {
            return 0.0;
        }
        self.val_window.get(len - 1 - idx).copied().unwrap_or(0.0)
    }
}
