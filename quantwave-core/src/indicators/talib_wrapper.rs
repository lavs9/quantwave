//! Macro-generated TA-Lib-compatible streaming indicators.
//! Struct names intentionally mirror TA-Lib identifiers (`CDLDOJI`, `HT_SINE`, …).
#![allow(non_camel_case_types)]

/// O(1) unary pointwise transform — no history buffer.
#[macro_export]
macro_rules! native_pointwise_1 {
    ($name:ident, $op:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                ($op)(input)
            }

            fn next_batch(&mut self, inputs: &[f64]) -> Vec<Self::Output>
            where
                f64: Copy,
            {
                inputs.iter().map(|&x| ($op)(x)).collect()
            }
        }
    };
}

/// O(1) binary element-wise operator.
#[macro_export]
macro_rules! native_binary_2 {
    ($name:ident, $op:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::traits::Next<(f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (a, b): (f64, f64)) -> Self::Output {
                ($op)(a, b)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64): Copy,
            {
                inputs.iter().map(|&(a, b)| ($op)(a, b)).collect()
            }
        }
    };
}

/// O(1) CDL streaming: bounded OHLC window + single talib batch per bar.
#[macro_export]
macro_rules! native_cdl {
    ($name:ident, $talib_func:path) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            open: Vec<f64>,
            high: Vec<f64>,
            low: Vec<f64>,
            close: Vec<f64>,
        }

        impl $name {
            const MAX_WINDOW: usize = 32;

            pub fn new() -> Self {
                Self {
                    open: Vec::with_capacity(Self::MAX_WINDOW),
                    high: Vec::with_capacity(Self::MAX_WINDOW),
                    low: Vec::with_capacity(Self::MAX_WINDOW),
                    close: Vec::with_capacity(Self::MAX_WINDOW),
                }
            }

            #[inline]
            fn trim_window(o: &mut Vec<f64>, h: &mut Vec<f64>, l: &mut Vec<f64>, c: &mut Vec<f64>) {
                let max = Self::MAX_WINDOW;
                if o.len() > max {
                    let drop = o.len() - max;
                    o.drain(0..drop);
                    h.drain(0..drop);
                    l.drain(0..drop);
                    c.drain(0..drop);
                }
            }

            #[inline]
            fn last_signal(o: &[f64], h: &[f64], l: &[f64], c: &[f64]) -> f64 {
                $talib_func(o, h, l, c)
                    .ok()
                    .and_then(|res| res.last().copied())
                    .unwrap_or(0) as f64
            }
        }

        impl $crate::traits::Next<(f64, f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
                self.open.push(open);
                self.high.push(high);
                self.low.push(low);
                self.close.push(close);
                Self::trim_window(
                    &mut self.open,
                    &mut self.high,
                    &mut self.low,
                    &mut self.close,
                );
                Self::last_signal(&self.open, &self.high, &self.low, &self.close)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64, f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64, f64, f64): Copy,
            {
                self.open.clear();
                self.high.clear();
                self.low.clear();
                self.close.clear();
                for &(o, h, l, c) in inputs {
                    self.open.push(o);
                    self.high.push(h);
                    self.low.push(l);
                    self.close.push(c);
                }
                $talib_func(&self.open, &self.high, &self.low, &self.close)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|v| v as f64)
                    .collect()
            }
        }
    };
}

