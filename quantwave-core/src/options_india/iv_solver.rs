// Copyright © 2013-2024 Peter Jäckel.
// Ported to Rust for QuantWave.
// Permission to use, copy, modify, and distribute this software is freely granted,
// provided that this notice is preserved.

use std::f64;

// Constants from normaldistribution.h and lets_be_rational.cpp
const SQRT_TWO: f64 = 1.4142135623730950488016887242096980785696718753769;
const SQRT_TWO_PI: f64 = 2.5066282746310005024157652848110452530069867406099;
const LN_TWO_PI: f64 = 1.8378770664093454835606594728112352797227949472756;
const TWO_PI: f64 = 6.283185307179586476925286766559005768394338798750;
const SQRT_PI_OVER_TWO: f64 = 1.253314137315500251207882642405522626503493370305;
const SQRT_THREE: f64 = 1.732050807568877293527446341505872366942805253810;
const SQRT_ONE_OVER_THREE: f64 = 0.577350269189625764509148780501957455647601751270;
const TWO_PI_OVER_SQRT_TWENTY_SEVEN: f64 = 1.209199576156145233729385505094770488189377498728;
const SQRT_THREE_OVER_THIRD_ROOT_TWO_PI: f64 = 0.938643487427383566075051356115075878414688769574;
const PI_OVER_SIX: f64 = 0.523598775598298873077107230546583814032861566563;

const DBL_EPSILON: f64 = f64::EPSILON;
const DBL_MIN: f64 = f64::MIN_POSITIVE;
const DBL_MAX: f64 = f64::MAX;

const ETA: f64 = -13.0;
const VOLATILITY_VALUE_TO_SIGNAL_PRICE_IS_BELOW_INTRINSIC: f64 = -f64::MAX;
const VOLATILITY_VALUE_TO_SIGNAL_PRICE_IS_ABOVE_MAXIMUM: f64 = f64::MAX;

// ---------------------------------------------------------------------------------------------------------------------
// Cody's Error Function implementations (from erf_cody.cpp)
// ---------------------------------------------------------------------------------------------------------------------

fn d_int(x: f64) -> f64 {
    if x > 0.0 {
        x.floor()
    } else {
        -(-x).floor()
    }
}

fn smoothened_exponential_of_negative_square(y: f64) -> f64 {
    let y_tilde = d_int(y * 16.0) / 16.0;
    let del = (y - y_tilde) * (y + y_tilde);
    (-y_tilde * y_tilde).exp() * (-del).exp()
}

fn smoothened_exponential_of_positive_square(x: f64) -> f64 {
    let x_tilde = d_int(x * 16.0) / 16.0;
    let del = (x - x_tilde) * (x + x_tilde);
    (x_tilde * x_tilde).exp() * (del).exp()
}

const CODY_A: [f64; 5] = [
    3.1611237438705656,
    113.864154151050156,
    377.485237685302021,
    3209.37758913846947,
    0.185777706184603153,
];
const CODY_B: [f64; 4] = [
    23.6012909523441209,
    244.024637934444173,
    1282.61652607737228,
    2844.23683343917062,
];
const CODY_C: [f64; 9] = [
    0.564188496988670089,
    8.88314979438837594,
    66.1191906371416295,
    298.635138197400131,
    881.95222124176909,
    1712.04761263407058,
    2051.07837782607147,
    1230.33935479799725,
    2.15311535474403846E-8,
];
const CODY_D: [f64; 8] = [
    15.7449261107098347,
    117.693950891312499,
    537.181101862009858,
    1621.38957456669019,
    3290.79923573345963,
    4362.61909014324716,
    3439.36767414372164,
    1230.33935480374942,
];
const CODY_P: [f64; 6] = [
    0.305326634961232344,
    0.360344899949804439,
    0.125781726111229246,
    0.0160837851487422766,
    6.58749161529837803E-4,
    0.0163153871373020978,
];
const CODY_Q: [f64; 5] = [
    2.56852019228982242,
    1.87295284992346047,
    0.527905102951428412,
    0.0605183413124413191,
    0.00233520497626869185,
];

fn cody_ab(z: f64) -> f64 {
    ((((CODY_A[4] * z + CODY_A[0]) * z + CODY_A[1]) * z + CODY_A[2]) * z + CODY_A[3])
        / ((((z + CODY_B[0]) * z + CODY_B[1]) * z + CODY_B[2]) * z + CODY_B[3])
}

fn cody_cd(y: f64) -> f64 {
    ((((((((CODY_C[8] * y + CODY_C[0]) * y + CODY_C[1]) * y + CODY_C[2]) * y + CODY_C[3]) * y
        + CODY_C[4])
        * y
        + CODY_C[5])
        * y
        + CODY_C[6])
        * y
        + CODY_C[7])
        / ((((((((y + CODY_D[0]) * y + CODY_D[1]) * y + CODY_D[2]) * y + CODY_D[3]) * y + CODY_D[4])
            * y
            + CODY_D[5])
            * y
            + CODY_D[6])
            * y
            + CODY_D[7])
}

fn cody_pq(z: f64) -> f64 {
    z * (((((CODY_P[5] * z + CODY_P[0]) * z + CODY_P[1]) * z + CODY_P[2]) * z + CODY_P[3]) * z
        + CODY_P[4])
        / (((((z + CODY_Q[0]) * z + CODY_Q[1]) * z + CODY_Q[2]) * z + CODY_Q[3]) * z + CODY_Q[4])
}

const ONE_OVER_SQRT_PI: f64 = 0.56418958354775628695;
const THRESHOLD: f64 = 0.46875;
const XNEG: f64 = -26.6287357137514;
const XBIG: f64 = 26.543;
const XHUGE: f64 = 6.71E7;
const XMAX: f64 = 2.53E307;

pub fn erfc_cody(x: f64) -> f64 {
    let y = x.abs();
    if y <= THRESHOLD {
        return 1.0 - x * cody_ab(y * y);
    }
    let erfc_abs_x = if y >= XBIG {
        0.0
    } else {
        let val = if y <= 4.0 {
            cody_cd(y)
        } else {
            (ONE_OVER_SQRT_PI - cody_pq(1.0 / (y * y))) / y
        };
        val * smoothened_exponential_of_negative_square(y)
    };
    if x < 0.0 {
        2.0 - erfc_abs_x
    } else {
        erfc_abs_x
    }
}

