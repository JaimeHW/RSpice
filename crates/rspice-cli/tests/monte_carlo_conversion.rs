//! Trial samples and campaign context must survive every numeric export format.
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

fn source(dir: &Path, name: &str, runs: usize, start: usize, seed: &str) -> PathBuf {
    let deck = dir.join(format!("{name}.sp"));
    std::fs::write(&deck, format!("Monte Carlo divider\nV1 in 0 5\nR1 in out {{rtop}}\nR2 out 0 1k\n.param rtop=1k\n.MC {runs} START {start} SEED {seed}\n.end\n")).unwrap();
    let destination = dir.join(format!("{name}.json"));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(&destination)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    destination
}

fn convert(source: &Path, destination: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", format])
        .output()
        .unwrap()
}

fn run(deck: &Path, destination: &Path, format: &str, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(destination)
        .args(["-f", format])
        .args(extra)
        .output()
        .unwrap()
}

fn compare(source: &Path, baseline: &Path, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(source)
        .arg(baseline)
        .arg("--json")
        .args(flags)
        .output()
        .unwrap()
}

fn hex(value: &str) -> String {
    value.bytes().map(|byte| format!("{byte:02x}")).collect()
}

fn column<'a>(table: &'a Value, name: &str) -> &'a Value {
    table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == name)
        .unwrap_or_else(|| panic!("missing {name}: {table}"))
}

#[test]
fn monte_carlo_trial_values_statistics_and_metadata_survive_every_format() {
    let dir = common::test_dir("mc_complete_conversion");
    let source = source(&dir, "source", 6, 7, "11");
    let original = common::read_json(&source);
    let altered = dir.join("reordered.json");
    let mut changed = original.clone();
    for variable in changed["payload"]["statistics"].as_array_mut().unwrap() {
        variable["samples"].as_array_mut().unwrap().reverse();
    }
    std::fs::write(&altered, changed.to_string()).unwrap();
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("converted.{extension}"));
        let output = convert(&source, &destination, format);
        assert!(output.status.success(), "{format}: {output:?}");
        let equal = compare(&destination, &source, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
        let different = compare(&altered, &destination, &[]);
        assert_eq!(different.status.code(), Some(3), "{format}: {different:?}");
        let decoded = dir.join("decoded.json");
        assert!(convert(&destination, &decoded, "json").status.success());
        let table = common::read_json(&decoded);
        assert_eq!(
            table["scale"]["values"],
            json!([7.0, 8.0, 9.0, 10.0, 11.0, 12.0])
        );
        for variable in original["payload"]["statistics"].as_array().unwrap() {
            let name = variable["name"].as_str().unwrap();
            let identity = hex(name);
            assert_eq!(column(&table, name)["values"], variable["samples"]);
            for (metric, field) in [
                ("mean", "mean"),
                ("standard_deviation", "standardDeviation"),
                ("minimum", "minimum"),
                ("maximum", "maximum"),
            ] {
                assert_eq!(
                    column(&table, &format!("mc:{metric}({identity})"))["values"],
                    json!(vec![variable[field].clone(); 6])
                );
            }
            for (index, count) in variable["histogram"].as_array().unwrap().iter().enumerate() {
                assert_eq!(
                    column(&table, &format!("mc:histogram({identity},{index})"))["values"][0]
                        .as_f64()
                        .unwrap(),
                    count.as_u64().unwrap() as f64
                );
            }
            for (index, edge) in variable["binEdges"].as_array().unwrap().iter().enumerate() {
                assert_eq!(
                    column(&table, &format!("mc:bin_edge({identity},{index})"))["values"][0],
                    *edge
                );
            }
        }
        for scalar in original["scalars"].as_array().unwrap() {
            if scalar["value"]["representation"] == "text" {
                let marker = format!(
                    "mc:identity:text({},{})",
                    hex(scalar["name"].as_str().unwrap()),
                    hex(scalar["value"]["value"].as_str().unwrap())
                );
                assert_eq!(column(&table, &marker)["values"], json!(vec![1.0; 6]));
            }
        }
    }
    // A legacy campaign with no optional scalar metadata still compares the
    // actual population, not just the four aggregate run counters.
    changed["scalars"].as_array_mut().unwrap().retain(|scalar| {
        [
            "completed_runs",
            "failed_runs",
            "successful_runs",
            "all_converged",
        ]
        .contains(&scalar["name"].as_str().unwrap())
    });
    let mut legacy = original.clone();
    legacy["scalars"] = changed["scalars"].clone();
    let legacy_path = dir.join("legacy.json");
    std::fs::write(&legacy_path, legacy.to_string()).unwrap();
    std::fs::write(&altered, changed.to_string()).unwrap();
    assert_eq!(compare(&altered, &legacy_path, &[]).status.code(), Some(3));
}

