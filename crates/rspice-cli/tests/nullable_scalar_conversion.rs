mod common;

use common::{read_json, test_dir};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .output()
        .unwrap()
}

fn source(dir: &Path) -> PathBuf {
    let deck = dir.join("source.cir");
    let input = dir.join("source.json");
    std::fs::write(
        &deck,
        "* absent scalar exports\nV1 in 0 AC 1\nR1 in 0 1k\n.AC LIN 3 1 3\n.END\n",
    )
    .unwrap();
    let output = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        input.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(output.status.success(), "{output:?}");
    let mut document = read_json(&input);
    assert_eq!(document["pointCount"], 3);
    document["signals"] = json!([]);
    let mut scalars = vec![
        json!({"name":"uncomputed_real","displayName":"Uncomputed real","unit":{"unit":"hertz"},"value":{"representation":"real","value":null}}),
        json!({"name":"uncomputed_complex","displayName":"Uncomputed complex","unit":{"unit":"volt"},"value":{"representation":"complex","value":null}}),
    ];
    for reason in [
        "positive_infinity",
        "negative_infinity",
        "no_crossover",
        "empty_domain",
    ] {
        scalars.push(json!({"name":reason,"displayName":reason,"unit":{"unit":"hertz"},"value":{"representation":"unavailable","reason":reason}}));
    }
    document["scalars"] = json!(scalars);
    std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
    input
}

#[test]
fn missing_scalar_values_and_their_reasons_survive_all_table_formats() {
    let dir = test_dir("nullable_scalar_formats");
    let input = source(&dir);
    for format in ["json", "csv", "tsv", "ascii", "raw", "hdf5"] {
        let flat = dir.join(format!("flat.{format}"));
        let decoded = dir.join(format!("decoded-{format}.json"));
        for (input, output, format) in [(&input, &flat, format), (&flat, &decoded, "json")] {
            let result = cli(&[
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                format,
            ]);
            assert!(result.status.success(), "{format}: {result:?}");
        }
        let table = read_json(&decoded);
        let column = |name: &str| {
            table["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|column| column["name"] == name)
                .unwrap_or_else(|| panic!("{format}: missing {name}"))
        };
        for name in [
            "uncomputed_real",
            "uncomputed_complex",
            "positive_infinity",
            "negative_infinity",
            "no_crossover",
            "empty_domain",
        ] {
            let metric = column(name);
            assert_eq!(
                metric.get("values").or_else(|| metric.get("real")).unwrap(),
                &json!([null, null, null])
            );
            if name == "uncomputed_complex" {
                assert_eq!(metric["imag"], json!([null, null, null]));
            }
            if !name.starts_with("uncomputed") {
                let reason = column(&format!("{name}:unavailable({name})"));
                assert_eq!(
                    reason.get("values").or_else(|| reason.get("real")).unwrap(),
                    &json!([1.0, 1.0, 1.0])
                );
                if !matches!(format, "csv" | "tsv") {
                    assert_eq!(reason["unit"], "1");
                }
            }
        }
        let result = cli(&[
            "compare",
            input.to_str().unwrap(),
            flat.to_str().unwrap(),
            "--json",
        ]);
        // Matching uncomputed gaps still do not establish numeric coverage.
        assert_eq!(result.status.code(), Some(3), "{format}: {result:?}");
        let report: Value = serde_json::from_slice(&result.stdout).unwrap();
        let problems = report["problems"].as_array().unwrap();
        assert_eq!(problems.len(), 3, "{format}: {report}");
        assert!(
            problems
                .iter()
                .all(|problem| problem.as_str().unwrap().contains("uncomputed"))
        );
        let result = cli(&[
            "compare",
            input.to_str().unwrap(),
            flat.to_str().unwrap(),
            "--json",
            "--variables",
            "no_crossover",
            "--variables",
            "positive_infinity",
            "--variables",
            "negative_infinity",
            "--variables",
            "empty_domain",
        ]);
        assert!(result.status.success(), "{format}: {result:?}");
    }
}

#[test]
fn scalar_unavailability_reasons_participate_in_comparison() {
    let dir = test_dir("nullable_scalar_compare");
    let input = source(&dir);
    let other = dir.join("other.json");
    let mut document = read_json(&input);
    document["scalars"][4]["value"]["reason"] = json!("empty_domain");
    std::fs::write(&other, serde_json::to_vec(&document).unwrap()).unwrap();
    let result = cli(&[
        "compare",
        input.to_str().unwrap(),
        other.to_str().unwrap(),
        "--json",
        "--variables",
        "no_crossover",
    ]);
    assert_eq!(result.status.code(), Some(3), "{result:?}");
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["comparison_passed"], false);
}

