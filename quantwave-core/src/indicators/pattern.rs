talib_cdl!(CDLDOJI, talib_rs::pattern::cdl_doji);
talib_cdl!(CDLHAMMER, talib_rs::pattern::cdl_hammer);
talib_cdl!(CDLENGULFING, talib_rs::pattern::cdl_engulfing);
talib_cdl!(CDLCLOSINGMARUBOZU, talib_rs::pattern::cdl_closingmarubozu);
talib_cdl!(CDLDRAGONFLYDOJI, talib_rs::pattern::cdl_dragonflydoji);
talib_cdl!(CDLGRAVESTONEDOJI, talib_rs::pattern::cdl_gravestonedoji);
talib_cdl!(CDLHIGHWAVE, talib_rs::pattern::cdl_highwave);
talib_cdl!(CDLLONGLEGGEDDOJI, talib_rs::pattern::cdl_longleggeddoji);
talib_cdl!(CDLLONGLINE, talib_rs::pattern::cdl_longline);
talib_cdl!(CDLMARUBOZU, talib_rs::pattern::cdl_marubozu);
talib_cdl!(CDLRICKSHAWMAN, talib_rs::pattern::cdl_rickshawman);
talib_cdl!(CDLSHORTLINE, talib_rs::pattern::cdl_shortline);
talib_cdl!(CDLSPINNINGTOP, talib_rs::pattern::cdl_spinningtop);
talib_cdl!(CDLTAKURI, talib_rs::pattern::cdl_takuri);
talib_cdl!(CDL2CROWS, talib_rs::pattern::cdl_2crows);
talib_cdl!(CDLCOUNTERATTACK, talib_rs::pattern::cdl_counterattack);
talib_cdl!(CDLDARKCLOUDCOVER, talib_rs::pattern::cdl_darkcloudcover);
talib_cdl!(CDLDOJISTAR, talib_rs::pattern::cdl_dojistar);
talib_cdl!(CDLHANGINGMAN, talib_rs::pattern::cdl_hangingman);
talib_cdl!(CDLHARAMI, talib_rs::pattern::cdl_harami);
talib_cdl!(CDLHARAMICROSS, talib_rs::pattern::cdl_haramicross);
talib_cdl!(CDLHIKKAKE, talib_rs::pattern::cdl_hikkake);
talib_cdl!(CDLHIKKAKEMOD, talib_rs::pattern::cdl_hikkakemod);
talib_cdl!(CDLHOMINGPIGEON, talib_rs::pattern::cdl_homingpigeon);
talib_cdl!(CDLINNECK, talib_rs::pattern::cdl_inneck);
talib_cdl!(CDLINVERTEDHAMMER, talib_rs::pattern::cdl_invertedhammer);
talib_cdl!(CDLKICKING, talib_rs::pattern::cdl_kicking);
talib_cdl!(CDLKICKINGBYLENGTH, talib_rs::pattern::cdl_kickingbylength);
talib_cdl!(CDLMATCHINGLOW, talib_rs::pattern::cdl_matchinglow);
talib_cdl!(CDLONNECK, talib_rs::pattern::cdl_onneck);
talib_cdl!(CDLPIERCING, talib_rs::pattern::cdl_piercing);
talib_cdl!(CDLSEPARATINGLINES, talib_rs::pattern::cdl_separatinglines);
talib_cdl!(CDLSHOOTINGSTAR, talib_rs::pattern::cdl_shootingstar);
talib_cdl!(CDLSTICKSANDWICH, talib_rs::pattern::cdl_sticksandwich);
talib_cdl!(CDLTHRUSTING, talib_rs::pattern::cdl_thrusting);
talib_cdl!(CDLBELTHOLD, talib_rs::pattern::cdl_belthold);
talib_cdl!(CDL3BLACKCROWS, talib_rs::pattern::cdl_3blackcrows);
talib_cdl!(CDL3INSIDE, talib_rs::pattern::cdl_3inside);
talib_cdl!(CDL3LINESTRIKE, talib_rs::pattern::cdl_3linestrike);
talib_cdl!(CDL3OUTSIDE, talib_rs::pattern::cdl_3outside);
talib_cdl!(CDL3STARSINSOUTH, talib_rs::pattern::cdl_3starsinsouth);
talib_cdl!(CDL3WHITESOLDIERS, talib_rs::pattern::cdl_3whitesoldiers);
talib_cdl!(CDLABANDONEDBABY, talib_rs::pattern::cdl_abandonedbaby);
talib_cdl!(CDLADVANCEBLOCK, talib_rs::pattern::cdl_advanceblock);
talib_cdl!(CDLBREAKAWAY, talib_rs::pattern::cdl_breakaway);
talib_cdl!(CDLCONCEALBABYSWALL, talib_rs::pattern::cdl_concealbabyswall);
talib_cdl!(CDLEVENINGDOJISTAR, talib_rs::pattern::cdl_eveningdojistar);
talib_cdl!(CDLEVENINGSTAR, talib_rs::pattern::cdl_eveningstar);
talib_cdl!(CDLGAPSIDESIDEWHITE, talib_rs::pattern::cdl_gapsidesidewhite);
talib_cdl!(CDLIDENTICAL3CROWS, talib_rs::pattern::cdl_identical3crows);
talib_cdl!(CDLLADDERBOTTOM, talib_rs::pattern::cdl_ladderbottom);
talib_cdl!(CDLMATHOLD, talib_rs::pattern::cdl_mathold);
talib_cdl!(CDLMORNINGDOJISTAR, talib_rs::pattern::cdl_morningdojistar);
talib_cdl!(CDLMORNINGSTAR, talib_rs::pattern::cdl_morningstar);
talib_cdl!(CDLRISEFALL3METHODS, talib_rs::pattern::cdl_risefall3methods);
talib_cdl!(CDLSTALLEDPATTERN, talib_rs::pattern::cdl_stalledpattern);
talib_cdl!(CDLTASUKIGAP, talib_rs::pattern::cdl_tasukigap);
talib_cdl!(CDLTRISTAR, talib_rs::pattern::cdl_tristar);
talib_cdl!(CDLUNIQUE3RIVER, talib_rs::pattern::cdl_unique3river);
talib_cdl!(CDLUPSIDEGAP2CROWS, talib_rs::pattern::cdl_upsidegap2crows);
talib_cdl!(CDLXSIDEGAP3METHODS, talib_rs::pattern::cdl_xsidegap3methods);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cdl_doji_parity(
            o in prop::collection::vec(10.0..100.0, 1..100),
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            
            let mut doji = CDLDOJI::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| doji.next((o[i], h[i], l[i], c[i]))).collect();
            let batch_results = talib_rs::pattern::cdl_doji(&o[..len], &h[..len], &l[..len], &c[..len]).unwrap_or_else(|_| vec![0; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }

        #[test]
        fn test_cdl_hammer_parity(
            o in prop::collection::vec(10.0..100.0, 1..100),
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            
            let mut hammer = CDLHAMMER::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| hammer.next((o[i], h[i], l[i], c[i]))).collect();
            let batch_results = talib_rs::pattern::cdl_hammer(&o[..len], &h[..len], &l[..len], &c[..len]).unwrap_or_else(|_| vec![0; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }
    }
}
