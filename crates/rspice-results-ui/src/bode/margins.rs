//! A loop's margins, printed the one way both stability sheets print them.
//!
//! The Bode card and the Nyquist panel read the same two numbers off the
//! same interpolant of the same loop gain, so each margin reads on its own
//! row followed by the frequency it was found at, at the reader's
//! significant digits, in the same words on both: the phase margin and the
//! gain crossover, where the loop's gain is unity; the gain margin and the
//! phase crossover, where its phase is −180°. One figure a row keeps every
//! row on one line of the inspector at any digits.

use rspice_ui_kit::plot::fmt_si_significant;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use crate::presentation::{angle_readout, frequency_readout};

/// One loop margin and the frequency it is read at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Margin {
    pub value: f64,
    pub hertz: f64,
}

/// The rows of a loop's margins: the phase margin in degrees and its gain
/// crossover, then the gain margin in decibels and its phase crossover. A
/// margin the sweep does not reach reads as a dash, and so does its
/// frequency.
pub fn rows(
    phase_degrees: Option<Margin>,
    gain_decibels: Option<Margin>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> [(&'static str, String, bool); 4] {
    let dash = || "\u{2014}".to_owned();
    let frequency = |margin: Option<Margin>| {
        margin.map_or_else(dash, |margin| {
            frequency_readout(quantity_policy, margin.hertz, significant_digits)
        })
    };
    [
        (
            "Phase margin",
            phase_degrees.map_or_else(dash, |margin| {
                angle_readout(
                    quantity_policy,
                    margin.value.to_radians(),
                    significant_digits,
                )
            }),
            true,
        ),
        ("Gain crossover", frequency(phase_degrees), false),
        (
            "Gain margin",
            gain_decibels.map_or_else(dash, |margin| {
                fmt_si_significant(margin.value, "dB", significant_digits)
            }),
            true,
        ),
        ("Phase crossover", frequency(gain_decibels), false),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each margin is followed by the frequency it was found at, at the
    /// digits the reader chose; a margin the sweep does not reach reads as a
    /// dash beside a dash.
    #[test]
    fn each_margin_is_followed_by_the_frequency_it_was_found_at() {
        let policy = QuantityPresentationPolicy::default();
        let phase = Margin {
            value: 27.144_6,
            hertz: 1_232.8,
        };
        let gain = Margin {
            value: 6.020_6,
            hertz: 1_732.05,
        };
        let values = |phase, gain, digits| {
            rows(phase, gain, &policy, digits).map(|(label, value, _)| format!("{label}: {value}"))
        };
        assert_eq!(
            values(Some(phase), Some(gain), 4),
            [
                "Phase margin: 27.14 °",
                "Gain crossover: 1.233 kHz",
                "Gain margin: 6.021 dB",
                "Phase crossover: 1.732 kHz",
            ]
        );
        let negative = Margin {
            value: -3.5,
            hertz: 2.0e6,
        };
        assert_eq!(
            values(None, Some(negative), 3),
            [
                "Phase margin: \u{2014}",
                "Gain crossover: \u{2014}",
                "Gain margin: \u{2212}3.5 dB",
                "Phase crossover: 2 MHz",
            ]
        );
    }
}