pub fn erf_cody(x: f64) -> f64 {
    let y = x.abs();
    if y <= THRESHOLD {
        return x * cody_ab(y * y);
    }
    let erfc_abs_x = if y >= XBIG {
        0.0
    } else {
        let val = if y <= 4.0 {
            cody_cd(y)
        } else {
            (ONE_OVER_SQRT_PI - cody_pq(1.0 / (y * y))) / y
        };
        val * smoothened_exponential_of_negative_square(y)
    };
    if x < 0.0 {
        erfc_abs_x - 1.0
    } else {
        1.0 - erfc_abs_x
    }
}

pub fn erfcx_cody(x: f64) -> f64 {
    let y = x.abs();
    if y <= THRESHOLD {
        let z = y * y;
        return z.exp() * (1.0 - x * cody_ab(z));
    }
    if x < XNEG {
        return f64::MAX;
    }
    let result = if y <= 4.0 {
        cody_cd(y)
    } else {
        (ONE_OVER_SQRT_PI - cody_pq(1.0 / (y * y))) / y
    };
    if x < 0.0 {
        let expx2 = smoothened_exponential_of_positive_square(x);
        (expx2 + expx2) - result
    } else {
        result
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Normal Distribution Functions (from normaldistribution.cpp)
// ---------------------------------------------------------------------------------------------------------------------

pub fn norm_pdf(x: f64) -> f64 {
    (1.0 / SQRT_TWO_PI) * (-0.5 * x * x).exp()
}

pub fn norm_cdf(z: f64) -> f64 {
    if z <= -10.0 {
        let mut sum = 1.0;
        if z >= -1.0 / f64::EPSILON.sqrt() {
            let zsqr = z * z;
            let mut i = 1.0;
            let mut g = 1.0;
            let mut a = f64::MAX;
            let mut lasta;
            loop {
                lasta = a;
                let x = (4.0 * i - 3.0) / zsqr;
                let y = x * ((4.0 * i - 1.0) / zsqr);
                a = g * (x - y);
                sum -= a;
                g *= y;
                i += 1.0;
                a = a.abs();
                if !(lasta > a && a >= (sum * f64::EPSILON).abs()) {
                    break;
                }
            }
        }
        return -norm_pdf(z) * sum / z;
    }
    0.5 * erfc_cody(-z * (1.0 / SQRT_TWO))
}

// ---------------------------------------------------------------------------------------------------------------------
// PJ-2024-Inverse-Normal implementation (from normaldistribution.cpp)
// ---------------------------------------------------------------------------------------------------------------------

fn inverse_norm_cdf_for_low_probabilities(p: f64) -> f64 {
    let r = (-p.ln()).sqrt();
    if r < 6.7 {
        if r < 3.41 {
            if r < 2.05 {
                (3.691562302945566191 + r * (4.7170590600740689449E1 + r * (6.5451292110261454609E1 + r * (-7.4594687726045926821E1 + r * (-8.3383894003636969722E1 - 1.3054072340494093704E1 * r))))) / (1.0 + r * (2.0837211328697753726E1 + r * (7.1813812182579255459E1 + r * (5.9270122556046077717E1 + r * (9.2216887978737432303 + 1.8295174852053530579E-4 * r)))))
            } else {
                (3.2340179116317970288 + r * (1.449177828689122096E1 + r * (6.8397370256591532878E-1 + r * (-1.81254427791789183E1 + r * (-1.005916339568646151E1 - 1.2013147879435525574E0 * r))))) / (1.0 + r * (8.8820931773304337525 + r * (1.4656370665176799712E1 + r * (7.1369811056109768745 + r * (8.4884892199149255469E-1 + 1.0957576098829595323E-5 * r)))))
            }
        } else {
            (3.1252235780087584807 + r * (9.9483724317036560676 + r * (-5.1633929115525534628 + r * (-1.1070534689309368061E1 + r * (-2.8699061335882526744 - 1.5414319494013597492E-1 * r))))) / (1.0 + r * (7.076769154309171622 + r * (8.1086341122361532407 + r * (2.0307076064309043613 + r * (1.0897972234131828901E-1 + 1.3565983564441297634E-7 * r)))))
        }
    } else {
        if r < 12.9 {
            (2.6161264950897283681 + r * (2.250881388987032271 + r * (-3.688196041019692267 + r * (-2.9644251353150605663 + r * (-4.7595169546783216436E-1 - 1.612303318390145052E-2 * r))))) / (1.0 + r * (3.2517455169035921495 + r * (2.1282030272153188194 + r * (3.3663746405626400164E-1 + r * (1.1400087282177594359E-2 + 3.0848093570966787291E-9 * r)))))
        } else {
            (2.3226849047872302955 + r * (-4.2799650734502094297E-2 + r * (-2.5894451568465728432 + r * (-8.6385181219213758847E-1 + r * (-6.5127593753781672404E-2 - 1.0566357727202585402E-3 * r))))) / (1.0 + r * (1.9361316119254412206 + r * (6.1320841329197493341E-1 + r * (4.6054974512474443189E-2 + r * (7.471447992167225483E-4 + 2.3135343206304887818E-11 * r)))))
        }
    }
}

const U_MAX: f64 = 0.3413447460685429;

fn inverse_norm_cdfm05_for_midrange_probabilities(u: f64) -> f64 {
    let s = U_MAX * U_MAX - u * u;
    u * ((2.92958954698308805 + s * (5.0260572167303103E1 + s * (3.01870541922933937E2 + s * (7.4997781456657924E2 + s * (6.90489242061408612E2 + s * (1.34233243502653864E2 - 7.58939881401259242 * s)))))) / (1.0 + s * (1.8918538074574598E1 + s * (1.29404120448755281E2 + s * (3.86821208540417453E2 + s * (4.79123914509756757E2 + 1.79227008508102628E2 * s))))))
}

pub fn inverse_norm_cdf(p: f64) -> f64 {
    let u = p - 0.5;
    if u.abs() < U_MAX {
        return inverse_norm_cdfm05_for_midrange_probabilities(u);
    }
    if u > 0.0 {
        -inverse_norm_cdf_for_low_probabilities(1.0 - p)
    } else {
        inverse_norm_cdf_for_low_probabilities(p)
    }
}

pub fn erfinv(e: f64) -> f64 {
    if e.abs() < (2.0 * U_MAX) {
        return inverse_norm_cdfm05_for_midrange_probabilities(0.5 * e) * (1.0 / SQRT_TWO);
    }
    let val = if e < 0.0 {
        inverse_norm_cdf_for_low_probabilities(0.5 * e + 0.5)
    } else {
        -inverse_norm_cdf_for_low_probabilities(-0.5 * e + 0.5)
    };
    val * (1.0 / SQRT_TWO)
}

// ---------------------------------------------------------------------------------------------------------------------
// Rational Cubic Interpolation (from rationalcubic.cpp)
// ---------------------------------------------------------------------------------------------------------------------

const MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 = -(1.0 - 1.4901161193847656e-08); // -(1 - sqrt(DBL_EPSILON))
const MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 = 2.0 / (f64::EPSILON * f64::EPSILON);

fn is_zero(x: f64) -> bool {
    x.abs() < f64::MIN_POSITIVE
}

fn rational_cubic_interpolation(
    x: f64,
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    r: f64,
) -> f64 {
    let h = x_r - x_l;
    if h.abs() <= 0.0 {
        return 0.5 * (y_l + y_r);
    }
    let t = (x - x_l) / h;
    if !(r >= MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE) {
        let omt = 1.0 - t;
        let t2 = t * t;
        let omt2 = omt * omt;
        return (y_r * t2 * t + (r * y_r - h * d_r) * t2 * omt + (r * y_l + h * d_l) * t * omt2
            + y_l * omt2 * omt)
            / (1.0 + (r - 3.0) * t * omt);
    }
    y_r * t + y_l * (1.0 - t)
}

fn rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_l: f64,
) -> f64 {
    let h = x_r - x_l;
    let numerator = 0.5 * h * second_derivative_l + (d_r - d_l);
    let denominator = (y_r - y_l) / h - d_l;
    if is_zero(denominator) {
        if numerator > 0.0 {
            MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        } else {
            MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        }
    } else {
        numerator / denominator
    }
}

fn rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64,
) -> f64 {
    let h = x_r - x_l;
    let numerator = 0.5 * h * second_derivative_r + (d_r - d_l);
    let denominator = d_r - (y_r - y_l) / h;
    if is_zero(denominator) {
        if numerator > 0.0 {
            MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        } else {
            MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        }
    } else {
        numerator / denominator
    }
}

