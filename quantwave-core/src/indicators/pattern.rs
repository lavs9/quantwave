pub use crate::indicators::incremental::cdl_doji::CDLDOJI;
use crate::indicators::metadata::IndicatorMetadata;
pub use crate::indicators::patterns::batch_b::CDLCOUNTERATTACK;
pub use crate::indicators::patterns::batch_b::CDLDARKCLOUDCOVER;
pub use crate::indicators::patterns::batch_b::CDLDOJISTAR;
pub use crate::indicators::patterns::batch_b::CDLENGULFING;
pub use crate::indicators::patterns::batch_b::CDLHANGINGMAN;
pub use crate::indicators::patterns::batch_b::CDLHARAMI;
pub use crate::indicators::patterns::batch_b::CDLHARAMICROSS;
pub use crate::indicators::patterns::batch_b::CDLHOMINGPIGEON;
pub use crate::indicators::patterns::batch_b::CDLINNECK;
pub use crate::indicators::patterns::batch_b::CDLINVERTEDHAMMER;
pub use crate::indicators::patterns::batch_b::CDLMATCHINGLOW;
pub use crate::indicators::patterns::batch_b::CDLONNECK;
pub use crate::indicators::patterns::batch_c::CDL2CROWS;
pub use crate::indicators::patterns::batch_c::CDLHIKKAKE;
pub use crate::indicators::patterns::batch_c::CDLHIKKAKEMOD;
pub use crate::indicators::patterns::batch_c::CDLKICKING;
pub use crate::indicators::patterns::batch_c::CDLKICKINGBYLENGTH;
pub use crate::indicators::patterns::batch_c::CDLPIERCING;
pub use crate::indicators::patterns::batch_c::CDLSEPARATINGLINES;
pub use crate::indicators::patterns::batch_c::CDLSHOOTINGSTAR;
pub use crate::indicators::patterns::batch_c::CDLSTICKSANDWICH;
pub use crate::indicators::patterns::batch_c::CDLTHRUSTING;
pub use crate::indicators::patterns::batch_d::CDL3BLACKCROWS;
pub use crate::indicators::patterns::batch_d::CDL3INSIDE;
pub use crate::indicators::patterns::batch_d::CDL3OUTSIDE;
pub use crate::indicators::patterns::batch_d::CDL3STARSINSOUTH;
pub use crate::indicators::patterns::batch_d::CDL3WHITESOLDIERS;
pub use crate::indicators::patterns::batch_d::CDLABANDONEDBABY;
pub use crate::indicators::patterns::batch_d::CDLADVANCEBLOCK;
pub use crate::indicators::patterns::batch_d::CDLEVENINGDOJISTAR;
pub use crate::indicators::patterns::batch_d::CDLEVENINGSTAR;
pub use crate::indicators::patterns::batch_d::CDLMORNINGDOJISTAR;
pub use crate::indicators::patterns::batch_d::CDLMORNINGSTAR;
pub use crate::indicators::patterns::batch_e::CDLBREAKAWAY;
pub use crate::indicators::patterns::batch_e::CDLCONCEALBABYSWALL;
pub use crate::indicators::patterns::batch_e::CDLGAPSIDESIDEWHITE;
pub use crate::indicators::patterns::batch_e::CDLIDENTICAL3CROWS;
pub use crate::indicators::patterns::batch_e::CDLLADDERBOTTOM;
pub use crate::indicators::patterns::batch_e::CDLMATHOLD;
pub use crate::indicators::patterns::batch_e::CDLRISEFALL3METHODS;
pub use crate::indicators::patterns::batch_e::CDLSTALLEDPATTERN;
pub use crate::indicators::patterns::batch_e::CDLTASUKIGAP;
pub use crate::indicators::patterns::batch_e::CDLTRISTAR;
pub use crate::indicators::patterns::batch_e::CDLUNIQUE3RIVER;
pub use crate::indicators::patterns::batch_e::CDLUPSIDEGAP2CROWS;
pub use crate::indicators::patterns::batch_e::CDLXSIDEGAP3METHODS;
pub use crate::indicators::patterns::cdl3linestrike::CDL3LINESTRIKE;
pub use crate::indicators::patterns::cdlbelthold::CDLBELTHOLD;
pub use crate::indicators::patterns::cdlclosingmarubozu::CDLCLOSINGMARUBOZU;
pub use crate::indicators::patterns::cdldragonflydoji::CDLDRAGONFLYDOJI;
pub use crate::indicators::patterns::cdlgravestonedoji::CDLGRAVESTONEDOJI;
pub use crate::indicators::patterns::cdlhammer::CDLHAMMER;
pub use crate::indicators::patterns::cdlhighwave::CDLHIGHWAVE;
pub use crate::indicators::patterns::cdllongleggeddoji::CDLLONGLEGGEDDOJI;
pub use crate::indicators::patterns::cdllongline::CDLLONGLINE;
pub use crate::indicators::patterns::cdlmarubozu::CDLMARUBOZU;
pub use crate::indicators::patterns::cdlrickshawman::CDLRICKSHAWMAN;
pub use crate::indicators::patterns::cdlshortline::CDLSHORTLINE;
pub use crate::indicators::patterns::cdlspinningtop::CDLSPINNINGTOP;
pub use crate::indicators::patterns::cdltakuri::CDLTAKURI;
#[allow(unused_imports)]
use crate::traits::Next;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {}
}

