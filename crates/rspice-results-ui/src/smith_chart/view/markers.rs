//! What a Smith marker reads beyond its coefficient: the match it
//! measures, the lumped element it equals at its frequency, and how it
//! differs from the first marker.

use std::f64::consts::{PI, TAU};

use num_complex::Complex64;
use rspice_ui_kit::plot::fmt_column_reading;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use super::{admittance_from_gamma, impedance_from_gamma};
use crate::presentation::{angle_readout, frequency_readout};

/// Return loss −20·log₁₀|Γ| and mismatch loss −10·log₁₀(1 − |Γ|²), in dB.
/// A perfect match has an infinite return loss; a full reflection an
/// infinite mismatch loss.
pub(super) fn match_rows(gamma: Complex64, digits: usize) -> Vec<(String, String)> {
    let magnitude = gamma.norm();
    let decibels = |value: f64| {
        if value.is_finite() {
            fmt_column_reading(value, "dB", digits)
        } else {
            "∞".to_owned()
        }
    };
    vec![
        ("RL".to_owned(), decibels(-20.0 * magnitude.log10())),
        (
            "Mismatch".to_owned(),
            if magnitude < 1.0 {
                decibels(-10.0 * (1.0 - magnitude * magnitude).log10())
            } else {
                "∞".to_owned()
            },
        ),
    ]
}

/// The element a reactance of `reactance` ohms is at `frequency`: an
/// inductance X/ω when positive, a capacitance −1/(ωX) when negative.
fn series_element(reactance: f64, frequency: f64) -> Option<(f64, &'static str)> {
    let omega = TAU * frequency;
    if reactance > 0.0 {
        Some((reactance / omega, "H"))
    } else if reactance < 0.0 {
        Some((-1.0 / (omega * reactance), "F"))
    } else {
        None
    }
}

/// The element a susceptance of `susceptance` siemens is at `frequency`: a
/// capacitance B/ω when positive, an inductance −1/(ωB) when negative.
fn shunt_element(susceptance: f64, frequency: f64) -> Option<(f64, &'static str)> {
    let omega = TAU * frequency;
    if susceptance > 0.0 {
        Some((susceptance / omega, "F"))
    } else if susceptance < 0.0 {
        Some((-1.0 / (omega * susceptance), "H"))
    } else {
        None
    }
}

/// The resistance and reactive element in series, and the resistance and
/// reactive element in parallel, that present Γ on `reference` ohms at
/// `frequency`: `50 Ω + 3.979 nH` and `62.5 Ω ∥ 1.273 pF`. None at zero
/// frequency, where no element has a value.
pub(super) fn element_rows(
    gamma: Complex64,
    reference: f64,
    frequency: f64,
    digits: usize,
) -> Vec<(String, String)> {
    if !(frequency.is_finite() && frequency > 0.0) {
        return Vec::new();
    }
    let with = |resistance: String, joint: &str, element: Option<(f64, &str)>| match element {
        Some((value, unit)) => format!(
            "{resistance} {joint} {}",
            fmt_column_reading(value, unit, digits)
        ),
        None => resistance,
    };
    let mut rows = Vec::new();
    if let Some((resistance, reactance)) = impedance_from_gamma(gamma.re, gamma.im, reference) {
        rows.push((
            "Series".to_owned(),
            with(
                fmt_column_reading(resistance, "Ω", digits),
                "+",
                series_element(reactance, frequency),
            ),
        ));
    }
    if let Some((conductance, susceptance)) = admittance_from_gamma(gamma.re, gamma.im, reference) {
        let resistance = if conductance > 0.0 {
            fmt_column_reading(1.0 / conductance, "Ω", digits)
        } else {
            "∞ Ω".to_owned()
        };
        rows.push((
            "Shunt".to_owned(),
            with(resistance, "∥", shunt_element(susceptance, frequency)),
        ));
    }
    rows
}

