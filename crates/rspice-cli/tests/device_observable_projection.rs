//! Public CLI contracts for `.SAVE @device[param]` projection.
//!
//! Device observables must be qualified data columns when the core result
//! publishes them. An unavailable probe is a typed execution failure, never a
//! successful scale-only artifact.

mod common;

use common::{TestDirectory, test_dir};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn run_fixture(tag: &str, fixture: &str) -> (TestDirectory, PathBuf, Output) {
    let dir = test_dir(tag);
    let deck = dir.join("input.cir");
    let output_path = dir.join("result.csv");
    std::fs::write(&deck, fixture).expect("write device-observable fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "-o",
            output_path.to_str().expect("UTF-8 output path"),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice device-observable fixture");
    (dir, output_path, output)
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn csv_column(csv: &str, name: &str) -> Vec<f64> {
    let mut lines = csv.lines();
    let header = lines.next().expect("CSV header");
    let index = header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing column {name} in {header}"));
    lines
        .map(|line| {
            line.split(',')
                .nth(index)
                .expect("CSV cell")
                .parse()
                .expect("numeric CSV value")
        })
        .collect()
}

fn op_csv_value(csv: &str, name: &str) -> f64 {
    csv.lines()
        .skip(1)
        .find_map(|line| {
            let (signal, value) = line.split_once(',')?;
            signal
                .eq_ignore_ascii_case(name)
                .then(|| value.parse().expect("numeric OP value"))
        })
        .unwrap_or_else(|| panic!("missing OP signal {name} in {csv}"))
}

fn cleanup(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn op_save_device_parameter_exports_the_qualified_authored_signal() {
    let (dir, output_path, output) = run_fixture(
        "op",
        include_str!("fixtures/device_observables/op_diode_id.cir"),
    );
    assert_success(&output);
    let csv = std::fs::read_to_string(&output_path).expect("read OP CSV");
    let current = op_csv_value(&csv, "@D1[Id]");
    assert!(
        current.is_finite() && current > 0.0,
        "unexpected ID: {current}"
    );
    assert_eq!(
        csv.lines().count(),
        2,
        "SAVE must restrict OP output: {csv}"
    );
    cleanup(&dir);
}

#[test]
fn dc_save_device_parameter_exports_one_value_per_sweep_coordinate() {
    let (dir, output_path, output) = run_fixture(
        "dc",
        include_str!("fixtures/device_observables/dc_diode_id.cir"),
    );
    assert_success(&output);
    let csv = std::fs::read_to_string(&output_path).expect("read DC CSV");
    let currents = csv_column(&csv, "@D1[Id]");
    assert_eq!(currents.len(), 3, "one device value per DC point: {csv}");
    assert!(currents.iter().all(|value| value.is_finite()));
    assert!(currents[2] > currents[1] && currents[1] >= currents[0]);
    cleanup(&dir);
}

#[test]
fn tran_save_device_parameter_is_a_waveform_not_a_time_only_success() {
    let (dir, output_path, output) = run_fixture(
        "tran",
        include_str!("fixtures/device_observables/tran_diode_id.cir"),
    );
    assert_success(&output);
    let csv = std::fs::read_to_string(&output_path).expect("read TRAN CSV");
    let times = csv_column(&csv, "time");
    let currents = csv_column(&csv, "@D1[Id]");
    assert!(!times.is_empty());
    assert_eq!(currents.len(), times.len());
    assert!(
        currents
            .iter()
            .all(|value| value.is_finite() && *value > 0.0)
    );
    cleanup(&dir);
}

fn assert_unavailable_failure(tag: &str, fixture: &str, authored_symbol: &str, analysis: &str) {
    let (dir, output_path, output) = run_fixture(tag, fixture);
    assert!(!output.status.success(), "unavailable signal must fail");
    assert_ne!(
        output.status.code(),
        Some(101),
        "authored input must not panic"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("requested signal")
            && stderr.contains(authored_symbol)
            && stderr.contains("is unavailable")
            && stderr.contains(analysis),
        "missing typed unavailable-signal detail: {stderr}"
    );
    assert!(
        !output_path.exists(),
        "failure must not publish a scale-only artifact"
    );
    cleanup(&dir);
}

#[test]
fn unavailable_op_device_parameter_preserves_the_authored_symbol() {
    assert_unavailable_failure(
        "op_unavailable",
        include_str!("fixtures/device_observables/op_unavailable_authored_case.cir"),
        "@D1[NotAParameter]",
        "DC OP",
    );
}

#[test]
fn ac_device_parameter_fails_instead_of_publishing_frequency_only() {
    assert_unavailable_failure(
        "ac_unavailable",
        include_str!("fixtures/device_observables/ac_unavailable_resistor_r.cir"),
        "@R1[r]",
        "AC",
    );
}

#[test]
fn unavailable_ac_device_parameter_fails_even_without_an_output_file() {
    let dir = test_dir("ac_unavailable_no_output");
    let deck = dir.join("input.cir");
    std::fs::write(
        &deck,
        include_str!("fixtures/device_observables/ac_unavailable_resistor_r.cir"),
    )
    .expect("write AC unavailable fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run", deck.to_str().expect("UTF-8 deck path")])
        .output()
        .expect("run AC unavailable fixture without output");
    assert!(
        !output.status.success(),
        "output selection is deck semantics"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("requested signal '@R1[r]' is unavailable for AC analysis"),
        "authored AC signal diagnostic was not preserved: {stderr}"
    );
    cleanup(&dir);
}
