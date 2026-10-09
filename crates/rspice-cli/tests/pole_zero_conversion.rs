//! A root set alone does not describe the retained transfer or its qualification.
mod common;

use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const FORMATS: [(&str, &str); 6] = [
    ("json", "json"),
    ("csv", "csv"),
    ("tsv", "tsv"),
    ("raw", "raw"),
    ("ascii", "ascii.raw"),
    ("hdf5", "h5"),
];
const RC: &str =
    "* current RC\nR1 out 0 RESISTANCE\nC1 out 0 CAPACITANCE\n.pz out 0 out 0 cur pz\n.end\n";

fn run(directory: &Path, name: &str, circuit: &str) -> PathBuf {
    run_format(directory, name, circuit, "json", "json")
}

fn run_format(
    directory: &Path,
    name: &str,
    circuit: &str,
    format: &str,
    extension: &str,
) -> PathBuf {
    let deck = directory.join(format!("{name}.sp"));
    let path = directory.join(format!("{name}.{extension}"));
    std::fs::write(&deck, circuit).unwrap();
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

#[test]
fn direct_pole_zero_exports_use_the_same_complete_projection_as_conversion() {
    let directory = common::test_dir("pz_direct_projection");
    let source = original(&directory);
    let baseline = RC.replace("RESISTANCE", "1k").replace("CAPACITANCE", "1u");
    let altered = RC
        .replace("RESISTANCE", "2k")
        .replace("CAPACITANCE", "500n");
    for (format, extension) in FORMATS {
        let native = run_format(&directory, "native", &baseline, format, extension);
        let changed = run_format(&directory, "changed", &altered, format, extension);
        let converted = directory.join(format!("converted.{extension}"));
        convert(&source, &converted, format);
        let same = compare(&native, &converted, &[]);
        assert!(same.status.success(), "{format}: {same:?}");
        let different = compare(&changed, &native, &[]);
        assert_eq!(different.status.code(), Some(3), "{format}: {different:?}");
        assert!(String::from_utf8_lossy(&different.stdout).contains("dc_gain"));
    }
}

#[test]
fn empty_root_sets_retain_static_gains_and_integrators_retain_missing_dc_gain() {
    let directory = common::test_dir("pz_static_and_integrator");
    for (format, extension) in FORMATS {
        let source = run_format(
            &directory,
            "static",
            "* resistor\nR1 out 0 1k\n.pz out 0 out 0 cur pz\n.end\n",
            format,
            extension,
        );
        let changed = run_format(
            &directory,
            "changed",
            "* resistor\nR1 out 0 2k\n.pz out 0 out 0 cur pz\n.end\n",
            format,
            extension,
        );
        let same = compare(&source, &source, &[]);
        assert!(same.status.success(), "{format}: {same:?}");
        let different = compare(&changed, &source, &[]);
        assert_eq!(different.status.code(), Some(3), "{format}: {different:?}");
        let integrator = run_format(
            &directory,
            "integrator",
            "* capacitor\nC1 out 0 1u\n.pz out 0 out 0 cur pz\n.end\n",
            format,
            extension,
        );
        let decoded = directory.join("decoded.json");
        convert(&integrator, &decoded, "json");
        let table = common::read_json(&decoded);
        assert_eq!(column(&table, "dc_gain")["values"], json!([null]));
        assert_eq!(
            column(&table, "high_frequency_gain")["values"],
            json!([0.0])
        );
        let unknown = compare(&integrator, &integrator, &[]);
        assert_eq!(unknown.status.code(), Some(3), "{format}: {unknown:?}");
        let root = compare(&integrator, &integrator, &["--variables", "pole(1)"]);
        assert!(root.status.success(), "{format}: {root:?}");
    }
}

#[test]
fn direct_pole_zero_exports_admit_the_complete_table_before_replacing_output() {
    let directory = common::test_dir("pz_native_admission");
    let source = original(&directory);
    let deck = directory.join("original.sp");
    let config = directory.join("limits.toml");
    let destination = directory.join("protected.csv");
    // The result-value budget also covers extraction workspace. Use the
    // independent external-data budget to isolate flat export admission.
    std::fs::write(&config, "[resources]\nmax_external_data_values=16\n").unwrap();
    std::fs::write(&destination, "previous").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--config"])
        .arg(&config)
        .arg("run")
        .arg(&deck)
        .arg("-o")
        .arg(&destination)
        .args(["-f", "csv"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
    // JSON retains the compact payload and does not pay the table expansion.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--config"])
        .arg(&config)
        .arg("run")
        .arg(&deck)
        .arg("-o")
        .arg(&source)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn pole_zero_step_manifests_describe_root_gain_and_evidence_units() {
    let directory = common::test_dir("pz_flat_manifest");
    run_format(
        &directory,
        "sweep",
        "* stepped RC\n.param r=1000\nR1 out 0 {r}\nC1 out 0 1u\n.step param r list 1000 2000\n.pz out 0 out 0 cur pz\n.end\n",
        "csv",
        "csv",
    );
    let manifest = common::read_json(&directory.join("sweep.step_schema.json"));
    let schema = manifest["analyses"][0]["union_schema"].as_array().unwrap();
    for (name, unit, value_type) in [
        ("pole(1)", "radian_per_second", "complex"),
        ("dc_gain", "ohm", "real"),
        ("high_frequency_gain", "ohm", "real"),
        ("pz:poles_evidence(qualified)", "dimensionless", "real"),
    ] {
        let descriptor = schema
            .iter()
            .find(|descriptor| descriptor["display_name"] == name)
            .unwrap();
        assert_eq!(descriptor["unit"], unit, "{name}: {descriptor}");
        assert_eq!(descriptor["value_type"], value_type, "{name}: {descriptor}");
    }
    assert_eq!(
        manifest["analyses"][0]["coordinates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

fn original(directory: &Path) -> PathBuf {
    run(
        directory,
        "original",
        &RC.replace("RESISTANCE", "1k").replace("CAPACITANCE", "1u"),
    )
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

fn column<'a>(table: &'a Value, name: &str) -> &'a Value {
    table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|column| column["name"] == name)
        .unwrap()
}

#[test]
fn pole_zero_gains_and_evidence_survive_every_conversion_format() {
    let directory = common::test_dir("pz_complete_conversion");
    let source = original(&directory);
    let changed = run(
        &directory,
        "changed",
        &RC.replace("RESISTANCE", "2k")
            .replace("CAPACITANCE", "500n"),
    );
    let before = common::read_json(&source);
    let after = common::read_json(&changed);
    assert_eq!(before["payload"]["dcGain"], 1000.0);
    assert_eq!(after["payload"]["dcGain"], 2000.0);
    let left = before["payload"]["poles"][0]["real"].as_f64().unwrap();
    let right = after["payload"]["poles"][0]["real"].as_f64().unwrap();
    assert!((left - right).abs() < 1e-9);
    assert_eq!(compare(&changed, &source, &[]).status.code(), Some(3));
    for (format, extension) in FORMATS {
        let baseline = directory.join(format!("baseline.{extension}"));
        let modified = directory.join(format!("modified.{extension}"));
        convert(&source, &baseline, format);
        convert(&changed, &modified, format);
        let same = compare(&baseline, &source, &[]);
        assert!(same.status.success(), "{format}: {same:?}");
        let different = compare(&modified, &baseline, &[]);
        assert_eq!(different.status.code(), Some(3), "{format}: {different:?}");
        assert!(String::from_utf8_lossy(&different.stdout).contains("dc_gain"));
        let decoded = directory.join("decoded.json");
        convert(&baseline, &decoded, "json");
        let table = common::read_json(&decoded);
        assert_eq!(column(&table, "dc_gain")["values"], json!([1000.0]));
        assert_eq!(column(&table, "dc_gain")["unit"], "ohm");
        assert_eq!(
            column(&table, "high_frequency_gain")["values"],
            json!([0.0])
        );
        assert_eq!(column(&table, "pole(1)")["unit"], "rad/s");
        assert_eq!(column(&table, "pz:input(I(OUT,0))")["values"], json!([1.0]));
        assert_eq!(
            column(&table, "pz:output(V(OUT,0))")["values"],
            json!([1.0])
        );
        assert_eq!(
            column(&table, "pz:poles_evidence(qualified)")["values"],
            json!([1.0])
        );
        assert_eq!(
            column(&table, "pz:zeros_evidence(qualified_empty)")["values"],
            json!([1.0])
        );
        assert_eq!(column(&table, "pz:infinite_zeros")["values"], json!([2.0]));
    }
}

#[test]
fn pole_zero_comparison_checks_identity_and_qualification_independently_of_tolerances() {
    let directory = common::test_dir("pz_evidence_comparison");
    let source = original(&directory);
    let original = common::read_json(&source);
    let altered = directory.join("altered.json");
    for case in [
        "input",
        "output",
        "unknown",
        "approximate",
        "not_requested",
        "stability",
    ] {
        let mut document = original.clone();
        let payload = &mut document["payload"];
        match case {
            "input" => payload["input"] = json!("I(other,0)"),
            "output" => payload["output"] = json!("V(other,0)"),
            "unknown" => payload["poleEvidence"] = json!({"evidence":"legacy-unknown"}),
            "not_requested" => payload["zeroEvidence"] = json!({"evidence":"not-requested"}),
            "approximate" => {
                payload["poleEvidence"]["evidence"] = json!("approximate");
                let tolerance = payload["poleEvidence"]["certificate"]["qualificationTolerance"]
                    .as_f64()
                    .unwrap();
                payload["poleEvidence"]["certificate"]["maxBackwardError"] = json!(tolerance * 2.0);
            }
            "stability" => {
                payload["poleEvidence"]["certificate"]["asymptoticallyStable"] = json!(false)
            }
            _ => panic!("unknown case"),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        let same = compare(&altered, &altered, &[]);
        assert!(same.status.success(), "{case}: {same:?}");
        let different = compare(
            &altered,
            &source,
            &["--abstol", "1e100", "--reltol", "1e100"],
        );
        assert_eq!(different.status.code(), Some(3), "{case}: {different:?}");
    }
    let pair = run(
        &directory,
        "two-poles",
        "* two poles\nR1 a 0 1k\nR2 a b 2k\nC1 a 0 1u\nC2 b 0 1u\n.pz a 0 b 0 cur pz\n.end\n",
    );
    let mut document = common::read_json(&pair);
    let poles = document["payload"]["poles"].as_array_mut().unwrap();
    assert_eq!(poles.len(), 2);
    poles.reverse();
    std::fs::write(&altered, document.to_string()).unwrap();
    let reordered = compare(&altered, &pair, &[]);
    assert!(reordered.status.success(), "{reordered:?}");
}

#[test]
fn pole_zero_missing_gains_and_legacy_units_are_not_fabricated() {
    let directory = common::test_dir("pz_missing_gains");
    let source = original(&directory);
    let mut document = common::read_json(&source);
    document["schemaVersion"] = json!(11);
    document["payload"]["dcGain"] = Value::Null;
    document["payload"]
        .as_object_mut()
        .unwrap()
        .remove("rootUnit");
    document["payload"]
        .as_object_mut()
        .unwrap()
        .remove("gainUnit");
    let legacy = directory.join("legacy.json");
    std::fs::write(&legacy, document.to_string()).unwrap();
    for (format, extension) in FORMATS {
        let destination = directory.join(format!("converted.{extension}"));
        convert(&legacy, &destination, format);
        let decoded = directory.join("decoded.json");
        convert(&destination, &decoded, "json");
        let table = common::read_json(&decoded);
        assert_eq!(column(&table, "dc_gain")["values"], json!([null]));
        assert!(column(&table, "dc_gain")["unit"].is_null());
        assert!(column(&table, "pole(1)")["unit"].is_null());
        let unknown = compare(&destination, &destination, &[]);
        assert_eq!(unknown.status.code(), Some(3), "{format}: {unknown:?}");
        let roots = compare(&destination, &destination, &["--variables", "pole(1)"]);
        assert!(roots.status.success(), "{format}: {roots:?}");
    }
}

#[test]
fn pole_zero_projection_limits_and_collisions_preserve_existing_destinations() {
    let directory = common::test_dir("pz_projection_admission");
    let source = original(&directory);
    let destination = directory.join("protected.csv");
    let config = directory.join("limits.toml");
    for limit in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=16\n")).unwrap();
        std::fs::write(&destination, "previous").unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--config"])
            .arg(&config)
            .arg("convert")
            .arg(&source)
            .arg(&destination)
            .args(["--to", "csv"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(75), "{limit}: {output:?}");
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
    }
    let mut document = common::read_json(&source);
    document["scalars"].as_array_mut().unwrap().push(json!({"name":"dc_gain", "displayName":"collision", "unit":{"unit":"ohm"}, "value":{"representation":"real", "value":10.0}}));
    std::fs::write(&source, document.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&source)
        .arg(&destination)
        .args(["--to", "csv"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
}

#[test]
fn malformed_pole_zero_evidence_cannot_be_compared_or_blessed() {
    let directory = common::test_dir("pz_bad_indicators");
    let source = original(&directory);
    let flat = directory.join("flat.json");
    convert(&source, &flat, "json");
    let original = common::read_json(&flat);
    for case in [
        "false",
        "near_true",
        "gap",
        "unit",
        "unknown",
        "conflict",
        "empty_input",
        "complex",
        "missing_state",
    ] {
        let mut table = original.clone();
        let columns = table["signals"].as_array_mut().unwrap();
        let name = if case == "empty_input" {
            "pz:input(I(OUT,0))"
        } else {
            "pz:poles_evidence(qualified)"
        };
        let marker = columns
            .iter_mut()
            .find(|column| column["name"] == name)
            .unwrap();
        match case {
            "false" => marker["values"][0] = json!(0),
            "near_true" => marker["values"][0] = json!(1.000001),
            "gap" => marker["values"][0] = Value::Null,
            "unit" => marker["unit"] = json!("V"),
            "unknown" => marker["name"] = json!("pz:poles_evidence(unknown)"),
            "empty_input" => marker["name"] = json!("pz:input()"),
            "missing_state" => marker["name"] = json!("pz:poles_evidence"),
            "complex" => {
                marker.as_object_mut().unwrap().remove("values");
                marker["real"] = json!([1.0]);
                marker["imag"] = json!([0.0]);
            }
            "conflict" => {
                let mut second = marker.clone();
                second["name"] = json!("pz:poles_evidence(legacy_unknown)");
                columns.push(second);
            }
            _ => panic!("unknown case"),
        }
        std::fs::write(&flat, table.to_string()).unwrap();
        let result = compare(
            &flat,
            &flat,
            &[
                "--variables",
                "pole(1)",
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
            ],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stdout)
                .contains("invalid or conflicting PZ evidence indicator")
        );
        let golden = directory.join(format!("{case}-golden.json"));
        let result = compare(
            &flat,
            &golden,
            &["--bless", "--abstol", "1e100", "--reltol", "1e100"],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert!(!golden.exists());
    }
}
