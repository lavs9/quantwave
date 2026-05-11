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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

impl QuantWaveExt for LazyFrame {
    fn ta(&self) -> QuantWaveNamespace<'_> {
        QuantWaveNamespace(self)
    }
}
