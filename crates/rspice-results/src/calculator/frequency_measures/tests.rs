//! Closed-form oracles for bandwidth, and the Bode sheet itself for the
//! margins: the calculator's margin must be the number the sheet prints for
//! the same retained response.

use num_complex::Complex64;

use crate::analysis_type::AnalysisType;
use crate::bode::retained::ac_bode_summary_for_analysis;
use crate::calculator::evaluator::evaluate;
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::{CalcValue, RealValue};
use crate::waveform::RetainedWaveform;

/// `per_decade` log-spaced frequencies from `f0` to `f1`, both exact.
fn decades(f0: f64, f1: f64, per_decade: usize) -> Vec<f64> {
    let (l0, l1) = (f0.log10(), f1.log10());
    let n = ((l1 - l0) * per_decade as f64).round() as usize;
    (0..=n)
        .map(|i| 10f64.powf(l0 + (l1 - l0) * i as f64 / n as f64))
        .collect()
}

/// The traces an AC analysis retains for `V(out)`: its magnitude carrying
/// the complex samples, and its wrapped phase in degrees.
fn ac_response(frequency: &[f64], h: impl Fn(f64) -> Complex64) -> Vec<RetainedWaveform> {
    let samples: Vec<Complex64> = frequency.iter().map(|&f| h(f)).collect();
    vec![
        RetainedWaveform::new(
            "|V(out)|",
            frequency.to_vec(),
            samples.iter().map(|v| v.norm()).collect::<Vec<_>>(),
        )
        .with_complex_components(
            "V(out)",
            samples.iter().map(|v| v.re).collect::<Vec<_>>(),
            samples.iter().map(|v| v.im).collect::<Vec<_>>(),
        ),
        RetainedWaveform::new(
            "phase(V(out))",
            frequency.to_vec(),
            samples
                .iter()
                .map(|v| v.arg().to_degrees())
                .collect::<Vec<_>>(),
        ),
    ]
}

/// Plain decimal literals: these fixtures need no SPICE suffixes.
fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn measure(waveforms: &[RetainedWaveform], expression: &str) -> Result<f64, String> {
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, &WaveformsContext::new(waveforms)).map_err(|e| e.to_string())? {
        CalcValue::Real(RealValue::Scalar(value)) => Ok(value),
        other => Err(format!("{expression} is not a scalar: {other:?}")),
    }
}

#[track_caller]
fn relative(waveforms: &[RetainedWaveform], expression: &str, expected: f64, tolerance: f64) {
    let actual = measure(waveforms, expression)
        .unwrap_or_else(|error| panic!("{expression} failed: {error}"));
    assert!(
        ((actual - expected) / expected).abs() <= tolerance,
        "{expression}: expected {expected}, got {actual} (relative tolerance {tolerance})"
    );
}

/// `20 log10 |h(f)|`.
fn db(h: Complex64) -> f64 {
    20.0 * h.norm().log10()
}

#[test]
fn bandwidth_of_a_low_pass_is_its_corner_below_the_low_frequency_gain() {
    let fc = 1.0e3;
    let h = move |f: f64| Complex64::new(1.0, f / fc).inv();
    let frequency = decades(1.0, 1.0e6, 1000);
    let response = ac_response(&frequency, h);
    // |h|² = 1 / (1 + (f/fc)²) reaches the reference gain less `drop` dB at
    // f = fc √(10^((drop − g_ref)/10) − 1).
    let corner = |drop: f64| {
        let reference = db(h(frequency[0]));
        fc * (10f64.powf((drop - reference) / 10.0) - 1.0).sqrt()
    };
    relative(&response, "bandwidth(V(out))", corner(3.0), 1e-6);
    relative(
        &response,
        "bandwidth(V(out), 3, \"low\")",
        corner(3.0),
        1e-6,
    );
    relative(&response, "bandwidth(V(out), 6)", corner(6.0), 1e-6);
    // The Bode sheet's f₋₃dB is the same number.
    let summary = ac_bode_summary_for_analysis(AnalysisType::Ac, &response, 0, |_| false)
        .expect("a Bode summary");
    assert_eq!(
        measure(&response, "bandwidth(V(out))").ok(),
        summary.metrics.f3db
    );
}