pub const CDLDOJI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Doji",
    description: "A candlestick pattern where the open and close are virtually equal.",
    usage: "Indicates indecision between buyers and sellers.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdldoji.json",
    category: "Patterns",
};

pub const CDLHAMMER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hammer",
    description: "A bullish reversal pattern with a small body and long lower shadow.",
    usage: "Signals a potential bottom after a downtrend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhammer.json",
    category: "Patterns",
};

pub const CDLENGULFING_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Engulfing",
    description: "A pattern where a larger candle completely covers the previous smaller candle.",
    usage: "Signals a strong shift in momentum.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlengulfing.json",
    category: "Patterns",
};

pub const CDLCLOSINGMARUBOZU_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Closing Marubozu",
    description: "A candle with no shadow at the closing end.",
    usage: "Indicates strong conviction in the direction of the close.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlclosingmarubozu.json",
    category: "Patterns",
};

pub const CDLDRAGONFLYDOJI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Dragonfly Doji",
    description: "A doji with a long lower shadow and no upper shadow.",
    usage: "Signals a potential bullish reversal.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdldragonflydoji.json",
    category: "Patterns",
};

pub const CDLGRAVESTONEDOJI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Gravestone Doji",
    description: "A doji with a long upper shadow and no lower shadow.",
    usage: "Signals a potential bearish reversal.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlgravestonedoji.json",
    category: "Patterns",
};

pub const CDLHIGHWAVE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "High-Wave Candle",
    description: "A candle with very long shadows and a small body.",
    usage: "Indicates extreme market confusion and volatility.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhighwave.json",
    category: "Patterns",
};

pub const CDLLONGLEGGEDDOJI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Long-Legged Doji",
    description: "A doji with long shadows on both sides.",
    usage: "Indicates a significant struggle between bulls and bears.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdllongleggeddoji.json",
    category: "Patterns",
};

pub const CDLLONGLINE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Long Line Candle",
    description: "A candle with an unusually long body.",
    usage: "Indicates strong momentum in the direction of the candle.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdllongline.json",
    category: "Patterns",
};

pub const CDLMARUBOZU_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Marubozu",
    description: "A candle with no shadows at either end.",
    usage: "Indicates total dominance by one side for the entire period.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlmarubozu.json",
    category: "Patterns",
};

pub const CDLRICKSHAWMAN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Rickshaw Man",
    description: "A long-legged doji where the body is in the center of the range.",
    usage: "Indicates total equilibrium and indecision.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlrickshawman.json",
    category: "Patterns",
};

