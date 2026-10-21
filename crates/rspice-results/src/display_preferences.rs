//! The reader's display choices for retained results.
//!
//! None of these changes a stored sample or an engineering export; they
//! decide only how values are labelled, read under a cursor and split into
//! traces. Their serde spellings are stored in sessions and preference
//! files, so they are part of those wire formats.

use serde::{Deserialize, Serialize};

/// Validated number of significant digits used by result presentation.
///
/// This never changes a stored waveform sample or an engineering export. It
/// controls only human-facing labels and readouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct DisplayedSignificantDigits(u8);

impl DisplayedSignificantDigits {
    pub const MIN: u8 = 3;
    pub const MAX: u8 = 17;
    pub const DEFAULT: u8 = 7;

    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl Default for DisplayedSignificantDigits {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<u8> for DisplayedSignificantDigits {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err("displayed significant digits must be between 3 and 17")
        }
    }
}

impl<'de> Deserialize<'de> for DisplayedSignificantDigits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CursorInterpolation {
    #[default]
    MonotoneCubicWhereValid,
    Linear,
    NearestAcceptedPoint,
}

impl CursorInterpolation {
    pub const fn index(self) -> usize {
        match self {
            Self::MonotoneCubicWhereValid => 0,
            Self::Linear => 1,
            Self::NearestAcceptedPoint => 2,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::MonotoneCubicWhereValid),
            1 => Ok(Self::Linear),
            2 => Ok(Self::NearestAcceptedPoint),
            _ => Err("cursor interpolation index is outside its domain"),
        }
    }
}

/// How a complex quantity is split into traces.
///
/// Magnitude and phase draws the phase in degrees; the pane shows it in
/// whichever angle unit the reader chose, so a separate radians mode would
/// only say the same thing twice. Sessions that stored that retired mode
/// read back as magnitude and phase.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplexNumberDisplay {
    #[default]
    #[serde(alias = "magnitude-phase-radians")]
    MagnitudePhaseDegrees,
    RealImaginary,
}

impl ComplexNumberDisplay {
    pub const fn index(self) -> usize {
        match self {
            Self::MagnitudePhaseDegrees => 0,
            Self::RealImaginary => 1,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            // Index 2 was magnitude and phase in radians.
            0 | 2 => Ok(Self::MagnitudePhaseDegrees),
            1 => Ok(Self::RealImaginary),
            _ => Err("complex-number display index is outside its domain"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGITS: [u8; 3] = [DisplayedSignificantDigits::DEFAULT, 3, 17];
    const INTERPOLATIONS: [CursorInterpolation; 3] = [
        CursorInterpolation::MonotoneCubicWhereValid,
        CursorInterpolation::Linear,
        CursorInterpolation::NearestAcceptedPoint,
    ];
    const COMPLEX: [ComplexNumberDisplay; 2] = [
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        ComplexNumberDisplay::RealImaginary,
    ];

    #[derive(Serialize)]
    struct Stored {
        displayed_significant_digits: DisplayedSignificantDigits,
        cursor_interpolation: CursorInterpolation,
        complex_number_display: ComplexNumberDisplay,
    }

    fn digits() -> [DisplayedSignificantDigits; 3] {
        DIGITS.map(|digits| DisplayedSignificantDigits::try_from(digits).unwrap())
    }

    fn stored() -> Stored {
        Stored {
            displayed_significant_digits: digits()[2],
            cursor_interpolation: INTERPOLATIONS[2],
            complex_number_display: COMPLEX[1],
        }
    }

    /// Preference files store JSON and sessions store RON. These spellings
    /// are the ones those files already hold, so they may never drift.
    #[test]
    fn display_choices_keep_their_stored_json_and_ron_spellings() {
        let values = (digits(), INTERPOLATIONS, COMPLEX);
        assert_eq!(
            serde_json::to_string(&values).unwrap(),
            r#"[[7,3,17],["monotone-cubic-where-valid","linear","nearest-accepted-point"],["magnitude-phase-degrees","real-imaginary"]]"#
        );
        assert_eq!(
            ron::to_string(&values).unwrap(),
            "((7,3,17),(r#monotone-cubic-where-valid,linear,r#nearest-accepted-point),\
             (r#magnitude-phase-degrees,r#real-imaginary))"
        );
        assert_eq!(
            serde_json::to_string(&stored()).unwrap(),
            r#"{"displayed_significant_digits":17,"cursor_interpolation":"nearest-accepted-point","complex_number_display":"real-imaginary"}"#
        );
        assert_eq!(
            ron::to_string(&stored()).unwrap(),
            "(displayed_significant_digits:17,cursor_interpolation:r#nearest-accepted-point,\
             complex_number_display:r#real-imaginary)"
        );
    }

    #[test]
    fn stored_display_choices_read_back_and_retired_or_invalid_values_resolve() {
        let values = (digits(), INTERPOLATIONS, COMPLEX);
        let json = serde_json::to_string(&values).unwrap();
        assert_eq!(serde_json::from_str::<(_, _, _)>(&json).unwrap(), values);
        let ron = ron::to_string(&values).unwrap();
        assert_eq!(ron::from_str::<(_, _, _)>(&ron).unwrap(), values);
        assert_eq!(
            serde_json::from_str::<ComplexNumberDisplay>(r#""magnitude-phase-radians""#).unwrap(),
            ComplexNumberDisplay::MagnitudePhaseDegrees
        );
        assert!(serde_json::from_str::<DisplayedSignificantDigits>("2").is_err());
        assert!(serde_json::from_str::<DisplayedSignificantDigits>("18").is_err());
    }
}
