//! CLI `.DISTO` must expose the core Volterra products, not an ordinary AC
//! substitute. These end-to-end oracles cover harmonic and fixed-F2 two-tone
//! semantics, physical product frequencies, peak phasors, normalization, and
//! the dedicated HDF5 round trip.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

const BIAS: f64 = 0.5;
const IS: f64 = 1.0e-12;
const A1: f64 = 1.0e-3;
const A2: f64 = 2.0e-3;
const TEMP_K: f64 = 300.15;
const BOLTZMANN: f64 = 1.380_649e-23;
const ELECTRON_CHARGE: f64 = 1.602_176_634e-19;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("audit_regressions")
        .join(name)
}

struct TestDirectory(PathBuf);

impl std::ops::Deref for TestDirectory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn test_dir(tag: &str) -> TestDirectory {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "rspice_disto_output_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&dir).expect("create DISTO test directory");
    TestDirectory(dir)
}

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

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("read JSON output"))
        .expect("parse JSON output")
}

fn values<'a>(document: &'a Value, name: &str) -> &'a [Value] {
    document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| {
            signal["name"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("missing signal '{name}' in {document:#}"))["values"]
        .as_array()
        .expect("real-valued signal")
}

fn complex_values<'a>(document: &'a Value, name: &str) -> (&'a [Value], &'a [Value]) {
    let signal = document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| {
            signal["name"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("missing signal '{name}' in {document:#}"));
    (
        signal["real"].as_array().expect("complex real values"),
        signal["imag"].as_array().expect("complex imaginary values"),
    )
}

fn numbers(values: &[Value]) -> Vec<f64> {
    values
        .iter()
        .map(|value| value.as_f64().expect("finite JSON number"))
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
    assert_eq!(document["analysis"], "disto");
    assert_eq!(document["scale"]["name"], "frequency(f1)");
    assert_eq!(
        numbers(document["scale"]["values"].as_array().unwrap()),
        [1.0e3, 1.5e3, 2.0e3]
    );
    assert_eq!(
        numbers(values(&document, "frequency(2f1)")),
        [2.0e3, 3.0e3, 4.0e3]
    );
    assert_eq!(
        numbers(values(&document, "frequency(3f1)")),
        [3.0e3, 4.5e3, 6.0e3]
    );

    let vt = thermal_voltage();
    let expected_hd2 = diode_bias_current() * A1.powi(2) / (4.0 * vt.powi(2));
    let expected_hd3 = diode_bias_current() * A1.powi(3) / (24.0 * vt.powi(3));
    let hd2 = numbers(values(&document, "magnitude(2f1:I(V1))"));
    let hd3 = numbers(values(&document, "magnitude(3f1:I(V1))"));
    for actual in hd2 {
        assert_close(actual, expected_hd2, 2.0e-5, "second harmonic current");
    }
    for actual in hd3 {
        assert_close(actual, expected_hd3, 2.0e-3, "third harmonic current");
    }

    let (real, imag) = complex_values(&document, "peak(2f1:I(V1))");
    let real = numbers(real);
    let imag = numbers(imag);
    assert!(
        real.iter().all(|value| *value < 0.0),
        "the voltage-source branch convention makes the diode product current negative real"
    );
    assert!(
        imag.iter()
            .all(|value| value.abs() <= expected_hd2 * 1.0e-10)
    );
    let phase = numbers(values(&document, "phase_deg(2f1:I(V1))"));
    assert!(
        phase
            .iter()
            .all(|value| (value.abs() - 180.0).abs() <= 1.0e-8)
    );
    let ratios = numbers(values(&document, "magnitude_ratio_to_f1(2f1:I(V1))"));
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
    assert_eq!(numbers(values(&document, "f2_over_f1")), [0.9, 0.9, 0.9]);
    assert_eq!(
        numbers(values(&document, "frequency(f2)")),
        [900.0, 900.0, 900.0]
    );
    assert_eq!(
        numbers(values(&document, "frequency(f1+f2)")),
        [1_900.0, 2_400.0, 2_900.0]
    );
    assert_eq!(
        numbers(values(&document, "frequency(f1-f2)")),
        [100.0, 600.0, 1_100.0]
    );
    assert_eq!(
        numbers(values(&document, "frequency(2f1-f2)")),
        [1_100.0, 2_100.0, 3_100.0]
    );

    let vt = thermal_voltage();
    let expected_im2 = diode_bias_current() * A1 * A2 / (2.0 * vt.powi(2));
    let expected_im3 = diode_bias_current() * A1.powi(2) * A2 / (8.0 * vt.powi(3));
    for product in ["f1+f2", "f1-f2"] {
        for actual in numbers(values(&document, &format!("magnitude({product}:I(V1))"))) {
            assert_close(actual, expected_im2, 2.0e-5, product);
        }
    }
    for actual in numbers(values(&document, "magnitude(2f1-f2:I(V1))")) {
        assert_close(actual, expected_im3, 2.0e-3, "third-order difference");
    }
    for product in ["f1+f2", "2f1-f2"] {
        assert!(
            numbers(values(&document, &format!("phase_deg({product}:I(V1))")))
                .iter()
                .all(|phase| (phase.abs() - 180.0).abs() <= 1.0e-8),
            "{product} branch-current phase must match the real negative diode oracle"
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
