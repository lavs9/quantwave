//! Native moving-average type selector.
//!
//! Wire-compatible with TA-Lib's `TA_MAType` (and therefore with
//! `talib_rs::MaType`): the discriminants are the same 0..=8 integers, so the
//! Python-side `matype=<int>` arguments keep their meaning.
//!
//! This type exists so that `talib-rs` never appears in a **public** QuantWave
//! signature. It is a plain data enum with no third-party lineage; the
//! conversion to `talib_rs::MaType` lives behind `#[cfg(test)]` (see the bottom
//! of this file) because talib-rs is only ever used as a parity **oracle**.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Moving-average family used by `APO`, `PPO`, `BBANDS`, `MACDEXT`, `STOCH`, …
///
/// Discriminants match TA-Lib's `TA_MAType` exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(i32)]
pub enum MaType {
    /// Simple moving average.
    #[default]
    Sma = 0,
    /// Exponential moving average.
    Ema = 1,
    /// Weighted moving average.
    Wma = 2,
    /// Double exponential moving average.
    Dema = 3,
    /// Triple exponential moving average.
    Tema = 4,
    /// Triangular moving average.
    Trima = 5,
    /// Kaufman adaptive moving average.
    Kama = 6,
    /// MESA adaptive moving average (uses `fastlimit = 0.5`, `slowlimit = 0.05`;
    /// the period argument is ignored, matching C TA-Lib's `ta_MA.c`).
    Mama = 7,
    /// Tillson T3 (uses `vfactor = 0.7` when selected through this enum,
    /// matching C TA-Lib's `ta_MA.c`).
    T3 = 8,
}

/// Error returned when an integer or string does not name a `MaType`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidMaType(pub String);

impl fmt::Display for InvalidMaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid matype {:?}: expected 0..=8 or one of {}",
            self.0,
            MaType::ALL
                .iter()
                .map(|m| m.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::error::Error for InvalidMaType {}

impl MaType {
    /// Every variant, in TA-Lib discriminant order. Useful for parameterised
    /// parity tests — see `MaStream`'s proptests.
    pub const ALL: [MaType; 9] = [
        MaType::Sma,
        MaType::Ema,
        MaType::Wma,
        MaType::Dema,
        MaType::Tema,
        MaType::Trima,
        MaType::Kama,
        MaType::Mama,
        MaType::T3,
    ];

    /// TA-Lib integer code (0..=8).
    #[inline]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Lower-case slug (`"sma"`, `"ema"`, …) — matches the indicator slugs.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            MaType::Sma => "sma",
            MaType::Ema => "ema",
            MaType::Wma => "wma",
            MaType::Dema => "dema",
            MaType::Tema => "tema",
            MaType::Trima => "trima",
            MaType::Kama => "kama",
            MaType::Mama => "mama",
            MaType::T3 => "t3",
        }
    }

    /// Lenient `u8` decode used by the Polars expression plugins, where the
    /// value has already been validated on the Python side. Unknown codes fall
    /// back to `Sma`, preserving the pre-existing plugin behaviour.
    #[inline]
    pub const fn from_u8_or_sma(code: u8) -> Self {
        match code {
            1 => MaType::Ema,
            2 => MaType::Wma,
            3 => MaType::Dema,
            4 => MaType::Tema,
            5 => MaType::Trima,
            6 => MaType::Kama,
            7 => MaType::Mama,
            8 => MaType::T3,
            _ => MaType::Sma,
        }
    }
}

impl TryFrom<i32> for MaType {
    type Error = InvalidMaType;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MaType::Sma),
            1 => Ok(MaType::Ema),
            2 => Ok(MaType::Wma),
            3 => Ok(MaType::Dema),
            4 => Ok(MaType::Tema),
            5 => Ok(MaType::Trima),
            6 => Ok(MaType::Kama),
            7 => Ok(MaType::Mama),
            8 => Ok(MaType::T3),
            other => Err(InvalidMaType(other.to_string())),
        }
    }
}

impl FromStr for MaType {
    type Err = InvalidMaType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.trim().to_ascii_lowercase();
        MaType::ALL
            .into_iter()
            .find(|m| m.as_str() == lower)
            .ok_or_else(|| InvalidMaType(s.to_string()))
    }
}

impl fmt::Display for MaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Conversion to the parity oracle.
///
/// Deliberately `#[cfg(test)]`-only: `talib-rs` is a test/bench oracle, not a
/// runtime dependency, so this bridge must never be reachable from library
/// code. Integration tests and benches (which compile without `cfg(test)`)
/// should go through the stable integer code instead:
/// `talib_rs::MaType::try_from(m.to_i32()).unwrap()`.
#[cfg(test)]
impl From<MaType> for talib_rs::MaType {
    fn from(m: MaType) -> Self {
        talib_rs::MaType::try_from(m.to_i32()).expect("MaType discriminants match TA-Lib's")
    }
}

#[cfg(test)]
impl From<talib_rs::MaType> for MaType {
    fn from(m: talib_rs::MaType) -> Self {
        MaType::try_from(m as i32).expect("TA-Lib discriminants match MaType's")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_match_talib() {
        for m in MaType::ALL {
            let oracle: talib_rs::MaType = m.into();
            assert_eq!(m.to_i32(), oracle as i32, "{m} discriminant drifted");
            assert_eq!(MaType::from(oracle), m);
        }
    }

    #[test]
    fn round_trips_through_i32_and_str() {
        for m in MaType::ALL {
            assert_eq!(MaType::try_from(m.to_i32()).unwrap(), m);
            assert_eq!(m.as_str().parse::<MaType>().unwrap(), m);
            assert_eq!(MaType::from_u8_or_sma(m.to_i32() as u8), m);
        }
        assert!(MaType::try_from(9).is_err());
        assert!("nope".parse::<MaType>().is_err());
        assert_eq!(MaType::from_u8_or_sma(200), MaType::Sma);
        assert_eq!(MaType::default(), MaType::Sma);
    }
}
