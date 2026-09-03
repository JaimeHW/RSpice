//! Public CLI contracts for device observables.
//!
//! Two contracts, one signal kind. The flat formats publish exactly the
//! qualified columns the deck's `.SAVE`/`.PRINT` projection selected, and an
//! unavailable probe is a typed execution failure rather than a successful
//! scale-only artifact. The shared typed document instead publishes the
//! complete device inventory the solver computed, projection or no projection,
//! because a document that carried only the authored subset could not be told
//! apart from one whose devices had no state.

mod common;

use common::{AxisRunSet, TestDirectory, read_json, test_dir};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn run_source(tag: &str, source: &str, format: &str) -> (TestDirectory, PathBuf, Output) {
    let dir = test_dir(tag);
    let deck = dir.join("input.cir");
    let output_path = dir.join(format!("result.{format}"));
    std::fs::write(&deck, source).expect("write device-observable fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "-o",
            output_path.to_str().expect("UTF-8 output path"),
            "-f",
            format,
        ])
        .output()
        .expect("run rspice device-observable fixture");
    (dir, output_path, output)
}

fn run_fixture(tag: &str, fixture: &str) -> (TestDirectory, PathBuf, Output) {
    run_source(tag, fixture, "csv")
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

/// One deck whose authored projection names a single device parameter, so the
/// flat artifact carries one column while the typed document must carry the
/// device's whole state.
const NARROW_PROJECTION: &str = "* narrow device-observable projection\n\
                                 V1 in 0 DC 0.6 SIN(0.6 0.05 1k)\n\
                                 R1 in mid 100\n\
                                 D1 mid 0 DMODEL\n\
                                 .MODEL DMODEL D(IS=1e-12 N=1)\n\
                                 .SAVE V(mid) @D1[Id]\n";

/// The device parameters one published document carries for a named device.
fn document_device_parameters(document: &serde_json::Value, device: &str) -> Vec<String> {
    document["deviceStates"]
        .as_array()
        .unwrap_or_else(|| panic!("document has no deviceStates array: {document:#}"))
        .iter()
        .find(|state| state["deviceName"] == device)
        .unwrap_or_else(|| panic!("document names no device '{device}': {document:#}"))["parameters"]
        .as_array()
        .expect("device parameter list")
        .iter()
        .map(|parameter| {
            parameter["name"]
                .as_str()
                .expect("parameter name")
                .to_ascii_lowercase()
        })
        .collect()
}

/// The complete inventory the diode model computes. A document that published
/// only `id` would be indistinguishable from one whose device had no other
/// state, which is exactly what a consumer reading the typed result needs to
/// be able to tell apart.
const DIODE_INVENTORY: [&str; 4] = ["vd", "id", "gd", "cd"];

fn assert_complete_diode_inventory(document: &serde_json::Value, what: &str) {
    let parameters = document_device_parameters(document, "D1");
    for expected in DIODE_INVENTORY {
        assert!(
            parameters.iter().any(|name| name == expected),
            "{what}: the typed document dropped the '{expected}' observable; it carries {parameters:?}"
        );
    }
}

/// Every family whose solver captures per-device state publishes the complete
/// inventory in its typed document, even when the authored projection selected
/// one parameter for the flat table.
#[test]
fn the_typed_document_publishes_the_complete_device_inventory_for_every_family() {
    for (tag, cards, artifact) in [
        ("doc_op", ".OP\n", "op-001"),
        ("doc_dc", ".DC V1 0.5 0.7 0.1\n", "dc-001"),
        ("doc_tran", ".TRAN 100u 1m\n", "tran-001"),
    ] {
        let (dir, requested, output) =
            run_source(tag, &format!("{NARROW_PROJECTION}{cards}.END\n"), "json");
        assert_success(&output);
        let stem = requested.file_stem().expect("stem").to_string_lossy();
        // A single-analysis deck keeps the requested path; the loop asks for
        // the namespaced one only when the family publishes it.
        let namespaced = requested.with_file_name(format!("{stem}.{artifact}.json"));
        let path = if namespaced.exists() {
            namespaced
        } else {
            requested.clone()
        };
        assert_complete_diode_inventory(&read_json(&path), artifact);

        // The flat artifact stays projection-driven: it has no representation
        // for a per-family payload, so it publishes the authored columns only.
        let (_flat_dir, flat_path, flat_output) =
            run_source(tag, &format!("{NARROW_PROJECTION}{cards}.END\n"), "csv");
        assert_success(&flat_output);
        let flat_stem = flat_path.file_stem().expect("stem").to_string_lossy();
        let flat_namespaced = flat_path.with_file_name(format!("{flat_stem}.{artifact}.csv"));
        let flat = std::fs::read_to_string(if flat_namespaced.exists() {
            &flat_namespaced
        } else {
            &flat_path
        })
        .expect("read flat artifact");
        let header = flat.lines().next().expect("header").to_ascii_lowercase();
        assert!(
            header.contains("@d1[id]") || flat.to_ascii_lowercase().contains("@d1[id]"),
            "{artifact}: the authored device column is missing from {flat}"
        );
        for dropped in ["[gd]", "[cd]"] {
            assert!(
                !flat.to_ascii_lowercase().contains(dropped),
                "{artifact}: the flat artifact published an unprojected observable {dropped}"
            );
        }
        cleanup(&dir);
    }
}

/// A stepped implicit operating point publishes the same complete inventory at
/// every coordinate. A sweep that dropped the device report would publish
/// fewer observables than the identical deck run without an axis.
#[test]
fn every_axis_coordinate_publishes_the_complete_device_inventory() {
    let (dir, requested, output) = run_source(
        "doc_axis",
        "* stepped device-observable inventory\n\
         .param rs=100\n\
         V1 in 0 DC 0.6\n\
         R1 in mid {rs}\n\
         D1 mid 0 DMODEL\n\
         .MODEL DMODEL D(IS=1e-12 N=1)\n\
         .SAVE V(mid) @D1[Id]\n\
         .STEP PARAM rs LIST 100 200\n\
         .OP\n\
         .END\n",
        "json",
    );
    assert_success(&output);
    let run_set = AxisRunSet::read(&requested);
    assert_eq!(run_set.coordinates.len(), 2);
    for coordinate in &run_set.coordinates {
        assert_complete_diode_inventory(&read_json(coordinate.only_artifact()), &coordinate.tag);
    }
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