#[test]
fn monte_carlo_keeps_exact_large_seeds_and_case_sensitive_variable_identities() {
    let dir = common::test_dir("mc_exact_identity");
    let source = source(&dir, "source", 3, 0, "18446744073709551615");
    let original = common::read_json(&source);
    let mut document = original.clone();
    document["scalars"]
        .as_array_mut()
        .unwrap()
        .retain(|scalar| {
            !scalar["name"]
                .as_str()
                .unwrap()
                .starts_with("mean_confidence")
        });
    document["payload"]["statistics"]
        .as_array_mut()
        .unwrap()
        .truncate(2);
    document["payload"]["statistics"][0]["name"] = json!("Gain");
    document["payload"]["statistics"][1]["name"] = json!("gain");
    std::fs::write(&source, document.to_string()).unwrap();
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("exact.{extension}"));
        let output = convert(&source, &destination, format);
        assert!(output.status.success(), "{format}: {output:?}");
        let equal = compare(&source, &destination, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
        let decoded = dir.join("decoded.json");
        assert!(convert(&destination, &decoded, "json").status.success());
        let table = common::read_json(&decoded);
        column(&table, "mc:sample(4761696e)");
        column(&table, "mc:sample(6761696e)");
        column(
            &table,
            &format!(
                "mc:identity:count({},18446744073709551615)",
                hex("sampling_seed")
            ),
        );
    }
}

#[test]
fn monte_carlo_confidence_absence_is_preserved_and_not_changed_by_tolerances() {
    let dir = common::test_dir("mc_confidence_absence");
    let source = source(&dir, "single", 1, 0, "11");
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("single-flat.{extension}"));
        let output = convert(&source, &destination, format);
        assert!(output.status.success(), "{format}: {output:?}");
        let equal = compare(&source, &destination, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
    }
    let flat = dir.join("single-flat.json");
    let original = common::read_json(&flat);
    let mut changed = original.clone();
    let status = changed["signals"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|signal| {
            signal["name"]
                .as_str()
                .unwrap()
                .ends_with(":monte_carlo_status")
        })
        .unwrap();
    status["values"][0] = json!(3.0);
    let altered = dir.join("altered.json");
    std::fs::write(&altered, changed.to_string()).unwrap();
    assert_eq!(
        compare(&altered, &flat, &["--abstol", "1e100", "--reltol", "1e100"])
            .status
            .code(),
        Some(3)
    );
}

