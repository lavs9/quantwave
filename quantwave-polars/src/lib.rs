use polars::prelude::*;
use quantwave_core::*;
use quantwave_core::traits::Next;

pub trait QuantWaveExt {
    fn ta(&self) -> QuantWaveNamespace<'_>;
}

pub struct QuantWaveNamespace<'a>(&'a LazyFrame);

impl<'a> QuantWaveNamespace<'a> {
    pub fn acos(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<ACOS>(name, "acos") }
    pub fn asin(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<ASIN>(name, "asin") }
    pub fn atan(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<ATAN>(name, "atan") }
    pub fn ceil(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<CEIL>(name, "ceil") }
    pub fn cos(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<COS>(name, "cos") }
    pub fn cosh(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<COSH>(name, "cosh") }
    pub fn exp(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<EXP>(name, "exp") }
    pub fn floor(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<FLOOR>(name, "floor") }
    pub fn ln(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<LN>(name, "ln") }
    pub fn log10(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<LOG10>(name, "log10") }
    pub fn sin(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<SIN>(name, "sin") }
    pub fn sinh(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<SINH>(name, "sinh") }
    pub fn sqrt(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<SQRT>(name, "sqrt") }
    pub fn tan(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<TAN>(name, "tan") }
    pub fn tanh(self, name: &str) -> LazyFrame { self.math_transform_1_in_1_out::<TANH>(name, "tanh") }

    pub fn add(self, in1: &str, in2: &str) -> LazyFrame { self.math_operator_2_in_1_out::<ADD>(in1, in2, "add") }
    pub fn sub(self, in1: &str, in2: &str) -> LazyFrame { self.math_operator_2_in_1_out::<SUB>(in1, in2, "sub") }
    pub fn mult(self, in1: &str, in2: &str) -> LazyFrame { self.math_operator_2_in_1_out::<MULT>(in1, in2, "mult") }
    pub fn div(self, in1: &str, in2: &str) -> LazyFrame { self.math_operator_2_in_1_out::<DIV>(in1, in2, "div") }

    pub fn max(self, name: &str, period: usize) -> LazyFrame { self.math_operator_1_in_1_out_period::<MAX>(name, period, "max") }
    pub fn maxindex(self, name: &str, period: usize) -> LazyFrame { self.math_operator_1_in_1_out_period::<MAXINDEX>(name, period, "maxindex") }
    pub fn min(self, name: &str, period: usize) -> LazyFrame { self.math_operator_1_in_1_out_period::<MIN>(name, period, "min") }
    pub fn minindex(self, name: &str, period: usize) -> LazyFrame { self.math_operator_1_in_1_out_period::<MININDEX>(name, period, "minindex") }
    pub fn sum(self, name: &str, period: usize) -> LazyFrame { self.math_operator_1_in_1_out_period::<SUM>(name, period, "sum") }

