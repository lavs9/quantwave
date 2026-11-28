//! Node.js / WASM integration tests (`wasm-pack test --node quantwave-wasm`).
//! Verifies that the compiled WASM module runs inside Node.js and produces
//! bit-identical output to `quantwave-core` and gold-standard fixtures.

use quantwave_core::SuperTrend as CoreSuperTrend;
use quantwave_core::indicators::ichimoku::IchimokuCloud as CoreIchimoku;
use quantwave_core::indicators::incremental::bbands::BBANDS as CoreBbands;
use quantwave_core::indicators::incremental::dmi::{ADX as CoreAdx, ADXR as CoreAdxr};
use quantwave_core::indicators::incremental::macd::MACD as CoreMacd;
use quantwave_core::indicators::incremental::rsi::RSI as CoreRsi;
use quantwave_core::indicators::incremental::stoch::STOCH as CoreStoch;
use quantwave_core::indicators::kalman::KalmanFilter as CoreKalman;
use quantwave_core::indicators::roofing_filter::RoofingFilter as CoreRoofingFilter;
use quantwave_core::indicators::super_smoother::SuperSmoother as CoreSuperSmoother;
use quantwave_core::indicators::ttm_squeeze::TTMSqueeze as CoreTtmSqueeze;
use quantwave_core::indicators::volatility::TaATR as CoreAtr;
use quantwave_core::traits::Next;
use quantwave_core::{
    EMA as CoreEma, MaType, MarketStructure as CoreMarketStructure, SMA as CoreSma, WMA as CoreWma,
};
use quantwave_wasm::{
    Adx, Adxr, Atr, Bbands, Ema, Ichimoku, KalmanFilter, Macd, MarketStructure, RoofingFilter, Rsi,
    Sma, Stoch, SuperSmoother, SuperTrend, TtmSqueeze, Wma,
};
use wasm_bindgen_test::*;

fn assert_bits_eq(a: f64, b: f64, label: &str) {
    assert_eq!(a.to_bits(), b.to_bits(), "{label}: {a} != {b}");
}

