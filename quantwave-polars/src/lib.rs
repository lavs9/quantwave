use polars::prelude::*;
use quantwave_core::traits::Next;
use quantwave_core::*;

pub mod prelude {
    pub use crate::{QuantWaveExt, QuantWaveNamespace};
}

pub trait QuantWaveExt {
    fn ta(&self) -> QuantWaveNamespace<'_>;
}

pub struct QuantWaveNamespace<'a>(&'a LazyFrame);

impl<'a> QuantWaveNamespace<'a> {
    pub fn acos(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<ACOS>(name, "acos")
    }
    pub fn asin(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<ASIN>(name, "asin")
    }
    pub fn atan(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<ATAN>(name, "atan")
    }
    pub fn ceil(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<CEIL>(name, "ceil")
    }
    pub fn cos(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<COS>(name, "cos")
    }
    pub fn cosh(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<COSH>(name, "cosh")
    }
    pub fn exp(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<EXP>(name, "exp")
    }
    pub fn floor(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<FLOOR>(name, "floor")
    }
    pub fn ln(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<LN>(name, "ln")
    }
    pub fn log10(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<LOG10>(name, "log10")
    }
    pub fn sin(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<SIN>(name, "sin")
    }
    pub fn sinh(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<SINH>(name, "sinh")
    }
    pub fn sqrt(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<SQRT>(name, "sqrt")
    }
    pub fn tan(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<TAN>(name, "tan")
    }
    pub fn tanh(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<TANH>(name, "tanh")
    }

    pub fn add(self, in1: &str, in2: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<ADD>(in1, in2, "add")
    }
    pub fn sub(self, in1: &str, in2: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<SUB>(in1, in2, "sub")
    }
    pub fn mult(self, in1: &str, in2: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<MULT>(in1, in2, "mult")
    }
    pub fn div(self, in1: &str, in2: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<DIV>(in1, in2, "div")
    }

    pub fn max(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MAX>(name, period, "max")
    }
    pub fn maxindex(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MAXINDEX>(name, period, "maxindex")
    }
    pub fn min(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MIN>(name, period, "min")
    }
    pub fn minindex(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MININDEX>(name, period, "minindex")
    }
    pub fn sum(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<SUM>(name, period, "sum")
    }

    pub fn sma(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<SMA>(name, period, "sma")
    }
    pub fn ema(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<EMA>(name, period, "ema")
    }
    pub fn wma(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<WMA>(name, period, "wma")
    }
    pub fn dema(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<DEMA>(name, period, "dema")
    }
    pub fn trima(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TRIMA>(name, period, "trima")
    }
    pub fn kama(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<KAMA>(name, period, "kama")
    }
    pub fn midpoint(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MIDPOINT>(name, period, "midpoint")
    }
    pub fn ht_trendline(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<HT_TRENDLINE>(name, "ht_trendline")
    }
    pub fn midprice(self, high: &str, low: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<MIDPRICE>(high, low, period, "midprice")
    }

    pub fn rsi(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<RSI>(name, period, "rsi")
    }
    pub fn mom(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<MOM>(name, period, "mom")
    }
    pub fn roc(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<ROC>(name, period, "roc")
    }
    pub fn rocp(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<ROCP>(name, period, "rocp")
    }
    pub fn rocr(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<ROCR>(name, period, "rocr")
    }
    pub fn rocr100(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<ROCR100>(name, period, "rocr100")
    }
    pub fn trix(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TRIX>(name, period, "trix")
    }
    pub fn cmo(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<CMO>(name, period, "cmo")
    }

    pub fn adx(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<ADX>(high, low, close, period, "adx")
    }
    pub fn adxr(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<ADXR>(high, low, close, period, "adxr")
    }
    pub fn cci(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<CCI>(high, low, close, period, "cci")
    }
    pub fn willr(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<WILLR>(high, low, close, period, "willr")
    }
    pub fn dx(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<DX>(high, low, close, period, "dx")
    }
    pub fn plus_di(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<PLUS_DI>(high, low, close, period, "plus_di")
    }
    pub fn minus_di(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<MINUS_DI>(high, low, close, period, "minus_di")
    }

    pub fn ta_atr(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<TaATR>(high, low, close, period, "ta_atr")
    }
    pub fn ta_natr(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        self.ta_3_in_1_out_period::<TaNATR>(high, low, close, period, "ta_natr")
    }
    pub fn ta_trange(self, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_3_in_1_out_default::<TaTRANGE>(high, low, close, "ta_trange")
    }

    pub fn obv(self, close: &str, volume: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<OBV>(close, volume, "obv")
    }
    pub fn ad(self, high: &str, low: &str, close: &str, volume: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<AD>(high, low, close, volume, "ad")
    }
    pub fn adosc(
        self,
        high: &str,
        low: &str,
        close: &str,
        volume: &str,
        fast: usize,
        slow: usize,
    ) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        let close_str = close.to_string();
        let volume_str = volume.to_string();
        self.0.clone().with_columns([as_struct(vec![
            col(&high_str),
            col(&low_str),
            col(&close_str),
            col(&volume_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_h = ca.field_by_name(&high_str)?;
                let s_l = ca.field_by_name(&low_str)?;
                let s_c = ca.field_by_name(&close_str)?;
                let s_v = ca.field_by_name(&volume_str)?;

                let high = s_h.f64()?;
                let low = s_l.f64()?;
                let close = s_c.f64()?;
                let volume = s_v.f64()?;

                let mut indicator = ADOSC::new(fast, slow);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let v = volume.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l, c, v)));
                }

                Ok(Some(Column::from(Series::new("adosc".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("adosc")])
    }

    pub fn aroon(self, high: &str, low: &str, period: usize) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high_str), col(&low_str)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_h = ca.field_by_name(&high_str)?;
                        let s_l = ca.field_by_name(&low_str)?;
                        let high = s_h.f64()?;
                        let low = s_l.f64()?;

                        let mut indicator = AROON::new(period);
                        let mut up_vals = Vec::with_capacity(s.len());
                        let mut down_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(f64::NAN);
                            let l = low.get(i).unwrap_or(f64::NAN);
                            let (up, down) = indicator.next((h, l));
                            up_vals.push(up);
                            down_vals.push(down);
                        }

