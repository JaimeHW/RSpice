//! CLI `.DISTO` must expose the core Volterra products, not an ordinary AC
//! substitute. These end-to-end oracles cover harmonic and fixed-F2 two-tone
//! semantics, physical product frequencies, peak phasors, normalization, and
//! the dedicated HDF5 round trip.

mod common;

use common::{fixture, read_json, test_dir};

use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

const BIAS: f64 = 0.5;
const IS: f64 = 1.0e-12;
const A1: f64 = 1.0e-3;
const A2: f64 = 2.0e-3;
const TEMP_K: f64 = 300.15;
const BOLTZMANN: f64 = 1.380_649e-23;
const ELECTRON_CHARGE: f64 = 1.602_176_634e-19;

fn run(deck: &Path, output: Option<(&Path, &str)>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args(["--quiet", "run", deck.to_str().expect("UTF-8 deck path")]);
    if let Some((path, format)) = output {
        command.args([
            "--output",
            path.to_str().expect("UTF-8 output path"),
            "--format",
            format,
        ]);
    }
    command.output().expect("run rspice DISTO deck")
}

/// The complex samples of one distortion series.
///
/// The typed document keys a distortion series by the signal it measures and
/// the sub-result it belongs to: the fundamental at a tone, or one Volterra
/// product. It publishes the peak phasor itself, so magnitude, phase, and the
/// ratio to the fundamental are computed here from the published number rather
/// than being read back from a column the writer pre-computed.
fn series(document: &Value, signal: &str, qualifier: &Value) -> Vec<(f64, f64)> {
    let found = document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|candidate| {
            candidate["descriptor"]["canonicalName"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case(signal))
                && &candidate["qualifier"] == qualifier
        })
        .unwrap_or_else(|| {
            panic!("missing series '{signal}' qualified {qualifier} in {document:#}")
        });
    found["values"]["samples"]
        .as_array()
        .expect("complex samples")
        .iter()
        .map(|sample| {
            (
                sample["real"].as_f64().expect("real part"),
                sample["imaginary"].as_f64().expect("imaginary part"),
            )
        })
        .collect()
}

fn fundamental(document: &Value, signal: &str, tone: &str) -> Vec<(f64, f64)> {
    series(
        document,
        signal,
        &serde_json::json!({"kind": "distortion-fundamental", "tone": tone}),
    )
}

fn product(document: &Value, signal: &str, product: &str) -> Vec<(f64, f64)> {
    series(
        document,
        signal,
        &serde_json::json!({"kind": "distortion-product", "product": product}),
    )
}

/// Frequencies the document declares for one named Volterra product.
fn product_frequencies(document: &Value, product: &str) -> Vec<f64> {
    document["payload"]["products"]
        .as_array()
        .expect("distortion products")
        .iter()
        .find(|entry| entry["product"].as_str() == Some(product))
        .unwrap_or_else(|| panic!("missing product '{product}' in {document:#}"))["frequencies"]
        .as_array()
        .expect("product frequencies")
        .iter()
        .map(|value| value.as_f64().expect("finite frequency"))
        .collect()
}

fn magnitudes(samples: &[(f64, f64)]) -> Vec<f64> {
    samples.iter().map(|(re, im)| re.hypot(*im)).collect()
}

fn phases_deg(samples: &[(f64, f64)]) -> Vec<f64> {
    samples
        .iter()
        .map(|(re, im)| im.atan2(*re).to_degrees())
        .collect()
}

