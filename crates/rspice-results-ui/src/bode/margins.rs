//! A loop's margins, printed the one way both stability sheets print them.
//!
//! The Bode card and the Nyquist panel read the same two numbers off the
//! same interpolant of the same loop gain, so each margin reads with the
//! frequency it was found at, at the reader's significant digits, in the same
//! words on both: "27.14 ° at 1.233 kHz", "6.021 dB at 1.732 kHz".

use rspice_ui_kit::plot::fmt_si_significant;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use crate::presentation::{angle_readout, frequency_readout};

/// A phase margin and the unity-gain frequency it is read at.
pub fn phase_margin(
    degrees: f64,
    frequency: f64,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> String {
    format!(
        "{} at {}",
        angle_readout(quantity_policy, degrees.to_radians(), significant_digits),
        frequency_readout(quantity_policy, frequency, significant_digits)
    )
}

/// A gain margin in decibels and the phase inversion's frequency it is read
/// at.
pub fn gain_margin(
    decibels: f64,
    frequency: f64,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> String {
    format!(
        "{} at {}",
        fmt_si_significant(decibels, "dB", significant_digits),
        frequency_readout(quantity_policy, frequency, significant_digits)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each margin carries the frequency it was found at, at the digits the
    /// reader chose.
    #[test]
    fn a_margin_reads_with_the_frequency_it_was_found_at() {
        let policy = QuantityPresentationPolicy::default();

        assert_eq!(
            phase_margin(27.144_6, 1_232.8, &policy, 4),
            "27.14 ° at 1.233 kHz"
        );
        assert_eq!(
            gain_margin(6.020_6, 1_732.05, &policy, 4),
            "6.021 dB at 1.732 kHz"
        );
        assert_eq!(
            gain_margin(-3.5, 2.0e6, &policy, 3),
            "\u{2212}3.5 dB at 2 MHz"
        );
    }
}