fn minimum_rational_cubic_control_parameter(
    d_l: f64,
    d_r: f64,
    s: f64,
    prefer_shape_preservation: bool,
) -> f64 {
    let monotonic = d_l * s >= 0.0 && d_r * s >= 0.0;
    let convex = d_l <= s && s <= d_r;
    let concave = d_l >= s && s >= d_r;
    if !monotonic && !convex && !concave {
        return MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    let d_r_m_d_l = d_r - d_l;
    let d_r_m_s = d_r - s;
    let s_m_d_l = s - d_l;
    let mut r1 = f64::MIN;
    let mut r2 = r1;
    if monotonic {
        if !is_zero(s) {
            r1 = (d_r + d_l) / s;
        } else if prefer_shape_preservation {
            r1 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
    }
    if convex || concave {
        if !(is_zero(s_m_d_l) || is_zero(d_r_m_s)) {
            r2 = (d_r_m_d_l / d_r_m_s).abs().max((d_r_m_d_l / s_m_d_l).abs());
        } else if prefer_shape_preservation {
            r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
    } else if monotonic && prefer_shape_preservation {
        r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE.max(r1.max(r2))
}

fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_l: f64,
    prefer_shape_preservation: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_l,
    );
    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        prefer_shape_preservation,
    );
    r.max(r_min)
}

fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64,
    prefer_shape_preservation: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_r,
    );
    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        prefer_shape_preservation,
    );
    r.max(r_min)
}

// ---------------------------------------------------------------------------------------------------------------------
// Let's Be Rational core implementation
// ---------------------------------------------------------------------------------------------------------------------

fn householder3_factor(nu: f64, h2: f64, h3: f64) -> f64 {
    (1.0 + 0.5 * h2 * nu) / (1.0 + nu * (h2 + h3 * nu * (1.0 / 6.0)))
}

fn householder4_factor(nu: f64, h2: f64, h3: f64, h4: f64) -> f64 {
    (1.0 + nu * (h2 + nu * h3 * (1.0 / 6.0)))
        / (1.0 + nu * (1.5 * h2 + nu * (h2 * h2 * 0.25 + h3 * (1.0 / 3.0) + nu * h4 * (1.0 / 24.0))))
}

fn householder_factor_3(nu: f64, h2: f64, h3: f64) -> f64 {
    householder3_factor(nu, h2, h3)
}

fn householder_factor_4(nu: f64, h2: f64, h3: f64, h4: f64) -> f64 {
    householder4_factor(nu, h2, h3, h4)
}

fn normalised_intrinsic(theta_x: f64) -> f64 {
    if theta_x <= 0.0 {
        0.0
    } else {
        (0.5 * theta_x).sinh() * 2.0
    }
}

fn square(x: f64) -> f64 {
    x * x
}

const SIXTEENTH_ROOT_DBL_EPSILON: f64 = 0.10511205190671433; // sqrt(sqrt(sqrt(sqrt(f64::EPSILON))))
const TAU: f64 = 2.0 * SIXTEENTH_ROOT_DBL_EPSILON;