#[test]
fn monte_carlo_projection_limits_and_invalid_identities_preserve_destinations() {
    let dir = common::test_dir("mc_conversion_admission");
    let source = source(&dir, "source", 6, 0, "11");
    let destination = dir.join("protected.csv");
    let config = dir.join("limits.toml");
    for limit in ["max_result_values", "max_external_data_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=400\n")).unwrap();
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
    let flat = dir.join("flat.json");
    assert!(convert(&source, &flat, "json").status.success());
    let original = common::read_json(&flat);
    for case in ["false", "missing", "unit", "hex", "duplicate"] {
        let mut changed = original.clone();
        let signals = changed["signals"].as_array_mut().unwrap();
        let marker = signals
            .iter_mut()
            .find(|signal| {
                signal["name"]
                    .as_str()
                    .unwrap()
                    .starts_with("mc:identity:variable(")
            })
            .unwrap();
        match case {
            "false" => marker["values"][0] = json!(0.0),
            "missing" => marker["values"][0] = Value::Null,
            "unit" => marker["unit"] = json!("V"),
            "hex" => marker["name"] = json!("mc:identity:variable(zz)"),
            "duplicate" => {
                let mut duplicate = marker.clone();
                duplicate["name"] = json!(duplicate["name"].as_str().unwrap().to_ascii_uppercase());
                signals.push(duplicate);
            }
            _ => panic!("case"),
        }
        let altered = dir.join("altered.json");
        std::fs::write(&altered, changed.to_string()).unwrap();
        let result = compare(
            &altered,
            &flat,
            &["--variables", "completed_runs", "--bless"],
        );
        assert_eq!(result.status.code(), Some(3), "{case}: {result:?}");
        assert_eq!(common::read_json(&flat), original);
    }
}

#[test]
fn monte_carlo_does_not_interpolate_between_distinct_trials() {
    let dir = common::test_dir("mc_discrete_trials");
    let source = source(&dir, "source", 3, 7, "11");
    let flat = dir.join("flat.json");
    assert!(convert(&source, &flat, "json").status.success());
    assert!(compare(&flat, &source, &["--interpolate"]).status.success());
    let mut changed = common::read_json(&flat);
    changed["scale"]["values"][1] = json!(7.5);
    let altered = dir.join("fractional-trial.json");
    std::fs::write(&altered, changed.to_string()).unwrap();
    let output = compare(&altered, &flat, &["--interpolate", "--variables", "V(OUT)"]);
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    changed["scale"]["values"] = json!([10.0, 11.0, 12.0]);
    std::fs::write(&altered, changed.to_string()).unwrap();
    let output = compare(
        &altered,
        &flat,
        &[
            "--abstol",
            "1e100",
            "--reltol",
            "1e100",
            "--variables",
            "V(OUT)",
        ],
    );
    assert_eq!(output.status.code(), Some(3), "{output:?}");
}

#[test]
fn monte_carlo_preserves_legacy_provenance_and_rejects_empty_verification() {
    let dir = common::test_dir("mc_legacy_population");
    let source = source(&dir, "source", 3, 7, "11");
    let original = common::read_json(&source);
    let mut legacy = original.clone();
    legacy["payload"]
        .as_object_mut()
        .unwrap()
        .remove("successfulTrialIndices");
    let legacy_path = dir.join("legacy.json");
    std::fs::write(&legacy_path, legacy.to_string()).unwrap();
    let flat = dir.join("flat.json");
    assert!(convert(&legacy_path, &flat, "json").status.success());
    let table = common::read_json(&flat);
    assert_eq!(table["scale"]["name"], "sample_index");
    assert_eq!(table["scale"]["values"], json!([0.0, 1.0, 2.0]));
    column(&table, "mc:identity:trial_coordinates(unknown)");
    assert!(compare(&flat, &legacy_path, &[]).status.success());
    assert_eq!(compare(&flat, &source, &[]).status.code(), Some(3));

    for empty_indices in [false, true] {
        let mut empty = original.clone();
        empty["payload"]["statistics"] = json!([]);
        if empty_indices {
            empty["payload"]["successfulTrialIndices"] = json!([]);
        } else {
            empty["payload"]
                .as_object_mut()
                .unwrap()
                .remove("successfulTrialIndices");
        }
        empty["scalars"]
            .as_array_mut()
            .unwrap()
            .retain(|scalar| scalar["name"] == "successful_runs");
        empty["scalars"][0]["value"]["value"] = json!(0);
        std::fs::write(&legacy_path, empty.to_string()).unwrap();
        let converted = convert(&legacy_path, &flat, "json");
        assert!(converted.status.success(), "{converted:?}");
        let output = compare(&flat, &flat, &["--variables", "successful_runs"]);
        assert_eq!(output.status.code(), Some(3), "{output:?}");
    }
}

