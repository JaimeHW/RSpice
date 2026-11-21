//! A loop's margins, printed the one way both stability sheets print them.
//!
//! The Bode card and the Nyquist panel read the same two numbers off the
//! same interpolant of the same loop gain, so each margin reads on its own
//! row followed by the frequency it was found at, at the reader's
//! significant digits, in the same words on both: the phase margin and the
//! gain crossover, where the loop's gain is unity; the gain margin and the
//! phase crossover, where its phase is −180°. One figure a row keeps every
//! row on one line of the inspector at any digits.

use rspice_results::bode::AcBodeMetrics;
use rspice_ui_kit::plot::{self, fmt_si_significant};
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

/// What a loop's margins mark on the strips of a Bode sheet, tagged the way
/// the Nyquist sheet tags them on its locus.
#[derive(Debug, Clone, Default)]
pub struct MarginMarks {
    /// The tagged points, on the pane's first axis.
    pub markers: Vec<plot::Marker>,
    /// The levels the marks stand on, drawn as reference lines.
    pub references: Vec<f64>,
}

/// The gain margin on the strip drawing the loop's magnitude: tagged on the
/// 0 dB reference at the phase crossover, where the curve stands the margin
/// below it. The strip draws its own 0 dB reference.
pub fn gain_marks(
    metrics: &AcBodeMetrics,
    color: egui::Color32,
    significant_digits: usize,
) -> MarginMarks {
    let (Some(hertz), Some(decibels)) = (metrics.f180, metrics.gm_db) else {
        return MarginMarks::default();
    };
    MarginMarks {
        markers: vec![plot::Marker::point(
            hertz,
            0.0,
            color,
            format!(
                "GM {}",
                fmt_si_significant(decibels, "dB", significant_digits)
            ),
        )],
        references: Vec::new(),
    }
}

/// The phase margin on the strip drawing the loop's phase: tagged at the
/// gain crossover on the inversion it is read from, −180° or the odd
/// multiple of it the continuous phase stands nearest there, which the
/// strip draws as a reference line.
pub fn phase_marks(
    metrics: &AcBodeMetrics,
    color: egui::Color32,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> MarginMarks {
    let (Some(hertz), Some(margin), Some(phase)) =
        (metrics.ugf, metrics.pm_deg, metrics.pm_phase_deg)
    else {
        return MarginMarks::default();
    };
    let inversion = phase - margin;
    MarginMarks {
        markers: vec![plot::Marker::point(
            hertz,
            inversion,
            color,
            format!(
                "PM {}",
                angle_readout(quantity_policy, margin.to_radians(), significant_digits)
            ),
        )],
        references: vec![inversion],
    }
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

    /// Each margin is tagged where it is read on the strips: the gain margin
    /// on the 0 dB reference at the phase crossover, the phase margin at the
    /// gain crossover on the inversion the continuous phase stands nearest
    /// there, an odd multiple of −180° on a wound loop.
    #[test]
    fn each_margin_is_tagged_on_the_reference_it_is_read_against() {
        let policy = QuantityPresentationPolicy::default();
        let color = egui::Color32::RED;
        let metrics = AcBodeMetrics {
            ugf: Some(1_232.8),
            pm_deg: Some(27.144_6),
            pm_phase_deg: Some(-152.855_4),
            f180: Some(1_732.05),
            gm_db: Some(6.020_6),
            ..Default::default()
        };

        let gain = gain_marks(&metrics, color, 4);
        assert!(gain.references.is_empty());
        let [mark] = &gain.markers[..] else {
            panic!("one gain margin mark");
        };
        assert_eq!(
            (mark.x, mark.y, mark.label.as_str()),
            (1_732.05, 0.0, "GM 6.021 dB")
        );

        let phase = phase_marks(&metrics, color, &policy, 4);
        let [mark] = &phase.markers[..] else {
            panic!("one phase margin mark");
        };
        assert_eq!(mark.x, 1_232.8);
        assert!((mark.y + 180.0).abs() < 1e-9);
        assert_eq!(
            mark.label,
            format!(
                "PM {}",
                angle_readout(&policy, 27.144_6_f64.to_radians(), 4)
            )
        );
        assert_eq!(phase.references, [mark.y]);

        let wound = AcBodeMetrics {
            pm_phase_deg: Some(-512.855_4),
            ..metrics
        };
        let wound = phase_marks(&wound, color, &policy, 4);
        assert!((wound.markers[0].y + 540.0).abs() < 1e-9);
        assert_eq!(wound.references, [wound.markers[0].y]);

        let unmeasured = AcBodeMetrics::default();
        assert!(gain_marks(&unmeasured, color, 4).markers.is_empty());
        let none = phase_marks(&unmeasured, color, &policy, 4);
        assert!(none.markers.is_empty() && none.references.is_empty());
    }
}