#[test]
fn scalar_determinations_require_consistent_indicators_even_under_loose_tolerances() {
    let dir = test_dir("nullable_scalar_evidence");
    let input = source(&dir);
    let flat = dir.join("flat.json");
    let changed = dir.join("changed.json");
    let result = cli(&[
        "convert",
        input.to_str().unwrap(),
        flat.to_str().unwrap(),
        "--to",
        "json",
    ]);
    assert!(result.status.success(), "{result:?}");
    let original = read_json(&flat);
    for defect in [
        "false",
        "missing",
        "approximate",
        "unit",
        "duplicate",
        "finite_metric",
    ] {
        let mut table = original.clone();
        let columns = table["signals"].as_array_mut().unwrap();
        let indicator = columns
            .iter()
            .position(|column| column["name"] == "no_crossover:unavailable(no_crossover)")
            .unwrap();
        match defect {
            "false" => columns[indicator]["values"][0] = json!(0.0),
            "missing" => columns[indicator]["values"][0] = Value::Null,
            "approximate" => columns[indicator]["values"][0] = json!(1.000001),
            "unit" => columns[indicator]["unit"] = json!("V"),
            "duplicate" => {
                let mut duplicate = columns[indicator].clone();
                duplicate["name"] = json!("no_crossover:unavailable(empty_domain)");
                columns.push(duplicate);
            }
            "finite_metric" => {
                let column = columns
                    .iter_mut()
                    .find(|column| column["name"] == "no_crossover")
                    .unwrap();
                column["values"] = json!([0.0, 0.0, 0.0]);
            }
            _ => panic!("unknown defect: {defect}"),
        }
        std::fs::write(&changed, serde_json::to_vec(&table).unwrap()).unwrap();
        // Compare the malformed document to itself: equal numbers cannot
        // legitimize a contradictory declaration of scalar availability.
        let result = cli(&[
            "compare",
            changed.to_str().unwrap(),
            changed.to_str().unwrap(),
            "--variables",
            "no_crossover",
            "--abstol",
            "1e100",
            "--reltol",
            "1e100",
            "--json",
        ]);
        assert_eq!(result.status.code(), Some(3), "{defect}: {result:?}");
        let report: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert!(
            report["problems"]
                .as_array()
                .unwrap()
                .iter()
                .any(|problem| problem
                    .as_str()
                    .unwrap()
                    .contains("invalid scalar unavailability")),
            "{defect}: {report}"
        );
    }
}

#[test]
fn scalar_reason_columns_obey_limits_and_cannot_collide_with_authored_names() {
    let dir = test_dir("nullable_scalar_safety");
    let input = source(&dir);
    let output = dir.join("protected.json");
    let config = dir.join("limits.toml");
    // Three coordinates, three uncomputed components, and four metric/reason
    // pairs expand to 36 cells; the typed source only retains nine values.
    for limit in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=35\n")).unwrap();
        std::fs::write(&output, "predecessor").unwrap();
        let result = cli(&[
            "--config",
            config.to_str().unwrap(),
            "convert",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "json",
        ]);
        assert_eq!(result.status.code(), Some(75), "{result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
    let mut document = read_json(&input);
    document["scalars"].as_array_mut().unwrap().push(json!({"name":"no_crossover:unavailable(no_crossover)","displayName":"Collision","unit":{"unit":"dimensionless"},"value":{"representation":"real","value":1.0}}));
    std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
    let result = cli(&[
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        "json",
    ]);
    assert_eq!(result.status.code(), Some(1), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("duplicate column"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
}