fn axis_values(document: &Value, name: &str) -> Vec<f64> {
    document["axes"]
        .as_array()
        .expect("document axes")
        .iter()
        .find(|axis| axis["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("missing axis '{name}' in {document:#}"))["values"]["values"]
        .as_array()
        .expect("axis coordinates")
        .iter()
        .map(|value| value.as_f64().expect("finite coordinate"))
        .collect()
}

fn assert_close(actual: f64, expected: f64, relative_tolerance: f64, label: &str) {
    let relative_error = (actual - expected).abs() / expected.abs().max(1.0e-300);
    assert!(
        relative_error <= relative_tolerance,
        "{label}: actual={actual:.12e}, expected={expected:.12e}, relative error={relative_error:.3e}"
    );
}

fn thermal_voltage() -> f64 {
    BOLTZMANN * TEMP_K / ELECTRON_CHARGE
}

fn diode_bias_current() -> f64 {
    IS * (BIAS / thermal_voltage()).exp()
}

#[test]
fn one_tone_exports_nonzero_physical_harmonics_and_normalization() {
    let dir = test_dir("one_tone");
    let output_path = dir.join("one_tone.json");
    let output = run(&fixture("disto_one_tone.cir"), Some((&output_path, "json")));
    assert!(
        output.status.success(),
        "one-tone DISTO failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let document = read_json(&output_path);
    assert_eq!(document["resultKind"], "distortion");
    assert_eq!(document["analysis"]["tag"], "disto-001");
    assert_eq!(axis_values(&document, "frequency"), [1.0e3, 1.5e3, 2.0e3]);
    assert_eq!(
        product_frequencies(&document, "second-harmonic"),
        [2.0e3, 3.0e3, 4.0e3]
    );
    assert_eq!(
        product_frequencies(&document, "third-harmonic"),
        [3.0e3, 4.5e3, 6.0e3]
    );

    let vt = thermal_voltage();
    let expected_hd2 = diode_bias_current() * A1.powi(2) / (4.0 * vt.powi(2));
    let expected_hd3 = diode_bias_current() * A1.powi(3) / (24.0 * vt.powi(3));
    let hd2_samples = product(&document, "i(v1)", "second-harmonic");
    let hd3_samples = product(&document, "i(v1)", "third-harmonic");
    for actual in magnitudes(&hd2_samples) {
        assert_close(actual, expected_hd2, 2.0e-5, "second harmonic current");
    }
    for actual in magnitudes(&hd3_samples) {
        assert_close(actual, expected_hd3, 2.0e-3, "third harmonic current");
    }

    assert!(
        hd2_samples.iter().all(|(re, _)| *re < 0.0),
        "the voltage-source branch convention makes the diode product current negative real"
    );
    assert!(
        hd2_samples
            .iter()
            .all(|(_, im)| im.abs() <= expected_hd2 * 1.0e-10)
    );
    assert!(
        phases_deg(&hd2_samples)
            .iter()
            .all(|value| (value.abs() - 180.0).abs() <= 1.0e-8)
    );

    // The published fundamental is what a magnitude ratio normalizes against,
    // and it is non-zero at every point, so every ratio is finite.
    let f1 = magnitudes(&fundamental(&document, "i(v1)", "f1"));
    assert!(f1.iter().all(|value| *value > 0.0));
    let ratios = magnitudes(&hd2_samples)
        .iter()
        .zip(&f1)
        .map(|(product, fundamental)| product / fundamental)
        .collect::<Vec<_>>();
    assert!(ratios.iter().all(|ratio| ratio.is_finite() && *ratio > 0.0));
}

#[test]
fn two_tone_accepts_ratio_below_one_and_exports_fixed_f2_products() {
    let dir = test_dir("two_tone");
    let output_path = dir.join("two_tone.json");
    let output = run(&fixture("disto_two_tone.cir"), Some((&output_path, "json")));
    assert!(
        output.status.success(),
        "valid f2/f1=0.9 was rejected:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let document = read_json(&output_path);
    assert_eq!(document["payload"]["f2OverF1"], 0.9);
    assert_eq!(
        product_frequencies(&document, "sum"),
        [1_900.0, 2_400.0, 2_900.0]
    );
    assert_eq!(
        product_frequencies(&document, "difference"),
        [100.0, 600.0, 1_100.0]
    );
    assert_eq!(
        product_frequencies(&document, "third-order-difference"),
        [1_100.0, 2_100.0, 3_100.0]
    );
    // f2 = 0.9 * f1 is published as its own fundamental tone, not as a column
    // of a repeated constant.
    assert_eq!(
        fundamental(&document, "i(v1)", "f2").len(),
        axis_values(&document, "frequency").len()
    );

    let vt = thermal_voltage();
    let expected_im2 = diode_bias_current() * A1 * A2 / (2.0 * vt.powi(2));
    let expected_im3 = diode_bias_current() * A1.powi(2) * A2 / (8.0 * vt.powi(3));
    for name in ["sum", "difference"] {
        for actual in magnitudes(&product(&document, "i(v1)", name)) {
            assert_close(actual, expected_im2, 2.0e-5, name);
        }
    }
    for actual in magnitudes(&product(&document, "i(v1)", "third-order-difference")) {
        assert_close(actual, expected_im3, 2.0e-3, "third-order difference");
    }
    for name in ["sum", "third-order-difference"] {
        assert!(
            phases_deg(&product(&document, "i(v1)", name))
                .iter()
                .all(|phase| (phase.abs() - 180.0).abs() <= 1.0e-8),
            "{name} branch-current phase must match the real negative diode oracle"
        );
    }
}

#[test]
fn ratio_at_or_above_one_fails_with_the_core_contract() {
    let output = run(&fixture("disto_invalid_ratio.cir"), None);
    assert!(
        !output.status.success(),
        "invalid ratio unexpectedly succeeded"
    );
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        diagnostic.contains("strictly between 0 and 1") && diagnostic.contains("1.1"),
        "diagnostic did not preserve the core ratio contract:\n{diagnostic}"
    );
}

#[test]
fn zero_f1_denominator_fails_instead_of_exporting_zero_or_infinity() {
    let dir = test_dir("zero_f1_ratio");
    let output_path = dir.join("undefined_ratio.json");
    let output = run(
        &fixture("disto_zero_fundamental_ratio.cir"),
        Some((&output_path, "json")),
    );
    assert!(
        !output.status.success(),
        "undefined 0/0 normalization unexpectedly succeeded"
    );
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let diagnostic_lower = diagnostic.to_ascii_lowercase();
    assert!(
        diagnostic_lower.contains("cannot normalize 2f1 signal 'i(vquiet)' at f1 point 0")
            && diagnostic_lower.contains("f1 magnitude is zero")
            && diagnostic_lower.contains("no finite magnitude ratio exists"),
        "undefined normalization did not fail precisely:\n{diagnostic}"
    );
    assert!(
        !output_path.exists(),
        "a failed normalization must not publish an output artifact"
    );
}

#[test]
fn dedicated_hdf5_section_round_trips_product_identity_and_provenance() {
    let dir = test_dir("hdf5");
    let hdf5_path = dir.join("two_tone.h5");
    let csv_path = dir.join("roundtrip.csv");
    let output = run(&fixture("disto_two_tone.cir"), Some((&hdf5_path, "hdf5")));
    assert!(
        output.status.success(),
        "HDF5 DISTO export failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let signature = std::fs::read(&hdf5_path).expect("read HDF5 output");
    assert_eq!(&signature[..8], b"\x89HDF\r\n\x1a\n");

    let converted = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "convert",
            hdf5_path.to_str().unwrap(),
            csv_path.to_str().unwrap(),
            "--to",
            "csv",
        ])
        .output()
        .expect("convert DISTO HDF5");
    assert!(
        converted.status.success(),
        "DISTO HDF5 round trip failed:\n{}",
        String::from_utf8_lossy(&converted.stderr)
    );
    let csv = std::fs::read_to_string(csv_path).expect("read round-trip CSV");
    let header = csv.lines().next().expect("CSV header");
    for required in [
        "f2_over_f1",
        "frequency(f2)",
        "frequency(f1+f2)",
        "Re(peak(2f1-f2:I(V1)))",
        "magnitude(2f1-f2:I(V1))",
        "phase_deg(2f1-f2:I(V1))",
        "magnitude_ratio_to_f1(2f1-f2:I(V1))",
    ] {
        assert!(
            header.contains(required),
            "round-trip HDF5 lost '{required}':\n{header}"
        );
    }
}

#[test]
fn every_flat_output_format_retains_typed_product_payload() {
    let dir = test_dir("flat_formats");
    for format in ["csv", "tsv", "ascii", "raw"] {
        let output_path = dir.join(format!("one_tone.{format}"));
        let output = run(&fixture("disto_one_tone.cir"), Some((&output_path, format)));
        assert!(
            output.status.success(),
            "{format} DISTO export failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bytes = std::fs::read(&output_path).expect("read flat DISTO output");
        let visible = String::from_utf8_lossy(&bytes);
        for required in [
            "frequency(2f1)",
            "peak(2f1:I(V1))",
            "magnitude(2f1:I(V1))",
            "phase_deg(2f1:I(V1))",
            "magnitude_ratio_to_f1(2f1:I(V1))",
        ] {
            assert!(
                visible.contains(required),
                "{format} DISTO output lost '{required}'"
            );
        }
    }
}
