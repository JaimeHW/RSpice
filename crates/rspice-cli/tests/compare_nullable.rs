//! Missing sample availability is a contract, independent of numeric tolerance.
mod common;

use serde_json::{Value, json};
use std::process::Command;

fn table(name: &str, times: &[f64], values: &[Option<f64>]) -> Value {
    json!({
        "scale":{"name":"time","values":times},
        "signals":[{"name":name,"values":values}]
    })
}

fn compare(result: Value, golden: Value, flags: &[&str], expected: i32) -> Value {
    let directory = common::test_dir("compare_nullable");
    let result_path = directory.join("result.json");
    let golden_path = directory.join("golden.json");
    std::fs::write(&result_path, result.to_string()).unwrap();
    std::fs::write(&golden_path, golden.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(&result_path)
        .arg(&golden_path)
        .arg("--json")
        .args(flags)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(expected), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn nullable_comparison_checks_availability_and_requires_numeric_coverage() {
    let values = table("V(out)", &[0.0, 1.0, 2.0], &[None, Some(1.0), Some(2.0)]);
    let passed = compare(values.clone(), values.clone(), &[], 0);
    assert_eq!(passed["num_differences"], 0);
    assert_eq!(passed["problems"], json!([]));

    let fabricated = table(
        "V(out)",
        &[0.0, 1.0, 2.0],
        &[Some(0.0), Some(1.0), Some(2.0)],
    );
    for (result, golden) in [
        (values.clone(), fabricated.clone()),
        (fabricated, values.clone()),
    ] {
        for flags in [
            vec!["--abstol", "1e100", "--reltol", "1e100"],
            vec!["--ignore-missing", "--allow-truncated", "--fail-fast"],
        ] {
            let failed = compare(result.clone(), golden.clone(), &flags, 3);
            assert!(
                failed["problems"][0]
                    .as_str()
                    .unwrap()
                    .contains("undefined")
            );
        }
    }
    let absent = table("V(out)", &[0.0, 1.0, 2.0], &[None, None, None]);
    let failed = compare(absent.clone(), absent, &[], 3);
    assert!(
        failed["problems"][0]
            .as_str()
            .unwrap()
            .contains("no defined samples")
    );
    let changed = table("V(out)", &[0.0, 1.0, 2.0], &[None, Some(1.0), Some(3.0)]);
    let failed = compare(values, changed, &[], 3);
    assert_eq!(failed["num_differences"], 1);
    assert_eq!(failed["differences"][0]["index"], 2);
}

#[test]
fn interpolation_preserves_missing_endpoints_and_first_defined_events() {
    for (name, middle) in [("V(out)", 3.0), ("E(sample)", 2.0)] {
        let source = table(name, &[0.0, 2.0, 4.0], &[None, Some(2.0), Some(4.0)]);
        let golden = table(
            name,
            &[0.0, 1.0, 2.0, 3.0, 4.0],
            &[None, None, Some(2.0), Some(middle), Some(4.0)],
        );
        let passed = compare(source.clone(), golden.clone(), &["--interpolate"], 0);
        assert_eq!(passed["problems"], json!([]));
        let mut fabricated = golden;
        fabricated["signals"][0]["values"][1] = json!(1.0);
        let failed = compare(source, fabricated, &["--interpolate"], 3);
        assert!(failed["problems"][0].as_str().unwrap().contains("index 1"));
    }
}

#[test]
fn selection_excludes_unavailable_signals_without_discarding_the_coordinate() {
    let source = json!({
        "scale":{"name":"time","values":[0,1]},
        "signals":[
            {"name":"V(out)","values":[1,2]},
            {"name":"E(unavailable)","values":[null,null]}
        ]
    });
    compare(
        source.clone(),
        source.clone(),
        &["--variables", "V(out)"],
        0,
    );
    compare(source.clone(), source, &[], 3);
}