fn asymptotic_expansion_of_scaled_normalised_black(h: f64, t: f64) -> f64 {
    let e = square(t / h);
    let r = (h + t) * (h - t);
    let q = square(h / r);

    let a0 = 2.0;
    let a1 = -6.0 - 2.0 * e;
    let a2 = 30.0 + e * (60.0 + 6.0 * e);
    let a3 = -2.1E2 + e * (-1.05E3 + e * (-6.3E2 - 30.0 * e));
    let a4 = 1.89E3 + e * (1.764E4 + e * (2.646E4 + e * (7.56E3 + 2.1E2 * e)));
    let a5 = -2.079E4 + e * (-3.1185E5 + e * (-8.7318E5 + e * (-6.237E5 + e * (-1.0395E5 - 1.89E3 * e))));
    let a6 = 2.7027E5
        + e * (5.94594E6
            + e * (2.675673E7 + e * (3.567564E7 + e * (1.486485E7 + e * (1.62162E6 + 2.079E4 * e)))));
    let a7 = -4.05405E6
        + e * (-1.2297285E8
            + e * (-8.1162081E8
                + e * (-1.73918745E9
                    + e * (-1.35270135E9 + e * (-3.6891855E8 + e * (-2.837835E7 - 2.7027E5 * e))))));
    let a8 = 6.891885E7
        + e * (2.756754E9
            + e * (2.50864614E10
                + e * (7.88431644E10
                    + e * (9.85539555E10
                        + e * (5.01729228E10
                            + e * (9.648639E9 + e * (5.513508E8 + 4.05405E6 * e)))))));
    let a9 = -1.30945815E9
        + e * (-6.678236565E10
            + e * (-8.013883878E11
                + e * (-3.4726830138E12
                    + e * (-6.3665855253E12
                        + e * (-5.2090245207E12
                            + e * (-1.8699062382E12
                                + e * (-2.671294626E11
                                    + e * (-1.178512335E10 - 6.891885E7 * e))))))));
    let a10 = 2.749862115E10
        + e * (1.7415793395E12
            + e * (2.664616389435E13
                + e * (1.52263793682E14
                    + e * (3.848890340295E14
                        + e * (4.618668408354E14
                            + e * (2.664616389435E14
                                + e * (7.10564370516E13
                                    + e * (7.83710702775E12 + e * (2.749862115E11 + 1.30945815E9 * e)))))))));
    let a11 = -6.3246828645E11
        + e * (-4.870005805665E13
            + e * (-9.2530110307635E14
                + e * (-6.74147946527055E15
                    + e * (-2.24715982175685E16
                        + e * (-3.71802806872497E16
                            + e * (-3.14602375045959E16
                                + e * (-1.34829589305411E16
                                    + e * (-2.77590330922905E15
                                        + e * (-2.4350029028325E14
                                            + e * (-6.95715115095E12 - 2.749862115E10 * e))))))))));
    let a12 = 1.581170716125E13
        + e * (1.454677058835E15
            + e * (3.36030400590885E16
                + e * (3.04027505296515E17
                    + e * (1.29211689751018875E18
                        + e * (2.81916414002223E18
                            + e * (3.289024830025935E18
                                + e * (2.067387036016302E18
                                    + e * (6.8406188691715875E17
                                        + e * (1.12010133530295E17
                                            + e * (8.0007238235925E15
                                                + e * (1.89740485935E14 + 6.3246828645E11 * e)))))))))));
    let a13 = -4.2691609335375E14
        + e * (-4.624924344665625E16
            + e * (-1.2764791191277125E18
                + e * (-1.40412703104048375E19
                    + e * (-7.41067044160255312E19
                        + e * (-2.06151377739125569E20
                            + e * (-3.17155965752500875E20
                                + e * (-2.74868503652167425E20
                                    + e * (-1.33392067948845956E20
                                        + e * (-3.51031757760120938E19
                                            + e * (-4.6804234368016125E18
                                                + e * (-2.774954606799375E17
                                                    + e * (-5.54990921359875E15 - 1.581170716125E13 * e))))))))))));
    let a14 = 1.238056670725875E16
        + e * (1.5599514051146025E18
            + e * (5.06984206662245812E19
                + e * (6.66322100184665925E20
                    + e * (4.27556680951827302E21
                        + e * (1.47701398874267613E22
                            + e * (2.89721974714909549E22
                                + e * (3.31110828245610914E22
                                    + e * (2.2155209831140142E22
                                        + e * (8.55113361903654604E21
                                            + e * (1.83238577550783129E21
                                                + e * (2.02793682664898325E20
                                                    + e * (1.01396841332449162E19
                                                        + e * (1.733279339016225E17 + 4.2691609335375E14 * e)))))))))))));
    let a15 = -3.8379756792502125E17
        + e * (-5.56506473491280812E19
            + e * (-2.10359446979704147E21
                + e * (-3.25556286992399275E22
                    + e * (-2.49593153360839444E23
                        + e * (-1.04829124411552567E24
                            + e * (-2.55352995361474201E24
                                + e * (-3.72085793241005264E24
                                    + e * (-3.28310994036181115E24
                                        + e * (-1.74715207352587611E24
                                            + e * (-5.49104937393846778E23
                                                + e * (-9.76668860977197826E22
                                                    + e * (-9.11557603578717971E21
                                                        + e * (-3.89554531443896569E20
                                                            + e * (-5.75696351887531875E18 - 1.238056670725875E16 * e))))))))))))));
    let a16 = 1.26653197415257012E19
        + e * (2.09399953059891594E21
            + e * (9.10889795810528434E22
                + e * (1.63960163245895118E24
                    + e * (1.48019591819210871E25
                        + e * (7.42789224401858187E25
                            + e * (2.19979885688242617E26
                                + e * (3.98058840769200926E26
                                    + e * (4.47816195865351041E26
                                        + e * (3.1425697955463231E26
                                            + e * (1.36178024473674001E26
                                                + e * (3.55247020366106089E25
                                                    + e * (5.32870530549159134E24
                                                        + e * (4.25081904711579936E23
                                                            + e * (1.57049964794918696E22
                                                                + e * (2.0264511586441122E20 + 3.8379756792502125E17 * e)))))))))))))));

    let mut omega = 0.0;
    let thresholds = [
        12.347, 12.958, 13.729, 14.718, 16.016, 17.769, 20.221, 23.816, 29.419, 38.93, 57.171,
        99.347,
    ];
    let val_to_check = -h - t + TAU + 0.5;
    let idx = thresholds.iter().position(|&thr| thr > val_to_check).unwrap_or(thresholds.len());

    if idx <= 0 { omega = q * (a16 + omega); }
    if idx <= 1 { omega = q * (a15 + omega); }
    if idx <= 2 { omega = q * (a14 + omega); }
    if idx <= 3 { omega = q * (a13 + omega); }
    if idx <= 4 { omega = q * (a12 + omega); }
    if idx <= 5 { omega = q * (a11 + omega); }
    if idx <= 6 { omega = q * (a10 + omega); }
    if idx <= 7 { omega = q * (a9 + omega); }
    if idx <= 8 { omega = q * (a8 + omega); }
    if idx <= 9 { omega = q * (a7 + omega); }
    if idx <= 10 { omega = q * (a6 + omega); }
    if idx <= 11 { omega = q * (a5 + omega); }
    
    omega = a0 + q * (a1 + q * (a2 + q * (a3 + q * (a4 + omega))));

    (t / r) * omega
}

fn yprime_tail_expansion_rational_function_part(w: f64) -> f64 {
    w * (-2.9999999999994663866
        + w * (-1.7556263323542206288E2
            + w * (-3.4735035445495633334E3
                + w * (-2.7805745693864308643E4
                    + w * (-8.3836021460741980839E4 - 6.6818249032616849037E4 * w)))))
        / (1.0
            + w * (6.3520877744831739102E1
                + w * (1.4404389037604337538E3
                    + w * (1.4562545638507033944E4
                        + w * (6.6886794165651675684E4
                            + w * (1.2569970380923908488E5 + 6.9286518679803751694E4 * w))))))
}

