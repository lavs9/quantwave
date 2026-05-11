use polars::prelude::*;
use quantwave_core::SuperTrend;
use quantwave_core::traits::Next;

pub trait QuantWaveExt {
    fn ta(&self) -> QuantWaveNamespace<'_>;
}

pub struct QuantWaveNamespace<'a>(&'a LazyFrame);

impl<'a> QuantWaveNamespace<'a> {
    pub fn supertrend(self, period: usize, multiplier: f64) -> LazyFrame {
        self.0.clone().with_columns([
            as_struct(vec![col("high"), col("low"), col("close")])
                .map(move |s| {
                    let ca = s.struct_()?;
                    let s_high = ca.field_by_name("high".into())?;
                    let s_low = ca.field_by_name("low".into())?;
                    let s_close = ca.field_by_name("close".into())?;
                    
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
}

impl QuantWaveExt for LazyFrame {
    fn ta(&self) -> QuantWaveNamespace<'_> {
        QuantWaveNamespace(self)
    }
}