                        let s_up = Series::new("aroon_up".into(), up_vals);
                        let s_down = Series::new("aroon_down".into(), down_vals);
                        let struct_series = StructChunked::from_series(
                            "aroon_result".into(),
                            s.len(),
                            [s_up, s_down].iter(),
                        )?;
                        Ok(Some(Column::from(struct_series.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("aroon_up".into(), DataType::Float64),
                        Field::new("aroon_down".into(), DataType::Float64),
                    ])),
                )
                .alias("aroon")])
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stoch(
        self,
        high: &str,
        low: &str,
        close: &str,
        fastk: usize,
        slowk: usize,
        slowk_matype: talib::MaType,
        slowd: usize,
        slowd_matype: talib::MaType,
    ) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        let close_str = close.to_string();
        self.0.clone().with_columns([as_struct(vec![
            col(&high_str),
            col(&low_str),
            col(&close_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_h = ca.field_by_name(&high_str)?;
                let s_l = ca.field_by_name(&low_str)?;
                let s_c = ca.field_by_name(&close_str)?;
                let high = s_h.f64()?;
                let low = s_l.f64()?;
                let close = s_c.f64()?;

                let mut indicator = STOCH::new(fastk, slowk, slowk_matype, slowd, slowd_matype);
                let mut k_vals = Vec::with_capacity(s.len());
                let mut d_vals = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let (k, d) = indicator.next((h, l, c));
                    k_vals.push(k);
                    d_vals.push(d);
                }

                let s_k = Series::new("slowk".into(), k_vals);
                let s_d = Series::new("slowd".into(), d_vals);
                let struct_series =
                    StructChunked::from_series("stoch_result".into(), s.len(), [s_k, s_d].iter())?;
                Ok(Some(Column::from(struct_series.into_series())))
            },
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("slowk".into(), DataType::Float64),
                Field::new("slowd".into(), DataType::Float64),
            ])),
        )
        .alias("stoch")])
    }

    pub fn avgprice(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<AVGPRICE>(open, high, low, close, "avgprice")
    }
    pub fn medprice(self, high: &str, low: &str) -> LazyFrame {
        self.math_operator_2_in_1_out::<MEDPRICE>(high, low, "medprice")
    }
    pub fn typprice(self, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_3_in_1_out_default::<TYPPRICE>(high, low, close, "typprice")
    }
    pub fn wclprice(self, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_3_in_1_out_default::<WCLPRICE>(high, low, close, "wclprice")
    }

    pub fn ht_dcperiod(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<HT_DCPERIOD>(name, "ht_dcperiod")
    }
    pub fn ht_dcphase(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<HT_DCPHASE>(name, "ht_dcphase")
    }
    pub fn ht_trendmode(self, name: &str) -> LazyFrame {
        self.math_transform_1_in_1_out::<HT_TRENDMODE>(name, "ht_trendmode")
    }

    pub fn ta_stddev(self, name: &str, period: usize, nbdev: f64) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = TaSTDDEV::new(period, nbdev);
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }
                    Ok(Some(Column::from(Series::new("ta_stddev".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("ta_stddev")])
    }
    pub fn ta_var(self, name: &str, period: usize, nbdev: f64) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = TaVAR::new(period, nbdev);
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }
                    Ok(Some(Column::from(Series::new("ta_var".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("ta_var")])
    }
    pub fn ta_beta(self, in1: &str, in2: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<TaBETA>(in1, in2, period, "ta_beta")
    }
    pub fn ta_correl(self, in1: &str, in2: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<TaCORREL>(in1, in2, period, "ta_correl")
    }
    pub fn ta_linearreg(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TaLINEARREG>(name, period, "ta_linearreg")
    }
    pub fn ta_linearreg_slope(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TaLINEARREG_SLOPE>(
            name,
            period,
            "ta_linearreg_slope",
        )
    }
    pub fn ta_linearreg_intercept(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TaLINEARREG_INTERCEPT>(
            name,
            period,
            "ta_linearreg_intercept",
        )
    }
    pub fn ta_linearreg_angle(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TaLINEARREG_ANGLE>(
            name,
            period,
            "ta_linearreg_angle",
        )
    }
    pub fn ta_tsf(self, name: &str, period: usize) -> LazyFrame {
        self.math_operator_1_in_1_out_period::<TaTSF>(name, period, "ta_tsf")
    }

    pub fn cdl_2crows(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL2CROWS>(open, high, low, close, "cdl_2crows")
    }
    pub fn cdl_3blackcrows(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3BLACKCROWS>(open, high, low, close, "cdl_3blackcrows")
    }
    pub fn cdl_3inside(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3INSIDE>(open, high, low, close, "cdl_3inside")
    }
    pub fn cdl_3linestrike(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3LINESTRIKE>(open, high, low, close, "cdl_3linestrike")
    }
    pub fn cdl_3outside(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3OUTSIDE>(open, high, low, close, "cdl_3outside")
    }
    pub fn cdl_3starsinsouth(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3STARSINSOUTH>(open, high, low, close, "cdl_3starsinsouth")
    }
    pub fn cdl_3whitesoldiers(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDL3WHITESOLDIERS>(
            open,
            high,
            low,
            close,
            "cdl_3whitesoldiers",
        )
    }
    pub fn cdl_abandonedbaby(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLABANDONEDBABY>(open, high, low, close, "cdl_abandonedbaby")
    }
    pub fn cdl_advanceblock(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLADVANCEBLOCK>(open, high, low, close, "cdl_advanceblock")
    }
    pub fn cdl_belthold(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLBELTHOLD>(open, high, low, close, "cdl_belthold")
    }
    pub fn cdl_breakaway(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLBREAKAWAY>(open, high, low, close, "cdl_breakaway")
    }
    pub fn cdl_closingmarubozu(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLCLOSINGMARUBOZU>(
            open,
            high,
            low,
            close,
            "cdl_closingmarubozu",
        )
    }
    pub fn cdl_concealbabyswall(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLCONCEALBABYSWALL>(
            open,
            high,
            low,
            close,
            "cdl_concealbabyswall",
        )
    }
    pub fn cdl_counterattack(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLCOUNTERATTACK>(open, high, low, close, "cdl_counterattack")
    }
    pub fn cdl_darkcloudcover(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLDARKCLOUDCOVER>(
            open,
            high,
            low,
            close,
            "cdl_darkcloudcover",
        )
    }
    pub fn cdl_doji(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLDOJI>(open, high, low, close, "cdl_doji")
    }
    pub fn cdl_dojistar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLDOJISTAR>(open, high, low, close, "cdl_dojistar")
    }
    pub fn cdl_dragonflydoji(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLDRAGONFLYDOJI>(open, high, low, close, "cdl_dragonflydoji")
    }
    pub fn cdl_engulfing(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLENGULFING>(open, high, low, close, "cdl_engulfing")
    }
    pub fn cdl_eveningdojistar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLEVENINGDOJISTAR>(
            open,
            high,
            low,
            close,
            "cdl_eveningdojistar",
        )
    }
    pub fn cdl_eveningstar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLEVENINGSTAR>(open, high, low, close, "cdl_eveningstar")
    }
    pub fn cdl_gapsidesidewhite(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLGAPSIDESIDEWHITE>(
            open,
            high,
            low,
            close,
            "cdl_gapsidesidewhite",
        )
    }
    pub fn cdl_gravestonedoji(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLGRAVESTONEDOJI>(
            open,
            high,
            low,
            close,
            "cdl_gravestonedoji",
        )
    }
    pub fn cdl_hammer(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHAMMER>(open, high, low, close, "cdl_hammer")
    }
    pub fn cdl_hangingman(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHANGINGMAN>(open, high, low, close, "cdl_hangingman")
    }
    pub fn cdl_harami(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHARAMI>(open, high, low, close, "cdl_harami")
    }
    pub fn cdl_haramicross(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHARAMICROSS>(open, high, low, close, "cdl_haramicross")
    }
    pub fn cdl_highwave(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHIGHWAVE>(open, high, low, close, "cdl_highwave")
    }
    pub fn cdl_hikkake(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHIKKAKE>(open, high, low, close, "cdl_hikkake")
    }
    pub fn cdl_hikkakemod(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHIKKAKEMOD>(open, high, low, close, "cdl_hikkakemod")
    }
    pub fn cdl_homingpigeon(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLHOMINGPIGEON>(open, high, low, close, "cdl_homingpigeon")
    }
    pub fn cdl_identical3crows(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLIDENTICAL3CROWS>(
            open,
            high,
            low,
            close,
            "cdl_identical3crows",
        )
    }
    pub fn cdl_inneck(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLINNECK>(open, high, low, close, "cdl_inneck")
    }
    pub fn cdl_invertedhammer(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLINVERTEDHAMMER>(
            open,
            high,
            low,
            close,
            "cdl_invertedhammer",
        )
    }
    pub fn cdl_kicking(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLKICKING>(open, high, low, close, "cdl_kicking")
    }
    pub fn cdl_kickingbylength(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLKICKINGBYLENGTH>(
            open,
            high,
            low,
            close,
            "cdl_kickingbylength",
        )
    }
    pub fn cdl_ladderbottom(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLLADDERBOTTOM>(open, high, low, close, "cdl_ladderbottom")
    }
    pub fn cdl_longleggeddoji(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLLONGLEGGEDDOJI>(
            open,
            high,
            low,
            close,
            "cdl_longleggeddoji",
        )
    }
    pub fn cdl_longline(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLLONGLINE>(open, high, low, close, "cdl_longline")
    }
    pub fn cdl_marubozu(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLMARUBOZU>(open, high, low, close, "cdl_marubozu")
    }
    pub fn cdl_matchinglow(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLMATCHINGLOW>(open, high, low, close, "cdl_matchinglow")
    }
    pub fn cdl_mathold(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLMATHOLD>(open, high, low, close, "cdl_mathold")
    }
    pub fn cdl_morningdojistar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLMORNINGDOJISTAR>(
            open,
            high,
            low,
            close,
            "cdl_morningdojistar",
        )
    }
    pub fn cdl_morningstar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLMORNINGSTAR>(open, high, low, close, "cdl_morningstar")
    }
    pub fn cdl_onneck(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLONNECK>(open, high, low, close, "cdl_onneck")
    }
    pub fn cdl_piercing(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLPIERCING>(open, high, low, close, "cdl_piercing")
    }
    pub fn cdl_rickshawman(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLRICKSHAWMAN>(open, high, low, close, "cdl_rickshawman")
    }
    pub fn cdl_risefall3methods(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLRISEFALL3METHODS>(
            open,
            high,
            low,
            close,
            "cdl_risefall3methods",
        )
    }
    pub fn cdl_separatinglines(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSEPARATINGLINES>(
            open,
            high,
            low,
            close,
            "cdl_separatinglines",
        )
    }
    pub fn cdl_shootingstar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSHOOTINGSTAR>(open, high, low, close, "cdl_shootingstar")
    }
    pub fn cdl_shortline(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSHORTLINE>(open, high, low, close, "cdl_shortline")
    }
    pub fn cdl_spinningtop(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSPINNINGTOP>(open, high, low, close, "cdl_spinningtop")
    }
    pub fn cdl_stalledpattern(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSTALLEDPATTERN>(
            open,
            high,
            low,
            close,
            "cdl_stalledpattern",
        )
    }
    pub fn cdl_sticksandwich(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLSTICKSANDWICH>(open, high, low, close, "cdl_sticksandwich")
    }
    pub fn cdl_takuri(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLTAKURI>(open, high, low, close, "cdl_takuri")
    }
    pub fn cdl_tasukigap(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLTASUKIGAP>(open, high, low, close, "cdl_tasukigap")
    }
    pub fn cdl_thrusting(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLTHRUSTING>(open, high, low, close, "cdl_thrusting")
    }
    pub fn cdl_tristar(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLTRISTAR>(open, high, low, close, "cdl_tristar")
    }
    pub fn cdl_unique3river(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLUNIQUE3RIVER>(open, high, low, close, "cdl_unique3river")
    }
    pub fn cdl_upsidegap2crows(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLUPSIDEGAP2CROWS>(
            open,
            high,
            low,
            close,
            "cdl_upsidegap2crows",
        )
    }
    pub fn cdl_xsidegap3methods(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<CDLXSIDEGAP3METHODS>(
            open,
            high,
            low,
            close,
            "cdl_xsidegap3methods",
        )
    }

    pub fn macd(self, name: &str, fast: usize, slow: usize, signal: usize) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = MACD::new(fast, slow, signal);
                    let mut macd_vals = Vec::with_capacity(s.len());
                    let mut signal_vals = Vec::with_capacity(s.len());
                    let mut hist_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (m, s_val, h) = indicator.next(val);
                        macd_vals.push(m);
                        signal_vals.push(s_val);
                        hist_vals.push(h);
                    }

                    let s_macd = Series::new("macd".into(), macd_vals);
                    let s_signal = Series::new("macd_signal".into(), signal_vals);
                    let s_hist = Series::new("macd_hist".into(), hist_vals);

                    let struct_series = StructChunked::from_series(
                        "macd_result".into(),
                        s.len(),
                        [s_macd, s_signal, s_hist].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("macd".into(), DataType::Float64),
                    Field::new("macd_signal".into(), DataType::Float64),
                    Field::new("macd_hist".into(), DataType::Float64),
                ])),
            )
            .alias("macd")])
    }

    pub fn bbands(
        self,
        name: &str,
        period: usize,
        nbdevup: f64,
        nbdevdn: f64,
        matype: talib::MaType,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = BBANDS::new(period, nbdevup, nbdevdn, matype);
                    let mut upper_vals = Vec::with_capacity(s.len());
                    let mut middle_vals = Vec::with_capacity(s.len());
                    let mut lower_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (u, m, l) = indicator.next(val);
                        upper_vals.push(u);
                        middle_vals.push(m);
                        lower_vals.push(l);
                    }

                    let s_upper = Series::new("upper".into(), upper_vals);
                    let s_middle = Series::new("middle".into(), middle_vals);
                    let s_lower = Series::new("lower".into(), lower_vals);

                    let struct_series = StructChunked::from_series(
                        "bbands_result".into(),
                        s.len(),
                        [s_upper, s_middle, s_lower].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("upper".into(), DataType::Float64),
                    Field::new("middle".into(), DataType::Float64),
                    Field::new("lower".into(), DataType::Float64),
                ])),
            )
            .alias("bbands")])
    }

    fn ta_3_in_1_out_period<I>(
        self,
        in1: &str,
        in2: &str,
        in3: &str,
        period: usize,
        output_name: &str,
    ) -> LazyFrame
    where
        I: Next<(f64, f64, f64), Output = f64> + Send + Sync + 'static,
        I: From<usize>,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let in3_str = in3.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns(
            [as_struct(vec![col(&in1_str), col(&in2_str), col(&in3_str)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s1 = ca.field_by_name(&in1_str)?;
                        let s2 = ca.field_by_name(&in2_str)?;
                        let s3 = ca.field_by_name(&in3_str)?;

                        let ca1 = s1.f64()?;
                        let ca2 = s2.f64()?;
                        let ca3 = s3.f64()?;

                        let mut indicator = I::from(period);
                        let mut values = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let v1 = ca1.get(i).unwrap_or(f64::NAN);
                            let v2 = ca2.get(i).unwrap_or(f64::NAN);
                            let v3 = ca3.get(i).unwrap_or(f64::NAN);
                            values.push(indicator.next((v1, v2, v3)));
                        }

                        Ok(Some(Column::from(Series::new(
                            output_name_for_closure.clone().into(),
                            values,
                        ))))
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias(&output_name_str)],
        )
    }

    fn ta_3_in_1_out_default<I>(
        self,
        in1: &str,
        in2: &str,
        in3: &str,
        output_name: &str,
    ) -> LazyFrame
    where
        I: Next<(f64, f64, f64), Output = f64> + Default + Send + Sync + 'static,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let in3_str = in3.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns(
            [as_struct(vec![col(&in1_str), col(&in2_str), col(&in3_str)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s1 = ca.field_by_name(&in1_str)?;
                        let s2 = ca.field_by_name(&in2_str)?;
                        let s3 = ca.field_by_name(&in3_str)?;

                        let ca1 = s1.f64()?;
                        let ca2 = s2.f64()?;
                        let ca3 = s3.f64()?;

                        let mut indicator = I::default();
                        let mut values = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let v1 = ca1.get(i).unwrap_or(f64::NAN);
                            let v2 = ca2.get(i).unwrap_or(f64::NAN);
                            let v3 = ca3.get(i).unwrap_or(f64::NAN);
                            values.push(indicator.next((v1, v2, v3)));
                        }

                        Ok(Some(Column::from(Series::new(
                            output_name_for_closure.clone().into(),
                            values,
                        ))))
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias(&output_name_str)],
        )
    }

    fn ta_4_in_1_out_default<I>(
        self,
        in1: &str,
        in2: &str,
        in3: &str,
        in4: &str,
        output_name: &str,
    ) -> LazyFrame
    where
        I: Next<(f64, f64, f64, f64), Output = f64> + Default + Send + Sync + 'static,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let in3_str = in3.to_string();
        let in4_str = in4.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([as_struct(vec![
            col(&in1_str),
            col(&in2_str),
            col(&in3_str),
            col(&in4_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s1 = ca.field_by_name(&in1_str)?;
                let s2 = ca.field_by_name(&in2_str)?;
                let s3 = ca.field_by_name(&in3_str)?;
                let s4 = ca.field_by_name(&in4_str)?;

                let ca1 = s1.f64()?;
                let ca2 = s2.f64()?;
                let ca3 = s3.f64()?;
                let ca4 = s4.f64()?;

                let mut indicator = I::default();
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let v1 = ca1.get(i).unwrap_or(f64::NAN);
                    let v2 = ca2.get(i).unwrap_or(f64::NAN);
                    let v3 = ca3.get(i).unwrap_or(f64::NAN);
                    let v4 = ca4.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((v1, v2, v3, v4)));
                }

                Ok(Some(Column::from(Series::new(
                    output_name_for_closure.clone().into(),
                    values,
                ))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias(&output_name_str)])
    }

    fn math_transform_1_in_1_out<I>(self, name: &str, output_name: &str) -> LazyFrame
    where
        I: Next<f64, Output = f64> + Default + Send + Sync + 'static,
    {
        let name = name.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = I::default();
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new(
                        output_name_for_closure.clone().into(),
                        values,
                    ))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias(&output_name_str)])
    }

    fn math_operator_2_in_1_out<I>(self, in1: &str, in2: &str, output_name: &str) -> LazyFrame
    where
        I: Next<(f64, f64), Output = f64> + Default + Send + Sync + 'static,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0
            .clone()
            .with_columns([as_struct(vec![col(&in1_str), col(&in2_str)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s1 = ca.field_by_name(&in1_str)?;
                        let s2 = ca.field_by_name(&in2_str)?;

                        let ca1 = s1.f64()?;
                        let ca2 = s2.f64()?;

                        let mut indicator = I::default();
                        let mut values = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let v1 = ca1.get(i).unwrap_or(f64::NAN);
                            let v2 = ca2.get(i).unwrap_or(f64::NAN);
                            values.push(indicator.next((v1, v2)));
                        }

                        Ok(Some(Column::from(Series::new(
                            output_name_for_closure.clone().into(),
                            values,
                        ))))
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias(&output_name_str)])
    }

    fn math_operator_1_in_1_out_period<I>(
        self,
        name: &str,
        period: usize,
        output_name: &str,
    ) -> LazyFrame
    where
        I: Next<f64, Output = f64> + Send + Sync + 'static,
        I: From<usize>,
    {
        let name = name.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = I::from(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new(
                        output_name_for_closure.clone().into(),
                        values,
                    ))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias(&output_name_str)])
    }

    fn math_operator_2_in_1_out_period<I>(
        self,
        in1: &str,
        in2: &str,
        period: usize,
        output_name: &str,
    ) -> LazyFrame
    where
        I: Next<(f64, f64), Output = f64> + Send + Sync + 'static,
        I: From<usize>,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0
            .clone()
            .with_columns([as_struct(vec![col(&in1_str), col(&in2_str)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s1 = ca.field_by_name(&in1_str)?;
                        let s2 = ca.field_by_name(&in2_str)?;

                        let ca1 = s1.f64()?;
                        let ca2 = s2.f64()?;

                        let mut indicator = I::from(period);
                        let mut values = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let v1 = ca1.get(i).unwrap_or(f64::NAN);
                            let v2 = ca2.get(i).unwrap_or(f64::NAN);
                            values.push(indicator.next((v1, v2)));
                        }

                        Ok(Some(Column::from(Series::new(
                            output_name_for_closure.clone().into(),
                            values,
                        ))))
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias(&output_name_str)])
    }

    pub fn supertrend(self, period: usize, multiplier: f64) -> LazyFrame {
        self.0
            .clone()
            .with_columns([as_struct(vec![col("high"), col("low"), col("close")])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name("high")?;
                        let s_low = ca.field_by_name("low")?;
                        let s_close = ca.field_by_name("close")?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut st = SuperTrend::new(period, multiplier);
                        let mut values = Vec::with_capacity(s.len());
                        let mut directions = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (val, dir) = st.next((h, l, c));
                            values.push(val);
                            directions.push(dir as f64);
                        }

                        let st_series = Series::new("supertrend".into(), values);
                        let dir_series = Series::new("supertrend_direction".into(), directions);

                        let out = StructChunked::from_series(
                            "supertrend_output".into(),
                            s.len(),
                            [st_series, dir_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("supertrend".into(), DataType::Float64),
                        Field::new("supertrend_direction".into(), DataType::Float64),
                    ])),
                )
                .alias("supertrend_data")])
    }

    pub fn anchored_vwap(self, price: &str, volume: &str, anchor: &str) -> LazyFrame {
        let price = price.to_string();
        let volume = volume.to_string();
        let anchor = anchor.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&price), col(&volume), col(&anchor)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_price = ca.field_by_name(&price)?;
                        let s_volume = ca.field_by_name(&volume)?;
                        let s_anchor = ca.field_by_name(&anchor)?;

                        let price = s_price.f64()?;
                        let volume = s_volume.f64()?;
                        let anchor = s_anchor.bool()?;

                        let mut avwap = quantwave_core::AnchoredVWAP::new();
                        let mut values = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let p = price.get(i).unwrap_or(0.0);
                            let v = volume.get(i).unwrap_or(0.0);
                            let a = anchor.get(i).unwrap_or(false);
                            values.push(avwap.next((p, v, a)));
                        }

                        Ok(Some(Column::from(Series::new(
                            "anchored_vwap".into(),
                            values,
                        ))))
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias("avwap")])
    }

    pub fn hma(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut hma = quantwave_core::HMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(hma.next(val));
                    }

                    Ok(Some(Column::from(Series::new("hma".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("hma")])
    }

    pub fn vpn(
        self,
        high: &str,
        low: &str,
        close: &str,
        volume: &str,
        period: usize,
        smooth_period: usize,
    ) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        let close_str = close.to_string();
        let volume_str = volume.to_string();

        self.0.clone().with_columns([as_struct(vec![
            col(&high_str),
            col(&low_str),
            col(&close_str),
            col(&volume_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_h = ca.field_by_name(&high_str)?;
                let s_l = ca.field_by_name(&low_str)?;
                let s_c = ca.field_by_name(&close_str)?;
                let s_v = ca.field_by_name(&volume_str)?;

                let high = s_h.f64()?;
                let low = s_l.f64()?;
                let close = s_c.f64()?;
                let volume = s_v.f64()?;

                let mut indicator = quantwave_core::VPNIndicator::new(period, smooth_period);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let v = volume.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l, c, v)));
                }

                Ok(Some(Column::from(Series::new("vpn".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("vpn")])
    }

    pub fn gap_momentum(
        self,
        open: &str,
        close: &str,
        period: usize,
        signal_period: usize,
    ) -> LazyFrame {
        let open_str = open.to_string();
        let close_str = close.to_string();

        self.0.clone().with_columns([as_struct(vec![
            col(&open_str),
            col(&close_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_o = ca.field_by_name(&open_str)?;
                let s_c = ca.field_by_name(&close_str)?;

                let open = s_o.f64()?;
                let close = s_c.f64()?;

                let mut indicator = quantwave_core::GapMomentum::new(period, signal_period);
                let mut ratio_vals = Vec::with_capacity(s.len());
                let mut signal_vals = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let o = open.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let (ratio, signal) = indicator.next((o, c));
                    ratio_vals.push(ratio);
                    signal_vals.push(signal);
                }

                let s_ratio = Series::new("gap_ratio".into(), ratio_vals);
                let s_signal = Series::new("gap_signal".into(), signal_vals);
                let struct_series = StructChunked::from_series(
                    "gap_momentum_result".into(),
                    s.len(),
                    [s_ratio, s_signal].iter(),
                )?;
                Ok(Some(Column::from(struct_series.into_series())))
            },
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("gap_ratio".into(), DataType::Float64),
                Field::new("gap_signal".into(), DataType::Float64),
            ])),
        )
        .alias("gap_momentum")])
    }

    pub fn autotune_filter(self, name: &str, window: usize, bandwidth: f64) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = quantwave_core::AutoTuneFilter::new(window, bandwidth);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new("autotune".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("autotune")])
    }

    pub fn adaptive_ema(
        self,
        high: &str,
        low: &str,
        close: &str,
        period: usize,
        pds: usize,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();

        self.0.clone().with_columns([as_struct(vec![col(&h_str), col(&l_str), col(&c_str)])
            .map(
                move |s| {
                    let ca = s.struct_()?;
                    let f_h = ca.field_by_name(&h_str)?;
                    let high = f_h.f64()?;
                    let f_l = ca.field_by_name(&l_str)?;
                    let low = f_l.f64()?;
                    let f_c = ca.field_by_name(&c_str)?;
                    let close = f_c.f64()?;

                    let mut indicator = quantwave_core::AdaptiveEMA::new(period, pds);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let h = high.get(i).unwrap_or(f64::NAN);
                        let l = low.get(i).unwrap_or(f64::NAN);
                        let c = close.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next((h, l, c)));
                    }

                    Ok(Some(Column::from(Series::new("adaptive_ema".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("adaptive_ema")])
    }

    pub fn tradj_ema(
        self,
        high: &str,
        low: &str,
        close: &str,
        period: usize,
        pds: usize,
        mltp: f64,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();

        self.0.clone().with_columns([as_struct(vec![col(&h_str), col(&l_str), col(&c_str)])
            .map(
                move |s| {
                    let ca = s.struct_()?;
                    let f_h = ca.field_by_name(&h_str)?;
                    let high = f_h.f64()?;
                    let f_l = ca.field_by_name(&l_str)?;
                    let low = f_l.f64()?;
                    let f_c = ca.field_by_name(&c_str)?;
                    let close = f_c.f64()?;

                    let mut indicator = quantwave_core::TRAdjEMA::new(period, pds, mltp);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let h = high.get(i).unwrap_or(f64::NAN);
                        let l = low.get(i).unwrap_or(f64::NAN);
                        let c = close.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next((h, l, c)));
                    }

                    Ok(Some(Column::from(Series::new("tradj_ema".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("tradj_ema")])
    }

    pub fn obvm(
        self,
        high: &str,
        low: &str,
        close: &str,
        volume: &str,
        obvm_period: usize,
        signal_period: usize,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();
        let v_str = volume.to_string();

        self.0.clone().with_columns([as_struct(vec![
            col(&h_str),
            col(&l_str),
            col(&c_str),
            col(&v_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let f_h = ca.field_by_name(&h_str)?;
                let high = f_h.f64()?;
                let f_l = ca.field_by_name(&l_str)?;
                let low = f_l.f64()?;
                let f_c = ca.field_by_name(&c_str)?;
                let close = f_c.f64()?;
                let f_v = ca.field_by_name(&v_str)?;
                let volume = f_v.f64()?;

                let mut indicator = quantwave_core::Obvm::new(obvm_period, signal_period);
                let mut obvm_vals = Vec::with_capacity(s.len());
                let mut signal_vals = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let v = volume.get(i).unwrap_or(f64::NAN);
                    let (o, sig) = indicator.next((h, l, c, v));
                    obvm_vals.push(o);
                    signal_vals.push(sig);
                }

                let s_obvm = Series::new("obvm".into(), obvm_vals);
                let s_signal = Series::new("signal".into(), signal_vals);
                let struct_series = StructChunked::from_series(
                    "obvm_data".into(),
                    s.len(),
                    [s_obvm, s_signal].iter(),
                )?;
                Ok(Some(Column::from(struct_series.into_series())))
            },
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("obvm".into(), DataType::Float64),
                Field::new("signal".into(), DataType::Float64),
            ])),
        )
        .alias("obvm_data")])
    }

    pub fn vfi(
        self,
        high: &str,
        low: &str,
        close: &str,
        volume: &str,
        period: usize,
        coef: f64,
        vcoef: f64,
        smoothing_period: usize,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();
        let v_str = volume.to_string();

        self.0.clone().with_columns([as_struct(vec![
            col(&h_str),
            col(&l_str),
            col(&c_str),
            col(&v_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let f_h = ca.field_by_name(&h_str)?;
                let high = f_h.f64()?;
                let f_l = ca.field_by_name(&l_str)?;
                let low = f_l.f64()?;
                let f_c = ca.field_by_name(&c_str)?;
                let close = f_c.f64()?;
                let f_v = ca.field_by_name(&v_str)?;
                let volume = f_v.f64()?;

                let mut indicator = quantwave_core::Vfi::new(period, coef, vcoef, smoothing_period);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let v = volume.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l, c, v)));
                }

                Ok(Some(Column::from(Series::new("vfi".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("vfi")])
    }

    pub fn sve_volatility_bands(
        self,
        high: &str,
        low: &str,
        close: &str,
        bands_period: usize,
        bands_deviation: f64,
        low_band_adjust: f64,
        mid_line_length: usize,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();

        self.0.clone().with_columns([as_struct(vec![col(&h_str), col(&l_str), col(&c_str)])
            .map(
                move |s| {
                    let ca = s.struct_()?;
                    let f_h = ca.field_by_name(&h_str)?;
                    let high = f_h.f64()?;
                    let f_l = ca.field_by_name(&l_str)?;
                    let low = f_l.f64()?;
                    let f_c = ca.field_by_name(&c_str)?;
                    let close = f_c.f64()?;

                    let mut indicator = quantwave_core::SVEVolatilityBands::new(
                        bands_period,
                        bands_deviation,
                        low_band_adjust,
                        mid_line_length,
                    );
                    let mut upper_vals = Vec::with_capacity(s.len());
                    let mut mid_vals = Vec::with_capacity(s.len());
                    let mut lower_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let h = high.get(i).unwrap_or(f64::NAN);
                        let l = low.get(i).unwrap_or(f64::NAN);
                        let c = close.get(i).unwrap_or(f64::NAN);
                        let (upper, mid, lower) = indicator.next((h, l, c));
                        upper_vals.push(upper);
                        mid_vals.push(mid);
                        lower_vals.push(lower);
                    }

                    let s_upper = Series::new("upper".into(), upper_vals);
                    let s_mid = Series::new("middle".into(), mid_vals);
                    let s_lower = Series::new("lower".into(), lower_vals);
                    let struct_series = StructChunked::from_series(
                        "sve_bands_data".into(),
                        s.len(),
                        [s_upper, s_mid, s_lower].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("upper".into(), DataType::Float64),
                    Field::new("middle".into(), DataType::Float64),
                    Field::new("lower".into(), DataType::Float64),
                ])),
            )
            .alias("sve_bands_data")])
    }

    pub fn exp_dev_bands(
        self,
        name: &str,
        period: usize,
        multiplier: f64,
        use_sma: bool,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = quantwave_core::ExpDevBands::new(period, multiplier, use_sma);
                    let mut upper_vals = Vec::with_capacity(s.len());
                    let mut mid_vals = Vec::with_capacity(s.len());
                    let mut lower_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (upper, mid, lower) = indicator.next(val);
                        upper_vals.push(upper);
                        mid_vals.push(mid);
                        lower_vals.push(lower);
                    }

                    let s_upper = Series::new("upper".into(), upper_vals);
                    let s_mid = Series::new("middle".into(), mid_vals);
                    let s_lower = Series::new("lower".into(), lower_vals);
                    let struct_series = StructChunked::from_series(
                        "exp_dev_bands_data".into(),
                        s.len(),
                        [s_upper, s_mid, s_lower].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("upper".into(), DataType::Float64),
                    Field::new("middle".into(), DataType::Float64),
                    Field::new("lower".into(), DataType::Float64),
                ])),
            )
            .alias("exp_dev_bands_data")])
    }

    pub fn sdo(
        self,
        name: &str,
        lookback_period: usize,
        period: usize,
        ema_pds: usize,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = quantwave_core::SDO::new(lookback_period, period, ema_pds);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new("sdo".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("sdo")])
    }

    pub fn rsmk(self, price: &str, benchmark: &str, length: usize, ema_length: usize) -> LazyFrame {
        let p_str = price.to_string();
        let b_str = benchmark.to_string();

        self.0.clone().with_columns([as_struct(vec![col(&p_str), col(&b_str)])
            .map(
                move |s| {
                    let ca = s.struct_()?;
                    let f_p = ca.field_by_name(&p_str)?;
                    let price = f_p.f64()?;
                    let f_b = ca.field_by_name(&b_str)?;
                    let benchmark = f_b.f64()?;

                    let mut indicator = quantwave_core::RSMK::new(length, ema_length);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let p = price.get(i).unwrap_or(f64::NAN);
                        let b = benchmark.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next((p, b)));
                    }

                    Ok(Some(Column::from(Series::new("rsmk".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("rsmk")])
    }

    pub fn rodc(
        self,
        name: &str,
        window_size: usize,
        threshold: f64,
        smooth_period: usize,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = quantwave_core::RODC::new(window_size, threshold, smooth_period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new("rodc".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("rodc")])
    }

    pub fn reverse_ema(self, name: &str, alpha: f64) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = quantwave_core::ReverseEMA::new(alpha);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new("reverse_ema".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("reverse_ema")])
    }

    pub fn harrington_adx(
        self,
        high: &str,
        low: &str,
        close: &str,
        adx_length: usize,
        adx_smooth_length: usize,
    ) -> LazyFrame {
        let h_str = high.to_string();
        let l_str = low.to_string();
        let c_str = close.to_string();

        self.0.clone().with_columns([as_struct(vec![col(&h_str), col(&l_str), col(&c_str)])
            .map(
                move |s| {
                    let ca = s.struct_()?;
                    let f_h = ca.field_by_name(&h_str)?;
                    let high = f_h.f64()?;
                    let f_l = ca.field_by_name(&l_str)?;
                    let low = f_l.f64()?;
                    let f_c = ca.field_by_name(&c_str)?;
                    let close = f_c.f64()?;

                    let mut indicator = quantwave_core::HarringtonADXOscillator::new(adx_length, adx_smooth_length);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let h = high.get(i).unwrap_or(f64::NAN);
                        let l = low.get(i).unwrap_or(f64::NAN);
                        let c = close.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next((h, l, c)));
                    }

                    Ok(Some(Column::from(Series::new("harrington_adx".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("harrington_adx")])
    }

    pub fn keltner_channels(
        self,
        high: &str,
        low: &str,
        close: &str,
        ema_period: usize,
        atr_period: usize,
        multiplier: f64,
    ) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut kc = quantwave_core::KeltnerChannels::new(
                            ema_period, atr_period, multiplier,
                        );
                        let mut uppers = Vec::with_capacity(s.len());
                        let mut middles = Vec::with_capacity(s.len());
                        let mut lowers = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (upper, middle, lower) = kc.next((h, l, c));
                            uppers.push(upper);
                            middles.push(middle);
                            lowers.push(lower);
                        }

                        let upper_series = Series::new("upper".into(), uppers);
                        let middle_series = Series::new("middle".into(), middles);
                        let lower_series = Series::new("lower".into(), lowers);

                        let out = StructChunked::from_series(
                            "keltner_output".into(),
                            s.len(),
                            [upper_series, middle_series, lower_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("upper".into(), DataType::Float64),
                        Field::new("middle".into(), DataType::Float64),
                        Field::new("lower".into(), DataType::Float64),
                    ])),
                )
                .alias("keltner_data")])
    }

    pub fn alma(self, name: &str, period: usize, offset: f64, sigma: f64) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut alma = quantwave_core::ALMA::new(period, offset, sigma);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(alma.next(val));
                    }

                    Ok(Some(Column::from(Series::new("alma".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("alma")])
    }

    pub fn donchian_channels(self, high: &str, low: &str, period: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;

                        let mut dc = quantwave_core::DonchianChannels::new(period);
                        let mut uppers = Vec::with_capacity(s.len());
                        let mut middles = Vec::with_capacity(s.len());
                        let mut lowers = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let (upper, middle, lower) = dc.next((h, l));
                            uppers.push(upper);
                            middles.push(middle);
                            lowers.push(lower);
                        }

                        let upper_series = Series::new("upper".into(), uppers);
                        let middle_series = Series::new("middle".into(), middles);
                        let lower_series = Series::new("lower".into(), lowers);

                        let out = StructChunked::from_series(
                            "donchian_output".into(),
                            s.len(),
                            [upper_series, middle_series, lower_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("upper".into(), DataType::Float64),
                        Field::new("middle".into(), DataType::Float64),
                        Field::new("lower".into(), DataType::Float64),
                    ])),
                )
                .alias("donchian_data")])
    }

    pub fn ttm_squeeze(
        self,
        high: &str,
        low: &str,
        close: &str,
        period: usize,
        multiplier_bb: f64,
        multiplier_kc: f64,
    ) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut ttm =
                            quantwave_core::TTMSqueeze::new(period, multiplier_bb, multiplier_kc);
                        let mut histograms = Vec::with_capacity(s.len());
                        let mut squeezed = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (hist, is_sq) = ttm.next((h, l, c));
                            histograms.push(hist);
                            squeezed.push(is_sq);
                        }

                        let hist_series = Series::new("histogram".into(), histograms);
                        let squeezed_series = Series::new("is_squeezed".into(), squeezed);

                        let out = StructChunked::from_series(
                            "ttm_squeeze_output".into(),
                            s.len(),
                            [hist_series, squeezed_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("histogram".into(), DataType::Float64),
                        Field::new("is_squeezed".into(), DataType::Boolean),
                    ])),
                )
                .alias("ttm_squeeze_data")])
    }

    pub fn vortex_indicator(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut vi = quantwave_core::VortexIndicator::new(period);
                        let mut plus_vals = Vec::with_capacity(s.len());
                        let mut minus_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (plus, minus) = vi.next((h, l, c));
                            plus_vals.push(plus);
                            minus_vals.push(minus);
                        }

                        let plus_series = Series::new("vi_plus".into(), plus_vals);
                        let minus_series = Series::new("vi_minus".into(), minus_vals);

                        let out = StructChunked::from_series(
                            "vortex_output".into(),
                            s.len(),
                            [plus_series, minus_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("vi_plus".into(), DataType::Float64),
                        Field::new("vi_minus".into(), DataType::Float64),
                    ])),
                )
                .alias("vortex_data")])
    }

    pub fn heikin_ashi(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        let open = open.to_string();
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([as_struct(vec![
            col(&open),
            col(&high),
            col(&low),
            col(&close),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_open = ca.field_by_name(&open)?;
                let s_high = ca.field_by_name(&high)?;
                let s_low = ca.field_by_name(&low)?;
                let s_close = ca.field_by_name(&close)?;

                let open = s_open.f64()?;
                let high = s_high.f64()?;
                let low = s_low.f64()?;
                let close = s_close.f64()?;

                let mut ha = quantwave_core::HeikinAshi::new();
                let mut ha_opens = Vec::with_capacity(s.len());
                let mut ha_highs = Vec::with_capacity(s.len());
                let mut ha_lows = Vec::with_capacity(s.len());
                let mut ha_closes = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let o = open.get(i).unwrap_or(0.0);
                    let h = high.get(i).unwrap_or(0.0);
                    let l = low.get(i).unwrap_or(0.0);
                    let c = close.get(i).unwrap_or(0.0);
                    let (ha_o, ha_h, ha_l, ha_c) = ha.next((o, h, l, c));
                    ha_opens.push(ha_o);
                    ha_highs.push(ha_h);
                    ha_lows.push(ha_l);
                    ha_closes.push(ha_c);
                }

                let o_series = Series::new("ha_open".into(), ha_opens);
                let h_series = Series::new("ha_high".into(), ha_highs);
                let l_series = Series::new("ha_low".into(), ha_lows);
                let c_series = Series::new("ha_close".into(), ha_closes);

                let out = StructChunked::from_series(
                    "heikin_ashi_output".into(),
                    s.len(),
                    [o_series, h_series, l_series, c_series].iter(),
                )?;
                Ok(Some(Column::from(out.into_series())))
            },
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("ha_open".into(), DataType::Float64),
                Field::new("ha_high".into(), DataType::Float64),
                Field::new("ha_low".into(), DataType::Float64),
                Field::new("ha_close".into(), DataType::Float64),
            ])),
        )
        .alias("heikin_ashi_data")])
    }

    pub fn wavetrend(
        self,
        high: &str,
        low: &str,
        close: &str,
        n1: usize,
        n2: usize,
        n3: usize,
    ) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut wt = quantwave_core::WaveTrend::new(n1, n2, n3);
                        let mut wt1_vals = Vec::with_capacity(s.len());
                        let mut wt2_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (wt1, wt2) = wt.next((h, l, c));
                            wt1_vals.push(wt1);
                            wt2_vals.push(wt2);
                        }

                        let wt1_series = Series::new("wt1".into(), wt1_vals);
                        let wt2_series = Series::new("wt2".into(), wt2_vals);

                        let out = StructChunked::from_series(
                            "wavetrend_output".into(),
                            s.len(),
                            [wt1_series, wt2_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("wt1".into(), DataType::Float64),
                        Field::new("wt2".into(), DataType::Float64),
                    ])),
                )
                .alias("wavetrend_data")])
    }

    pub fn tema(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut tema = quantwave_core::TEMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(tema.next(val));
                    }

                    Ok(Some(Column::from(Series::new("tema".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("tema")])
    }

    pub fn zlema(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([col(&name)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut zlema = quantwave_core::ZLEMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(zlema.next(val));
                    }

                    Ok(Some(Column::from(Series::new("zlema".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("zlema")])
    }

    pub fn atr_trailing_stop(
        self,
        high: &str,
        low: &str,
        close: &str,
        period: usize,
        multiplier: f64,
    ) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut atr_ts = quantwave_core::ATRTrailingStop::new(period, multiplier);
                        let mut stops = Vec::with_capacity(s.len());
                        let mut directions = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (stop, dir) = atr_ts.next((h, l, c));
                            stops.push(stop);
                            directions.push(dir as f64);
                        }

                        let stop_series = Series::new("stop".into(), stops);
                        let dir_series = Series::new("direction".into(), directions);

                        let out = StructChunked::from_series(
                            "atr_ts_output".into(),
                            s.len(),
                            [stop_series, dir_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("stop".into(), DataType::Float64),
                        Field::new("direction".into(), DataType::Float64),
                    ])),
                )
                .alias("atr_ts_data")])
    }

    pub fn pivot_points(self, high: &str, low: &str, close: &str) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low), col(&close)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;
                        let s_close = ca.field_by_name(&close)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;
                        let close = s_close.f64()?;

                        let mut pivot = quantwave_core::PivotPoints::new();
                        let mut p_vals = Vec::with_capacity(s.len());
                        let mut r1_vals = Vec::with_capacity(s.len());
                        let mut s1_vals = Vec::with_capacity(s.len());
                        let mut r2_vals = Vec::with_capacity(s.len());
                        let mut s2_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let c = close.get(i).unwrap_or(0.0);
                            let (p, r1, s1, r2, s2) = pivot.next((h, l, c));
                            p_vals.push(p);
                            r1_vals.push(r1);
                            s1_vals.push(s1);
                            r2_vals.push(r2);
                            s2_vals.push(s2);
                        }

                        let p_series = Series::new("p".into(), p_vals);
                        let r1_series = Series::new("r1".into(), r1_vals);
                        let s1_series = Series::new("s1".into(), s1_vals);
                        let r2_series = Series::new("r2".into(), r2_vals);
                        let s2_series = Series::new("s2".into(), s2_vals);

                        let out = StructChunked::from_series(
                            "pivot_output".into(),
                            s.len(),
                            [p_series, r1_series, s1_series, r2_series, s2_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("p".into(), DataType::Float64),
                        Field::new("r1".into(), DataType::Float64),
                        Field::new("s1".into(), DataType::Float64),
                        Field::new("r2".into(), DataType::Float64),
                        Field::new("s2".into(), DataType::Float64),
                    ])),
                )
                .alias("pivot_points_data")])
    }

    pub fn bill_williams_fractals(self, high: &str, low: &str) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;

                        let mut fractals = quantwave_core::BillWilliamsFractals::new();
                        let mut bearish_vals = Vec::with_capacity(s.len());
                        let mut bullish_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let (bear, bull) = fractals.next((h, l));
                            bearish_vals.push(bear);
                            bullish_vals.push(bull);
                        }

                        let bearish_series = Series::new("bearish".into(), bearish_vals);
                        let bullish_series = Series::new("bullish".into(), bullish_vals);

                        let out = StructChunked::from_series(
                            "fractals_output".into(),
                            s.len(),
                            [bearish_series, bullish_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("bearish".into(), DataType::Boolean),
                        Field::new("bullish".into(), DataType::Boolean),
                    ])),
                )
                .alias("fractals_data")])
    }

    pub fn ichimoku_cloud(
        self,
        high: &str,
        low: &str,
        p1: usize,
        p2: usize,
        p3: usize,
    ) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0
            .clone()
            .with_columns([as_struct(vec![col(&high), col(&low)])
                .map(
                    move |s| {
                        let ca = s.struct_()?;
                        let s_high = ca.field_by_name(&high)?;
                        let s_low = ca.field_by_name(&low)?;

                        let high = s_high.f64()?;
                        let low = s_low.f64()?;

                        let mut ic = quantwave_core::IchimokuCloud::new(p1, p2, p3);
                        let mut t_vals = Vec::with_capacity(s.len());
                        let mut k_vals = Vec::with_capacity(s.len());
                        let mut sa_vals = Vec::with_capacity(s.len());
                        let mut sb_vals = Vec::with_capacity(s.len());

                        for i in 0..s.len() {
                            let h = high.get(i).unwrap_or(0.0);
                            let l = low.get(i).unwrap_or(0.0);
                            let (t, k, sa, sb) = ic.next((h, l));
                            t_vals.push(t);
                            k_vals.push(k);
                            sa_vals.push(sa);
                            sb_vals.push(sb);
                        }

                        let t_series = Series::new("tenkan".into(), t_vals);
                        let k_series = Series::new("kijun".into(), k_vals);
                        let sa_series = Series::new("senkou_a".into(), sa_vals);
                        let sb_series = Series::new("senkou_b".into(), sb_vals);

                        let out = StructChunked::from_series(
                            "ichimoku_output".into(),
                            s.len(),
                            [t_series, k_series, sa_series, sb_series].iter(),
                        )?;
                        Ok(Some(Column::from(out.into_series())))
                    },
                    GetOutput::from_type(DataType::Struct(vec![
                        Field::new("tenkan".into(), DataType::Float64),
                        Field::new("kijun".into(), DataType::Float64),
                        Field::new("senkou_a".into(), DataType::Float64),
                        Field::new("senkou_b".into(), DataType::Float64),
                    ])),
                )
                .alias("ichimoku_data")])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polars_heikin_ashi() -> PolarsResult<()> {
        let df = df![
            "open" => [10.0, 11.0],
            "high" => [12.0, 13.0],
            "low" => [8.0, 10.0],
            "close" => [11.0, 12.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .heikin_ashi("open", "high", "low", "close")
            .collect()?;

        let ha = out.column("heikin_ashi_data")?.struct_()?;
        assert_eq!(
            ha.field_by_name("ha_open".into())?.f64()?.get(0),
            Some(10.5)
        );
        assert_eq!(
            ha.field_by_name("ha_close".into())?.f64()?.get(0),
            Some(10.25)
        );

        Ok(())
    }

    #[test]
    fn test_polars_tema_zlema() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0]
        ]?;

        let out = df.clone().lazy().ta().tema("price", 3).collect()?;

        let tema = out.column("tema")?.f64()?;
        assert!(tema.get(4).is_some());

        let out2 = df.lazy().ta().zlema("price", 3).collect()?;

        let zlema = out2.column("zlema")?.f64()?;
        assert!(zlema.get(4).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_atr_ts() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .atr_trailing_stop("high", "low", "close", 14, 2.5)
            .collect()?;

        let atr_ts = out.column("atr_ts_data")?.struct_()?;
        assert!(atr_ts.field_by_name("stop".into())?.f64()?.get(0).is_some());
        assert!(
            atr_ts
                .field_by_name("direction".into())?
                .f64()?
                .get(0)
                .is_some()
        );

        Ok(())
    }

    #[test]
    fn test_polars_pivot_points() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .pivot_points("high", "low", "close")
            .collect()?;

        let pivot = out.column("pivot_points_data")?.struct_()?;
        assert!(pivot.field_by_name("p".into())?.f64()?.get(0).is_some());
        assert!(pivot.field_by_name("r1".into())?.f64()?.get(0).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_fractals() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 11.0, 15.0, 12.0, 10.0],
            "low" => [5.0, 6.0, 2.0, 6.0, 7.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .bill_williams_fractals("high", "low")
            .collect()?;

        let fractals = out.column("fractals_data")?.struct_()?;
        assert!(
            fractals
                .field_by_name("bearish".into())?
                .bool()?
                .get(4)
                .unwrap()
        );
        assert!(
            fractals
                .field_by_name("bullish".into())?
                .bool()?
                .get(4)
                .unwrap()
        );

        Ok(())
    }

    #[test]
    fn test_polars_ichimoku() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 11.0, 15.0, 12.0, 10.0],
            "low" => [5.0, 6.0, 2.0, 6.0, 7.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .ichimoku_cloud("high", "low", 9, 26, 52)
            .collect()?;

        let ichimoku = out.column("ichimoku_data")?.struct_()?;
        assert!(
            ichimoku
                .field_by_name("tenkan".into())?
                .f64()?
                .get(4)
                .is_some()
        );
        assert!(
            ichimoku
                .field_by_name("kijun".into())?
                .f64()?
                .get(4)
                .is_some()
        );

        Ok(())
    }

    #[test]
    fn test_polars_wavetrend() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .wavetrend("high", "low", "close", 10, 21, 4)
            .collect()?;

        let wt = out.column("wavetrend_data")?.struct_()?;
        assert!(wt.field_by_name("wt1".into())?.f64()?.get(0).is_some());
        assert!(wt.field_by_name("wt2".into())?.f64()?.get(0).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_vortex() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .vortex_indicator("high", "low", "close", 14)
            .collect()?;

        let vortex = out.column("vortex_data")?.struct_()?;
        assert!(
            vortex
                .field_by_name("vi_plus".into())?
                .f64()?
                .get(0)
                .is_some()
        );
        assert!(
            vortex
                .field_by_name("vi_minus".into())?
                .f64()?
                .get(0)
                .is_some()
        );

        Ok(())
    }

    #[test]
    fn test_polars_ttm_squeeze() -> PolarsResult<()> {
        let df = df![
            "high" => [11.0, 12.0, 13.0, 14.0],
            "low" => [9.0, 10.0, 11.0, 12.0],
            "close" => [10.0, 11.0, 12.0, 13.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .ttm_squeeze("high", "low", "close", 20, 2.0, 1.5)
            .collect()?;

        let ttm = out.column("ttm_squeeze_data")?.struct_()?;
        assert!(
            ttm.field_by_name("histogram".into())?
                .f64()?
                .get(0)
                .is_some()
        );
        assert!(
            ttm.field_by_name("is_squeezed".into())?
                .bool()?
                .get(0)
                .is_some()
        );

        Ok(())
    }

    #[test]
    fn test_polars_donchian() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0, 13.0, 15.0],
            "low" => [8.0, 7.0, 9.0, 10.0, 12.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .donchian_channels("high", "low", 3)
            .collect()?;

        let donchian = out.column("donchian_data")?.struct_()?;
        // bar 4: H=13, L=10. Window (12,7), (11,9), (13,10). Upper=13, Lower=7, Middle=10
        assert_eq!(
            donchian.field_by_name("upper".into())?.f64()?.get(3),
            Some(13.0)
        );
        assert_eq!(
            donchian.field_by_name("middle".into())?.f64()?.get(3),
            Some(10.0)
        );
        assert_eq!(
            donchian.field_by_name("lower".into())?.f64()?.get(3),
            Some(7.0)
        );

        Ok(())
    }

    #[test]
    fn test_polars_alma() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        ]?;

        let out = df.lazy().ta().alma("price", 9, 0.85, 6.0).collect()?;

        let alma = out.column("alma")?.f64()?;
        assert!(alma.get(9).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_keltner() -> PolarsResult<()> {
        let df = df![
            "high" => [12.0],
            "low" => [8.0],
            "close" => [10.0]
        ]?;

        let out = df
            .lazy()
            .ta()
            .keltner_channels("high", "low", "close", 3, 3, 2.0)
            .collect()?;

        let keltner = out.column("keltner_data")?.struct_()?;
        assert_eq!(
            keltner.field_by_name("middle".into())?.f64()?.get(0),
            Some(10.0)
        );
        assert_eq!(
            keltner.field_by_name("upper".into())?.f64()?.get(0),
            Some(18.0)
        );
        assert_eq!(
            keltner.field_by_name("lower".into())?.f64()?.get(0),
            Some(2.0)
        );

        Ok(())
    }

    #[test]
    fn test_polars_hma() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        ]?;

        let out = df.lazy().ta().hma("price", 4).collect()?;

        let hma = out.column("hma")?.f64()?;
        assert!(hma.get(9).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_anchored_vwap() -> PolarsResult<()> {
        let df = df![
            "price" => [10.0, 12.0, 15.0, 16.0],
            "volume" => [100.0, 200.0, 100.0, 100.0],
            "anchor" => [false, false, true, false]
        ]?;

        let out = df
            .lazy()
            .ta()
            .anchored_vwap("price", "volume", "anchor")
            .collect()?;

        let avwap = out.column("avwap")?.f64()?;
        assert_eq!(avwap.get(0), Some(10.0));
        assert_eq!(avwap.get(1), Some(11.333333333333334));
        assert_eq!(avwap.get(2), Some(15.0));
        assert_eq!(avwap.get(3), Some(15.5));

        Ok(())
    }

    #[test]
    fn test_polars_math_transforms() -> PolarsResult<()> {
        let df = df![
            "val" => [0.0, 1.5707963267948966] // 0, PI/2
        ]?;

        let out = df.lazy().ta().sin("val").collect()?;

        let sin = out.column("sin")?.f64()?;
        assert!((sin.get(0).unwrap() - 0.0).abs() < 1e-10);
        assert!((sin.get(1).unwrap() - 1.0).abs() < 1e-10);

        Ok(())
    }

    #[test]
    fn test_polars_math_operators() -> PolarsResult<()> {
        let df = df![
            "v1" => [10.0, 20.0],
            "v2" => [5.0, 30.0]
        ]?;

        let out = df.lazy().ta().add("v1", "v2").ta().max("v1", 2).collect()?;

        let add = out.column("add")?.f64()?;
        assert_eq!(add.get(0), Some(15.0));
        assert_eq!(add.get(1), Some(50.0));

        let max = out.column("max")?.f64()?;
        assert_eq!(max.get(1), Some(20.0));

        Ok(())
    }

    #[test]
    fn test_polars_vpn() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 11.0, 12.0],
            "low" => [9.0, 10.0, 11.0],
            "close" => [9.5, 10.5, 11.5],
            "volume" => [1000.0, 1100.0, 1200.0]
        ]?;

        let out = df.lazy().ta().vpn("high", "low", "close", "volume", 30, 3).collect()?;
        let vpn = out.column("vpn")?.f64()?;
        assert!(vpn.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_gap_momentum() -> PolarsResult<()> {
        let df = df![
            "open" => [10.0, 11.0, 10.0],
            "close" => [10.5, 10.5, 9.5]
        ]?;

        let out = df.lazy().ta().gap_momentum("open", "close", 10, 5).collect()?;
        let gm = out.column("gap_momentum")?.struct_()?;
        assert!(gm.field_by_name("gap_ratio".into())?.f64()?.get(2).is_some());
        assert!(gm.field_by_name("gap_signal".into())?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_autotune() -> PolarsResult<()> {
        let df = df![
            "price" => [100.0; 50]
        ]?;

        let out = df.lazy().ta().autotune_filter("price", 20, 0.25).collect()?;
        let at = out.column("autotune")?.f64()?;
        assert!(at.get(49).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_adaptive_ema() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0, 10.5], "l" => [9.0, 10.0, 9.5], "c" => [9.5, 10.5, 10.0]]?;
        let out = df.lazy().ta().adaptive_ema("h", "l", "c", 10, 2).collect()?;
        assert!(out.column("adaptive_ema")?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_obvm() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0], "l" => [9.0, 10.0], "c" => [9.5, 10.5], "v" => [100.0, 200.0]]?;
        let out = df.lazy().ta().obvm("h", "l", "c", "v", 10, 3).collect()?;
        let data = out.column("obvm_data")?.struct_()?;
        assert!(data.field_by_name("obvm".into())?.f64()?.get(1).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_vfi() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0], "l" => [9.0, 10.0], "c" => [9.5, 10.5], "v" => [100.0, 200.0]]?;
        let out = df.lazy().ta().vfi("h", "l", "c", "v", 10, 0.2, 2.5, 3).collect()?;
        assert!(out.column("vfi")?.f64()?.get(1).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_sdo() -> PolarsResult<()> {
        let df = df!["p" => [10.0, 11.0, 12.0]]?;
        let out = df.lazy().ta().sdo("p", 2, 5, 3).collect()?;
        assert!(out.column("sdo")?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_rsmk() -> PolarsResult<()> {
        let df = df!["p" => [10.0, 11.0], "b" => [100.0, 101.0]]?;
        let out = df.lazy().ta().rsmk("p", "b", 90, 3).collect()?;
        assert!(out.column("rsmk")?.f64()?.get(1).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_rodc() -> PolarsResult<()> {
        let df = df!["p" => [10.0, 11.0, 10.0, 11.0, 12.0]]?;
        let out = df.lazy().ta().rodc("p", 10, 0.5, 3).collect()?;
        assert!(out.column("rodc")?.f64()?.get(4).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_reverse_ema() -> PolarsResult<()> {
        let df = df!["p" => [10.0, 11.0, 12.0]]?;
        let out = df.lazy().ta().reverse_ema("p", 0.1).collect()?;
        assert!(out.column("reverse_ema")?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_harrington_adx() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0, 12.0], "l" => [9.0, 10.0, 11.0], "c" => [9.5, 10.5, 11.5]]?;
        let out = df.lazy().ta().harrington_adx("h", "l", "c", 10, 1).collect()?;
        assert!(out.column("harrington_adx")?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_tradj_ema() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0, 10.5], "l" => [9.0, 10.0, 9.5], "c" => [9.5, 10.5, 10.0]]?;
        let out = df.lazy().ta().tradj_ema("h", "l", "c", 10, 2, 0.5).collect()?;
        assert!(out.column("tradj_ema")?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_sve_volatility_bands() -> PolarsResult<()> {
        let df = df!["h" => [10.0, 11.0, 10.5], "l" => [9.0, 10.0, 9.5], "c" => [9.5, 10.5, 10.0]]?;
        let out = df.lazy().ta().sve_volatility_bands("h", "l", "c", 10, 1.5, 1.0, 3).collect()?;
        let data = out.column("sve_bands_data")?.struct_()?;
        assert!(data.field_by_name("upper".into())?.f64()?.get(2).is_some());
        Ok(())
    }

    #[test]
    fn test_polars_exp_dev_bands() -> PolarsResult<()> {
        let df = df!["p" => [10.0, 11.0, 12.0, 11.0, 10.0]]?;
        let out = df.lazy().ta().exp_dev_bands("p", 10, 2.0, true).collect()?;
        let data = out.column("exp_dev_bands_data")?.struct_()?;
        assert!(data.field_by_name("upper".into())?.f64()?.get(4).is_some());
        Ok(())
    }
}

impl QuantWaveExt for LazyFrame {
    fn ta(&self) -> QuantWaveNamespace<'_> {
        QuantWaveNamespace(self)
    }
}