fn yprime(h: f64) -> f64 {
    if h < -4.0 {
        let w = 1.0 / (h * h);
        return w * (1.0 + yprime_tail_expansion_rational_function_part(w));
    }
    if h <= -0.46875 {
        return (1.0000000000594317229
            - h * (6.1911449879694112749E-1
                - h * (2.2180844736576013957E-1
                    - h * (4.5650900351352987865E-2
                        - h * (5.545521007735379052E-3
                            - h * (3.0717392274913902347E-4
                                - h * (4.2766597835908713583E-8 + 8.4592436406580605619E-10 * h)))))))
            / (1.0
                - h * (1.8724286369589162071
                    - h * (1.5685497236077651429
                        - h * (7.6576489836589035112E-1
                            - h * (2.3677701403094640361E-1
                                - h * (4.6762548903194957675E-2
                                    - h * (5.5290453576936595892E-3 - 3.0822020417927147113E-4 * h)))))));
    }
    1.0 + h * SQRT_PI_OVER_TWO * erfcx_cody(-(1.0 / SQRT_TWO) * h)
}

fn small_t_expansion_of_scaled_normalised_black(h: f64, t: f64) -> f64 {
    let a = yprime(h);
    let h2 = h * h;
    let t2 = t * t;
    let b0 = 2.0 * a;
    let b1 = (-1.0 + a * (3.0 + h2)) / 3.0;
    let b2 = (-7.0 - h2 + a * (15.0 + h2 * (10.0 + h2))) / 60.0;
    let b3 = (-57.0 + (-18.0 - h2) * h2 + a * (105.0 + h2 * (105.0 + h2 * (21.0 + h2)))) / 2520.0;
    let b4 = (-561.0 + h2 * (-285.0 + (-33.0 - h2) * h2) + a * (945.0 + h2 * (1260.0 + h2 * (378.0 + h2 * (36.0 + h2))))) / 181440.0;
    let b5 = (-6555.0 + h2 * (-4680.0 + h2 * (-840.0 + (-52.0 - h2) * h2)) + a * (10395.0 + h2 * (17325.0 + h2 * (6930.0 + h2 * (990.0 + h2 * (55.0 + h2)))))) / 19958400.0;
    let b6 = (-89055.0 + h2 * (-82845.0 + h2 * (-20370.0 + h2 * (-1926.0 + (-75.0 - h2) * h2))) + a * (135135.0 + h2 * (270270.0 + h2 * (135135.0 + h2 * (25740.0 + h2 * (2145.0 + h2 * (78.0 + h2))))))) / 3113510400.0;
    t * (b0 + t2 * (b1 + t2 * (b2 + t2 * (b3 + t2 * (b4 + t2 * (b5 + b6 * t2))))))
}

fn normalised_black_with_optimal_use_of_codys_functions(theta_x: f64, s: f64) -> f64 {
    let codys_threshold = 0.46875;
    let h = theta_x / s;
    let t = 0.5 * s;
    let q1 = -(1.0 / SQRT_TWO) * (h + t);
    let q2 = -(1.0 / SQRT_TWO) * (h - t);
    let two_b = if q1 < codys_threshold {
        if q2 < codys_threshold {
            (0.5 * theta_x).exp() * erfc_cody(q1) - (-0.5 * theta_x).exp() * erfc_cody(q2)
        } else {
            (0.5 * theta_x).exp() * erfc_cody(q1) - (-0.5 * (h * h + t * t)).exp() * erfcx_cody(q2)
        }
    } else {
        if q2 < codys_threshold {
            (-0.5 * (h * h + t * t)).exp() * erfcx_cody(q1) - (-0.5 * theta_x).exp() * erfc_cody(q2)
        } else {
            (-0.5 * (h * h + t * t)).exp() * (erfcx_cody(q1) - erfcx_cody(q2))
        }
    };
    (0.5 * two_b).max(0.0)
}

fn normalised_vega(x: f64, s: f64) -> f64 {
    let h = x / s;
    let t = 0.5 * s;
    (1.0 / SQRT_TWO_PI) * (-0.5 * (h * h + t * t)).exp()
}

fn inv_normalised_vega(x: f64, s: f64) -> f64 {
    let h = x / s;
    let t = 0.5 * s;
    SQRT_TWO_PI * (0.5 * (h * h + t * t)).exp()
}

fn ln_normalised_vega(x: f64, s: f64) -> f64 {
    let h = x / s;
    let t = 0.5 * s;
    -(LN_TWO_PI * 0.5) - 0.5 * (h * h + t * t)
}

fn is_region_i(theta_x: f64, s: f64) -> bool {
    theta_x < s * ETA && s * (0.5 * s - (TAU + 0.5 + ETA)) + theta_x < 0.0
}

fn is_region_ii(theta_x: f64, s: f64) -> bool {
    s * (s - (2.0 * TAU)) - theta_x / ETA < 0.0
}

fn normalised_black_internal(theta_x: f64, s: f64) -> f64 {
    if is_region_i(theta_x, s) {
        return asymptotic_expansion_of_scaled_normalised_black(theta_x / s, 0.5 * s)
            * normalised_vega(theta_x, s);
    }
    if is_region_ii(theta_x, s) {
        return small_t_expansion_of_scaled_normalised_black(theta_x / s, 0.5 * s)
            * normalised_vega(theta_x, s);
    }
    normalised_black_with_optimal_use_of_codys_functions(theta_x, s)
}

fn scaled_normalised_black_and_ln_vega_internal(theta_x: f64, s: f64) -> (f64, f64) {
    if is_region_i(theta_x, s) {
        return (
            asymptotic_expansion_of_scaled_normalised_black(theta_x / s, 0.5 * s),
            ln_normalised_vega(theta_x, s),
        );
    }
    if is_region_ii(theta_x, s) {
        return (
            small_t_expansion_of_scaled_normalised_black(theta_x / s, 0.5 * s),
            ln_normalised_vega(theta_x, s),
        );
    }
    let ln_vega = ln_normalised_vega(theta_x, s);
    (
        normalised_black_with_optimal_use_of_codys_functions(theta_x, s) * (-ln_vega).exp(),
        ln_vega,
    )
}

