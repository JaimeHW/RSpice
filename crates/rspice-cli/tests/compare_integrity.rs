//! Regression verification must fail on missing quantities and uncovered data.
mod common;

use common::test_dir;
use std::process::{Command, Output};

fn compare(result: &str, golden: &str, flags: &[&str]) -> Output {
    let dir = test_dir("comparison_integrity");
    let result_path = dir.join("result.csv");
    let golden_path = dir.join("golden.csv");
    std::fs::write(&result_path, result).unwrap();
    std::fs::write(&golden_path, golden).unwrap();
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(result_path)
        .arg(golden_path)
        .arg("--json")
        .args(flags)
        .output()
        .unwrap()
}

fn failed(output: Output) -> serde_json::Value {
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn voltage_cannot_supply_a_missing_current() {
    let voltage = "time,V(x)\n0,1\n1,2\n";
    for golden in ["time,I(x)\n0,1\n1,2\n", "time,V(x),I(x)\n0,1,9\n1,2,8\n"] {
        let json = failed(compare(
            voltage,
            golden,
            &["--abstol", "0", "--reltol", "0"],
        ));
        assert!(
            json["problems"]
                .as_array()
                .unwrap()
                .iter()
                .any(|x| x.as_str().unwrap().contains("I(x)"))
        );
    }
}

#[test]
fn bare_selectors_must_be_unambiguous_and_qualified_selectors_keep_their_quantity() {
    let both = "time,V(x),I(x)\n0,1,9\n1,2,8\n";
    let changed_current = "time,V(x),I(x)\n0,1,7\n1,2,6\n";
    let json = failed(compare(both, both, &["--variables", "x"]));
    assert!(json["problems"][0].as_str().unwrap().contains("ambiguous"));
    let voltage = compare(both, changed_current, &["--variables", "V(x)"]);
    assert!(voltage.status.success(), "{voltage:?}");
    failed(compare(both, changed_current, &["--variables", "I(x)"]));
}

#[test]
fn zero_and_near_zero_references_do_not_gain_an_absolute_relative_tolerance() {
    for reference in ["0", "1e-25"] {
        let golden = format!("time,V(x)\n0,{reference}\n");
        let json = failed(compare(
            "time,V(x)\n0,5e-7\n",
            &golden,
            &["--abstol", "1p", "--fail-fast"],
        ));
        assert!(json["max_abs_diff"].as_f64().unwrap() > 4e-7);
        assert_eq!(json["num_differences"], 1);
    }
    assert!(
        compare(
            "time,V(x)\n0,5e-13\n",
            "time,V(x)\n0,0\n",
            &["--abstol", "1p"]
        )
        .status
        .success()
    );
    assert!(
        compare(
            "time,V(x)\n0,1.0000005\n",
            "time,V(x)\n0,1\n",
            &["--abstol", "0"]
        )
        .status
        .success()
    );
}

#[test]
fn interpolation_cannot_extend_a_picosecond_trace_to_half_a_nanosecond() {
    let result = compare(
        "time,V(x)\n0,1\n1e-12,1\n",
        "time,V(x)\n0,1\n5e-10,1\n",
        &["--interpolate", "--abstol", "0", "--reltol", "0"],
    );
    assert_eq!(result.status.code(), Some(3), "{result:?}");
    assert!(String::from_utf8_lossy(&result.stderr).contains("extrapolate"));
}
