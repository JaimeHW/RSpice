//! `.TEMP` is a run axis, not an independent operating-point analysis.

mod common;

use common::{AxisRunSet, fixture, read_json, test_dir};

use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn run(deck: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            output.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run temperature deck")
}

/// Values of one named column in the flat swept table an axis deck publishes
/// when its coordinates share a topology and schema.
fn signal_real_values(document: &Value, name: &str) -> Vec<f64> {
    let signal = document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| {
            signal["name"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("missing '{name}' in {document:#}"));
    let values = signal["values"]
        .as_array()
        .or_else(|| signal["real"].as_array())
        .expect("real signal values");
    values
        .iter()
        .map(|value| value.as_f64().expect("finite signal value"))
        .collect()
}

/// Samples of one named series in a typed coordinate result document.
///
/// A complex family publishes one complex sample per point; its real part is
/// what a coordinate-to-coordinate comparison reads.
fn document_series(document: &Value, name: &str) -> Vec<f64> {
    document["signals"]
        .as_array()
        .expect("document signals")
        .iter()
        .find(|signal| {
            signal["descriptor"]["canonicalName"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("missing series '{name}' in {document:#}"))["values"]["samples"]
        .as_array()
        .expect("series samples")
        .iter()
        .map(|sample| {
            sample
                .as_f64()
                .or_else(|| sample["real"].as_f64())
                .expect("finite sample")
        })
        .collect()
}

#[test]
fn multi_temperature_wraps_every_authored_analysis_without_extra_runs() {
    let dir = test_dir("multi");
    let requested = dir.join("results.json");
    let output = run(&fixture("temp_wraps_tran_ac.cir"), &requested);
    assert!(
        output.status.success(),
        "temperature-axis deck failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!requested.exists(), "multi-run base path was overwritten");

    // The committed run-set manifest is the run's own statement of what it
    // published; the directory must contain exactly that and nothing else.
    let run_set = AxisRunSet::read(&requested);
    assert_eq!(run_set.axes, ["temperature"]);
    assert_eq!(run_set.coordinates.len(), 2);
    let expected = run_set.artifacts();
    let mut actual = std::fs::read_dir(&*dir)
        .expect("list temperature outputs")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    actual.sort();
    assert_eq!(
        actual,
        run_set.published_files(&requested),
        "unexpected nominal or OP artifact"
    );
    for coordinate in &run_set.coordinates {
        assert_eq!(
            coordinate.artifacts.len(),
            2,
            "each temperature wraps both authored analyses: {coordinate:?}"
        );
        coordinate.artifact("tran-001");
        coordinate.artifact("ac-001");
    }

    for path in &expected {
        let document = read_json(path);
        let expected_kind = if path.to_string_lossy().contains(".tran-") {
            "tran"
        } else {
            "ac"
        };
        assert_eq!(document["resultKind"], expected_kind, "{}", path.display());
        assert!(!document_series(&document, "v(out)").is_empty());
    }

    let cold_ac = read_json(&expected[1]);
    let hot_ac = read_json(&expected[3]);
    assert_ne!(
        document_series(&cold_ac, "v(out)"),
        document_series(&hot_ac, "v(out)"),
        "temperature coordinates produced identical AC data"
    );
}

#[test]
fn single_temperature_configures_transient_without_running_a_temp_op() {
    let dir = test_dir("single");
    let requested = dir.join("transient.json");
    let output = run(&fixture("temp_single_tran.cir"), &requested);
    assert!(
        output.status.success(),
        "single-temperature transient failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut files = std::fs::read_dir(&*dir)
        .expect("list outputs")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    files.sort();
    let run_set = AxisRunSet::read(&requested);
    assert_eq!(
        files.as_slice(),
        run_set.published_files(&requested).as_slice()
    );
    assert_eq!(run_set.coordinates.len(), 1);
    let transient = run_set.coordinate(1).only_artifact();
    assert_eq!(read_json(transient)["resultKind"], "tran");
}

#[test]
fn single_temperature_rejects_an_explicit_cli_analysis_mode() {
    let dir = test_dir("single_requested_mode");
    let deck = dir.join("single_temperature.cir");
    std::fs::write(
        &deck,
        "Single temperature with an incompatible requested mode\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .TEMP 25\n\
         .END\n",
    )
    .expect("write single-temperature deck");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--monte-carlo",
            "2",
        ])
        .output()
        .expect("run incompatible single-temperature deck");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be combined with authored .STEP/.TEMP run axes"),
        "explicit mode must fail before the temperature coordinate is simulated: {stderr}"
    );
}

#[test]
fn repeated_ac_cards_keep_stable_analysis_namespaces_at_each_temperature() {
    let dir = test_dir("repeated_ac");
    let deck = dir.join("repeated_ac.cir");
    let requested = dir.join("response.json");
    std::fs::write(
        &deck,
        "Repeated AC temperature namespaces\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1n\n\
         .TEMP 25 85\n\
         .AC LIN 1 1k 1k\n\
         .AC LIN 1 10k 10k\n\
         .SAVE V(out)\n\
         .END\n",
    )
    .expect("write repeated-AC temperature deck");

    let output = run(&deck, &requested);
    assert!(
        output.status.success(),
        "repeated-AC temperature deck failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!requested.exists(), "multi-run base path was overwritten");

    let run_set = AxisRunSet::read(&requested);
    for coordinate in &run_set.coordinates {
        let first_document = read_json(coordinate.artifact("ac-001"));
        let second_document = read_json(coordinate.artifact("ac-002"));
        assert_eq!(first_document["resultKind"], "ac");
        assert_eq!(second_document["resultKind"], "ac");
        assert_eq!(first_document["analysis"]["tag"], "ac-001");
        assert_eq!(second_document["analysis"]["tag"], "ac-002");
        let first_values = document_series(&first_document, "v(out)");
        let second_values = document_series(&second_document, "v(out)");
        assert_eq!(first_values.len(), 1);
        assert_eq!(second_values.len(), 1);
        assert_ne!(
            first_values, second_values,
            "the two authored AC configurations collapsed into one result"
        );
    }

    let mut actual = std::fs::read_dir(&*dir)
        .expect("list repeated-AC outputs")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    actual.sort();
    assert_eq!(
        actual,
        run_set.published_files(&requested),
        "unexpected OP or colliding AC artifact"
    );
}

#[test]
fn two_axis_implicit_op_uses_coordinate_artifacts_and_union_manifest() {
    let dir = test_dir("implicit_cartesian");
    let deck = dir.join("implicit_cartesian.cir");
    let requested = dir.join("operating_points.json");
    std::fs::write(
        &deck,
        "Two-dimensional implicit operating points\n\
         .param resistance=1k\n\
         V1 out 0 1\n\
         R1 out 0 {resistance}\n\
         .STEP PARAM resistance LIST 1k 2k\n\
         .TEMP 25 85\n\
         .SAVE V(out) I(V1)\n\
         .END\n",
    )
    .expect("write implicit Cartesian deck");

    let output = run(&deck, &requested);
    assert!(
        output.status.success(),
        "implicit Cartesian deck failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!requested.exists(), "Cartesian run overwrote the base path");

    let run_set = AxisRunSet::read(&requested);
    assert_eq!(run_set.axes, ["step", "temperature"]);
    let expected = run_set.artifacts();
    assert_eq!(expected.len(), 4);
    for artifact in &expected {
        assert!(artifact.exists(), "missing {}", artifact.display());
        assert!(
            artifact
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.contains(".run_") && name.contains(".implicit-op-001."))
        );
        let document = read_json(artifact);
        assert_eq!(document["resultKind"], "op");
        assert_eq!(document["analysis"]["tag"], "implicit-op-001");
        // A coordinate artifact names the coordinate and the topology it was
        // produced at, so it can be read back without its manifest.
        assert!(!document["coordinate"]["label"].is_null());
        assert!(
            document["topologyFingerprint"]
                .as_str()
                .is_some_and(|hex| hex.len() == 64)
        );
    }

    // The schema manifest groups its union by analysis instance, so a deck
    // that publishes several analyses at each coordinate has one entry per
    // analysis rather than one manifest per deck shape.
    let manifest_path = dir.join("operating_points.step_schema.json");
    let manifest = read_json(&manifest_path);
    assert_eq!(manifest["aggregation"], "coordinate_local");
    assert_eq!(manifest["missingness"], "union_validity_bitmap");
    let analyses = manifest["analyses"].as_array().expect("manifest analyses");
    assert_eq!(analyses.len(), 1);
    let entry = &analyses[0];
    assert_eq!(entry["analysis_id"], "implicit-op-001");
    let coordinates = entry["coordinates"]
        .as_array()
        .expect("manifest coordinates");
    assert_eq!(coordinates.len(), 4);
    for coordinate in coordinates {
        assert_eq!(
            coordinate["validity"].as_array().expect("validity").len(),
            entry["union_schema"]
                .as_array()
                .expect("union schema")
                .len()
        );
        let artifact = coordinate["artifact"].as_str().expect("artifact filename");
        assert!(artifact.contains(".run_") && artifact.contains(".implicit-op-001."));
    }
}

#[test]
fn temperature_only_deck_publishes_one_swept_table_and_no_per_point_artifact() {
    let dir = test_dir("temp_only");
    let deck = dir.join("temperature_only.cir");
    std::fs::write(
        &deck,
        "Temperature axis with no authored physical analysis\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         R2 out 0 1k TC1=0.01\n\
         .TEMP -40 25 125\n\
         .END\n",
    )
    .expect("write temperature-only deck");
    let requested = dir.join("sweep.json");

    let output = run(&deck, &requested);
    assert!(
        output.status.success(),
        "temperature-only deck failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // `.TEMP` is a run axis, so the deck publishes one swept operating-point
    // table under the axis namespace. The retired per-temperature analysis
    // artifact (`sweep.temp.json`) must not reappear.
    let mut files = std::fs::read_dir(&*dir)
        .expect("list temperature-only outputs")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    files.sort();
    assert_eq!(
        files.as_slice(),
        [requested.clone()].as_slice(),
        "temperature axis published an unexpected artifact set"
    );

    let document = read_json(&requested);
    let temperatures = document["scale"]["values"]
        .as_array()
        .expect("swept temperature axis");
    assert_eq!(temperatures.len(), 3);
    let out = signal_real_values(&document, "V(out)");
    assert_eq!(out.len(), 3);
    assert!(
        out[0] != out[2],
        "a temperature-dependent divider produced identical cold and hot operating points"
    );
}