fn compute_f_lower_map_and_first_two_derivatives(x: f64, s: f64) -> (f64, f64, f64) {
    let ax = x.abs();
    let z = SQRT_ONE_OVER_THREE * ax / s;
    let y = z * z;
    let s2 = s * s;
    let phi_z = 0.5 * erfc_cody((1.0 / SQRT_TWO) * z); // norm_cdf(-z)
    let pdf_z = norm_pdf(z);
    let fpp = PI_OVER_SIX * y / (s2 * s) * phi_z
        * (8.0 * SQRT_THREE * s * ax + (3.0 * s2 * (s2 - 8.0) - 8.0 * x * x) * phi_z / pdf_z)
        * (2.0 * y + 0.25 * s2).exp();
    let phi_z2 = phi_z * phi_z;
    let fp = TWO_PI * y * phi_z2 * (y + 0.125 * s2).exp();
    let f = TWO_PI_OVER_SQRT_TWENTY_SEVEN * ax * (phi_z2 * phi_z);
    (f, fp, fpp)
}

fn inverse_f_lower_map(x: f64, f: f64) -> f64 {
    (x / (SQRT_THREE
        * inverse_norm_cdf(
            SQRT_THREE_OVER_THIRD_ROOT_TWO_PI * f.cbrt() / x.abs().cbrt(),
        )))
    .abs()
}

fn compute_f_upper_map_and_first_two_derivatives(x: f64, s: f64) -> (f64, f64, f64) {
    let f = 0.5 * erfc_cody((0.5 / SQRT_TWO) * s); // norm_cdf(-0.5 * s)
    let w = square(x / s);
    let fp = -0.5 * (0.5 * w).exp();
    let fpp = SQRT_PI_OVER_TWO * (w + 0.125 * s * s).exp() * w / s;
    (f, fp, fpp)
}

fn inverse_f_upper_map(f: f64) -> f64 {
    -2.0 * inverse_norm_cdf(f)
}

fn one_minus_erfcx(x: f64) -> f64 {
    if x < -1.0 / 5.0 || x > 1.0 / 3.0 {
        return 1.0 - erfcx_cody(x);
    }
    x * (1.128379167095512573896
        - x * (1.0000000000000002
            + x * (1.1514967181784756
                + x * (5.7689001208873741E-1
                    + x * (1.4069188744609651E-1 + 1.4069285713634565E-2 * x))))
            / (1.0
                + x * (1.9037494962421563
                    + x * (1.5089908593742723
                        + x * (6.2486081658640257E-1
                            + x * (1.358008134514386E-1 + 1.2463320728346347E-2 * x))))))
}

fn implied_normalised_volatility_atm(beta: f64) -> f64 {
    let beta_max = 0.6826894921370859;
    if beta <= beta_max {
        let r = beta_max * beta_max - beta * beta;
        return beta
            * ((2.92958954698308816
                + r * (1.4014698674754995E1
                    + r * (2.44918990556468762E1
                        + r * (1.90763928424894996E1
                            + r * (6.43250149461895996
                                + r * (7.52328633671821543E-1 + 1.38781536163865582E-2 * r))))))
                / (1.0
                    + r * (5.22443271807813073
                        + r * (1.02258209975070629E1
                            + r * (9.28187483709036392
                                + r * (3.9095549184069553
                                    + r * (6.61214199809055912E-1 + 2.89411828874884851E-2 * r)))))));
    }
    -2.0 * inverse_norm_cdf_for_low_probabilities(0.5 * (1.0 - beta))
}

fn b_l_over_b_max(s_c: f64) -> f64 {
    if s_c >= 2.6267851073127395 {
        if s_c >= 7.348469228349534 {
            let s_c_val = s_c;
            return (1.4500072297240603183E-3 + s_c_val * (-1.5116692485011195757E-3 + s_c_val * (7.1682178310936334831E-2 + s_c_val * (3.921610857820463493E-2 + s_c_val * (2.9342405658628443931E-2 + s_c_val * (5.1832526171631521426E-3 + 1.6930208078421474854E-3 * s_c_val)))))) / (1.0 + s_c_val * (1.6176313502305414664 + s_c_val * (1.6823159175281531664 + s_c_val * (8.4878307567372222113E-1 + s_c_val * (3.7543742137375791321E-1 + s_c_val * (7.126137099644302999E-2 + 1.6116992546788676159E-2 * s_c_val))))));
        }
        return (-9.3325115354837883291E-5 + s_c * (5.3118033972794648837E-4 + s_c * (7.4114855448345002595E-2 + s_c * (7.4039658186822817454E-2 + s_c * (3.9225177407687604785E-2 + s_c * (1.0022913378254090083E-2 + 1.7012579407246055469E-3 * s_c)))))) / (1.0 + s_c * (2.2217238132228132256 + s_c * (2.3441816707087403282 + s_c * (1.3912323646271141826 + s_c * (5.3231258443501838354E-1 + s_c * (1.1744005919716101572E-1 + 1.6195405895930935811E-2 * s_c))))));
    }
    if s_c >= 0.7099295739719539 {
        return (1.9795737927598581235E-9 + s_c * (-2.7081288564685588037E-8 + s_c * (7.5610142272549044609E-2 + s_c * (6.917130174466834016E-2 + s_c * (2.9537058950963019803E-2 + s_c * (6.5849252702302307774E-3 + 6.9711400639834715731E-4 * s_c)))))) / (1.0 + s_c * (2.1941448525586579756 + s_c * (2.1297103549995181357 + s_c * (1.1571483187179784072 + s_c * (3.7831622253060456794E-1 + s_c * (7.1714862448829349869E-2 + 6.6361975827861200167E-3 * s_c))))));
    }
    let g = (8.0741072372882856924E-2 + s_c * (9.8078911786358897272E-2 + s_c * (3.9760631445677058375E-2 + s_c * (5.9716928459589189876E-3 + s_c * (-6.4036399341479799981E-6 + 4.5425102093616062245E-7 * s_c))))) / (1.0 + s_c * (1.8594977672287664353 + s_c * (1.3658801475711790419 + s_c * (4.6132707108655653215E-1 + 6.1254597049831720643E-2 * s_c))));
    (s_c * s_c) * (0.07560996640296361767172 + s_c * (s_c * g - 0.09672719281339436290858))
}

