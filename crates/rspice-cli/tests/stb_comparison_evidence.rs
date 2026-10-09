//! Equal return ratios do not imply equal natural circuit stability.
mod common;

use serde_json::json;
use std::path::Path;
use std::process::{Command, Output};

const LOOP: &str = "* hidden circuit mode\nE1 EO 0 CTRL 0 -1000\nVP EO X 0\nR1 X CTRL 1k\nC1 CTRL 0 159.154943091895n\nGhidden hidden 0 hidden 0 CONDUCTANCE\nChidden hidden 0 1\n.stb dec 10 10 10meg probe=VP\n.end\n";

fn run(
    directory: &Path,
    case: &str,
    conductance: &str,
    format: &str,
    extension: &str,
) -> std::path::PathBuf {
    let deck = directory.join(format!("{case}.sp"));
    std::fs::write(&deck, LOOP.replace("CONDUCTANCE", conductance)).unwrap();
    let path = directory.join(format!("{case}.{extension}"));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(&path)
        .args(["-f", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    path
}

fn compare(result: &Path, golden: &Path, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(result)
        .arg(golden)
        .arg("--json")
        .args(flags)
        .output()
        .unwrap()
}

fn convert(source: &Path, destination: &Path, format: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn hidden_unstable_modes_fail_comparison_in_typed_converted_and_native_outputs() {
    let directory = common::test_dir("stb_hidden_comparison");
    let stable = run(&directory, "stable", "1", "json", "json");
    let unstable = run(&directory, "unstable", "-1", "json", "json");
    // Establish that the regression is about lost circuit evidence, not curves.
    let stable_document = common::read_json(&stable);
    let unstable_document = common::read_json(&unstable);
    assert_eq!(stable_document["signals"], unstable_document["signals"]);
    assert_eq!(stable_document["scalars"], unstable_document["scalars"]);
    assert_ne!(
        stable_document["payload"]["circuitPoles"],
        unstable_document["payload"]["circuitPoles"]
    );
    let output = compare(
        &unstable,
        &stable,
        &["--abstol", "1e100", "--reltol", "1e100"],
    );
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("circuit_stability(stable)"));
    assert!(
        compare(&unstable, &stable, &["--variables", "L(jw)"])
            .status
            .success()
    );

    for (format, extension) in [
        ("json", "json"),
        ("csv", "csv"),
        ("tsv", "tsv"),
        ("raw", "raw"),
        ("ascii", "ascii.raw"),
        ("hdf5", "h5"),
    ] {
        let converted_stable = directory.join(format!("converted-stable.{extension}"));
        let converted_unstable = directory.join(format!("converted-unstable.{extension}"));
        convert(&stable, &converted_stable, format);
        convert(&unstable, &converted_unstable, format);
        let same = compare(&converted_stable, &stable, &[]);
        assert!(same.status.success(), "{format}: {same:?}");
        let decoded = directory.join("decoded.json");
        convert(&converted_stable, &decoded, "json");
        let table = common::read_json(&decoded);
        let pole = table["signals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|column| column["name"] == "stb:pole(2)")
            .unwrap();
        assert_eq!(pole["unit"], "rad/s");
        assert!(
            pole["real"]
                .as_array()
                .unwrap()
                .iter()
                .all(|value| value == &json!(-1.0))
        );
        let changed = compare(&converted_unstable, &converted_stable, &[]);
        assert_eq!(changed.status.code(), Some(3), "{format}: {changed:?}");

        let native_stable = run(&directory, "native-stable", "1", format, extension);
        let native_unstable = run(&directory, "native-unstable", "-1", format, extension);
        let native = compare(&native_unstable, &native_stable, &[]);
        assert_eq!(native.status.code(), Some(3), "{format}: {native:?}");
        let shared = compare(
            &native_stable,
            &converted_stable,
            &["--variables", "stb:pole(2)"],
        );
        assert!(shared.status.success(), "{format}: {shared:?}");
    }
}

#[test]
fn stability_comparison_preserves_unknown_evidence_completion_and_pole_values() {
    let directory = common::test_dir("stb_evidence_variants");
    let stable = run(&directory, "stable", "1", "json", "json");
    let original = common::read_json(&stable);
    let altered = directory.join("altered.json");
    for variant in [
        "unknown",
        "unsupported",
        "numerical",
        "limited",
        "incomplete",
        "moved",
        "reordered",
    ] {
        let mut document = original.clone();
        let payload = &mut document["payload"];
        match variant {
            "unknown" => payload["circuitPoles"] = json!({"status":"not_computed"}),
            "unsupported" => {
                payload["circuitPoles"] = json!({"status":"unavailable", "cause":{"kind":"unsupported", "capability":"behavioral", "detail":"not supported"}})
            }
            "numerical" => {
                payload["circuitPoles"] = json!({"status":"unavailable", "cause":{"kind":"numerical", "detail":"no spectrum"}})
            }
            "limited" => {
                payload["circuitPoles"] = json!({"status":"unavailable", "cause":{"kind":"resource_limit", "resource":"matrix", "requested":7, "limit":6}})
            }
            "incomplete" => payload["success"] = json!(false),
            "moved" => payload["circuitPoles"]["spectrum"]["poles"][0][0] = json!(-2.0),
            "reordered" => payload["circuitPoles"]["spectrum"]["poles"]
                .as_array_mut()
                .unwrap()
                .reverse(),
            _ => unreachable!(),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        let result = compare(&altered, &stable, &[]);
        assert_eq!(
            result.status.code(),
            Some(if variant == "reordered" { 0 } else { 3 }),
            "{variant}: {result:?}"
        );
        // Explicit absence is a retained determination, unlike a missing curve.
        let same = compare(&altered, &altered, &[]);
        assert_eq!(
            same.status.code(),
            Some(if variant == "incomplete" { 3 } else { 0 }),
            "{variant}: {same:?}"
        );
    }
    let mut collision = original;
    collision["scalars"].as_array_mut().unwrap().push(json!({"name":"stb:finite_poles", "displayName":"collision", "unit":{"unit":"dimensionless"}, "value":{"representation":"real", "value":2.0}}));
    std::fs::write(&altered, collision.to_string()).unwrap();
    let destination = directory.join("protected.csv");
    std::fs::write(&destination, "previous").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&altered)
        .arg(&destination)
        .args(["--to", "csv"])
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous");
}

#[test]
fn stability_projection_obeys_expansion_limits_before_replacing_a_destination() {
    let directory = common::test_dir("stb_projection_limits");
    let source = run(&directory, "stable", "1", "json", "json");
    let destination = directory.join("protected.csv");
    let config = directory.join("limits.toml");
    // The retained JSON and base table fit; the complete pole columns expand
    // to 1,830 numeric cells and must be admitted before their allocation.
    for limit in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=1700\n")).unwrap();
        std::fs::write(&destination, "previous").unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--config"])
            .arg(&config)
            .arg("convert")
            .arg(&source)
            .arg(&destination)
            .args(["--to", "csv"])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(75), "{result:?}");
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
    }
}

#[test]
fn malformed_stability_indicators_cannot_pass_or_be_blessed_with_loose_tolerances() {
    let directory = common::test_dir("stb_indicator_integrity");
    let source = run(&directory, "stable", "1", "json", "json");
    let table_path = directory.join("table.json");
    convert(&source, &table_path, "json");
    let original = common::read_json(&table_path);
    for case in [
        "false",
        "near_true",
        "gap",
        "wrong_unit",
        "unknown",
        "contradictory",
    ] {
        let mut table = original.clone();
        let columns = table["signals"].as_array_mut().unwrap();
        let marker = columns
            .iter_mut()
            .find(|column| column["name"] == "stb:circuit_stability(stable)")
            .unwrap();
        match case {
            "false" => marker["values"][0] = json!(0),
            "near_true" => marker["values"][0] = json!(1.000001),
            "gap" => marker["values"][0] = json!(null),
            "wrong_unit" => marker["unit"] = json!("V"),
            "unknown" => marker["name"] = json!("stb:circuit_stability(unknown)"),
            "contradictory" => {
                let mut another = marker.clone();
                another["name"] = json!("stb:circuit_stability(unstable)");
                columns.push(another);
            }
            _ => unreachable!(),
        }
        std::fs::write(&table_path, table.to_string()).unwrap();
        let result = compare(
            &table_path,
            &table_path,
            &[
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
                "--variables",
                "L(jw)",
            ],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stdout)
                .contains("invalid or conflicting STB evidence indicator")
        );
        let golden = directory.join(format!("{case}-golden.json"));
        let result = compare(
            &table_path,
            &golden,
            &["--bless", "--abstol", "1e100", "--reltol", "1e100"],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert!(!golden.exists());
    }
}
