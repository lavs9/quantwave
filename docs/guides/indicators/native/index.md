# Native Indicators

QuantWave ships **221 production-grade native indicators** in Rust with bit-identical batch (Polars `.ta()`) and streaming (`Next<T>`) parity.

Every page follows [Documentation Standards](../../DOCUMENTATION_STANDARDS.md).

## Quick links

- [Indicator Gallery](../gallery.md) — curated starting points
- [Ehlers DSP Suite](../ehlers/index.md)
- [Regime Detection](../regimes/index.md)
- [ML Features](../../ml_features.md)
- [Price Action notebook](../../examples/notebooks/pa_flag_breakout_strategy.md)

## Complete indicator catalog

**221 indicators** across 14 categories. Click any name for formulas, parameters, usage examples, edge cases, and sources.

### Price Action (2)

| Indicator | Slug |
|-----------|------|
| [Market Structure (Swings + BOS)](market_structure.md) | `market_structure` |
| [S/R Interaction Monitor (Part 67)](s_r_interaction_monitor_part_67.md) | `sr_interaction_monitor` |

### Price Action / Patterns (1)

| Indicator | Slug |
|-----------|------|
| [geometric_patterns](geometric_patterns.md) | `geometric_patterns` |

### Classic (54)

| Indicator | Slug |
|-----------|------|
| [Absolute Price Oscillator (APO)](absolute_price_oscillator_apo.md) | `apo` |
| [Accumulation/Distribution Line (AD)](accumulation_distribution_line_ad.md) | `ad` |
| [Anchored VWAP](anchored_vwap.md) | `vwap` |
| [Arnaud Legoux Moving Average](arnaud_legoux_moving_average.md) | `alma` |
| [Aroon Indicator](aroon_indicator.md) | `aroon` |
| [ATR Trailing Stop](atr_trailing_stop.md) | `atr_ts` |
| [Average Directional Index (ADX)](average_directional_index_adx.md) | `adx` |
| [Average Price (AVGPRICE)](average_price_avgprice.md) | `avgprice` |
| [Average True Range](average_true_range.md) | `atr` |
| [Beta (BETA)](beta_beta.md) | `beta` |
| [Bill Williams Alligator](bill_williams_alligator.md) | `alligator` |
| [Bill Williams Fractals](bill_williams_fractals.md) | `fractals` |
| [Bollinger Bands](bollinger_bands.md) | `bbands` |
| [Chaikin Oscillator (ADOSC)](chaikin_oscillator_adosc.md) | `adosc` |
| [Chande Momentum Oscillator (CMO)](chande_momentum_oscillator_cmo.md) | `cmo` |
| [Commodity Channel Index (CCI)](commodity_channel_index_cci.md) | `cci` |
| [Correlation Coefficient (CORREL)](correlation_coefficient_correl.md) | `correl` |
| [Donchian Channels](donchian_channels.md) | `donchian` |
| [Double Exponential Moving Average (DEMA)](double_exponential_moving_average_dema.md) | `dema` |
| [Exponential Moving Average](exponential_moving_average.md) | `ema` |
| [Heikin-Ashi](heikin_ashi.md) | `heikin_ashi` |
| [Hull Moving Average](hull_moving_average.md) | `hma` |
| [Ichimoku Cloud](ichimoku_cloud.md) | `ichimoku` |
| [KAMA](kama.md) | `kama` |
| [Keltner Channels](keltner_channels.md) | `keltner` |
| [Linear Regression](linear_regression.md) | `linreg` |
| [Median Price (MEDPRICE)](median_price_medprice.md) | `medprice` |
| [Momentum (MOM)](momentum_mom.md) | `mom` |
| [Money Flow Index (MFI)](money_flow_index_mfi.md) | `mfi` |
| [Moving Average Convergence Divergence (MACD)](moving_average_convergence_divergence_macd.md) | `macd` |
| [Normalized Average True Range (NATR)](normalized_average_true_range_natr.md) | `natr` |
| [On-Balance Volume (OBV)](on_balance_volume_obv.md) | `obv` |
| [Parabolic SAR](parabolic_sar.md) | `sar` |
| [Percentage Price Oscillator (PPO)](percentage_price_oscillator_ppo.md) | `ppo` |
| [Pivot Points](pivot_points.md) | `pivot_points` |
| [Rate of Change (ROC)](rate_of_change_roc.md) | `roc` |
| [Relative Strength Index (RSI)](relative_strength_index_rsi.md) | `rsi` |
| [Simple Moving Average](simple_moving_average.md) | `sma` |
| [Standard Deviation](standard_deviation.md) | `stddev` |
| [Stochastic Oscillator](stochastic_oscillator.md) | `stoch` |
| [SuperTrend](supertrend.md) | `supertrend` |
| [Tilson T3 Moving Average](tilson_t3_moving_average.md) | `t3` |
| [Triangular Moving Average (TRIMA)](triangular_moving_average_trima.md) | `trima` |
| [Triple Exponential Moving Average](triple_exponential_moving_average.md) | `tema` |
| [TRIX](trix.md) | `trix` |
| [True Range](true_range.md) | `true_range` |
| [TTM Squeeze](ttm_squeeze.md) | `ttm_squeeze` |
| [Typical Price (TYPPRICE)](typical_price_typprice.md) | `typprice` |
| [Ultimate Oscillator](ultimate_oscillator.md) | `ultosc` |
| [Vortex Indicator](vortex_indicator.md) | `vortex` |
| [Weighted Close Price (WCLPRICE)](weighted_close_price_wclprice.md) | `wclprice` |
| [Weighted Moving Average](weighted_moving_average.md) | `wma` |
| [Williams %R](williams_r.md) | `willr` |
| [Zero Lag Exponential Moving Average](zero_lag_exponential_moving_average.md) | `zlema` |