fn b_u_over_b_max(s_c: f64) -> f64 {
    if s_c > 1.7888543819998317 {
        if s_c > 6.164414002968976 {
            let s_c_val = s_c;
            return (7.91133825948419359E-1 + s_c_val * (1.24653733210880042 + s_c_val * (1.32747426980537386 + s_c_val * (6.95009705717846778E-1 + s_c_val * (3.05965944268228457E-1 + s_c_val * (6.02200363391352887E-2 + 1.29050244454344842E-2 * s_c_val)))))) / (1.0 + s_c_val * (1.58117486714634672 + s_c_val * (1.60144713247629644 + s_c_val * (8.30040185836882436E-1 + s_c_val * (3.53071863813401531E-1 + s_c_val * (6.95901684131758475E-2 + 1.44197580643890011E-2 * s_c_val))))));
        }
        return (7.8990640048967596475E-1 + s_c * (1.5993699253596663678 + s_c * (1.6481729039140370242 + s_c * (9.8227188109869200166E-1 + s_c * (3.6313557966186936883E-1 + s_c * (7.8277036261179606301E-2 + 9.3404307364538726214E-3 * s_c)))))) / (1.0 + s_c * (2.0247407005640401446 + s_c * (2.0087454279103740489 + s_c * (1.1627561803056961973 + s_c * (4.2004672123723823581E-1 + s_c * (8.9130862793887234546E-2 + 1.0436767768858021717E-2 * s_c))))));
    }
    if s_c >= 0.7745966692414833 {
        return (7.8990944435755287611E-1 + s_c * (-1.2655410534988972886 + s_c * (-2.8803040699221003256 + s_c * (-2.6936198689113258727 + s_c * (-1.1213067281643205754 + s_c * (-2.1277793801691629892E-1 + 5.1486445905299802703E-6 * s_c)))))) / (1.0 + s_c * (-1.6021222722060444448 + s_c * (-3.7242680976480704555 + s_c * (-3.2083117718907365085 + s_c * (-1.2922333835930958583 - 2.3762328334050001161E-1 * s_c)))));
    }
    let g = (-6.063099881233561706E-2 + s_c * (-8.1011946637120604985E-2 + s_c * (-4.2505564862438753828E-2 + s_c * (-8.9880000946868691788E-3 + s_c * (-7.5603072110443268356E-6 + 4.3879556621540147458E-7 * s_c))))) / (1.0 + s_c * (1.8400371530721828756 + s_c * (1.5709283443886143691 + s_c * (6.8913245453611400484E-1 + 1.4703173061720980923E-1 * s_c))));
    0.7899085945560627246288 + (s_c * s_c) * (0.0614616805805147403487 + s_c * g)
}

