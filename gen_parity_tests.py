import os

INDICATORS = [
    # name, talib_rs_path, inputs (num), outputs (num), args (list of (name, type, default_val))
    ("MACDEXT", "talib_rs::momentum::macd_ext", 1, 3, [("fastperiod", "usize", "12"), ("fastmatype", "talib_rs::MaType", "talib_rs::MaType::Sma"), ("slowperiod", "usize", "26"), ("slowmatype", "talib_rs::MaType", "talib_rs::MaType::Sma"), ("signalperiod", "usize", "9"), ("signalmatype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("MACDFIX", "talib_rs::momentum::macd_fix", 1, 3, [("signalperiod", "usize", "9")]),
    ("STOCHF", "talib_rs::momentum::stochf", 3, 2, [("fastk_period", "usize", "5"), ("fastd_period", "usize", "3"), ("fastd_matype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("STOCHRSI", "talib_rs::momentum::stochrsi", 1, 2, [("timeperiod", "usize", "14"), ("fastk_period", "usize", "5"), ("fastd_period", "usize", "3"), ("fastd_matype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("APO", "talib_rs::momentum::apo", 1, 1, [("fastperiod", "usize", "12"), ("slowperiod", "usize", "26"), ("matype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("PPO", "talib_rs::momentum::ppo", 1, 1, [("fastperiod", "usize", "12"), ("slowperiod", "usize", "26"), ("matype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("BOP", "talib_rs::momentum::bop", 4, 1, []),
    ("AROONOSC", "talib_rs::momentum::aroon_osc", 2, 1, [("timeperiod", "usize", "14")]),
    ("MFI", "talib_rs::momentum::mfi", 4, 1, [("timeperiod", "usize", "14")]),
    ("ULTOSC", "talib_rs::momentum::ultosc", 3, 1, [("timeperiod1", "usize", "7"), ("timeperiod2", "usize", "14"), ("timeperiod3", "usize", "28")]),
    ("T3", "talib_rs::overlap::t3", 1, 1, [("timeperiod", "usize", "5"), ("v_factor", "f64", "0.7")]),
    ("MAMA", "talib_rs::overlap::mama", 1, 2, [("fastlimit", "f64", "0.5"), ("slowlimit", "f64", "0.05")]),
    ("SAR", "talib_rs::overlap::sar", 2, 1, [("acceleration", "f64", "0.02"), ("maximum", "f64", "0.2")]),
    ("SAREXT", "talib_rs::overlap::sar_ext", 2, 1, [("startvalue", "f64", "0.0"), ("offsetonreverse", "f64", "0.0"), ("accelerationinitlong", "f64", "0.02"), ("accelerationlong", "f64", "0.02"), ("accelerationmaxlong", "f64", "0.2"), ("accelerationinitshort", "f64", "0.02"), ("accelerationshort", "f64", "0.02"), ("accelerationmaxshort", "f64", "0.2")]),
    ("MAVP", "talib_rs::overlap::mavp", 2, 1, [("minperiod", "usize", "2"), ("maxperiod", "usize", "30"), ("matype", "talib_rs::MaType", "talib_rs::MaType::Sma")]),
    ("HT_PHASOR", "talib_rs::cycle::ht_phasor", 1, 2, []),
    ("HT_SINE", "talib_rs::cycle::ht_sine", 1, 2, []),
    ("PLUS_DM", "talib_rs::momentum::plus_dm", 2, 1, [("timeperiod", "usize", "14")]),
    ("MINUS_DM", "talib_rs::momentum::minus_dm", 2, 1, [("timeperiod", "usize", "14")]),
]

def generate_rust_code():
    code = """use quantwave_core::*;
use quantwave_core::traits::Next;
use proptest::prelude::*;

proptest! {
"""
    for name, path, num_in, num_out, args in INDICATORS:
        fn_name = f"test_{name.lower()}_parity_auto"
        arg_decls = "\n            ".join([f"let {arg[0]} = {arg[2]};" for arg in args])
        arg_names = ", ".join([arg[0] for arg in args])
        
        # Inputs
        if num_in == 1:
            inputs = "input in prop::collection::vec(1.0..100.0, 10..100)"
            next_args = "input[i]"
            talib_args = "&input"
        elif num_in == 2:
            inputs = "in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)"
            next_args = "(in1[i], in2[i])"
            talib_args = "&in1, &in2"
        elif num_in == 3:
            inputs = "in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)"
            next_args = "(in1[i], in2[i], in3[i])"
            talib_args = "&in1, &in2, &in3"
        elif num_in == 4:
            inputs = "in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)"
            next_args = "(in1[i], in2[i], in3[i], in4[i])"
            talib_args = "&in1, &in2, &in3, &in4"

        call_talib_args = talib_args + (", " + arg_names if arg_names else "")
        call_new_args = arg_names
        
        # Output unpacking
        if num_out == 1:
            b_tuple = "let b_res"
            nan_default = "vec![f64::NAN; len]"
            check_logic = """
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }
"""
        elif num_out == 2:
            b_tuple = "let (b1, b2)"
            nan_default = "(vec![f64::NAN; len], vec![f64::NAN; len])"
            check_logic = """
            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
            }
"""
        elif num_out == 3:
            b_tuple = "let (b1, b2, b3)"
            nan_default = "(vec![f64::NAN; len], vec![f64::NAN; len], vec![f64::NAN; len])"
            check_logic = """
            for (i, (s1, s2, s3)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
                if s3.is_nan() {
                    assert!(b3[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s3, b3[i], epsilon = 1e-6);
                }
            }
"""

        len_expr = "let len = input.len();" if num_in == 1 else "let len = in1.len().min(in2.len());" if num_in == 2 else "let len = in1.len().min(in2.len()).min(in3.len());" if num_in == 3 else "let len = in1.len().min(in2.len()).min(in3.len()).min(in4.len());"

        if num_in == 2:
            inputs = "h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)"
            len_expr = """let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }"""

        elif num_in == 3:
            inputs = "h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)"
            len_expr = """let len = h_in.len().min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(h.max(l).max(c));
                in2.push(h.min(l).min(c));
                in3.push(c);
            }"""

        elif num_in == 4:
            inputs = "o_in in prop::collection::vec(1.0..100.0, 10..100), h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)"
            len_expr = """let len = o_in.len().min(h_in.len()).min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            let mut in4 = Vec::with_capacity(len);
            for i in 0..len {
                let o: f64 = o_in[i];
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(o);
                in2.push(o.max(h).max(l).max(c));
                in3.push(o.min(h).min(l).min(c));
                in4.push(c);
            }"""

        test_code = f"""
        #[test]
        fn {fn_name}({inputs}) {{
            {len_expr}
            if len == 0 {{ return Ok(()); }}
            {arg_decls}
            let mut indicator = {name}::new({call_new_args});
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next({next_args})).collect();
            {b_tuple} = {path}({call_talib_args}).unwrap_or_else(|_| {nan_default});
            {check_logic}
        }}
"""
        code += test_code
    
    code += "}\n"
    return code

with open("quantwave-core/tests/test_all_talib_parity.rs", "w") as f:
    f.write(generate_rust_code())