### Ehlers DSP (82)

| Indicator | Slug |
|-----------|------|
| [AM Detector](am_detector.md) | `am_detector` |
| [AutoTune Filter](autotune_filter.md) | `autotune_filter` |
| [BandPass](bandpass.md) | `bandpass` |
| [Butterworth2](butterworth2.md) | `butterworth2` |
| [Butterworth3](butterworth3.md) | `butterworth3` |
| [Center of Gravity Oscillator](center_of_gravity_oscillator.md) | `cg` |
| [ChannelCycle](channelcycle.md) | `channel_cycle` |
| [Classic Laguerre Filter](classic_laguerre_filter.md) | `classic_laguerre` |
| [Continuation Index](continuation_index.md) | `continuation_index` |
| [Correlation Trend](correlation_trend.md) | `correlation_trend` |
| [CorrelationCycle](correlationcycle.md) | `correlation_cycle` |
| [Cyber Cycle](cyber_cycle.md) | `cyber_cycle` |
| [CyberneticOscillator](cyberneticoscillator.md) | `cybernetic_oscillator` |
| [Cycle/Trend Analytics](cycle_trend_analytics.md) | `cycle_trend_analytics` |
| [DMH](dmh.md) | `dmh` |
| [DSMA](dsma.md) | `dsma` |
| [Ehlers Autocorrelation](ehlers_autocorrelation.md) | `ehlers_autocorrelation` |
| [Ehlers Filter](ehlers_filter.md) | `ehlers_filter` |
| [Ehlers Loops](ehlers_loops.md) | `ehlers_loops` |
| [Ehlers Stochastic](ehlers_stochastic.md) | `ehlers_stochastic` |
| [EhlersUltimateOscillator](ehlersultimateoscillator.md) | `ehlers_ultimate_oscillator` |
| [EMD](emd.md) | `emd` |
| [Fisher Transform](fisher_transform.md) | `fisher` |
| [FisherHighPass](fisherhighpass.md) | `fisher_high_pass` |
| [FM Demodulator](fm_demodulator.md) | `fm_demodulator` |
| [FourierDominantCycle](fourierdominantcycle.md) | `fourier_dominant_cycle` |
| [FourierSeriesModel](fourierseriesmodel.md) | `fourier_series_model` |
| [Fractal Adaptive Moving Average](fractal_adaptive_moving_average.md) | `frama` |
| [GaussianFilter](gaussianfilter.md) | `gaussian_filter` |
| [Generalized Laguerre](generalized_laguerre.md) | `generalized_laguerre` |
| [GriffithsDominantCycle](griffithsdominantcycle.md) | `griffiths_dominant_cycle` |
| [GriffithsPredictor](griffithspredictor.md) | `griffiths_predictor` |
| [GriffithsSpectrum](griffithsspectrum.md) | `griffiths_spectrum` |
| [HammingFilter](hammingfilter.md) | `hamming_filter` |
| [HannFilter](hannfilter.md) | `hann_filter` |
| [HighPass](highpass.md) | `high_pass` |
| [Hilbert Transform - Dominant Cycle Period (HT_DCPERIOD)](hilbert_transform_dominant_cycle_period_ht_dcperiod.md) | `ht_dcperiod` |
| [Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE)](hilbert_transform_dominant_cycle_phase_ht_dcphase.md) | `ht_dcphase` |
| [Hilbert Transform - Phasor Components (HT_PHASOR)](hilbert_transform_phasor_components_ht_phasor.md) | `ht_phasor` |
| [Hilbert Transform - Sine Wave (HT_SINE)](hilbert_transform_sine_wave_ht_sine.md) | `ht_sine` |
| [Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE)](hilbert_transform_trend_vs_cycle_mode_ht_trendmode.md) | `ht_trendmode` |
| [Inverse Fisher Transform](inverse_fisher_transform.md) | `inverse_fisher` |
| [Laguerre Filter](laguerre_filter.md) | `laguerre_filter` |
| [Laguerre Oscillator](laguerre_oscillator.md) | `laguerre_oscillator` |
| [Laguerre RSI](laguerre_rsi.md) | `laguerre_rsi` |
| [MAD](mad.md) | `mad` |
| [MADH](madh.md) | `madh` |
| [MarketState](marketstate.md) | `market_state` |
| [MESA Adaptive Moving Average](mesa_adaptive_moving_average.md) | `mama` |
| [MESA Stochastic](mesa_stochastic.md) | `mesa_stochastic` |
| [MyRSI](myrsi.md) | `my_rsi` |
| [Noise Elimination Technology](noise_elimination_technology.md) | `noise_elimination` |
| [OCPriceRSI](ocpricersi.md) | `oc_price_rsi` |
| [One Euro Filter](one_euro_filter.md) | `one_euro_filter` |
| [Open-Close Average (OC2)](open_close_average_oc2.md) | `oc2` |
| [PairsRotation](pairsrotation.md) | `pairs_rotation` |
| [Precision Trend Analysis](precision_trend_analysis.md) | `precision_trend_analysis` |
| [Projected Moving Average](projected_moving_average.md) | `projected_moving_average` |
| [RecursiveMedian](recursivemedian.md) | `recursive_median` |
| [RecursiveMedianOscillator](recursivemedianoscillator.md) | `recursive_median_oscillator` |
| [Reflex](reflex.md) | `reflex` |
| [Reverse EMA](reverse_ema.md) | `reverse_ema` |
| [Reversion Index](reversion_index.md) | `reversion_index` |
| [RocketRSI](rocketrsi.md) | `rocket_rsi` |
| [Roofing Filter](roofing_filter.md) | `roofing_filter` |
| [RSIH](rsih.md) | `rsih` |
| [SimplePredictor](simplepredictor.md) | `simple_predictor` |
| [SuperSmoother](supersmoother.md) | `super_smoother` |
| [Swiss Army Knife Indicator](swiss_army_knife_indicator.md) | `swiss_army_knife` |
| [Synthetic Oscillator](synthetic_oscillator.md) | `synthetic_oscillator` |
| [Trendflex](trendflex.md) | `trendflex` |
| [TriangleFilter](trianglefilter.md) | `triangle_filter` |
| [TruncatedBandpass](truncatedbandpass.md) | `truncated_bandpass` |
| [Ultimate Bands](ultimate_bands.md) | `ultimate_bands` |
| [Ultimate Channel](ultimate_channel.md) | `ultimate_channel` |
| [Ultimate Strength Index](ultimate_strength_index.md) | `usi` |
| [UltimateSmoother](ultimatesmoother.md) | `ultimate_smoother` |
| [UndersampledDoubleMA](undersampleddoublema.md) | `undersampled_double_ma` |
| [Universal Oscillator](universal_oscillator.md) | `universal_oscillator` |
| [VossPredictor](vosspredictor.md) | `voss_predictor` |
| [WaveTrend Oscillator](wavetrend_oscillator.md) | `wavetrend` |
| [Zero Lag EC](zero_lag_ec.md) | `zero_lag` |

