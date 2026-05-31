import os

CODE = """
    pub fn macdext(
        self,
        name: &str,
        fastperiod: usize,
        fastmatype: talib::MaType,
        slowperiod: usize,
        slowmatype: talib::MaType,
        signalperiod: usize,
        signalmatype: talib::MaType,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = MACDEXT::new(
                        fastperiod,
                        fastmatype,
                        slowperiod,
                        slowmatype,
                        signalperiod,
                        signalmatype,
                    );
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
                        "macdext_result".into(),
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
            .alias("macdext")])
    }

    pub fn macdfix(self, name: &str, signalperiod: usize) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = MACDFIX::new(signalperiod);
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
                        "macdfix_result".into(),
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
            .alias("macdfix")])
    }

    pub fn stochf(
        self,
        high: &str,
        low: &str,
        close: &str,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: talib::MaType,
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

                let mut indicator = STOCHF::new(fastk_period, fastd_period, fastd_matype);
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

                let s_k = Series::new("fastk".into(), k_vals);
                let s_d = Series::new("fastd".into(), d_vals);
                let struct_series = StructChunked::from_series(
                    "stochf_result".into(),
                    s.len(),
                    [s_k, s_d].iter(),
                )?;
                Ok(Some(Column::from(struct_series.into_series())))
            },
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("fastk".into(), DataType::Float64),
                Field::new("fastd".into(), DataType::Float64),
            ])),
        )
        .alias("stochf")])
    }

    pub fn stochrsi(
        self,
        name: &str,
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: talib::MaType,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = STOCHRSI::new(timeperiod, fastk_period, fastd_period, fastd_matype);
                    let mut k_vals = Vec::with_capacity(s.len());
                    let mut d_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (k, d) = indicator.next(val);
                        k_vals.push(k);
                        d_vals.push(d);
                    }

                    let s_k = Series::new("fastk".into(), k_vals);
                    let s_d = Series::new("fastd".into(), d_vals);

                    let struct_series = StructChunked::from_series(
                        "stochrsi_result".into(),
                        s.len(),
                        [s_k, s_d].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("fastk".into(), DataType::Float64),
                    Field::new("fastd".into(), DataType::Float64),
                ])),
            )
            .alias("stochrsi")])
    }

    pub fn apo(
        self,
        name: &str,
        fastperiod: usize,
        slowperiod: usize,
        matype: talib::MaType,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = APO::new(fastperiod, slowperiod, matype);
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }
                    Ok(Some(Column::from(Series::new("apo".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("apo")])
    }

    pub fn ppo(
        self,
        name: &str,
        fastperiod: usize,
        slowperiod: usize,
        matype: talib::MaType,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = PPO::new(fastperiod, slowperiod, matype);
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }
                    Ok(Some(Column::from(Series::new("ppo".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("ppo")])
    }

    pub fn bop(self, open: &str, high: &str, low: &str, close: &str) -> LazyFrame {
        self.ta_4_in_1_out_default::<BOP>(open, high, low, close, "bop")
    }

    pub fn aroonosc(self, high: &str, low: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<AROONOSC>(high, low, period, "aroonosc")
    }

    pub fn mfi(
        self,
        high: &str,
        low: &str,
        close: &str,
        volume: &str,
        period: usize,
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

                let mut indicator = MFI::new(period);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    let v = volume.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l, c, v)));
                }

                Ok(Some(Column::from(Series::new("mfi".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("mfi")])
    }

    pub fn ultosc(
        self,
        high: &str,
        low: &str,
        close: &str,
        timeperiod1: usize,
        timeperiod2: usize,
        timeperiod3: usize,
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

                let mut indicator = ULTOSC::new(timeperiod1, timeperiod2, timeperiod3);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    let c = close.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l, c)));
                }

                Ok(Some(Column::from(Series::new("ultosc".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("ultosc")])
    }

    pub fn plus_dm(self, high: &str, low: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<PLUS_DM>(high, low, period, "plus_dm")
    }

    pub fn minus_dm(self, high: &str, low: &str, period: usize) -> LazyFrame {
        self.math_operator_2_in_1_out_period::<MINUS_DM>(high, low, period, "minus_dm")
    }

    pub fn t3(
        self,
        name: &str,
        period: usize,
        v_factor: f64,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = T3::new(period, v_factor);
                    let mut values = Vec::with_capacity(s.len());
                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        values.push(indicator.next(val));
                    }
                    Ok(Some(Column::from(Series::new("t3".into(), values))))
                },
                GetOutput::from_type(DataType::Float64),
            )
            .alias("t3")])
    }

    pub fn mama(
        self,
        name: &str,
        fastlimit: f64,
        slowlimit: f64,
    ) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = MAMA::new(fastlimit, slowlimit);
                    let mut mama_vals = Vec::with_capacity(s.len());
                    let mut fama_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (m, f) = indicator.next(val);
                        mama_vals.push(m);
                        fama_vals.push(f);
                    }

                    let s_mama = Series::new("mama".into(), mama_vals);
                    let s_fama = Series::new("fama".into(), fama_vals);

                    let struct_series = StructChunked::from_series(
                        "mama_result".into(),
                        s.len(),
                        [s_mama, s_fama].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("mama".into(), DataType::Float64),
                    Field::new("fama".into(), DataType::Float64),
                ])),
            )
            .alias("mama")])
    }

    pub fn sar(
        self,
        high: &str,
        low: &str,
        acceleration: f64,
        maximum: f64,
    ) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        self.0.clone().with_columns([as_struct(vec![
            col(&high_str),
            col(&low_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_h = ca.field_by_name(&high_str)?;
                let s_l = ca.field_by_name(&low_str)?;
                let high = s_h.f64()?;
                let low = s_l.f64()?;

                let mut indicator = SAR::new(acceleration, maximum);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l)));
                }

                Ok(Some(Column::from(Series::new("sar".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("sar")])
    }

    #[allow(clippy::too_many_arguments)]
    pub fn sarext(
        self,
        high: &str,
        low: &str,
        startvalue: f64,
        offsetonreverse: f64,
        accelerationinitlong: f64,
        accelerationlong: f64,
        accelerationmaxlong: f64,
        accelerationinitshort: f64,
        accelerationshort: f64,
        accelerationmaxshort: f64,
    ) -> LazyFrame {
        let high_str = high.to_string();
        let low_str = low.to_string();
        self.0.clone().with_columns([as_struct(vec![
            col(&high_str),
            col(&low_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_h = ca.field_by_name(&high_str)?;
                let s_l = ca.field_by_name(&low_str)?;
                let high = s_h.f64()?;
                let low = s_l.f64()?;

                let mut indicator = SAREXT::new(
                    startvalue,
                    offsetonreverse,
                    accelerationinitlong,
                    accelerationlong,
                    accelerationmaxlong,
                    accelerationinitshort,
                    accelerationshort,
                    accelerationmaxshort,
                );
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let h = high.get(i).unwrap_or(f64::NAN);
                    let l = low.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((h, l)));
                }

                Ok(Some(Column::from(Series::new("sarext".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("sarext")])
    }

    pub fn mavp(
        self,
        in1: &str,
        in2: &str,
        minperiod: usize,
        maxperiod: usize,
        matype: talib::MaType,
    ) -> LazyFrame {
        let in1_str = in1.to_string();
        let in2_str = in2.to_string();
        self.0.clone().with_columns([as_struct(vec![
            col(&in1_str),
            col(&in2_str),
        ])
        .map(
            move |s| {
                let ca = s.struct_()?;
                let s_1 = ca.field_by_name(&in1_str)?;
                let s_2 = ca.field_by_name(&in2_str)?;
                let in1_ca = s_1.f64()?;
                let in2_ca = s_2.f64()?;

                let mut indicator = MAVP::new(minperiod, maxperiod, matype);
                let mut values = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let i1 = in1_ca.get(i).unwrap_or(f64::NAN);
                    let i2 = in2_ca.get(i).unwrap_or(f64::NAN);
                    values.push(indicator.next((i1, i2)));
                }

                Ok(Some(Column::from(Series::new("mavp".into(), values))))
            },
            GetOutput::from_type(DataType::Float64),
        )
        .alias("mavp")])
    }

    pub fn ht_phasor(self, name: &str) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = HT_PHASOR::new();
                    let mut inphase_vals = Vec::with_capacity(s.len());
                    let mut quadrature_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (inp, q) = indicator.next(val);
                        inphase_vals.push(inp);
                        quadrature_vals.push(q);
                    }

                    let s_inphase = Series::new("inphase".into(), inphase_vals);
                    let s_quadrature = Series::new("quadrature".into(), quadrature_vals);

                    let struct_series = StructChunked::from_series(
                        "ht_phasor_result".into(),
                        s.len(),
                        [s_inphase, s_quadrature].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("inphase".into(), DataType::Float64),
                    Field::new("quadrature".into(), DataType::Float64),
                ])),
            )
            .alias("ht_phasor")])
    }

    pub fn ht_sine(self, name: &str) -> LazyFrame {
        let name_str = name.to_string();
        self.0.clone().with_columns([col(&name_str)
            .map(
                move |s| {
                    let ca = s.f64()?;
                    let mut indicator = HT_SINE::new();
                    let mut sine_vals = Vec::with_capacity(s.len());
                    let mut leadsine_vals = Vec::with_capacity(s.len());

                    for i in 0..s.len() {
                        let val = ca.get(i).unwrap_or(f64::NAN);
                        let (si, ls) = indicator.next(val);
                        sine_vals.push(si);
                        leadsine_vals.push(ls);
                    }

                    let s_sine = Series::new("sine".into(), sine_vals);
                    let s_leadsine = Series::new("leadsine".into(), leadsine_vals);

                    let struct_series = StructChunked::from_series(
                        "ht_sine_result".into(),
                        s.len(),
                        [s_sine, s_leadsine].iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("sine".into(), DataType::Float64),
                    Field::new("leadsine".into(), DataType::Float64),
                ])),
            )
            .alias("ht_sine")])
    }
"""

