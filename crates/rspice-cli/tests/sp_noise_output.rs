//! End-to-end `.SP DONOISE` coverage for parser compatibility, physical
//! two-port noise data, format retention, and fail-closed Touchstone policy.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

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
    let path = std::env::temp_dir().join(format!(
        "rspice_sp_noise_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create SP noise test directory");
    TestDirectory(path)
}

fn run(deck: &Path, output_path: Option<&Path>, format: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args(["--quiet", "run", deck.to_str().expect("UTF-8 deck path")]);
    if let Some(path) = output_path {
        command.args(["--output", path.to_str().expect("UTF-8 output path")]);
    }
    if let Some(format) = format {
        command.args(["--format", format]);
    }
    command.output().expect("run rspice SP deck")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("read SP JSON")).expect("parse SP JSON")
}

/// One `.SP` CSV artifact decoded into its scale and its named columns.
///
/// `.SP DONOISE` publishes the current covariance matrix, its reference
/// temperature, the 4kT normalization, and the two-port noise figures. The
/// shared typed result document has no home for the last three, so those runs
/// are exported in a flat format, which retains every column; this reads one
/// back so the physical assertions below are unchanged.
struct SpTable {
    scale: Vec<f64>,
    columns: Vec<(String, Vec<f64>)>,
}

impl SpTable {
    fn read(path: &Path) -> Self {
        let text = std::fs::read_to_string(path).expect("read SP CSV");
        let mut lines = text.lines().filter(|line| !line.trim().is_empty());
        let header = lines
            .next()
            .expect("SP CSV header")
            .split(',')
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut columns = header
            .iter()
            .skip(1)
            .map(|name| (name.clone(), Vec::new()))
            .collect::<Vec<(String, Vec<f64>)>>();
        let mut scale = Vec::new();
        for line in lines {
            let fields = line.split(',').collect::<Vec<_>>();
            assert_eq!(fields.len(), header.len(), "ragged SP CSV row: {line}");
            scale.push(fields[0].parse::<f64>().expect("numeric SP scale"));
            for (index, column) in columns.iter_mut().enumerate() {
                column
                    .1
                    .push(fields[index + 1].parse::<f64>().expect("numeric SP value"));
            }
        }
        Self { scale, columns }
    }

    fn column(&self, name: &str) -> &[f64] {
        self.columns
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, values)| values.as_slice())
            .unwrap_or_else(|| {
                panic!(
                    "missing column '{name}' in {:?}",
                    self.columns.iter().map(|(n, _)| n).collect::<Vec<_>>()
                )
            })
    }

    /// The real part of a complex column, which the flat writer splits into
    /// `Re(name)`/`Im(name)`.
    fn real_values(&self, name: &str) -> &[f64] {
        self.column(&format!("Re({name})"))
    }

    fn imag_values(&self, name: &str) -> &[f64] {
        self.column(&format!("Im({name})"))
    }

    /// Exported signal names with the complex real/imaginary decoration
    /// removed, in artifact order.
    fn signal_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for (name, _) in &self.columns {
            let bare = name
                .strip_prefix("Re(")
                .or_else(|| name.strip_prefix("Im("))
                .and_then(|rest| rest.strip_suffix(')'))
                .unwrap_or(name);
            if !names.iter().any(|seen| seen == bare) {
                names.push(bare.to_string());
            }
        }
        names
    }
}

fn assert_relative(actual: f64, expected: f64, tolerance: f64, label: &str) {
    let error = (actual - expected).abs() / expected.abs().max(f64::MIN_POSITIVE);
    assert!(
        error <= tolerance,
        "{label}: actual={actual:.16e}, expected={expected:.16e}, relative error={error:.3e}"
    );
}