/// How a marker differs from m1: in frequency, in |Γ| as a ratio in dB,
/// and in the angle of Γ.
pub(super) fn delta_rows(
    marker: (Complex64, f64),
    first: (Complex64, f64),
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<(String, String)> {
    let ((gamma, frequency), (first_gamma, first_frequency)) = (marker, first);
    let mut turn = gamma.arg() - first_gamma.arg();
    if turn > PI {
        turn -= TAU;
    } else if turn <= -PI {
        turn += TAU;
    }
    let ratio = 20.0 * (gamma.norm() / first_gamma.norm()).log10();
    vec![
        (
            "Δf".to_owned(),
            frequency_readout(quantity_policy, frequency - first_frequency, digits),
        ),
        (
            "Δ|Γ|".to_owned(),
            if ratio.is_finite() {
                fmt_column_reading(ratio, "dB", digits)
            } else {
                "—".to_owned()
            },
        ),
        (
            "Δ∠Γ".to_owned(),
            angle_readout(quantity_policy, turn, digits),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIGAHERTZ: f64 = 1.0e9;

    fn gamma_of(impedance: Complex64, reference: f64) -> Complex64 {
        (impedance - reference) / (impedance + reference)
    }

    fn row<'a>(rows: &'a [(String, String)], key: &str) -> &'a str {
        rows.iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
            .unwrap_or_else(|| panic!("no {key} row in {rows:?}"))
    }

    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-9 * expected.abs(),
            "{actual} != {expected}"
        );
    }

    /// A 25 Ω resistor in series with 5 nH, read at 1 GHz on 50 Ω, reads
    /// back as that inductance; 50 Ω with 2 pF reads back as that
    /// capacitance.
    #[test]
    fn a_marker_on_a_known_series_load_reads_its_element() {
        let omega = TAU * GIGAHERTZ;
        let inductive = gamma_of(Complex64::new(25.0, omega * 5.0e-9), 50.0);
        let (resistance, reactance) =
            impedance_from_gamma(inductive.re, inductive.im, 50.0).unwrap();
        close(resistance, 25.0);
        let (henries, unit) = series_element(reactance, GIGAHERTZ).unwrap();
        assert_eq!(unit, "H");
        close(henries, 5.0e-9);
        let rows = element_rows(inductive, 50.0, GIGAHERTZ, 4);
        assert_eq!(row(&rows, "Series"), "25.00 Ω + 5.000 nH");

        let capacitive = gamma_of(Complex64::new(50.0, -1.0 / (omega * 2.0e-12)), 50.0);
        let (_, reactance) = impedance_from_gamma(capacitive.re, capacitive.im, 50.0).unwrap();
        let (farads, unit) = series_element(reactance, GIGAHERTZ).unwrap();
        assert_eq!(unit, "F");
        close(farads, 2.0e-12);
        assert_eq!(
            row(&element_rows(capacitive, 50.0, GIGAHERTZ, 4), "Series"),
            "50.00 Ω + 2.000 pF"
        );
    }

    /// 100 Ω in parallel with 1 pF reads back as that parallel pair, and
    /// 200 Ω in parallel with 10 nH as that one.
    #[test]
    fn a_marker_on_a_known_parallel_load_reads_its_shunt_element() {
        let omega = TAU * GIGAHERTZ;
        let parallel = |conductance: f64, susceptance: f64| {
            gamma_of(1.0 / Complex64::new(conductance, susceptance), 50.0)
        };
        let capacitive = parallel(1.0 / 100.0, omega * 1.0e-12);
        let (_, susceptance) = admittance_from_gamma(capacitive.re, capacitive.im, 50.0).unwrap();
        let (farads, unit) = shunt_element(susceptance, GIGAHERTZ).unwrap();
        assert_eq!(unit, "F");
        close(farads, 1.0e-12);
        assert_eq!(
            row(&element_rows(capacitive, 50.0, GIGAHERTZ, 4), "Shunt"),
            "100.0 Ω ∥ 1.000 pF"
        );
        let inductive = parallel(1.0 / 200.0, -1.0 / (omega * 10.0e-9));
        let (_, susceptance) = admittance_from_gamma(inductive.re, inductive.im, 50.0).unwrap();
        let (henries, unit) = shunt_element(susceptance, GIGAHERTZ).unwrap();
        assert_eq!(unit, "H");
        close(henries, 10.0e-9);
    }

    /// |Γ| = 1/3 is a 9.54 dB return loss and a 0.51 dB mismatch loss, each
    /// to the hundredth of a decibel; a match has an infinite return loss and
    /// a short an infinite mismatch.
    #[test]
    fn the_match_reads_return_and_mismatch_loss() {
        let rows = match_rows(Complex64::from_polar(1.0 / 3.0, 1.0), 4);
        assert_eq!(row(&rows, "RL"), "9.54 dB");
        assert_eq!(row(&rows, "Mismatch"), "0.51 dB");
        assert_eq!(row(&match_rows(Complex64::new(0.0, 0.0), 4), "RL"), "∞");
        assert_eq!(
            row(&match_rows(Complex64::new(-1.0, 0.0), 4), "Mismatch"),
            "∞"
        );
    }

    /// A second marker states how far it is from m1 in frequency, in |Γ|
    /// and in angle, the angle taking the short way round.
    #[test]
    fn a_second_marker_reads_its_distance_from_the_first() {
        let first = (Complex64::from_polar(0.5, 170f64.to_radians()), GIGAHERTZ);
        let second = (
            Complex64::from_polar(0.25, -170f64.to_radians()),
            1.5 * GIGAHERTZ,
        );
        let rows = delta_rows(second, first, &QuantityPresentationPolicy::default(), 4);
        assert!(row(&rows, "Δ|Γ|").contains("6.02 dB"), "{rows:?}");
        assert!(row(&rows, "Δf").contains("500"), "{rows:?}");
        assert!(row(&rows, "Δ∠Γ").starts_with("20"), "{rows:?}");
    }
}
