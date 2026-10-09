//! Nominal output agreement must not hide changed parameter derivatives.
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

fn run(directory: &Path, name: &str, circuit: &str) -> PathBuf {
    let deck = directory.join(format!("{name}.sp"));
    let path = directory.join(format!("{name}.json"));
    std::fs::write(&deck, circuit).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(&path)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    path
}

fn source(directory: &Path, name: &str, ac: bool, changed: bool) -> PathBuf {
    let (r, c) = if changed {
        ("2k", "500n")
    } else {
        ("1k", "1u")
    };
    let sweep = if ac { " AC LIN 3 100 1000" } else { "" };
    run(
        directory,
        name,
        &format!(
            "* divider\nV1 in 0 DC 1 AC 1\nR1 in out {r}\nR2 out 0 {r}\nC1 out 0 {c}\n.sens V(out) R1{sweep}\n.end\n"
        ),
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
        .unwrap_or_else(|| panic!("missing {name}: {table}"))
}

#[test]
fn dc_and_ac_sensitivity_derivatives_survive_all_conversion_formats() {
    let directory = common::test_dir("sensitivity_conversion");
    for ac in [false, true] {
        let source = source(&directory, "source", ac, false);
        let changed = self::source(&directory, "changed", ac, true);
        let before = common::read_json(&source);
        let derivative = "dV(OUT)/d(R1)";
        let selected = compare(&changed, &source, &["--variables", derivative]);
        assert_eq!(selected.status.code(), Some(3), "{selected:?}");
        let nominal = if ac { "Nominal output" } else { "output_value" };
        let same_output = compare(&changed, &source, &["--variables", nominal]);
        assert!(same_output.status.success(), "{same_output:?}");
        for (format, extension) in FORMATS {
            let converted = directory.join(format!("converted.{extension}"));
            convert(&source, &converted, format);
            let equal = compare(&converted, &source, &[]);
            assert!(equal.status.success(), "{ac}/{format}: {equal:?}");
            let decoded = directory.join("decoded.json");
            convert(&converted, &decoded, "json");
            let table = common::read_json(&decoded);
            let absolute = column(&table, derivative);
            assert!(absolute["unit"].is_null());
            assert_eq!(column(&table, "normalized(dV(OUT)/d(R1))")["unit"], "1");
            assert!(column(&table, "nominal(R1)")["unit"].is_null());
            let entry = &before["payload"][if ac { "acEntries" } else { "entries" }][0];
            if ac {
                let real: Vec<_> = entry["absolute"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value["real"].clone())
                    .collect();
                let imag: Vec<_> = entry["absolute"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value["imaginary"].clone())
                    .collect();
                assert_eq!(absolute["real"], json!(real));
                assert_eq!(absolute["imag"], json!(imag));
                assert_eq!(
                    column(&table, "d|V(OUT)|/d(R1)")["values"],
                    entry["magnitude"]
                );
                assert_eq!(
                    column(&table, "darg(V(OUT))/d(R1)")["values"],
                    entry["phase"]
                );
            } else {
                assert_eq!(absolute["values"], json!([entry["absolute"]]));
                assert_eq!(
                    column(&table, "normalized(dV(OUT)/d(R1))")["values"],
                    json!([entry["normalized"]])
                );
            }
            assert_eq!(
                column(&table, "sens:parameter(R1,resistor,R1,R)")["values"][0],
                1.0
            );
        }
    }
}

#[test]
fn zero_output_sensitivity_determinations_round_trip_and_compare_as_known_absence() {
    let directory = common::test_dir("sensitivity_zero_conversion");
    for ac in [false, true] {
        let sweep = if ac { " AC LIN 3 1 3" } else { "" };
        let source = run(
            &directory,
            "zero",
            &format!("Zero output\nV1 out 0 DC 0 AC 0\nR1 out 0 1\n.sens V(out) V1{sweep}\n.end\n"),
        );
        let name = "normalized(dV(OUT)/d(V1))";
        for (format, extension) in FORMATS {
            let converted = directory.join(format!("converted.{extension}"));
            convert(&source, &converted, format);
            let equal = compare(&converted, &source, &[]);
            assert!(equal.status.success(), "{ac}/{format}: {equal:?}");
            let selected = compare(&converted, &source, &["--variables", name]);
            assert!(selected.status.success(), "{ac}/{format}: {selected:?}");
            let decoded = directory.join("decoded.json");
            convert(&converted, &decoded, "json");
            let table = common::read_json(&decoded);
            let values = column(&table, name);
            let values = values.get("values").or_else(|| values.get("real")).unwrap();
            assert!(values.as_array().unwrap().iter().all(Value::is_null));
            let status = column(&table, &format!("{name}:sensitivity_status"));
            assert!(
                status["values"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|value| value == &json!(1.0))
            );
        }
    }
}

fn status_table(frequency: &[f64], values: &[Option<f64>], status: &[f64]) -> Value {
    json!({"analysis":"sens_ac", "scale":{"name":"frequency", "type":"frequency", "unit":"Hz", "values":frequency}, "signals":[
        {"name":"derived", "type":"parameter", "values":values},
        {"name":"derived:sensitivity_status", "type":"parameter", "unit":"1", "values":status}
    ]})
}

#[test]
fn sensitivity_reasons_are_exact_per_sample_and_are_not_invented_by_interpolation() {
    let directory = common::test_dir("sensitivity_status_comparison");
    let source = directory.join("source.json");
    let golden = directory.join("golden.json");
    let baseline = status_table(&[1.0, 2.0, 3.0], &[Some(1.0), None, None], &[0.0, 1.0, 3.0]);
    let changed = status_table(&[1.0, 2.0, 3.0], &[Some(1.0), None, None], &[0.0, 3.0, 1.0]);
    std::fs::write(&source, changed.to_string()).unwrap();
    std::fs::write(&golden, baseline.to_string()).unwrap();
    for selection in [
        vec![],
        vec!["--variables", "derived"],
        vec!["--variables", "derived:sensitivity_status"],
    ] {
        let mut flags = vec!["--abstol", "1e100", "--reltol", "1e100"];
        flags.extend(selection);
        let different = compare(&source, &golden, &flags);
        assert_eq!(different.status.code(), Some(3), "{different:?}");
    }
    let mut unknown = baseline.clone();
    unknown["signals"].as_array_mut().unwrap().pop();
    std::fs::write(&source, unknown.to_string()).unwrap();
    let lost = compare(&source, &golden, &["--variables", "derived"]);
    assert_eq!(lost.status.code(), Some(3), "{lost:?}");
    std::fs::write(&source, baseline.to_string()).unwrap();
    std::fs::write(
        &golden,
        status_table(&[1.0, 3.0], &[Some(1.0), None], &[0.0, 3.0]).to_string(),
    )
    .unwrap();
    let retained = compare(
        &source,
        &golden,
        &["--interpolate", "--variables", "derived"],
    );
    assert!(retained.status.success(), "{retained:?}");
    for absent in [false, true] {
        let left = if absent {
            status_table(&[1.0, 3.0], &[None, None], &[1.0, 1.0])
        } else {
            status_table(&[1.0, 3.0], &[Some(1.0), Some(3.0)], &[0.0, 0.0])
        };
        let right = if absent {
            status_table(&[1.0, 2.0, 3.0], &[None, None, None], &[1.0, 1.0, 1.0])
        } else {
            status_table(
                &[1.0, 2.0, 3.0],
                &[Some(1.0), Some(2.0), Some(3.0)],
                &[0.0, 0.0, 0.0],
            )
        };
        std::fs::write(&source, left.to_string()).unwrap();
        std::fs::write(&golden, right.to_string()).unwrap();
        let result = compare(
            &source,
            &golden,
            &["--interpolate", "--variables", "derived"],
        );
        assert_eq!(
            result.status.code(),
            Some(if absent { 3 } else { 0 }),
            "{result:?}"
        );
    }
}

#[test]
fn sensitivity_identity_indicators_are_checked_even_when_a_derivative_is_selected() {
    let directory = common::test_dir("sensitivity_bad_identity");
    let source = source(&directory, "source", false, false);
    let flat = directory.join("flat.json");
    convert(&source, &flat, "json");
    let original = common::read_json(&flat);
    for case in [
        "false",
        "unit",
        "unknown_kind",
        "malformed_escape",
        "duplicate",
        "complex",
    ] {
        let mut table = original.clone();
        let columns = table["signals"].as_array_mut().unwrap();
        let marker = columns
            .iter_mut()
            .find(|column| column["name"] == "sens:parameter(R1,resistor,R1,R)")
            .unwrap();
        match case {
            "false" => marker["values"][0] = json!(0),
            "unit" => marker["unit"] = json!("V"),
            "unknown_kind" => marker["name"] = json!("sens:parameter(R1,invalid,R1,R)"),
            "malformed_escape" => marker["name"] = json!("sens:parameter(R1,resistor,R1,%Q0)"),
            "duplicate" => {
                let mut another = marker.clone();
                another["name"] = json!("sens:parameter(R1,capacitor,C1,C)");
                columns.push(another);
            }
            "complex" => {
                marker.as_object_mut().unwrap().remove("values");
                marker["real"] = json!([1]);
                marker["imag"] = json!([0]);
            }
            _ => panic!("case"),
        }
        std::fs::write(&flat, table.to_string()).unwrap();
        let result = compare(
            &flat,
            &flat,
            &[
                "--variables",
                "dV(OUT)/d(R1)",
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
            ],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        let golden = directory.join(format!("{case}-golden.json"));
        let result = compare(&flat, &golden, &["--bless"]);
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert!(!golden.exists());
    }
}

#[test]
fn malformed_sensitivity_status_cannot_pass_or_replace_a_baseline() {
    let directory = common::test_dir("sensitivity_bad_status");
    let source = directory.join("source.json");
    let original = status_table(&[1.0, 2.0], &[Some(1.0), None], &[0.0, 1.0]);
    for case in [
        "near_integer",
        "unknown",
        "false_present",
        "false_missing",
        "unit",
        "gap",
        "complex",
        "orphan",
    ] {
        let mut table = original.clone();
        let status = &mut table["signals"][1];
        match case {
            "near_integer" => status["values"][1] = json!(1.000001),
            "unknown" => status["values"][1] = json!(4),
            "false_present" => status["values"][1] = json!(0),
            "false_missing" => status["values"][0] = json!(1),
            "unit" => status["unit"] = json!("V"),
            "gap" => status["values"][1] = Value::Null,
            "complex" => {
                status.as_object_mut().unwrap().remove("values");
                status["real"] = json!([0, 1]);
                status["imag"] = json!([0, 0]);
            }
            "orphan" => status["name"] = json!("other:sensitivity_status"),
            _ => panic!("case"),
        }
        std::fs::write(&source, table.to_string()).unwrap();
        let bad = compare(
            &source,
            &source,
            &[
                "--variables",
                "derived",
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
            ],
        );
        assert_eq!(bad.status.code(), Some(3), "{case}: {bad:?}");
        let golden = directory.join(format!("{case}-golden.json"));
        std::fs::write(&golden, original.to_string()).unwrap();
        let bad = compare(
            &source,
            &golden,
            &["--bless", "--abstol", "1e100", "--reltol", "1e100"],
        );
        assert_eq!(bad.status.code(), Some(3), "{case}: {bad:?}");
        assert_eq!(common::read_json(&golden), original);
    }
}

#[test]
fn sensitivity_status_cannot_round_an_interpolated_gap_to_available() {
    let directory = common::test_dir("sensitivity_status_extreme_interpolation");
    let source = directory.join("source.json");
    let golden = directory.join("golden.json");
    std::fs::write(
        &source,
        status_table(&[0.0, 1e300], &[Some(1.0), None], &[0.0, 1.0]).to_string(),
    )
    .unwrap();
    std::fs::write(
        &golden,
        status_table(&[1e-300, 1e300], &[Some(1.0), None], &[0.0, 1.0]).to_string(),
    )
    .unwrap();
    let result = compare(
        &source,
        &golden,
        &["--interpolate", "--variables", "derived:sensitivity_status"],
    );
    assert_eq!(result.status.code(), Some(3), "{result:?}");
}

#[test]
fn sensitivity_projection_checks_identities_limits_and_name_collisions() {
    let directory = common::test_dir("sensitivity_projection_admission");
    let source = source(&directory, "source", false, false);
    let original = common::read_json(&source);
    let altered = directory.join("altered.json");
    let destination = directory.join("protected.csv");
    let config = directory.join("limits.toml");
    for limit in ["max_result_values", "max_external_data_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=6\n")).unwrap();
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
    let mut changed = original.clone();
    changed["payload"]["entries"][0]["parameter"] = json!("Other,parameter");
    std::fs::write(&altered, changed.to_string()).unwrap();
    let different = compare(
        &altered,
        &source,
        &["--abstol", "1e100", "--reltol", "1e100"],
    );
    assert_eq!(different.status.code(), Some(3), "{different:?}");
    let mut collision = original;
    collision["scalars"].as_array_mut().unwrap().push(json!({"name":"nominal(R1)", "displayName":"collision", "unit":{"unit":"dimensionless"}, "value":{"representation":"real", "value":0.0}}));
    std::fs::write(&altered, collision.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&altered)
        .arg(&destination)
        .args(["--to", "csv"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
}
