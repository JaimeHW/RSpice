//! The resolved sampling policy of the waveform readout preferences.

use rspice_results::display_preferences::{CursorInterpolation, DisplayedSignificantDigits};
use rspice_ui_kit::plot::SampleInterpolation;

/// How a cursor reads between samples under Results ▸ Cursor interpolation.
///
/// A reading is a point of the drawn line, and the line runs straight from
/// sample to sample, so the smooth choice reads it as the straight choice
/// does: a monotone cubic between two samples would read a value no curve on
/// screen passes through. It reads smooth again once the line is drawn
/// smooth. The nearest choice reads a sample, which the line passes through.
pub const fn cursor_interpolation(policy: CursorInterpolation) -> SampleInterpolation {
    match policy {
        CursorInterpolation::MonotoneCubicWhereValid | CursorInterpolation::Linear => {
            SampleInterpolation::Linear
        }
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

    /// A spectrum cursor between two bins reads the drawn line, or the
    /// nearest bin when that is the choice.
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
        assert!((read(CursorInterpolation::MonotoneCubicWhereValid) - 22.0).abs() < 1e-12);
    }
}