#[wasm_bindgen_test]
fn wasm_all_curated_indicators_match_core_bits() {
    let mut sma = Sma::new(5);
    let mut core_sma = CoreSma::new(5);
    let mut ema = Ema::new(5);
    let mut core_ema = CoreEma::new(5);
    let mut wma = Wma::new(5);
    let mut core_wma = CoreWma::new(5);
    let mut rsi = Rsi::new(14);
    let mut core_rsi = CoreRsi::new(14);
    let mut macd = Macd::new(12, 26, 9);
    let mut core_macd = CoreMacd::new(12, 26, 9);
    let mut bbands = Bbands::new(10, 2.0, 2.0);
    let mut core_bbands = CoreBbands::new(10, 2.0, 2.0, MaType::Sma);
    let mut ss = SuperSmoother::new(10);
    let mut core_ss = CoreSuperSmoother::new(10);
    let mut rf = RoofingFilter::new(48, 10);
    let mut core_rf = CoreRoofingFilter::new(48, 10);
    let mut kf = KalmanFilter::new(0.01, 0.1);
    let mut core_kf = CoreKalman::new(0.01, 0.1);

    let mut st = SuperTrend::new(10, 3.0);
    let mut core_st = CoreSuperTrend::new(10, 3.0);
    let mut atr = Atr::new(14);
    let mut core_atr = CoreAtr::new(14);
    let mut adx = Adx::new(14);
    let mut core_adx = CoreAdx::new(14);
    let mut adxr = Adxr::new(14);
    let mut core_adxr = CoreAdxr::new(14);
    let mut stoch = Stoch::new(14, 3, 3);
    let mut core_stoch = CoreStoch::new(14, 3, MaType::Sma, 3, MaType::Sma);
    let mut ichi = Ichimoku::new(9, 26, 52);
    let mut core_ichi = CoreIchimoku::new(9, 26, 52);
    let mut ttm = TtmSqueeze::new(20, 2.0, 1.5);
    let mut core_ttm = CoreTtmSqueeze::new(20, 2.0, 1.5);
    let mut ms = MarketStructure::new(3);
    let mut core_ms = CoreMarketStructure::new(3);

    for i in 0..80 {
        let wave = (i as f64 * 0.35).sin() * 4.0 + (i as f64) * 0.15;
        let c = 100.0 + wave;
        let h = c + 1.5 + (i % 3) as f64 * 0.4;
        let l = c - 1.5 - (i % 4) as f64 * 0.3;

        assert_bits_eq(sma.next(c), core_sma.next(c), "sma");
        assert_bits_eq(ema.next(c), core_ema.next(c), "ema");
        assert_bits_eq(wma.next(c), core_wma.next(c), "wma");
        assert_bits_eq(rsi.next(c), core_rsi.next(c), "rsi");
        assert_bits_eq(ss.next(c), core_ss.next(c), "super_smoother");
        assert_bits_eq(rf.next(c), core_rf.next(c), "roofing_filter");
        assert_bits_eq(kf.next(c), core_kf.next(c), "kalman_filter");

        let (m, sig, hist) = core_macd.next(c);
        let wm = macd.next(c);
        assert_bits_eq(wm[0], m, "macd.macd");
        assert_bits_eq(wm[1], sig, "macd.signal");
        assert_bits_eq(wm[2], hist, "macd.hist");

        let (u, mid, lo) = core_bbands.next(c);
        let wb = bbands.next(c);
        assert_bits_eq(wb[0], u, "bbands.upper");
        assert_bits_eq(wb[1], mid, "bbands.middle");
        assert_bits_eq(wb[2], lo, "bbands.lower");

        let (st_v, st_d) = core_st.next((h, l, c));
        let wst = st.next(h, l, c);
        assert_bits_eq(wst[0], st_v, "supertrend.val");
        assert_bits_eq(wst[1], st_d as f64, "supertrend.dir");

        assert_bits_eq(atr.next(h, l, c), core_atr.next((h, l, c)), "atr");
        assert_bits_eq(adx.next(h, l, c), core_adx.next((h, l, c)), "adx");
        assert_bits_eq(adxr.next(h, l, c), core_adxr.next((h, l, c)), "adxr");

        let (sk, sd) = core_stoch.next((h, l, c));
        let wsto = stoch.next(h, l, c);
        assert_bits_eq(wsto[0], sk, "stoch.k");
        assert_bits_eq(wsto[1], sd, "stoch.d");

        let (t, k, sa, sb) = core_ichi.next((h, l));
        let wic = ichi.next(h, l);
        assert_bits_eq(wic[0], t, "ichimoku.tenkan");
        assert_bits_eq(wic[1], k, "ichimoku.kijun");
        assert_bits_eq(wic[2], sa, "ichimoku.senkou_a");
        assert_bits_eq(wic[3], sb, "ichimoku.senkou_b");

        let (th, tsq) = core_ttm.next((h, l, c));
        let wttm = ttm.next(h, l, c);
        assert_bits_eq(wttm[0], th, "ttm.hist");
        assert_bits_eq(wttm[1], if tsq { 1.0 } else { 0.0 }, "ttm.sq");

        let cms = core_ms.next((h, l));
        let wms = ms.next(h, l);
        assert_bits_eq(wms[0], f64::from(cms.bias_u32()), "ms.bias");
        assert_bits_eq(wms[1], cms.last_high_price(), "ms.last_high_price");
        assert_bits_eq(
            wms[5],
            if cms.has_flip() { 1.0 } else { 0.0 },
            "ms.has_flip",
        );
        assert_bits_eq(wms[11], cms.bar_index as f64, "ms.bar_index");
    }
}

#[wasm_bindgen_test]
fn wasm_matches_sma_gold_fixture() {
    let input = [1.0, 2.0, 3.0, 4.0, 5.0];
    let expected = [1.0, 1.5, 2.0, 3.0, 4.0];
    let mut sma = Sma::new(3);
    for (x, want) in input.iter().zip(expected.iter()) {
        assert_bits_eq(sma.next(*x), *want, "sma_5 gold");
    }
}
