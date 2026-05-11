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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

impl QuantWaveExt for LazyFrame {
    fn ta(&self) -> QuantWaveNamespace<'_> {
        QuantWaveNamespace(self)
    }
}