pub const CDLSHORTLINE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Short Line Candle",
    description: "A candle with a small body and small shadows.",
    usage: "Indicates low volatility and consolidation.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlshortline.json",
    category: "Patterns",
};

pub const CDLSPINNINGTOP_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Spinning Top",
    description: "A candle with a small body and long shadows.",
    usage: "Indicates indecision after a strong move.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlspinningtop.json",
    category: "Patterns",
};

pub const CDLTAKURI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Takuri",
    description: "A dragonfly doji with an exceptionally long lower shadow.",
    usage: "Indicates a very strong rejection of lower prices.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdltakuri.json",
    category: "Patterns",
};

pub const CDL2CROWS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Two Crows",
    description: "A bearish reversal pattern consisting of three candles.",
    usage: "Signals a potential top in an uptrend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl2crows.json",
    category: "Patterns",
};

pub const CDL3BLACKCROWS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three Black Crows",
    description: "A bearish reversal pattern with three long red candles.",
    usage: "Signals a major trend reversal to the downside.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3blackcrows.json",
    category: "Patterns",
};

pub const CDL3INSIDE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three Inside Up/Down",
    description: "A reversal pattern confirmed by a third candle.",
    usage: "Signals a more reliable trend change than a simple harami.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3inside.json",
    category: "Patterns",
};

pub const CDL3LINESTRIKE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three-Line Strike",
    description: "A continuation pattern where four candles are involved.",
    usage: "Signals a temporary pause before the trend continues.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3linestrike.json",
    category: "Patterns",
};

pub const CDL3OUTSIDE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three Outside Up/Down",
    description: "A reversal pattern confirmed by a third candle.",
    usage: "Signals a more reliable trend change than a simple engulfing.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3outside.json",
    category: "Patterns",
};

pub const CDL3STARSINSOUTH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three Stars In The South",
    description: "A rare bullish reversal pattern in a downtrend.",
    usage: "Indicates a gradual exhaustion of selling pressure.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3starsinsouth.json",
    category: "Patterns",
};

pub const CDL3WHITESOLDIERS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Three White Soldiers",
    description: "A bullish reversal pattern with three long green candles.",
    usage: "Signals a major trend reversal to the upside.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdl3whitesoldiers.json",
    category: "Patterns",
};

pub const CDLABANDONEDBABY_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Abandoned Baby",
    description: "A very rare reversal pattern with a doji gapping away.",
    usage: "One of the most reliable reversal signals.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlabandonedbaby.json",
    category: "Patterns",
};

pub const CDLADVANCEBLOCK_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Advance Block",
    description: "A bearish reversal pattern in an uptrend.",
    usage: "Indicates weakening momentum in an upward move.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdladvanceblock.json",
    category: "Patterns",
};

pub const CDLBELTHOLD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Belt-Hold",
    description: "A single candle pattern signaling a reversal.",
    usage: "Indicates a strong opening in the opposite direction of the trend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlbelthold.json",
    category: "Patterns",
};

pub const CDLBREAKAWAY_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Breakaway",
    description: "A five-candle reversal pattern.",
    usage: "Signals the breakout from a short-term consolidation.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlbreakaway.json",
    category: "Patterns",
};

pub const CDLCONCEALBABYSWALL_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Concealed Baby Swallow",
    description: "A rare bullish reversal pattern.",
    usage: "Indicates a final 'flush out' before a reversal.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlconcealbabyswall.json",
    category: "Patterns",
};

pub const CDLCOUNTERATTACK_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Counterattack",
    description: "A reversal pattern where the second candle closes at the same level as the first.",
    usage: "Signals a stalemate after a strong move.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlcounterattack.json",
    category: "Patterns",
};

pub const CDLDARKCLOUDCOVER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Dark Cloud Cover",
    description: "A bearish reversal pattern in an uptrend.",
    usage: "Indicates a significant rejection of higher prices.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdldarkcloudcover.json",
    category: "Patterns",
};

