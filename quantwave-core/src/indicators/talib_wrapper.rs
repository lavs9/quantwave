#[macro_export]
macro_rules! talib_1_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                let res = $talib_func(&self.history, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }
        }
    };
}

#[macro_export]
macro_rules! talib_2_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            $( pub $param: $ptype, )*
            history_high: Vec<f64>,
            history_low: Vec<f64>,
        }

        impl $name {
            pub fn new($( $param: $ptype ),*) -> Self {
                Self {
                    $( $param, )*
                    history_high: Vec::new(),
                    history_low: Vec::new(),
                }
            }
        }

        impl crate::traits::Next<(f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
                self.history_high.push(high);
                self.history_low.push(low);
                let res = $talib_func(&self.history_high, &self.history_low, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<f64> for $name {
            type Output = (f64, f64);

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                if let Ok((res1, res2)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_1_in_3_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<f64> for $name {
            type Output = (f64, f64, f64);

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push(input);
                if let Ok((res1, res2, res3)) = $talib_func(&self.history, $( self.$param.clone() ),*) {
                    (*res1.last().unwrap_or(&f64::NAN), *res2.last().unwrap_or(&f64::NAN), *res3.last().unwrap_or(&f64::NAN))
                } else {
                    (f64::NAN, f64::NAN, f64::NAN)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! talib_2_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<(f64, f64)> for $name {
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
        }
    };
}

#[macro_export]
macro_rules! talib_3_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<(f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
                self.history_high.push(high);
                self.history_low.push(low);
                self.history_close.push(close);
                let res = $talib_func(&self.history_high, &self.history_low, &self.history_close, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }
        }
    };
}

#[macro_export]
macro_rules! talib_3_in_2_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<(f64, f64, f64)> for $name {
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
        }
    };
}

#[macro_export]
macro_rules! talib_4_in_1_out {
    ($name:ident, $talib_func:path $(, $param:ident: $ptype:ty)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
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

        impl crate::traits::Next<(f64, f64, f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (in1, in2, in3, in4): (f64, f64, f64, f64)) -> Self::Output {
                self.history_1.push(in1);
                self.history_2.push(in2);
                self.history_3.push(in3);
                self.history_4.push(in4);
                let res = $talib_func(&self.history_1, &self.history_2, &self.history_3, &self.history_4, $( self.$param.clone() ),*).unwrap_or_default();
                *res.last().unwrap_or(&f64::NAN)
            }
        }
    };
}
