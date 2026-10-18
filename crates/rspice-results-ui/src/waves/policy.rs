//! Validated waveform readout preferences and their resolved sampling policy.

use rspice_ui_kit::plot::SampleInterpolation;
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

pub const fn cursor_interpolation(policy: CursorInterpolation) -> SampleInterpolation {
    match policy {
        CursorInterpolation::MonotoneCubicWhereValid => SampleInterpolation::MonotoneCubic,
        CursorInterpolation::Linear => SampleInterpolation::Linear,
        CursorInterpolation::NearestAcceptedPoint => SampleInterpolation::Nearest,
    }
}

/// Only the presentation settings needed to sample and format cursor values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReadoutPolicy {
    pub displayed_significant_digits: DisplayedSignificantDigits,
    pub cursor_interpolation: CursorInterpolation,
}

impl ReadoutPolicy {
    pub const fn displayed_significant_digits(self) -> DisplayedSignificantDigits {
        self.displayed_significant_digits
    }

    pub const fn cursor_interpolation(self) -> CursorInterpolation {
        self.cursor_interpolation
    }

    /// What a cursor at `xq` reads off a series whose `x` is monotone, under
    /// Results ▸ Cursor interpolation: the rule the waveform panes read by.
    #[must_use]
    pub fn cursor_sample(self, x: &[f64], y: &[f64], xq: f64) -> f64 {
        rspice_ui_kit::plot::sample::sample_at_with(
            x,
            y,
            xq,
            cursor_interpolation(self.cursor_interpolation),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A spectrum cursor between two bins reads by the chosen rule, not
    /// always by a straight line.
    #[test]
    fn a_cursor_sample_follows_the_chosen_interpolation() {
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = [0.0, 10.0, 40.0, 90.0];
        let read = |cursor_interpolation| {
            ReadoutPolicy {
                cursor_interpolation,
                ..ReadoutPolicy::default()
            }
            .cursor_sample(&x, &y, 2.4)
        };
        assert!((read(CursorInterpolation::Linear) - 22.0).abs() < 1e-12);
        assert!((read(CursorInterpolation::NearestAcceptedPoint) - 10.0).abs() < 1e-12);
        let cubic = read(CursorInterpolation::MonotoneCubicWhereValid);
        assert!(cubic > 10.0 && cubic < 40.0 && (cubic - 22.0).abs() > 1e-6);
    }
}
