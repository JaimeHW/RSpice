//! Equal total spread must not hide a change in its statistical contributors.
mod common;

use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

const FORMATS: [(&str, &str); 6] = [
    ("json", "json"),
    ("csv", "csv"),
    ("tsv", "tsv"),
    ("raw", "raw"),
    ("ascii", "ascii.raw"),
    ("hdf5", "h5"),
];

fn run(
    dir: &Path,
    name: &str,
    changed: bool,
    card: &str,
    expression: &str,
    format: &str,
    extension: &str,
) -> PathBuf {
    let (s1, s2) = if changed { (5, 20) } else { (10, 10) };
    std::fs::write(dir.join(format!("{name}.scs")), format!("parameters p=1000 q=2000\nstatistics {{\n mismatch {{\n vary p dist=gauss std={s1}\n vary q dist=gauss std={s2}\n }}\n}}\n")).unwrap();
    let deck = dir.join(format!("{name}.sp"));
    std::fs::write(&deck, format!("Linear mismatch\n.include \"{name}.scs\"\nB1 out 0 V={{{expression}}}\nR1 out 0 1k\n.DCMATCH OUT=V(out) {card}\n.end\n")).unwrap();
    let destination = dir.join(format!("{name}.{extension}"));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(&destination)
        .args(["-f", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    destination
}

fn source(dir: &Path, name: &str, changed: bool) -> PathBuf {
    run(
        dir,
        name,
        changed,
        "CONTRIBUTORS=0 SIGMA=3",
        "2*p+q",
        "json",
        "json",
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

fn conversion(source: &Path, destination: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", format])
        .output()
        .unwrap()
}

fn convert(source: &Path, destination: &Path, format: &str) {
    let output = conversion(source, destination, format);
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
fn contributor_changes_cannot_hide_behind_equal_total_spread_in_any_format() {
    let dir = common::test_dir("dcmatch_conversion");
    let source = source(&dir, "source", false);
    let changed = self::source(&dir, "changed", true);
    let original = common::read_json(&source);
    let aggregate = compare(
        &changed,
        &source,
        &[
            "--variables",
            "nominal_value",
            "--variables",
            "sigma_total",
            "--variables",
            "sigma_mismatch",
            "--variables",
            "sigma_process",
            "--variables",
            "quoted_sigma",
        ],
    );
    assert!(aggregate.status.success(), "{aggregate:?}");
    let different = compare(&changed, &source, &[]);
    assert_eq!(different.status.code(), Some(3), "{different:?}");
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("converted.{extension}"));
        convert(&source, &destination, format);
        let equal = compare(&destination, &source, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
        let different = compare(&changed, &destination, &[]);
        assert_eq!(different.status.code(), Some(3), "{format}: {different:?}");
        let decoded = dir.join("decoded.json");
        convert(&destination, &decoded, "json");
        let table = common::read_json(&decoded);
        for contributor in original["payload"]["contributors"].as_array().unwrap() {
            let owner = format!(
                "{}:{}/{}",
                contributor["scope"].as_str().unwrap(),
                contributor["instance"].as_str().unwrap(),
                contributor["parameter"].as_str().unwrap()
            );
            for (name, field, unit) in [
                ("contribution", "contribution", Some("V")),
                ("share", "share", Some("1")),
                ("sigma_parameter", "sigmaParameter", None),
                ("sensitivity", "sensitivity", None),
            ] {
                let metric = column(&table, &format!("{name}({owner})"));
                assert_eq!(metric["values"], json!([contributor[field]]));
                assert_eq!(metric["unit"].as_str(), unit);
            }
        }
        assert_eq!(column(&table, "sigma_multiplier")["values"], json!([3.0]));
        assert_eq!(
            column(&table, "retained_contributors")["values"],
            json!([4.0])
        );
        assert_eq!(
            column(&table, "evaluated_contributors")["values"],
            json!([4.0])
        );
        assert_eq!(
            column(&table, "applied_correlations_mismatch")["values"],
            json!([0.0])
        );
    }
}

#[test]
fn mismatch_projection_preserves_zero_spread_trimmed_reports_and_scope_identity() {
    let dir = common::test_dir("dcmatch_projection_context");
    let full = source(&dir, "full", false);
    let trimmed = run(
        &dir,
        "trimmed",
        false,
        "CONTRIBUTORS=1 SIGMA=3",
        "2*p+q",
        "json",
        "json",
    );
    for (result, golden) in [(&full, &trimmed), (&trimmed, &full)] {
        let different = compare(result, golden, &[]);
        assert_eq!(different.status.code(), Some(3), "{different:?}");
    }
    let zero = run(
        &dir,
        "zero",
        false,
        "CONTRIBUTORS=0 SIGMA=3",
        "4",
        "json",
        "json",
    );
    let changed = run(
        &dir,
        "zero-changed",
        false,
        "CONTRIBUTORS=0 SIGMA=5",
        "4",
        "json",
        "json",
    );
    let different = compare(&changed, &zero, &[]);
    assert_eq!(different.status.code(), Some(3), "{different:?}");
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("zero.{extension}"));
        if destination != zero {
            convert(&zero, &destination, format);
        }
        let equal = compare(&destination, &zero, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
    }
    let original = common::read_json(&full);
    let mut document = original.clone();
    document["payload"]["contributors"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let altered = dir.join("altered.json");
    std::fs::write(&altered, document.to_string()).unwrap();
    let reordered = compare(&altered, &full, &[]);
    assert!(reordered.status.success(), "{reordered:?}");
    for field in ["output", "instance", "parameter", "scope"] {
        let mut document = original.clone();
        match field {
            "output" => document["payload"][field] = json!("V(other)"),
            "scope" => document["payload"]["contributors"][0][field] = json!("process"),
            _ => document["payload"]["contributors"][0][field] = json!("other"),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        let different = compare(&altered, &full, &["--abstol", "1e100", "--reltol", "1e100"]);
        assert_eq!(different.status.code(), Some(3), "{field}: {different:?}");
    }
}

#[test]
fn mismatch_projection_encodes_tuple_delimiters_without_aliasing_owners() {
    let dir = common::test_dir("dcmatch_projection_names");
    let source = source(&dir, "source", false);
    let mut document = common::read_json(&source);
    document["payload"]["contributors"][0]["instance"] = json!("a/b");
    document["payload"]["contributors"][0]["parameter"] = json!("c");
    document["payload"]["contributors"][1]["instance"] = json!("a");
    document["payload"]["contributors"][1]["parameter"] = json!("b/c");
    std::fs::write(&source, document.to_string()).unwrap();
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("escaped.{extension}"));
        convert(&source, &destination, format);
        let equal = compare(&destination, &source, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
        let decoded = dir.join("decoded.json");
        convert(&destination, &decoded, "json");
        let table = common::read_json(&decoded);
        column(&table, "contribution(mismatch:a%2Fb/c)");
        column(&table, "contribution(mismatch:a/b%2Fc)");
    }
}

#[test]
fn mismatch_projection_limits_precision_and_name_collisions_preserve_output() {
    let dir = common::test_dir("dcmatch_projection_admission");
    let source = source(&dir, "source", false);
    let original = common::read_json(&source);
    let destination = dir.join("protected.csv");
    let config = dir.join("limits.toml");
    for limit in ["max_result_values", "max_external_data_values"] {
        // Allow the compact report; deny the expanded identities and columns.
        std::fs::write(&config, format!("[resources]\n{limit}=30\n")).unwrap();
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
    for case in ["count", "collision"] {
        let mut document = original.clone();
        if case == "count" {
            document["payload"]["evaluatedContributors"] = json!(u64::MAX);
        } else {
            document["scalars"].as_array_mut().unwrap().push(json!({"name":"sigma_multiplier", "displayName":"collision", "unit":{"unit":"dimensionless"}, "value":{"representation":"real", "value":3.0}}));
        }
        std::fs::write(&source, document.to_string()).unwrap();
        let output = conversion(&source, &destination, "csv");
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
    }
}

#[test]
fn malformed_mismatch_identity_cannot_pass_comparison_or_replace_a_baseline() {
    let dir = common::test_dir("dcmatch_identity_validation");
    let source = source(&dir, "source", false);
    let flat = dir.join("flat.json");
    convert(&source, &flat, "json");
    let original = common::read_json(&flat);
    for case in [
        "false",
        "gap",
        "unit",
        "scope",
        "escape",
        "empty",
        "complex",
        "duplicate",
    ] {
        let mut table = original.clone();
        let columns = table["signals"].as_array_mut().unwrap();
        let marker = columns
            .iter_mut()
            .find(|column| column["name"] == "dcmatch:contributor(mismatch,B1,P)")
            .unwrap();
        match case {
            "false" => marker["values"][0] = json!(0.0),
            "gap" => marker["values"][0] = Value::Null,
            "unit" => marker["unit"] = json!("V"),
            "scope" => marker["name"] = json!("dcmatch:contributor(unknown,B1,P)"),
            "escape" => marker["name"] = json!("dcmatch:contributor(mismatch,%ZZ,P)"),
            "empty" => marker["name"] = json!("dcmatch:contributor(mismatch,B1,)"),
            "complex" => {
                marker.as_object_mut().unwrap().remove("values");
                marker["real"] = json!([1.0]);
                marker["imag"] = json!([0.0]);
            }
            "duplicate" => {
                let mut second = marker.clone();
                second["name"] = json!("dcmatch:contributor(mismatch,%42%31,P)");
                columns.push(second);
            }
            _ => panic!("case"),
        }
        std::fs::write(&flat, table.to_string()).unwrap();
        let bad = compare(
            &flat,
            &flat,
            &[
                "--variables",
                "sigma_total",
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
            ],
        );
        assert_eq!(bad.status.code(), Some(3), "{case}: {bad:?}");
        let golden = dir.join("golden.json");
        std::fs::write(&golden, original.to_string()).unwrap();
        let bad = compare(&flat, &golden, &["--bless"]);
        assert_eq!(bad.status.code(), Some(3), "{case}: {bad:?}");
        assert_eq!(common::read_json(&golden), original);
    }
}