#[test]
fn keyword_noise_exports_physical_covariance_and_two_port_parameters() {
    let dir = test_dir("oracle");
    let path = dir.join("sp.csv");
    let output = run(&fixture("sp_donoise_keyword.cir"), Some(&path), Some("csv"));
    assert!(
        output.status.success(),
        "SP DONOISE failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let table = SpTable::read(&path);
    assert_eq!(table.scale, vec![1000.0, 2000.0, 3000.0]);

    let temperature = 300.15;
    let expected_density = 4.0 * 1.380_649e-23 * temperature / 50.0;
    for &value in table.real_values("CY_A2_per_Hz_1_1") {
        assert_relative(value, expected_density, 1.0e-10, "C11");
    }
    for &value in table.real_values("CY_A2_per_Hz_1_2") {
        assert_relative(value, -expected_density, 1.0e-10, "C12");
    }
    for &value in table.real_values("noise_resistance_ohm") {
        assert_relative(value, 50.0, 1.0e-10, "Rn");
    }
    for &value in table.real_values("noise_factor_linear") {
        assert_relative(value, 2.0, 1.0e-10, "F");
    }
    for &value in table.real_values("minimum_noise_factor_linear") {
        assert_relative(value, 1.0, 1.0e-10, "Fmin");
    }
    for &value in table.real_values("noise_reference_temperature_K") {
        assert_relative(value, temperature, f64::EPSILON * 4.0, "temperature");
    }
    for &value in table.real_values("noise_normalization_4kT_J") {
        assert_relative(
            value,
            4.0 * 1.380_649e-23 * temperature,
            1.0e-12,
            "4kT normalization",
        );
    }
    assert!(
        table
            .real_values("optimum_source_reflection")
            .iter()
            .all(|value| (value - 1.0).abs() <= 1.0e-10)
    );
    assert!(
        table
            .imag_values("optimum_source_reflection")
            .iter()
            .all(|value| value.abs() <= 1.0e-12)
    );
}

/// `.SP DONOISE` publishes two typed documents: the scattering sweep under the
/// card's identity, and the port-noise sweep beside it under the same
/// identity. The noise document carries its own provenance -- the reference
/// temperature, the 4kT normalization it is measured against, and the two-port
/// figures -- because none of that is recoverable from the covariance alone.
#[test]
fn donoise_publishes_the_scattering_and_port_noise_documents_side_by_side() {
    let dir = test_dir("donoise_json");
    let requested = dir.join("sp.json");
    let output = run(
        &fixture("sp_donoise_keyword.cir"),
        Some(&requested),
        Some("json"),
    );
    assert!(
        output.status.success(),
        ".SP DONOISE failed to publish typed documents:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let scattering = read_json(&requested);
    assert_eq!(scattering["resultKind"], "sp");
    assert_eq!(scattering["analysis"]["tag"], "sp-001");

    let noise = read_json(&dir.join("sp.port-noise.json"));
    assert_eq!(noise["resultKind"], "port-noise");
    assert_eq!(
        noise["analysis"]["tag"], "sp-001",
        "port noise is the .SP card's second result and shares its identity"
    );
    let payload = &noise["payload"];
    assert_eq!(payload["portCount"], 2);
    assert_eq!(
        payload["covarianceNormalization"], "ampere-squared-per-hertz",
        "the covariance scale must be declared, not assumed"
    );
    let temperature = payload["referenceTemperatureKelvin"]
        .as_f64()
        .expect("reference temperature");
    assert!(
        (temperature - 300.15).abs() < 1.0e-9,
        "reference temperature is {temperature} K"
    );
    let normalization = payload["thermalNormalizationJoule"]
        .as_f64()
        .expect("4kT normalization");
    assert!(
        (normalization / (4.0 * 1.380_649e-23 * temperature) - 1.0).abs() < 1.0e-12,
        "4kT normalization is {normalization} J"
    );

    let two_port = payload["twoPort"].as_array().expect("two-port figures");
    assert_eq!(
        two_port.len(),
        3,
        "one entry per swept frequency: {two_port:#?}"
    );
    for entry in two_port {
        // A matched 50 ohm attenuator between 50 ohm ports: F = 2, Fmin = 1,
        // and the optimum source reflection is the matched load.
        assert!(
            (entry["noiseFactor"].as_f64().expect("F") - 2.0).abs() < 1.0e-9,
            "noise factor: {entry:#?}"
        );
        assert!(
            (entry["minimumNoiseFactor"].as_f64().expect("Fmin") - 1.0).abs() < 1.0e-9,
            "minimum noise factor: {entry:#?}"
        );
        assert!(
            (entry["optimumSourceReflection"]["real"]
                .as_f64()
                .expect("Sopt real")
                - 1.0)
                .abs()
                < 1.0e-9,
            "optimum source reflection: {entry:#?}"
        );
    }

    // The same deck without DONOISE publishes the scattering document alone.
    let plain = dir.join("plain.json");
    let output = run(
        &fixture("sp_donoise_numeric_false.cir"),
        Some(&plain),
        Some("json"),
    );
    assert!(
        output.status.success(),
        "plain .SP failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = read_json(&plain);
    assert_eq!(document["resultKind"], "sp");
    assert_eq!(document["analysis"]["tag"], "sp-001");
    assert!(
        !dir.join("plain.port-noise.json").exists(),
        "a deck that requested no port noise published a port-noise artifact"
    );
}

#[test]
fn numeric_noise_forms_are_explicit_and_do_not_claim_disabled_work() {
    let dir = test_dir("numeric");
    let keyword_path = dir.join("keyword.csv");
    let enabled_path = dir.join("enabled.csv");
    let disabled_path = dir.join("disabled.csv");
    for (fixture_name, path) in [
        ("sp_donoise_keyword.cir", &keyword_path),
        ("sp_donoise_numeric_true.cir", &enabled_path),
        ("sp_donoise_numeric_false.cir", &disabled_path),
    ] {
        let output = run(&fixture(fixture_name), Some(path), Some("csv"));
        assert!(
            output.status.success(),
            "{fixture_name} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let keyword = std::fs::read_to_string(&keyword_path).expect("read keyword form");
    let enabled = std::fs::read_to_string(&enabled_path).expect("read numeric-true form");
    assert_eq!(keyword, enabled);

    let names = SpTable::read(&disabled_path).signal_names();
    assert!(names.iter().all(|name| name.starts_with("S_")), "{names:?}");
    assert!(
        !names
            .iter()
            .any(|name| name.contains("noise") || name.starts_with("CY_"))
    );
}

#[test]
fn malformed_noise_form_fails_before_publishing_an_artifact() {
    let dir = test_dir("malformed");
    let path = dir.join("must-not-exist.json");
    let output = run(
        &fixture("sp_donoise_malformed.cir"),
        Some(&path),
        Some("json"),
    );
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(diagnostic.contains("trailing token"), "{diagnostic}");
    assert!(!path.exists(), "malformed .SP published an artifact");
}

#[test]
fn capable_formats_retain_noise_identity_and_hdf5_round_trips() {
    let dir = test_dir("formats");
    for format in ["csv", "tsv", "ascii", "raw"] {
        let path = dir.join(format!("sp.{format}"));
        let output = run(
            &fixture("sp_donoise_keyword.cir"),
            Some(&path),
            Some(format),
        );
        assert!(
            output.status.success(),
            "{format} SP noise export failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bytes = std::fs::read(&path).expect("read output");
        let visible = String::from_utf8_lossy(&bytes);
        for required in [
            "CY_A2_per_Hz_1_2",
            "noise_reference_temperature_K",
            "noise_normalization_4kT_J",
            "noise_resistance_ohm",
            "minimum_noise_factor_linear",
            "optimum_source_reflection",
        ] {
            assert!(visible.contains(required), "{format} lost {required}");
        }
    }

    let hdf5 = dir.join("sp.h5");
    let csv = dir.join("from_hdf5.csv");
    let output = run(
        &fixture("sp_donoise_keyword.cir"),
        Some(&hdf5),
        Some("hdf5"),
    );
    assert!(output.status.success(), "HDF5 export failed: {:?}", output);
    let converted = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "convert",
            hdf5.to_str().unwrap(),
            csv.to_str().unwrap(),
            "--to",
            "csv",
        ])
        .output()
        .expect("convert HDF5 SP noise");
    assert!(
        converted.status.success(),
        "HDF5 round trip failed:\n{}",
        String::from_utf8_lossy(&converted.stderr)
    );
    let header = std::fs::read_to_string(csv)
        .expect("read converted CSV")
        .lines()
        .next()
        .expect("CSV header")
        .to_string();
    assert!(header.contains("CY_A2_per_Hz_1_2"), "{header}");
    assert!(header.contains("noise_resistance_ohm"), "{header}");
}

#[test]
fn touchstone_refuses_to_silently_drop_requested_noise_data() {
    let dir = test_dir("touchstone");
    let path = dir.join("sp.s2p");
    let output = run(&fixture("sp_donoise_keyword.cir"), Some(&path), None);
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        diagnostic.contains("cannot retain the full .SP DONOISE covariance"),
        "{diagnostic}"
    );
    assert!(!path.exists(), "lossy Touchstone artifact was published");
}