#[test]
fn bandwidth_of_a_high_pass_is_its_corner_below_the_high_frequency_gain() {
    let fc = 1.0e3;
    let h = move |f: f64| {
        let s = Complex64::new(0.0, f / fc);
        s / (Complex64::new(1.0, 0.0) + s)
    };
    let frequency = decades(1.0, 1.0e6, 1000);
    let response = ac_response(&frequency, h);
    // |h|² = u / (1 + u) with u = (f/fc)², so a power ratio r is reached
    // at u = r / (1 − r).
    let reference = db(h(*frequency.last().unwrap()));
    let r = 10f64.powf((reference - 3.0) / 10.0);
    relative(
        &response,
        "bandwidth(V(out), 3, \"high\")",
        fc * (r / (1.0 - r)).sqrt(),
        1e-6,
    );
}

#[test]
fn bandwidth_of_a_band_pass_is_the_width_between_its_half_power_edges() {
    let (f0, q) = (1.0e3, 5.0);
    let h = move |f: f64| {
        let u = f / f0;
        let j = Complex64::new(0.0, u / q);
        j / (Complex64::new(1.0 - u * u, 0.0) + j)
    };
    // The grid holds f0 itself, where the peak is exactly 0 dB.
    let frequency = decades(10.0, 1.0e5, 2000);
    let response = ac_response(&frequency, h);
    // |h|² = 1 / (1 + Q²(u − 1/u)²) = 10^(−drop/10) at u − 1/u = ±k/Q with
    // k = √(10^(drop/10) − 1), so the two edges are f0 k / Q apart.
    let width = |drop: f64| f0 * (10f64.powf(drop / 10.0) - 1.0).sqrt() / q;
    relative(
        &response,
        "bandwidth(V(out), 3, \"band\")",
        width(3.0),
        1e-6,
    );
    relative(
        &response,
        "bandwidth(V(out), 10, \"band\")",
        width(10.0),
        1e-6,
    );
    assert!(
        measure(&response, "bandwidth(V(out), 3, \"rising\")")
            .unwrap_err()
            .contains("shape is one of")
    );
}

/// `L(f) = K / (1 + j f/f1)³`.
fn three_pole(k: f64, f1: f64) -> impl Fn(f64) -> Complex64 {
    move |f: f64| Complex64::new(k, 0.0) / Complex64::new(1.0, f / f1).powi(3)
}

#[test]
fn margins_match_the_closed_form_and_the_bode_sheet() {
    let (k, f1) = (4.0, 1.0e3);
    let frequency = decades(1.0, 1.0e6, 2000);
    let response = ac_response(&frequency, three_pole(k, f1));
    // The phase reaches −180° at f = √3 f1, where |L| = 4 / 8: GM = 20 log10 2.
    // |L| = 1 where (1 + x²)^(3/2) = 4, and PM = 180° − 3 atan x there.
    let x = (4f64.powf(2.0 / 3.0) - 1.0).sqrt();
    let gain_margin = 20.0 * 2f64.log10();
    let phase_margin = 180.0 - 3.0 * x.atan().to_degrees();
    let gm = measure(&response, "gainMargin(V(out))").unwrap();
    let pm = measure(&response, "phaseMargin(V(out))").unwrap();
    assert!((gm - gain_margin).abs() < 1e-5, "GM {gm} vs {gain_margin}");
    assert!(
        (pm - phase_margin).abs() < 1e-5,
        "PM {pm} vs {phase_margin}"
    );
    let summary = ac_bode_summary_for_analysis(AnalysisType::Ac, &response, 0, |_| false)
        .expect("a Bode summary");
    assert_eq!(Some(gm), summary.metrics.gm_db);
    assert_eq!(Some(pm), summary.metrics.pm_deg);
}

#[test]
fn margins_refuse_a_response_without_phase_or_without_a_crossing() {
    let frequency = decades(1.0, 1.0e6, 50);
    let response = ac_response(&frequency, three_pole(4.0, 1.0e3));
    assert!(
        measure(&response, "gainMargin(mag(V(out)))")
            .unwrap_err()
            .contains("a real response carries no phase")
    );
    // A single pole never reaches −180°, and a loop below unity gain never
    // crosses 0 dB.
    let single = ac_response(&frequency, |f| {
        Complex64::new(10.0, 0.0) / Complex64::new(1.0, f)
    });
    assert!(
        measure(&single, "gainMargin(V(out))")
            .unwrap_err()
            .contains("phase reaches -180°")
    );
    let quiet = ac_response(&frequency, three_pole(0.5, 1.0e3));
    assert!(
        measure(&quiet, "phaseMargin(V(out))")
            .unwrap_err()
            .contains("crosses 0 dB")
    );
}