#[macro_export]
macro_rules! talib_cdl {
    ($name:ident, $talib_func:path) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            history_open: Vec<f64>,
            history_high: Vec<f64>,
            history_low: Vec<f64>,
            history_close: Vec<f64>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    history_open: Vec::new(),
                    history_high: Vec::new(),
                    history_low: Vec::new(),
                    history_close: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
                self.history_open.push(open);
                self.history_high.push(high);
                self.history_low.push(low);
                self.history_close.push(close);
                if let Ok(res) = $talib_func(
                    &self.history_open,
                    &self.history_high,
                    &self.history_low,
                    &self.history_close,
                ) {
                    *res.last().unwrap_or(&0) as f64
                } else {
                    0.0
                }
            }

            fn next_batch(&mut self, inputs: &[(f64, f64, f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64, f64, f64): Copy,
            {
                self.history_open.clear();
                self.history_high.clear();
                self.history_low.clear();
                self.history_close.clear();
                for &(o, h, l, c) in inputs {
                    self.history_open.push(o);
                    self.history_high.push(h);
                    self.history_low.push(l);
                    self.history_close.push(c);
                }
                $talib_func(
                    &self.history_open,
                    &self.history_high,
                    &self.history_low,
                    &self.history_close,
                )
                .unwrap_or_default()
                .into_iter()
                .map(|v| v as f64)
                .collect()
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_1_out_i32 {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                if let Ok(res) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    *res.last().unwrap_or(&0) as f64
                } else {
                    0.0
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_1_out_no_result {
    ($name:ident, $talib_func:path) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            history: Vec<f64>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    history: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                let res = $talib_func(&self.history);
                *res.last().unwrap_or(&f64::NAN)
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                let res = $talib_func(&self.history, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }

            fn next_batch(&mut self, inputs: &[f64]) -> Vec<Self::Output>
            where
                f64: Copy,
            {
                self.history.clear();
                self.history.extend_from_slice(inputs);
                $talib_func(&self.history, $( self.$param.clone() ),*).unwrap_or_default()
            }
        }
    };
}

#[macro_export]
macro_rules! talib_2_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history_high: Vec<f64>,
            history_low: Vec<f64>,
        }

        impl $name {
            #[allow(clippy::too_many_arguments)]
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_high: Vec::new(),
                    history_low: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
                self.history_high.push(high);
                self.history_low.push(low);
                let res = $talib_func(&self.history_high, &self.history_low, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64): Copy,
            {
                self.history_high.clear();
                self.history_low.clear();
                for &(h, l) in inputs {
                    self.history_high.push(h);
                    self.history_low.push(l);
                }
                $talib_func(&self.history_high, &self.history_low, $( self.$param.clone() ),*).unwrap_or_default()
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = (f64, f64);

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                if let Ok((res1, res2)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN)
                }
            }

            fn next_batch(&mut self, inputs: &[f64]) -> Vec<Self::Output>
            where
                f64: Copy,
            {
                self.history.clear();
                self.history.extend_from_slice(inputs);
                if let Ok((r1, r2)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    r1.into_iter().zip(r2).map(|(a, b)| (a, b)).collect()
                } else {
                    vec![(f64::NAN, f64::NAN); inputs.len()]
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_3_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = (f64, f64, f64);

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                if let Ok((res1, res2, res3)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN), *res3.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN, f64::NAN)
                }
            }

            fn next_batch(&mut self, inputs: &[f64]) -> Vec<Self::Output>
            where
                f64: Copy,
            {
                self.history.clear();
                self.history.extend_from_slice(inputs);
                if let Ok((r1, r2, r3)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    r1.into_iter()
                        .zip(r2)
                        .zip(r3)
                        .map(|((a, b), c)| (a, b, c))
                        .collect()
                } else {
                    vec![(f64::NAN, f64::NAN, f64::NAN); inputs.len()]
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_2_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history_1: Vec<f64>,
            history_2: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_1: Vec::new(),
                    history_2: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64)> for $name {
            type Output = (f64, f64);

            fn next(&mut self, (in1, in2): (f64, f64)) -> Self::Output {
                self.history_1.push(in1);
                self.history_2.push(in2);
                if let Ok((res1, res2)) = $talib_func(&self.history_1, &self.history_2, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN)
                }
            }

            fn next_batch(&mut self, inputs: &[(f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64): Copy,
            {
                self.history_1.clear();
                self.history_2.clear();
                for &(a, b) in inputs {
                    self.history_1.push(a);
                    self.history_2.push(b);
                }
                if let Ok((r1, r2)) = $talib_func(&self.history_1, &self.history_2, $( self.$param.clone() ),*) {
                    r1.into_iter().zip(r2).map(|(x, y)| (x, y)).collect()
                } else {
                    vec![(f64::NAN, f64::NAN); inputs.len()]
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_3_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history_high: Vec<f64>,
            history_low: Vec<f64>,
            history_close: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_high: Vec::new(),
                    history_low: Vec::new(),
                    history_close: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
                self.history_high.push(high);
                self.history_low.push(low);
                self.history_close.push(close);
                let res = $talib_func(&self.history_high, &self.history_low, &self.history_close, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64, f64): Copy,
            {
                self.history_high.clear();
                self.history_low.clear();
                self.history_close.clear();
                for &(h, l, c) in inputs {
                    self.history_high.push(h);
                    self.history_low.push(l);
                    self.history_close.push(c);
                }
                $talib_func(
                    &self.history_high,
                    &self.history_low,
                    &self.history_close,
                    $( self.$param.clone() ),*
                )
                .unwrap_or_default()
            }
        }
    };
}

#[macro_export]
macro_rules! talib_3_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history_high: Vec<f64>,
            history_low: Vec<f64>,
            history_close: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_high: Vec::new(),
                    history_low: Vec::new(),
                    history_close: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64, f64)> for $name {
            type Output = (f64, f64);

            fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
                self.history_high.push(high);
                self.history_low.push(low);
                self.history_close.push(close);
                if let Ok((res1, res2)) = $talib_func(&self.history_high, &self.history_low, &self.history_close, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN)
                }
            }

            fn next_batch(&mut self, inputs: &[(f64, f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64, f64): Copy,
            {
                self.history_high.clear();
                self.history_low.clear();
                self.history_close.clear();
                for &(h, l, c) in inputs {
                    self.history_high.push(h);
                    self.history_low.push(l);
                    self.history_close.push(c);
                }
                if let Ok((r1, r2)) = $talib_func(
                    &self.history_high,
                    &self.history_low,
                    &self.history_close,
                    $( self.$param.clone() ),*
                ) {
                    r1.into_iter().zip(r2).map(|(a, b)| (a, b)).collect()
                } else {
                    vec![(f64::NAN, f64::NAN); inputs.len()]
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_4_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]

        pub struct $name {
            $( pub $param: $ptype, )*
            history_1: Vec<f64>,
            history_2: Vec<f64>,
            history_3: Vec<f64>,
            history_4: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_1: Vec::new(),
                    history_2: Vec::new(),
                    history_3: Vec::new(),
                    history_4: Vec::new(),
                }
            }
        }

        impl $crate::traits::Next<(f64, f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (in1, in2, in3, in4): (f64, f64, f64, f64)) -> Self::Output {
                self.history_1.push(in1);
                self.history_2.push(in2);
                self.history_3.push(in3);
                self.history_4.push(in4);
                let res = $talib_func(&self.history_1, &self.history_2, &self.history_3, &self.history_4, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64, f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64, f64, f64): Copy,
            {
                self.history_1.clear();
                self.history_2.clear();
                self.history_3.clear();
                self.history_4.clear();
                for &(a, b, c, d) in inputs {
                    self.history_1.push(a);
                    self.history_2.push(b);
                    self.history_3.push(c);
                    self.history_4.push(d);
                }
                $talib_func(
                    &self.history_1,
                    &self.history_2,
                    &self.history_3,
                    &self.history_4,
                    $( self.$param.clone() ),*
                )
                .unwrap_or_default()
            }
        }
    };
}