#[test]
fn monte_carlo_retains_scalar_types_units_and_exact_metadata() {
    let dir = common::test_dir("mc_scalar_metadata");
    let source = source(&dir, "source", 3, 0, "11");
    let mut document = common::read_json(&source);
    for (name, value) in [
        (
            "signed",
            json!({"representation":"integer", "value":i64::MIN + 1}),
        ),
        (
            "phasor",
            json!({"representation":"complex", "value":{"real":2.0,"imaginary":3.0}}),
        ),
        (
            "notes",
            json!({"representation":"text", "value":"Mixed Case, µV (test)"}),
        ),
    ] {
        document["scalars"].as_array_mut().unwrap().push(json!({
            "name":name, "displayName":name,
            "unit":{"unit":"custom","symbol":"µV"}, "value":value
        }));
    }
    std::fs::write(&source, document.to_string()).unwrap();
    for (format, extension) in FORMATS {
        let destination = dir.join(format!("metadata.{extension}"));
        let output = convert(&source, &destination, format);
        assert!(output.status.success(), "{format}: {output:?}");
        let equal = compare(&destination, &source, &[]);
        assert!(equal.status.success(), "{format}: {equal:?}");
    }
    let flat = dir.join("metadata.json");
    let table = common::read_json(&flat);
    column(
        &table,
        &format!("mc:identity:unit({},{})", hex("notes"), hex("µV")),
    );
    column(
        &table,
        &format!(
            "mc:identity:integer({},-9223372036854775807)",
            hex("signed")
        ),
    );
    for change_unit in [false, true] {
        let mut changed = document.clone();
        let notes = changed["scalars"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap();
        if change_unit {
            notes["unit"]["symbol"] = json!("mV");
        } else {
            notes["value"]["value"] = json!("mixed Case, µV (test)");
        }
        let altered = dir.join("changed-metadata.json");
        std::fs::write(&altered, changed.to_string()).unwrap();
        let output = compare(&altered, &flat, &["--abstol", "1e100", "--reltol", "1e100"]);
        assert_eq!(output.status.code(), Some(3), "{output:?}");
    }
}

#[test]
fn native_monte_carlo_exports_retain_complete_reports_and_voltage_units() {
    let dir = common::test_dir("mc_native_reports");
    for (runs, bootstrap) in [(1, false), (6, false), (6, true)] {
        let typed = source(&dir, "source", runs, 7, "18446744073709551615");
        let deck = dir.join("source.sp");
        if bootstrap {
            let authored = std::fs::read_to_string(&deck).unwrap().replace(
                "\n.end",
                " CI BOOTSTRAP RESAMPLES 30 BOOTSEED 18446744073709551615\n.end",
            );
            std::fs::write(&deck, authored).unwrap();
            let output = run(&deck, &typed, "json", &[]);
            assert!(output.status.success(), "{output:?}");
        }
        let original = common::read_json(&typed);
        assert_eq!(original["pointCount"], 0);
        assert!(original["signals"].as_array().unwrap().is_empty());
        for variable in original["payload"]["statistics"].as_array().unwrap() {
            assert_eq!(variable["unit"], json!({"unit":"volt"}));
        }
        for (format, extension) in FORMATS {
            let native = dir.join(format!("native.{extension}"));
            let output = run(&deck, &native, format, &[]);
            assert!(
                output.status.success(),
                "{runs}/{bootstrap}/{format}: {output:?}"
            );
            let equal = compare(&native, &typed, &[]);
            assert!(
                equal.status.success(),
                "{runs}/{bootstrap}/{format}: {equal:?}"
            );
            let decoded = dir.join("decoded.json");
            assert!(convert(&native, &decoded, "json").status.success());
            let table = common::read_json(&decoded);
            assert_eq!(column(&table, "V(OUT)")["unit"], "V");
            assert_eq!(
                column(&table, &format!("mc:mean({})", hex("V(OUT)")))["unit"],
                "V"
            );
            assert_eq!(
                column(&table, &format!("mc:histogram({},0)", hex("V(OUT)")))["unit"],
                "1"
            );
        }
    }
}

#[test]
fn native_monte_carlo_keeps_large_trial_indices_exact_without_limiting_json() {
    let dir = common::test_dir("mc_large_trial_indices");
    std::fs::write(
        dir.join("statistics.scs"),
        "parameters p=1000\nstatistics {\n mismatch {\n vary p dist=gauss std=0\n }\n}\n",
    )
    .unwrap();
    let deck = dir.join("large.sp");
    for start in [
        9_007_199_254_740_992_u64,
        9_007_199_254_740_993,
        9_007_199_254_740_994,
    ] {
        std::fs::write(&deck, format!("Large trial index\n.include \"statistics.scs\"\nV1 in 0 1\nR1 in 0 {{p}}\n.MC 1 START {start} SEED 11\n.end\n")).unwrap();
        for (format, extension) in FORMATS {
            let destination = dir.join(format!("large.{extension}"));
            std::fs::write(&destination, "previous").unwrap();
            let output = run(&deck, &destination, format, &[]);
            if format == "json" {
                assert!(output.status.success(), "{start}/{format}: {output:?}");
                assert_eq!(
                    common::read_json(&destination)["payload"]["successfulTrialIndices"][0]
                        .as_u64(),
                    Some(start)
                );
            } else if start.is_multiple_of(2) {
                assert!(output.status.success(), "{start}/{format}: {output:?}");
                let decoded = dir.join("decoded.json");
                assert!(convert(&destination, &decoded, "json").status.success());
                assert_eq!(
                    common::read_json(&decoded)["scale"]["values"][0].as_f64(),
                    Some(start as f64)
                );
            } else {
                assert_eq!(
                    output.status.code(),
                    Some(1),
                    "{start}/{format}: {output:?}"
                );
                assert!(
                    String::from_utf8_lossy(&output.stderr)
                        .contains("cannot be represented exactly"),
                    "{output:?}"
                );
                assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous");
            }
        }
    }
}

#[test]
fn native_monte_carlo_projection_respects_limits_and_publishes_manifest_units() {
    let dir = common::test_dir("mc_native_admission");
    let typed = source(&dir, "source", 6, 0, "11");
    let deck = dir.join("source.sp");
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_values=400\n").unwrap();
    let destination = dir.join("protected.csv");
    std::fs::write(&destination, "previous").unwrap();
    for (format, path) in [("csv", &destination), ("json", &typed)] {
        let output = run(&deck, path, format, &["--config", config.to_str().unwrap()]);
        assert_eq!(
            output.status.code(),
            Some(if format == "csv" { 75 } else { 0 }),
            "{format}: {output:?}"
        );
    }
    assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
    let authored = std::fs::read_to_string(&deck)
        .unwrap()
        .replace(".end", ".STEP PARAM rtop LIST 1000 2000\n.end");
    std::fs::write(&deck, authored).unwrap();
    let output = run(&deck, &dir.join("sweep.csv"), "csv", &[]);
    assert!(output.status.success(), "{output:?}");
    let manifest = common::read_json(&dir.join("sweep.step_schema.json"));
    let schema = manifest["analyses"][0]["union_schema"].as_array().unwrap();
    for (name, unit) in [
        ("V(OUT)".to_string(), "volt"),
        (format!("mc:mean({})", hex("V(OUT)")), "volt"),
        (
            format!("mc:histogram({},0)", hex("V(OUT)")),
            "dimensionless",
        ),
        ("completed_runs".into(), "dimensionless"),
    ] {
        let descriptor = schema
            .iter()
            .find(|descriptor| descriptor["display_name"] == name)
            .unwrap_or_else(|| panic!("missing {name}: {manifest}"));
        assert_eq!(descriptor["unit"], unit, "{name}: {descriptor}");
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
fn contradictory_monte_carlo_reports_cannot_be_exported_compared_or_blessed() {
    let dir = common::test_dir("mc_consistency");
    let source = source(&dir, "source", 6, 7, "11");
    let original = common::read_json(&source);
    let altered = dir.join("altered.json");
    let destination = dir.join("protected.csv");
    for case in [
        "failed",
        "converged",
        "count-type",
        "range",
        "conditional",
        "bounds",
        "unit",
    ] {
        let mut document = original.clone();
        let name = match case {
            "failed" => "failed_runs",
            "converged" => "all_converged",
            "count-type" => "completed_runs",
            "conditional" => "mean_confidence_conditional_on_success",
            _ => "mean_confidence_lower:56284f555429",
        };
        let scalar = document["scalars"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|scalar| scalar["name"] == name)
            .unwrap();
        match case {
            "failed" => scalar["value"]["value"] = json!(999),
            "converged" => scalar["value"]["value"] = json!(false),
            "count-type" => scalar["value"] = json!({"representation":"real","value":6.0}),
            "conditional" => scalar["value"]["value"] = json!(true),
            "bounds" => scalar["value"]["value"] = json!(100.0),
            "unit" => scalar["unit"] = json!({"unit":"ampere"}),
            "range" => document["payload"]["successfulTrialIndices"][5] = json!(13),
            _ => unreachable!(),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        std::fs::write(&destination, "previous").unwrap();
        let converted = convert(&altered, &destination, "csv");
        assert_eq!(converted.status.code(), Some(1), "{case}: {converted:?}");
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "previous");
        let output = compare(
            &altered,
            &altered,
            &["--variables", "V(OUT)", "--abstol", "1e100"],
        );
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        let output = compare(&altered, &source, &["--bless"]);
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert_eq!(common::read_json(&source), original);
    }
}

#[test]
fn flat_monte_carlo_counts_cannot_contradict_exact_metadata_or_population() {
    let dir = common::test_dir("mc_flat_consistency");
    let source = source(&dir, "source", 6, 7, "11");
    let flat = dir.join("flat.json");
    assert!(convert(&source, &flat, "json").status.success());
    let original = common::read_json(&flat);
    for case in ["numeric", "marker", "complex"] {
        let mut table = original.clone();
        for signal in table["signals"].as_array_mut().unwrap() {
            if signal["name"] == "failed_runs" {
                signal["values"] = json!(vec![999.0; 6]);
                if case == "complex" {
                    signal.as_object_mut().unwrap().remove("values");
                    signal["real"] = json!(vec![999.0; 6]);
                    signal["imag"] = json!(vec![0.0; 6]);
                }
            }
            if case == "marker"
                && signal["name"] == format!("mc:identity:count({},0)", hex("failed_runs"))
            {
                signal["name"] = json!(format!("mc:identity:count({},999)", hex("failed_runs")));
            }
        }
        let altered = dir.join("altered.json");
        std::fs::write(&altered, table.to_string()).unwrap();
        let output = compare(
            &altered,
            &altered,
            &[
                "--variables",
                "V(OUT)",
                "--abstol",
                "1e100",
                "--reltol",
                "1e100",
            ],
        );
        assert_eq!(output.status.code(), Some(3), "{output:?}");
        let output = compare(&altered, &flat, &["--bless", "--variables", "V(OUT)"]);
        assert_eq!(output.status.code(), Some(3), "{output:?}");
        assert_eq!(common::read_json(&flat), original);
    }
    // Partial convergence remains a valid, explicitly conditional population.
    let mut partial = common::read_json(&source);
    for scalar in partial["scalars"].as_array_mut().unwrap() {
        match scalar["name"].as_str().unwrap() {
            "completed_runs" => scalar["value"]["value"] = json!(8),
            "failed_runs" => scalar["value"]["value"] = json!(2),
            "all_converged" => scalar["value"]["value"] = json!(false),
            "mean_confidence_conditional_on_success" => scalar["value"]["value"] = json!(true),
            _ => {}
        }
    }
    let partial_path = dir.join("partial.json");
    std::fs::write(&partial_path, partial.to_string()).unwrap();
    let output = convert(&partial_path, &flat, "json");
    assert!(output.status.success(), "{output:?}");
    let equal = compare(&flat, &partial_path, &[]);
    assert!(equal.status.success(), "{equal:?}");
}