pub fn lets_be_rational(beta: f64, theta_x: f64, n_iterations: i32) -> f64 {
    if beta <= 0.0 {
        return if beta == 0.0 { 0.0 } else { VOLATILITY_VALUE_TO_SIGNAL_PRICE_IS_BELOW_INTRINSIC };
    }
    let b_max = (0.5 * theta_x).exp();
    if beta >= b_max {
        return VOLATILITY_VALUE_TO_SIGNAL_PRICE_IS_ABOVE_MAXIMUM;
    }
    if theta_x == 0.0 {
        return implied_normalised_volatility_atm(beta);
    }

    let mut iterations = 0;
    let mut f;
    let mut s;
    let mut ds = -DBL_MAX;
    let mut s_left = DBL_MIN;
    let mut s_right = DBL_MAX;

    let sqrt_ax = (-theta_x).sqrt();
    let s_c = SQRT_TWO * sqrt_ax;
    let ome = one_minus_erfcx(sqrt_ax);
    let b_c = 0.5 * b_max * ome;

    if beta < b_c {
        let s_l = s_c - SQRT_PI_OVER_TWO * ome;
        let b_l = b_l_over_b_max(s_c) * b_max;
        if beta < b_l {
            let (f_lower_map_l, d_f_lower_map_l_d_beta, d2_f_lower_map_l_d_beta2) =
                compute_f_lower_map_and_first_two_derivatives(theta_x, s_l);
            let r_ll = convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
                0.0,
                b_l,
                0.0,
                f_lower_map_l,
                1.0,
                d_f_lower_map_l_d_beta,
                d2_f_lower_map_l_d_beta2,
                true,
            );
            f = rational_cubic_interpolation(beta, 0.0, b_l, 0.0, f_lower_map_l, 1.0, d_f_lower_map_l_d_beta, r_ll);
            if !(f > 0.0) {
                let t = beta / b_l;
                f = (f_lower_map_l * t + b_l * (1.0 - t)) * t;
            }
            s = inverse_f_lower_map(theta_x, f);
            s_right = s_l;

            let ln_beta = beta.ln();
            while iterations < n_iterations && ds.abs() > DBL_EPSILON * s {
                let (bx, ln_vega) = scaled_normalised_black_and_ln_vega_internal(theta_x, s);
                let ln_b = bx.ln() + ln_vega;
                let bpob = 1.0 / bx;
                let b = ln_b.exp();
                if b > beta && s < s_right {
                    s_right = s;
                } else if b < beta && s > s_left {
                    s_left = s;
                }

                let h = theta_x / s;
                let x2_over_s3 = h * h / s;
                let b_h2 = x2_over_s3 - s / 4.0;
                let nu = (ln_beta - ln_b) * ln_b / ln_beta / bpob;
                let lambda = 1.0 / ln_b;
                let otl = 1.0 + 2.0 * lambda;
                let h2 = b_h2 - bpob * otl;
                let c = 3.0 * (x2_over_s3 / s);
                let b_h3 = b_h2 * b_h2 - c - 0.25;
                let mu = 6.0 * lambda * (1.0 + lambda);
                let h3 = b_h3 + (bpob * bpob) * (2.0 + mu) - (b_h2 * bpob) * 3.0 * otl;

                if theta_x < -190.0 {
                    let b_h4 = b_h2 * (b_h3 - 0.5) - (b_h2 - 2.0 / s) * 2.0 * c;
                    let bpob2 = bpob * bpob;
                    let bppob = b_h2 * bpob;
                    ds = nu * householder_factor_4(
                        nu,
                        h2,
                        h3,
                        b_h4 - bpob * (bpob2 * (6.0 + lambda * (22.0 + lambda * (36.0 + lambda * 24.0))) - bppob * (12.0 + 6.0 * mu)) - bppob * b_h2 * 3.0 * otl - b_h3 * bpob * 4.0 * otl,
                    );
                } else {
                    ds = nu * householder_factor_3(nu, h2, h3);
                }
                s += ds;
                if s > s_right { s = s_right; } else if s < s_left { s = s_left; }
                iterations += 1;
            }
            return s;
        } else {
            let inv_v_c = SQRT_TWO_PI / b_max;
            let inv_v_l = inv_normalised_vega(theta_x, s_l);
            let r_lm = convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
                b_l, b_c, s_l, s_c, inv_v_l, inv_v_c, 0.0, false,
            );
            s = rational_cubic_interpolation(beta, b_l, b_c, s_l, s_c, inv_v_l, inv_v_c, r_lm);
            s_left = s_l;
            s_right = s_c;
        }
    } else {
        let s_u = s_c + SQRT_PI_OVER_TWO * (2.0 - ome);
        let b_u = b_u_over_b_max(s_c) * b_max;
        if beta <= b_u {
            let inv_v_c = SQRT_TWO_PI / b_max;
            let inv_v_u = inv_normalised_vega(theta_x, s_u);
            let r_um = convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
                b_c, b_u, s_c, s_u, inv_v_c, inv_v_u, 0.0, false,
            );
            s = rational_cubic_interpolation(beta, b_c, b_u, s_c, s_u, inv_v_c, inv_v_u, r_um);
            s_left = s_c;
            s_right = s_u;
        } else {
            let (f_upper_map_h, d_f_upper_map_h_d_beta, d2_f_upper_map_h_d_beta2) =
                compute_f_upper_map_and_first_two_derivatives(theta_x, s_u);
            let r_uu = convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
                b_u, b_max, f_upper_map_h, 0.0, d_f_upper_map_h_d_beta, -0.5, d2_f_upper_map_h_d_beta2, true,
            );
            f = rational_cubic_interpolation(beta, b_u, b_max, f_upper_map_h, 0.0, d_f_upper_map_h_d_beta, -0.5, r_uu);
            if f <= 0.0 {
                let h = b_max - b_u;
                let t = (beta - b_u) / h;
                f = (f_upper_map_h * (1.0 - t) + 0.5 * h * t) * (1.0 - t);
            }
            s = inverse_f_upper_map(f);
            s_left = s_u;
            if beta > 0.5 * b_max {
                let beta_bar = b_max - beta;
                while iterations < n_iterations && ds.abs() > DBL_EPSILON * s {
                    let h = theta_x / s;
                    let t = s / 2.0;
                    let gp = (2.0 / SQRT_TWO_PI) / (erfcx_cody((t + h) * (1.0 / SQRT_TWO)) + erfcx_cody((t - h) * (1.0 / SQRT_TWO)));
                    let b_bar = normalised_vega(theta_x, s) / gp;
                    if b_bar < beta_bar && s < s_right {
                        s_right = s;
                    } else if b_bar > beta_bar && s > s_left {
                        s_left = s;
                    }

                    let g = (beta_bar / b_bar).ln();
                    let x2_over_s3 = (h * h) / s;
                    let b_h2 = x2_over_s3 - s / 4.0;
                    let c = 3.0 * (x2_over_s3 / s);
                    let b_h3 = b_h2 * b_h2 - c - 0.25;
                    let nu = -g / gp;
                    let h2 = b_h2 + gp;
                    let h3 = b_h3 + gp * (2.0 * gp + 3.0 * b_h2);

                    if theta_x < -580.0 {
                        let b_h4 = b_h2 * (b_h3 - 0.5) - (b_h2 - 2.0 / s) * 2.0 * c;
                        ds = nu * householder_factor_4(nu, h2, h3, b_h4 + gp * (6.0 * gp * (gp + 2.0 * b_h2) + 3.0 * b_h2 * b_h2 + 4.0 * b_h3));
                    } else {
                        ds = nu * householder_factor_3(nu, h2, h3);
                    }
                    s += ds;
                    if s > s_right { s = s_right; } else if s < s_left { s = s_left; }
                    iterations += 1;
                }
                return s;
            }
        }
    }

    while iterations < n_iterations && ds.abs() > DBL_EPSILON * s {
        let b = normalised_black_internal(theta_x, s);
        let inv_bp = inv_normalised_vega(theta_x, s);
        let nu = (beta - b) * inv_bp;
        let h = theta_x / s;
        let x2_over_s3 = (h * h) / s;
        let h2 = x2_over_s3 - s * 0.25;
        let h3 = h2 * h2 - 3.0 * (x2_over_s3 / s) - 0.25;
        if b > beta && s < s_right {
            s_right = s;
        } else if b < beta && s > s_left {
            s_left = s;
        }
        ds = nu * householder_factor_3(nu, h2, h3);
        s += ds;
        if s > s_right { s = s_right; } else if s < s_left { s = s_left; }
        iterations += 1;
    }
    s
}

pub fn normalised_black(x: f64, s: f64, theta: f64) -> f64 {
    if x == 0.0 {
        return erf_cody((0.5 / SQRT_TWO) * s);
    }
    let theta_x = if theta < 0.0 { -x } else { x };
    normalised_intrinsic(theta_x) + if s <= 0.0 { 0.0 } else { normalised_black_internal(-x.abs(), s) }
}

pub fn black(f: f64, k: f64, sigma: f64, t: f64, theta: f64) -> f64 {
    let s = sigma * t.sqrt();
    if k == f {
        return f * erf_cody((0.5 / SQRT_TWO) * s);
    }
    let mu = if theta < 0.0 {
        (k - f).max(0.0)
    } else {
        (f - k).max(0.0)
    };
    mu + if s <= 0.0 {
        0.0
    } else {
        (f.sqrt() * k.sqrt()) * normalised_black_internal(-(f / k).ln().abs(), s)
    }
}

pub fn implied_black_volatility(price: f64, f: f64, k: f64, t: f64, theta: f64) -> f64 {
    if price >= (if theta < 0.0 { k } else { f }) {
        return VOLATILITY_VALUE_TO_SIGNAL_PRICE_IS_ABOVE_MAXIMUM;
    }
    let mu = if theta < 0.0 { k - f } else { f - k };
    let adjusted_price = if mu > 0.0 { price - mu } else { price };
    lets_be_rational(
        adjusted_price / (f.sqrt() * k.sqrt()),
        -(f / k).ln().abs(),
        2,
    ) / t.sqrt()
}

pub fn normalised_implied_black_volatility(beta: f64, x: f64, theta: f64) -> f64 {
    let theta_x = if theta < 0.0 { -x } else { x };
    lets_be_rational(beta - normalised_intrinsic(theta_x), -x.abs(), 2)
}