MARKET_STRUCTURE_FIX = """
                    let struct_series = StructChunked::from_series(
                        "market_structure_result".into(),
                        s.len(),
                        [
                            s_bias, s_lhp, s_lhb, s_llp, s_llb, s_hasf, s_fb, s_fp, s_fba, s_fstr,
                            s_dep, s_bar,
                        ]
                        .iter(),
                    )?;
                    Ok(Some(Column::from(struct_series.into_series())))
                },
                GetOutput::from_type(DataType::Struct(vec![
                    Field::new("bias".into(), DataType::UInt32),
                    Field::new("last_high_price".into(), DataType::Float64),
                    Field::new("last_high_bar".into(), DataType::UInt64),
                    Field::new("last_low_price".into(), DataType::Float64),
                    Field::new("last_low_bar".into(), DataType::UInt64),
                    Field::new("has_flip".into(), DataType::Boolean),
                    Field::new("flip_bearish".into(), DataType::Boolean),
                    Field::new("flip_price".into(), DataType::Float64),
                    Field::new("flip_bar".into(), DataType::UInt64),
                    Field::new("flip_strength".into(), DataType::UInt32),
                    Field::new("swing_depth".into(), DataType::UInt32),
                    Field::new("bar_index".into(), DataType::UInt64),
                ])),
            )
            .alias("market_structure")])
"""

with open("quantwave-polars/src/lib.rs", "r") as f:
    content = f.read()

# Replace the broken market structure tail with the correct fix
marker = 'let s_bar = Series::new("bar_index".into(), bars);\n\n    }'
if marker in content:
    content = content.replace(marker, 'let s_bar = Series::new("bar_index".into(), bars);\n' + MARKET_STRUCTURE_FIX + '\n    }')

# Insert the wrappers before fn ta_3_in_1_out_period
insert_index = content.find("    fn ta_3_in_1_out_period<I>(")
if insert_index != -1:
    new_content = content[:insert_index] + CODE + "\n" + content[insert_index:]
    with open("quantwave-polars/src/lib.rs", "w") as f:
        f.write(new_content)
    print("Success")
else:
    print("Could not find insertion point.")