### Patterns (61)

| Indicator | Slug |
|-----------|------|
| [Abandoned Baby](abandoned_baby.md) | `cdlabandonedbaby` |
| [Advance Block](advance_block.md) | `cdladvanceblock` |
| [Belt-Hold](belt_hold.md) | `cdlbelthold` |
| [Breakaway](breakaway.md) | `cdlbreakaway` |
| [Closing Marubozu](closing_marubozu.md) | `cdlclosingmarubozu` |
| [Concealed Baby Swallow](concealed_baby_swallow.md) | `cdlconcealbabyswall` |
| [Counterattack](counterattack.md) | `cdlcounterattack` |
| [Dark Cloud Cover](dark_cloud_cover.md) | `cdldarkcloudcover` |
| [Doji](doji.md) | `cdldoji` |
| [Doji Star](doji_star.md) | `cdldojistar` |
| [Dragonfly Doji](dragonfly_doji.md) | `cdldragonflydoji` |
| [Engulfing](engulfing.md) | `cdlengulfing` |
| [Evening Doji Star](evening_doji_star.md) | `cdleveningdojistar` |
| [Evening Star](evening_star.md) | `cdleveningstar` |
| [Gravestone Doji](gravestone_doji.md) | `cdlgravestonedoji` |
| [Hammer](hammer.md) | `cdlhammer` |
| [Hanging Man](hanging_man.md) | `cdlhangingman` |
| [Harami](harami.md) | `cdlharami` |
| [Harami Cross](harami_cross.md) | `cdlharamicross` |
| [High-Wave Candle](high_wave_candle.md) | `cdlhighwave` |
| [Hikkake Pattern](hikkake_pattern.md) | `cdlhikkake` |
| [Homing Pigeon](homing_pigeon.md) | `cdlhomingpigeon` |
| [Identical Three Crows](identical_three_crows.md) | `cdlidentical3crows` |
| [In-Neck Pattern](in_neck_pattern.md) | `cdlinneck` |
| [Inverted Hammer](inverted_hammer.md) | `cdlinvertedhammer` |
| [Kicking](kicking.md) | `cdlkicking` |
| [Kicking - bull/bear determined by longer marubozu](kicking_bull_bear_determined_by_longer_marubozu.md) | `cdlkickingbylength` |
| [Ladder Bottom](ladder_bottom.md) | `cdlladderbottom` |
| [Long Line Candle](long_line_candle.md) | `cdllongline` |
| [Long-Legged Doji](long_legged_doji.md) | `cdllongleggeddoji` |
| [Marubozu](marubozu.md) | `cdlmarubozu` |
| [Mat Hold](mat_hold.md) | `cdlmathold` |
| [Matching Low](matching_low.md) | `cdlmatchinglow` |
| [Modified Hikkake Pattern](modified_hikkake_pattern.md) | `cdlhikkakemod` |
| [Morning Doji Star](morning_doji_star.md) | `cdlmorningdojistar` |
| [Morning Star](morning_star.md) | `cdlmorningstar` |
| [On-Neck Pattern](on_neck_pattern.md) | `cdlonneck` |
| [Piercing Pattern](piercing_pattern.md) | `cdlpiercing` |
| [Rickshaw Man](rickshaw_man.md) | `cdlrickshawman` |
| [Rising/Falling Three Methods](rising_falling_three_methods.md) | `cdlrisefall3methods` |
| [Separating Lines](separating_lines.md) | `cdlseparatinglines` |
| [Shooting Star](shooting_star.md) | `cdlshootingstar` |
| [Short Line Candle](short_line_candle.md) | `cdlshortline` |
| [Spinning Top](spinning_top.md) | `cdlspinningtop` |
| [Stalled Pattern](stalled_pattern.md) | `cdlstalledpattern` |
| [Stick Sandwich](stick_sandwich.md) | `cdlsticksandwich` |
| [Takuri](takuri.md) | `cdltakuri` |
| [Tasuki Gap](tasuki_gap.md) | `cdltasukigap` |
| [Three Black Crows](three_black_crows.md) | `cdl3blackcrows` |
| [Three Inside Up/Down](three_inside_up_down.md) | `cdl3inside` |
| [Three Outside Up/Down](three_outside_up_down.md) | `cdl3outside` |
| [Three Stars In The South](three_stars_in_the_south.md) | `cdl3starsinsouth` |
| [Three White Soldiers](three_white_soldiers.md) | `cdl3whitesoldiers` |
| [Three-Line Strike](three_line_strike.md) | `cdl3linestrike` |
| [Thrusting Pattern](thrusting_pattern.md) | `cdlthrusting` |
| [Tristar Pattern](tristar_pattern.md) | `cdltristar` |
| [Two Crows](two_crows.md) | `cdl2crows` |
| [Unique 3 River](unique_3_river.md) | `cdlunique3river` |
| [Up/Down-Gap Side-By-Side White Lines](up_down_gap_side_by_side_white_lines.md) | `cdlgapsidesidewhite` |
| [Up/Down-Gap Three Methods](up_down_gap_three_methods.md) | `cdlxsidegap3methods` |
| [Upside Gap Two Crows](upside_gap_two_crows.md) | `cdlupsidegap2crows` |