pub const CDLDOJISTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Doji Star",
    description: "A reversal pattern where a doji gaps away.",
    usage: "Signals a potential shift in trend momentum.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdldojistar.json",
    category: "Patterns",
};

pub const CDLEVENINGDOJISTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Evening Doji Star",
    description: "A bearish reversal pattern involving a doji.",
    usage: "A highly reliable signal of a market top.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdleveningdojistar.json",
    category: "Patterns",
};

pub const CDLEVENINGSTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Evening Star",
    description: "A bearish reversal pattern with three candles.",
    usage: "Signals a peak in an uptrend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdleveningstar.json",
    category: "Patterns",
};

pub const CDLGAPSIDESIDEWHITE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Up/Down-Gap Side-By-Side White Lines",
    description: "A continuation pattern involving gaps.",
    usage: "Signals that the current trend is likely to continue.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlgapsidesidewhite.json",
    category: "Patterns",
};

pub const CDLHANGINGMAN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hanging Man",
    description: "A bearish reversal pattern at the top of an uptrend.",
    usage: "Signals a potential breakdown after a period of buying.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhangingman.json",
    category: "Patterns",
};

pub const CDLHARAMI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Harami",
    description: "A two-candle reversal pattern where the second is inside the first.",
    usage: "Signals a potential trend exhaustion.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlharami.json",
    category: "Patterns",
};

pub const CDLHARAMICROSS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Harami Cross",
    description: "A harami where the second candle is a doji.",
    usage: "A more reliable reversal signal than a standard harami.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlharamicross.json",
    category: "Patterns",
};

pub const CDLHIKKAKE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hikkake Pattern",
    description: "A trap pattern used to identify false breakouts.",
    usage: "Signals a move in the opposite direction of the trap.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhikkake.json",
    category: "Patterns",
};

pub const CDLHIKKAKEMOD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Modified Hikkake Pattern",
    description: "A more conservative version of the Hikkake.",
    usage: "Provides a higher probability signal.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhikkakemod.json",
    category: "Patterns",
};

pub const CDLHOMINGPIGEON_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Homing Pigeon",
    description: "A bullish reversal pattern in a downtrend.",
    usage: "Signals a potential bottom.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlhomingpigeon.json",
    category: "Patterns",
};

pub const CDLIDENTICAL3CROWS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Identical Three Crows",
    description: "A bearish reversal pattern with three identical candles.",
    usage: "Signals a very strong trend change.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlidentical3crows.json",
    category: "Patterns",
};

pub const CDLINNECK_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "In-Neck Pattern",
    description: "A bearish continuation pattern.",
    usage: "Indicates that the downward move is not yet over.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlinneck.json",
    category: "Patterns",
};

pub const CDLINVERTEDHAMMER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Inverted Hammer",
    description: "A bullish reversal pattern at the bottom of a downtrend.",
    usage: "Signals a potential move to the upside.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlinvertedhammer.json",
    category: "Patterns",
};

pub const CDLKICKING_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Kicking",
    description: "A two-candle reversal pattern involving a gap.",
    usage: "Signals a violent shift in market sentiment.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlkicking.json",
    category: "Patterns",
};

pub const CDLKICKINGBYLENGTH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Kicking - bull/bear determined by longer marubozu",
    description: "A variation of the kicking pattern.",
    usage: "Focuses on the relative strength of the marubozus.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlkickingbylength.json",
    category: "Patterns",
};

pub const CDLLADDERBOTTOM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ladder Bottom",
    description: "A five-candle bullish reversal pattern.",
    usage: "Signals a major bottom after a steady decline.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlladderbottom.json",
    category: "Patterns",
};

pub const CDLMATCHINGLOW_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Matching Low",
    description: "A bullish reversal pattern with two identical lows.",
    usage: "Signals support at a specific price level.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlmatchinglow.json",
    category: "Patterns",
};

pub const CDLMATHOLD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Mat Hold",
    description: "A bullish continuation pattern.",
    usage: "Signals a strong trend that is likely to persist.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlmathold.json",
    category: "Patterns",
};

pub const CDLMORNINGDOJISTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Morning Doji Star",
    description: "A bullish reversal pattern involving a doji.",
    usage: "A highly reliable signal of a market bottom.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlmorningdojistar.json",
    category: "Patterns",
};

pub const CDLMORNINGSTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Morning Star",
    description: "A bullish reversal pattern with three candles.",
    usage: "Signals a trough in a downtrend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlmorningstar.json",
    category: "Patterns",
};

pub const CDLONNECK_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "On-Neck Pattern",
    description: "A bearish continuation pattern.",
    usage: "A weak signal that the downtrend will continue.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlonneck.json",
    category: "Patterns",
};

pub const CDLPIERCING_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Piercing Pattern",
    description: "A bullish reversal pattern in a downtrend.",
    usage: "Indicates a significant entry of buyers.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlpiercing.json",
    category: "Patterns",
};

pub const CDLRISEFALL3METHODS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Rising/Falling Three Methods",
    description: "A strong continuation pattern.",
    usage: "Signals a minor consolidation before the trend resumes.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlrisefall3methods.json",
    category: "Patterns",
};

pub const CDLSEPARATINGLINES_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Separating Lines",
    description: "A continuation pattern where candles move in opposite directions.",
    usage: "Signals trend persistence.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlseparatinglines.json",
    category: "Patterns",
};

pub const CDLSHOOTINGSTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Shooting Star",
    description: "A bearish reversal pattern at the top of an uptrend.",
    usage: "Signals a potential move to the downside.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlshootingstar.json",
    category: "Patterns",
};

pub const CDLSTALLEDPATTERN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Stalled Pattern",
    description: "A bearish reversal pattern in an uptrend.",
    usage: "Indicates that the upward momentum is slowing down.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlstalledpattern.json",
    category: "Patterns",
};

pub const CDLSTICKSANDWICH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Stick Sandwich",
    description: "A reversal pattern with alternating candle colors.",
    usage: "Signals a potential support or resistance level.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlsticksandwich.json",
    category: "Patterns",
};

pub const CDLTASUKIGAP_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Tasuki Gap",
    description: "A continuation pattern involving a gap.",
    usage: "Signals trend strength.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdltasukigap.json",
    category: "Patterns",
};

pub const CDLTHRUSTING_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Thrusting Pattern",
    description: "A bearish continuation pattern.",
    usage: "A weak signal of trend persistence.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlthrusting.json",
    category: "Patterns",
};

pub const CDLTRISTAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Tristar Pattern",
    description: "A rare reversal pattern consisting of three dojis.",
    usage: "Signals an extreme shift in momentum.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdltristar.json",
    category: "Patterns",
};

pub const CDLUNIQUE3RIVER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Unique 3 River",
    description: "A bullish reversal pattern.",
    usage: "Signals a potential bottom.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlunique3river.json",
    category: "Patterns",
};

pub const CDLUPSIDEGAP2CROWS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Upside Gap Two Crows",
    description: "A bearish reversal pattern.",
    usage: "Signals a potential top in an uptrend.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlupsidegap2crows.json",
    category: "Patterns",
};

pub const CDLXSIDEGAP3METHODS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Up/Down-Gap Three Methods",
    description: "A continuation pattern.",
    usage: "Signals that the trend is likely to continue after a gap.",
    keywords: &["pattern", "candlestick", "classic"],
    ehlers_summary: "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book 'Japanese Candlestick Charting Techniques'. These patterns provide a visual representation of market psychology and the balance of power between buyers and sellers at key price levels.",
    params: &[],
    formula_source: "https://www.investopedia.com/articles/active-trading/062315/using-bullish-candlestick-patterns-buy-stocks.asp",
    formula_latex: r#"
\text{Pattern Recognition Logic (TA-Lib Internal)}
"#,
    gold_standard_file: "cdlxsidegap3methods.json",
    category: "Patterns",
};
