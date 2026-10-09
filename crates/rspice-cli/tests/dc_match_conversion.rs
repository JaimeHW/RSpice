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
            "scope" => {
                // Relabel a zero-displacement contributor so the numerical
                // scope totals remain valid and only the identity changes.
                let entry = document["payload"]["contributors"]
                    .as_array_mut()
                    .unwrap()
                    .last_mut()
                    .unwrap();
                assert_eq!(entry["contribution"], json!(0.0));
                entry[field] = json!("process");
            }
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

#[test]
fn native_mismatch_exports_retain_complete_reports_and_extreme_finite_spreads() {
    let dir = common::test_dir("dcmatch_native_reports");
    for expression in ["2*p+q", "4", "1e-200*p", "1e200*p"] {
        let typed = run(
            &dir,
            "typed",
            false,
            "CONTRIBUTORS=0 SIGMA=3",
            expression,
            "json",
            "json",
        );
        let original = common::read_json(&typed);
        let sigma = original["payload"]["sigmaTotal"].as_f64().unwrap();
        if expression == "1e-200*p" || expression == "1e200*p" {
            let expected = if expression == "1e-200*p" {
                1e-199
            } else {
                1e201
            };
            assert!(
                (sigma / expected - 1.0).abs() < 2e-13,
                "{expression}: {sigma}"
            );
        }
        for (format, extension) in FORMATS {
            let native = run(
                &dir,
                "native",
                false,
                "CONTRIBUTORS=0 SIGMA=3",
                expression,
                format,
                extension,
            );
            let equal = compare(&native, &typed, &[]);
            assert!(equal.status.success(), "{expression}/{format}: {equal:?}");
        }
    }
}

#[test]
fn native_mismatch_projection_checks_its_budget_and_records_manifest_units() {
    let dir = common::test_dir("dcmatch_native_admission");
    let typed = source(&dir, "source", false);
    let deck = dir.join("source.sp");
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_values=30\n").unwrap();
    let destination = dir.join("protected.csv");
    std::fs::write(&destination, "previous").unwrap();
    for (format, destination) in [("csv", &destination), ("json", &typed)] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--config"])
            .arg(&config)
            .arg("run")
            .arg(&deck)
            .arg("-o")
            .arg(destination)
            .args(["-f", format])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(if format == "csv" { 75 } else { 0 }),
            "{format}: {output:?}"
        );
    }
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous");
    run(
        &dir,
        "sweep",
        false,
        "CONTRIBUTORS=0 SIGMA=3\n.STEP PARAM p LIST 1000 2000",
        "2*p+q",
        "csv",
        "csv",
    );
    let manifest = common::read_json(&dir.join("sweep.step_schema.json"));
    let schema = manifest["analyses"][0]["union_schema"].as_array().unwrap();
    for (name, unit) in [
        ("sigma_total", "volt"),
        ("contribution(mismatch:B1/P)", "volt"),
        ("share(mismatch:B1/P)", "dimensionless"),
        ("sigma_parameter(mismatch:B1/P)", "unspecified"),
        ("sensitivity(mismatch:B1/P)", "unspecified"),
        ("retained_contributors", "dimensionless"),
    ] {
        let descriptor = schema
            .iter()
            .find(|descriptor| descriptor["display_name"] == name)
            .unwrap_or_else(|| panic!("missing {name}: {manifest}"));
        assert_eq!(descriptor["unit"], unit, "{name}: {descriptor}");
        assert_eq!(descriptor["value_type"], "real");
    }
    assert_eq!(
        manifest["analyses"][0]["coordinates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn unrepresentable_quoted_mismatch_spread_fails_even_without_an_output_file() {
    let dir = common::test_dir("dcmatch_unrepresentable_quote");
    std::fs::write(dir.join("statistics.scs"), "parameters p=0 q=0\nstatistics {\n mismatch {\n vary p dist=gauss std=1\n vary q dist=gauss std=1\n }\n}\n").unwrap();
    let deck = dir.join("range.sp");
    std::fs::write(&deck, "Large spread\n.include \"statistics.scs\"\nB1 out 0 V={1e308*(p+q)}\nR1 out 0 1k\n.DCMATCH OUT=V(out) SIGMA=3\n.end\n").unwrap();
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("protected.{extension}"));
        std::fs::write(&destination, "previous").unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .arg("-o")
            .arg(&destination)
            .args(["-f", format])
            .output()
            .unwrap();
        assert!(!output.status.success(), "{format}: {output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("quoted sigma"),
            "{format}: {output:?}"
        );
        assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("quoted sigma"),
        "{output:?}"
    );
}

#[test]
fn contradictory_mismatch_documents_cannot_be_exported_compared_or_blessed() {
    let dir = common::test_dir("dcmatch_consistency");
    let source = source(&dir, "source", false);
    let original = common::read_json(&source);
    let altered = dir.join("altered.json");
    for case in [
        "total",
        "nominal",
        "negative-sigma",
        "displacement",
        "false-share",
        "duplicate",
        "unit",
        "quoted",
    ] {
        let mut document = original.clone();
        match case {
            "total" => document["payload"]["sigmaTotal"] = json!(1.0),
            "nominal" => document["payload"]["nominalValue"] = json!(1.0),
            "negative-sigma" => document["payload"]["sigmaMismatch"] = json!(-1.0),
            "displacement" => document["payload"]["contributors"][0]["sensitivity"] = json!(0.0),
            "false-share" => document["payload"]["contributors"][0]["share"] = json!(0.25),
            "duplicate" => {
                let mut entry = document["payload"]["contributors"][0].clone();
                entry["instance"] = json!(entry["instance"].as_str().unwrap().to_ascii_lowercase());
                document["payload"]["contributors"][1] = entry;
            }
            "unit" => document["scalars"][0]["unit"]["unit"] = json!("ampere"),
            "quoted" => document["scalars"][4]["value"]["value"] = json!(0.0),
            _ => panic!("case"),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        for (format, extension) in FORMATS {
            let destination = dir.join(format!("protected.{extension}"));
            std::fs::write(&destination, "previous").unwrap();
            let output = conversion(&altered, &destination, format);
            assert_eq!(output.status.code(), Some(1), "{case}/{format}: {output:?}");
            assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous");
        }
        let selected = compare(&altered, &altered, &["--variables", "sigma_total"]);
        assert_eq!(selected.status.code(), Some(1), "{case}: {selected:?}");
        let blessed = compare(&altered, &source, &["--bless"]);
        assert_eq!(blessed.status.code(), Some(1), "{case}: {blessed:?}");
        assert_eq!(common::read_json(&source), original);
    }
}