### Moving Averages (2)

| Indicator | Slug |
|-----------|------|
| [Adaptive Exponential Moving Average](adaptive_exponential_moving_average.md) | `adaptive_ema` |
| [True Range Adjusted Exponential Moving Average](true_range_adjusted_exponential_moving_average.md) | `tradj_ema` |

### Volume (2)

| Indicator | Slug |
|-----------|------|
| [Volume Positive Negative](volume_positive_negative.md) | `vpn` |
| [Volume Profile](volume_profile.md) | `volume_profile` |

### Momentum (2)

| Indicator | Slug |
|-----------|------|
| [Gap Momentum](gap_momentum.md) | `gap_momentum` |
| [Stochastic Distance Oscillator](stochastic_distance_oscillator.md) | `sdo` |

### Modern (2)

| Indicator | Slug |
|-----------|------|
| [Choppiness Index](choppiness_index.md) | `choppiness_index` |
| [Schaff Trend Cycle](schaff_trend_cycle.md) | `stc` |

### ML Features (4)

| Indicator | Slug |
|-----------|------|
| [Fractional Differentiation](fractional_differentiation.md) | `frac_diff` |
| [Hurst Exponent](hurst_exponent.md) | `hurst_exponent` |
| [Kalman Filter](kalman_filter.md) | `kalman_filter` |
| [Kinematic Kalman Filter](kinematic_kalman_filter.md) | `kinematic_kalman` |

### Rocket Science (4)

| Indicator | Slug |
|-----------|------|
| [Homodyne Discriminator](homodyne_discriminator.md) | `homodyne_discriminator` |
| [Instantaneous Trendline](instantaneous_trendline.md) | `instantaneous_trendline` |
| [Phasor](phasor.md) | `phasor` |
| [Sine Wave](sine_wave.md) | `sine_wave` |

### Statistics (1)

| Indicator | Slug |
|-----------|------|
| [System Evaluator](system_evaluator.md) | `system_evaluator` |

### Wilder (1)

| Indicator | Slug |
|-----------|------|
| [Harrington ADX Oscillator](harrington_adx_oscillator.md) | `harrington_adx` |

### Regime (3)

| Indicator | Slug |
|-----------|------|
| [gaussian_hmm](gaussian_hmm.md) | `gaussian_hmm` |
| [hmm_forecast](hmm_forecast.md) | `hmm_forecast` |
| [lambda_hmm](lambda_hmm.md) | `lambda_hmm` |
