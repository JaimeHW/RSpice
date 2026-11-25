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
use rspice_ui_kit::plot::{self, fmt_column_reading};
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
                fmt_column_reading(margin.value, "dB", significant_digits)
            }),
            true,
        ),
        ("Phase crossover", frequency(gain_decibels), false),
    ]
}

/// What a loop's margins mark on the strips of a Bode sheet: each margin a
/// span from the level it is read against to the curve, tagged in the words
/// the Nyquist sheet tags them on its locus.
#[derive(Debug, Clone, Default)]
pub struct MarginMarks {
    /// The tagged spans, on the pane's first axis, each standing on its
    /// reference (`y`) and reaching the curve (the span's `to`).
    pub markers: Vec<plot::Marker>,
    /// The levels the marks stand on, drawn as reference lines.
    pub references: Vec<f64>,
}

/// The gain margin on the strip drawing the loop's magnitude: a span at the
/// phase crossover from the 0 dB reference to the curve, which stands the
/// margin below it there, tagged beside it. The strip draws its own 0 dB
/// reference.
pub fn gain_marks(
    metrics: &AcBodeMetrics,
    color: egui::Color32,
    significant_digits: usize,
) -> MarginMarks {
    let (Some(hertz), Some(decibels)) = (metrics.f180, metrics.gm_db) else {
        return MarginMarks::default();
    };
    MarginMarks {
        markers: vec![plot::Marker::span(
            hertz,
            0.0,
            -decibels,
            color,
            format!(
                "GM {}",
                fmt_column_reading(decibels, "dB", significant_digits)
            ),
        )],
        references: Vec::new(),
    }
}

/// The phase margin on the strip drawing the loop's phase: a span at the
/// gain crossover from the inversion it is read from, −180° or the odd
/// multiple of it the continuous phase stands nearest there, to the curve,
/// tagged beside it. The strip draws the inversion as a reference line.
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
        markers: vec![plot::Marker::span(
            hertz,
            inversion,
            phase,
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
                "Phase margin: 27.14\u{b0}",
                "Gain crossover: 1.233 kHz",
                "Gain margin: 6.02 dB",
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
                "Gain margin: \u{2212}3.50 dB",
                "Phase crossover: 2.00 MHz",
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
            (1_732.05, 0.0, "GM 6.02 dB")
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

    /// Each margin spans from its reference to the curve, where the closed
    /// form of the loop puts them. For L(f) = 4 / (1 + jf/1 kHz)³ the phase
    /// reaches −180° at f = √3 kHz, where |L| = 4/8, so the gain span runs
    /// from 0 dB down to −20·log10(2) dB; the gain is unity where
    /// (1 + x²)^(3/2) = 4, and the phase span runs from −180° up to
    /// −3·atan(x) there.
    #[test]
    fn each_margin_spans_from_its_reference_to_the_curve_where_the_closed_form_puts_it() {
        let policy = QuantityPresentationPolicy::default();
        let frequency: Vec<f64> = (0..=250)
            .map(|step| 10.0_f64 * 10.0_f64.powf(f64::from(step) / 50.0))
            .collect();
        let ratio = |hertz: f64| hertz / 1.0e3;
        let gain_db: Vec<f64> = frequency
            .iter()
            .map(|&hertz| 20.0 * 4.0_f64.log10() - 30.0 * (1.0 + ratio(hertz).powi(2)).log10())
            .collect();
        let phase_deg: Vec<f64> = frequency
            .iter()
            .map(|&hertz| -3.0 * ratio(hertz).atan().to_degrees())
            .collect();
        let metrics =
            rspice_results::bode::metrics_from_curves(&frequency, &gain_db, Some(&phase_deg));

        let span_of = |marks: &MarginMarks| {
            let [mark] = &marks.markers[..] else {
                panic!("one margin span");
            };
            let plot::MarkerShape::Span { to } = mark.shape else {
                panic!("a margin is drawn as a span");
            };
            (mark.x, mark.y, to)
        };
        let close = |actual: f64, expected: f64, tolerance: f64| {
            assert!(
                (actual - expected).abs() <= tolerance,
                "{actual} is not within {tolerance} of {expected}"
            );
        };

        let (hertz, reference, curve) = span_of(&gain_marks(&metrics, egui::Color32::RED, 4));
        close(hertz, 3.0_f64.sqrt() * 1.0e3, 0.5);
        assert_eq!(reference, 0.0);
        close(curve, -20.0 * 2.0_f64.log10(), 1.0e-3);

        let unity = (4.0_f64.powf(2.0 / 3.0) - 1.0).sqrt();
        let marks = phase_marks(&metrics, egui::Color32::RED, &policy, 4);
        let (hertz, reference, curve) = span_of(&marks);
        close(hertz, unity * 1.0e3, 0.5);
        close(reference, -180.0, 1.0e-9);
        close(curve, -3.0 * unity.atan().to_degrees(), 1.0e-3);
        assert_eq!(marks.references, [reference]);
    }
}