    fn math_transform_1_in_1_out<I>(self, name: &str, output_name: &str) -> LazyFrame
    where
        I: Next<f64, Output = f64> + Default + Send + Sync + 'static,
    {
        let name = name.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut indicator = I::default();
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new(output_name_for_closure.clone().into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias(&output_name_str)
        ])
    }

    fn math_operator_2_in_1_out<I>(self, in1: &str, in2: &str, output_name: &str) -> LazyFrame
    where
        I: Next<(f64, f64), Output = f64> + Default + Send + Sync + 'static,
    {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([
            as_struct(vec![col(&in1_str), col(&in2_str)])
                .map(move |s| {
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

                    Ok(Some(Column::from(Series::new(output_name_for_closure.clone().into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias(&output_name_str)
        ])
    }

    fn math_operator_1_in_1_out_period<I>(self, name: &str, period: usize, output_name: &str) -> LazyFrame
    where
        I: Next<f64, Output = f64> + Send + Sync + 'static,
        I: From<usize>,
    {
        let name = name.to_string();
        let output_name_str = output_name.to_string();
        let output_name_for_closure = output_name_str.clone();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut indicator = I::from(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }

                    Ok(Some(Column::from(Series::new(output_name_for_closure.clone().into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias(&output_name_str)
        ])
    }

    pub fn supertrend(self, period: usize, multiplier: f64) -> LazyFrame {
        self.0.clone().with_columns([
            as_struct(vec![col("high"), col("low"), col("close")])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("supertrend_output".into(), s.len(), [st_series, dir_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("supertrend".into(), DataType::Float64),
                    Field::new("supertrend_direction".into(), DataType::Float64),
                ])))
                .alias("supertrend_data")
        ])
    }

    pub fn anchored_vwap(self, price: &str, volume: &str, anchor: &str) -> LazyFrame {
        let price = price.to_string();
        let volume = volume.to_string();
        let anchor = anchor.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&price), col(&volume), col(&anchor)])
                .map(move |s| {
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

                    Ok(Some(Column::from(Series::new("anchored_vwap".into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias("avwap")
        ])
    }

    pub fn hma(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut hma = quantwave_core::HMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(hma.next(val));
                    }

                    Ok(Some(Column::from(Series::new("hma".into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias("hma")
        ])
    }

    pub fn keltner_channels(self, high: &str, low: &str, close: &str, ema_period: usize, atr_period: usize, multiplier: f64) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
                    let ca = s.struct_()?;
                    let s_high = ca.field_by_name(&high)?;
                    let s_low = ca.field_by_name(&low)?;
                    let s_close = ca.field_by_name(&close)?;
                    
                    let high = s_high.f64()?;
                    let low = s_low.f64()?;
                    let close = s_close.f64()?;

                    let mut kc = quantwave_core::KeltnerChannels::new(ema_period, atr_period, multiplier);
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
                    
                    let out = StructChunked::from_series("keltner_output".into(), s.len(), [upper_series, middle_series, lower_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("upper".into(), DataType::Float64),
                    Field::new("middle".into(), DataType::Float64),
                    Field::new("lower".into(), DataType::Float64),
                ])))
                .alias("keltner_data")
        ])
    }

    pub fn alma(self, name: &str, period: usize, offset: f64, sigma: f64) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut alma = quantwave_core::ALMA::new(period, offset, sigma);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(alma.next(val));
                    }

                    Ok(Some(Column::from(Series::new("alma".into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias("alma")
        ])
    }

    pub fn donchian_channels(self, high: &str, low: &str, period: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("donchian_output".into(), s.len(), [upper_series, middle_series, lower_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("upper".into(), DataType::Float64),
                    Field::new("middle".into(), DataType::Float64),
                    Field::new("lower".into(), DataType::Float64),
                ])))
                .alias("donchian_data")
        ])
    }

    pub fn ttm_squeeze(self, high: &str, low: &str, close: &str, period: usize, multiplier_bb: f64, multiplier_kc: f64) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
                    let ca = s.struct_()?;
                    let s_high = ca.field_by_name(&high)?;
                    let s_low = ca.field_by_name(&low)?;
                    let s_close = ca.field_by_name(&close)?;
                    
                    let high = s_high.f64()?;
                    let low = s_low.f64()?;
                    let close = s_close.f64()?;

                    let mut ttm = quantwave_core::TTMSqueeze::new(period, multiplier_bb, multiplier_kc);
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
                    
                    let out = StructChunked::from_series("ttm_squeeze_output".into(), s.len(), [hist_series, squeezed_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("histogram".into(), DataType::Float64),
                    Field::new("is_squeezed".into(), DataType::Boolean),
                ])))
                .alias("ttm_squeeze_data")
        ])
    }

    pub fn vortex_indicator(self, high: &str, low: &str, close: &str, period: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("vortex_output".into(), s.len(), [plus_series, minus_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("vi_plus".into(), DataType::Float64),
                    Field::new("vi_minus".into(), DataType::Float64),
                ])))
                .alias("vortex_data")
        ])
    }

    pub fn heikin_ashi(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        let open = open.to_string();
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&open), col(&high), col(&low), col(&close)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("heikin_ashi_output".into(), s.len(), [o_series, h_series, l_series, c_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("ha_open".into(), DataType::Float64),
                    Field::new("ha_high".into(), DataType::Float64),
                    Field::new("ha_low".into(), DataType::Float64),
                    Field::new("ha_close".into(), DataType::Float64),
                ])))
                .alias("heikin_ashi_data")
        ])
    }

    pub fn wavetrend(self, high: &str, low: &str, close: &str, n1: usize, n2: usize, n3: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("wavetrend_output".into(), s.len(), [wt1_series, wt2_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("wt1".into(), DataType::Float64),
                    Field::new("wt2".into(), DataType::Float64),
                ])))
                .alias("wavetrend_data")
        ])
    }

    pub fn tema(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut tema = quantwave_core::TEMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(tema.next(val));
                    }

                    Ok(Some(Column::from(Series::new("tema".into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias("tema")
        ])
    }

    pub fn zlema(self, name: &str, period: usize) -> LazyFrame {
        let name = name.to_string();
        self.0.clone().with_columns([
            col(&name)
                .map(move |s| {
                    let ca = s.f64()?;
                    let mut zlema = quantwave_core::ZLEMA::new(period);
                    let mut values = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(0.0);
                        values.push(zlema.next(val));
                    }

                    Ok(Some(Column::from(Series::new("zlema".into(), values))))
                }, GetOutput::from_type(DataType::Float64))
                .alias("zlema")
        ])
    }

    pub fn atr_trailing_stop(self, high: &str, low: &str, close: &str, period: usize, multiplier: f64) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("atr_ts_output".into(), s.len(), [stop_series, dir_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("stop".into(), DataType::Float64),
                    Field::new("direction".into(), DataType::Float64),
                ])))
                .alias("atr_ts_data")
        ])
    }

    pub fn pivot_points(self, high: &str, low: &str, close: &str) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();
        let close = close.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low), col(&close)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("pivot_output".into(), s.len(), [p_series, r1_series, s1_series, r2_series, s2_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("p".into(), DataType::Float64),
                    Field::new("r1".into(), DataType::Float64),
                    Field::new("s1".into(), DataType::Float64),
                    Field::new("r2".into(), DataType::Float64),
                    Field::new("s2".into(), DataType::Float64),
                ])))
                .alias("pivot_points_data")
        ])
    }

    pub fn bill_williams_fractals(self, high: &str, low: &str) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("fractals_output".into(), s.len(), [bearish_series, bullish_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("bearish".into(), DataType::Boolean),
                    Field::new("bullish".into(), DataType::Boolean),
                ])))
                .alias("fractals_data")
        ])
    }

    pub fn ichimoku_cloud(self, high: &str, low: &str, p1: usize, p2: usize, p3: usize) -> LazyFrame {
        let high = high.to_string();
        let low = low.to_string();

        self.0.clone().with_columns([
            as_struct(vec![col(&high), col(&low)])
                .map(move |s| {
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
                    
                    let out = StructChunked::from_series("ichimoku_output".into(), s.len(), [t_series, k_series, sa_series, sb_series].iter())?;
                    Ok(Some(Column::from(out.into_series())))
                }, GetOutput::from_type(DataType::Struct(vec![
                    Field::new("tenkan".into(), DataType::Float64),
                    Field::new("kijun".into(), DataType::Float64),
                    Field::new("senkou_a".into(), DataType::Float64),
                    Field::new("senkou_b".into(), DataType::Float64),
                ])))
                .alias("ichimoku_data")
        ])
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

        let out = df.lazy()
            .ta()
            .heikin_ashi("open", "high", "low", "close")
            .collect()?;

        let ha = out.column("heikin_ashi_data")?.struct_()?;
        assert_eq!(ha.field_by_name("ha_open".into())?.f64()?.get(0), Some(10.5));
        assert_eq!(ha.field_by_name("ha_close".into())?.f64()?.get(0), Some(10.25));

        Ok(())
    }

    #[test]
    fn test_polars_tema_zlema() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0]
        ]?;

        let out = df.clone().lazy()
            .ta()
            .tema("price", 3)
            .collect()?;

        let tema = out.column("tema")?.f64()?;
        assert!(tema.get(4).is_some());

        let out2 = df.lazy()
            .ta()
            .zlema("price", 3)
            .collect()?;

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

        let out = df.lazy()
            .ta()
            .atr_trailing_stop("high", "low", "close", 14, 2.5)
            .collect()?;

        let atr_ts = out.column("atr_ts_data")?.struct_()?;
        assert!(atr_ts.field_by_name("stop".into())?.f64()?.get(0).is_some());
        assert!(atr_ts.field_by_name("direction".into())?.f64()?.get(0).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_pivot_points() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df.lazy()
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

        let out = df.lazy()
            .ta()
            .bill_williams_fractals("high", "low")
            .collect()?;

        let fractals = out.column("fractals_data")?.struct_()?;
        assert!(fractals.field_by_name("bearish".into())?.bool()?.get(4).unwrap());
        assert!(fractals.field_by_name("bullish".into())?.bool()?.get(4).unwrap());

        Ok(())
    }

    #[test]
    fn test_polars_ichimoku() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 11.0, 15.0, 12.0, 10.0],
            "low" => [5.0, 6.0, 2.0, 6.0, 7.0]
        ]?;

        let out = df.lazy()
            .ta()
            .ichimoku_cloud("high", "low", 9, 26, 52)
            .collect()?;

        let ichimoku = out.column("ichimoku_data")?.struct_()?;
        assert!(ichimoku.field_by_name("tenkan".into())?.f64()?.get(4).is_some());
        assert!(ichimoku.field_by_name("kijun".into())?.f64()?.get(4).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_wavetrend() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0],
            "low" => [8.0, 10.0, 9.0],
            "close" => [9.0, 11.0, 10.0]
        ]?;

        let out = df.lazy()
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

        let out = df.lazy()
            .ta()
            .vortex_indicator("high", "low", "close", 14)
            .collect()?;

        let vortex = out.column("vortex_data")?.struct_()?;
        assert!(vortex.field_by_name("vi_plus".into())?.f64()?.get(0).is_some());
        assert!(vortex.field_by_name("vi_minus".into())?.f64()?.get(0).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_ttm_squeeze() -> PolarsResult<()> {
        let df = df![
            "high" => [11.0, 12.0, 13.0, 14.0],
            "low" => [9.0, 10.0, 11.0, 12.0],
            "close" => [10.0, 11.0, 12.0, 13.0]
        ]?;

        let out = df.lazy()
            .ta()
            .ttm_squeeze("high", "low", "close", 20, 2.0, 1.5)
            .collect()?;

        let ttm = out.column("ttm_squeeze_data")?.struct_()?;
        assert!(ttm.field_by_name("histogram".into())?.f64()?.get(0).is_some());
        assert!(ttm.field_by_name("is_squeezed".into())?.bool()?.get(0).is_some());

        Ok(())
    }

    #[test]
    fn test_polars_donchian() -> PolarsResult<()> {
        let df = df![
            "high" => [10.0, 12.0, 11.0, 13.0, 15.0],
            "low" => [8.0, 7.0, 9.0, 10.0, 12.0]
        ]?;

        let out = df.lazy()
            .ta()
            .donchian_channels("high", "low", 3)
            .collect()?;

        let donchian = out.column("donchian_data")?.struct_()?;
        // bar 4: H=13, L=10. Window (12,7), (11,9), (13,10). Upper=13, Lower=7, Middle=10
        assert_eq!(donchian.field_by_name("upper".into())?.f64()?.get(3), Some(13.0));
        assert_eq!(donchian.field_by_name("middle".into())?.f64()?.get(3), Some(10.0));
        assert_eq!(donchian.field_by_name("lower".into())?.f64()?.get(3), Some(7.0));

        Ok(())
    }

    #[test]
    fn test_polars_alma() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        ]?;

        let out = df.lazy()
            .ta()
            .alma("price", 9, 0.85, 6.0)
            .collect()?;

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

        let out = df.lazy()
            .ta()
            .keltner_channels("high", "low", "close", 3, 3, 2.0)
            .collect()?;

        let keltner = out.column("keltner_data")?.struct_()?;
        assert_eq!(keltner.field_by_name("middle".into())?.f64()?.get(0), Some(10.0));
        assert_eq!(keltner.field_by_name("upper".into())?.f64()?.get(0), Some(18.0));
        assert_eq!(keltner.field_by_name("lower".into())?.f64()?.get(0), Some(2.0));

        Ok(())
    }

    #[test]
    fn test_polars_hma() -> PolarsResult<()> {
        let df = df![
            "price" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        ]?;

        let out = df.lazy()
            .ta()
            .hma("price", 4)
            .collect()?;

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

        let out = df.lazy()
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

        let out = df.lazy()
            .ta()
            .sin("val")
            .collect()?;

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

        let out = df.lazy()
            .ta()
            .add("v1", "v2")
            .ta()
            .max("v1", 2)
            .collect()?;

        let add = out.column("add")?.f64()?;
        assert_eq!(add.get(0), Some(15.0));
        assert_eq!(add.get(1), Some(50.0));

        let max = out.column("max")?.f64()?;
        assert_eq!(max.get(1), Some(20.0));

        Ok(())
    }
}

impl QuantWaveExt for LazyFrame {
    fn ta(&self) -> QuantWaveNamespace<'_> {
        QuantWaveNamespace(self)
    }
}
