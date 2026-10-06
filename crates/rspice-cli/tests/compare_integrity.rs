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

#[test]
fn signal_selection_still_compares_the_independent_coordinate() {
    let json = failed(compare(
        "time,V(x)\n0,1\n2,1\n",
        "time,V(x)\n0,1\n1,1\n",
        &["--variables", "V(x)", "--abstol", "0", "--reltol", "0"],
    ));
    assert_eq!(json["differences"][0]["variable"], "time");
    assert!(
        compare(
            "time,V(x)\n0,1\n2,1\n",
            "time,V(x)\n0,1\n1,1\n",
            &["--variables", "V(x)", "--interpolate"],
        )
        .status
        .success()
    );
    failed(compare(
        "time,V(x)\n0,1\n1,1\n",
        "frequency,V(x)\n0,1\n1,1\n",
        &["--variables", "V(x)", "--ignore-missing"],
    ));
    let interpolated = compare(
        "time,V(x)\n0,1\n1,1\n",
        "frequency,V(x)\n0,1\n1,1\n",
        &["--variables", "V(x)", "--interpolate"],
    );
    assert_eq!(interpolated.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&interpolated.stderr).contains("independent coordinates differ")
    );
}

#[test]
fn duplicate_quantities_cannot_hide_later_columns_or_be_blessed() {
    let unique = "time,V(x)\n0,1\n1,2\n";
    let duplicate = "time,V(x),v(X)\n0,1,9\n1,2,8\n";
    for flags in [&[][..], &["--variables", "V(x)"][..], &["--bless"][..]] {
        for (result, golden) in [(unique, duplicate), (duplicate, unique)] {
            let output = compare(result, golden, flags);
            assert_eq!(output.status.code(), Some(3), "{output:?}");
            assert!(String::from_utf8_lossy(&output.stderr).contains("duplicate variable"));
        }
    }
}

#[test]
fn interpolation_preserves_finite_values_across_the_binary64_range() {
    for (result, golden) in [
        // Subtracting finite opposite-sign values must not overflow.
        (
            "time,V(x)\n0,-1e308\n1,1e308\n",
            "time,V(x)\n0,-1e308\n0.5,0\n1,1e308\n",
        ),
        // The coordinate interval itself can exceed f64::MAX.
        (
            "time,V(x)\n-1e308,-1\n1e308,1\n",
            "time,V(x)\n-1e308,-1\n0,0\n1e308,1\n",
        ),
        // Multiplying before division overflows or underflows needlessly.
        (
            "time,V(x)\n0,0\n1e200,1e200\n",
            "time,V(x)\n0,0\n5e199,5e199\n1e200,1e200\n",
        ),
        (
            "time,V(x)\n0,0\n1e-200,1e-200\n",
            "time,V(x)\n0,0\n5e-201,5e-201\n1e-200,1e-200\n",
        ),
        // Even a normalized weight can underflow before scaling the signal.
        (
            "time,V(x)\n0,0\n1e308,1e308\n",
            "time,V(x)\n0,0\n1e-308,1e-308\n1e308,1e308\n",
        ),
        // Exact grid points retain their source sample, including tiny endpoints.
        (
            "time,V(x)\n0,1e308\n1,1e-308\n2,0\n",
            "time,V(x)\n0,1e308\n1,1e-308\n2,0\n",
        ),
    ] {
        let output = compare(
            result,
            golden,
            &["--interpolate", "--abstol", "0", "--reltol", "0"],
        );
        assert!(output.status.success(), "{result} => {golden}: {output:?}");
    }
}

#[test]
fn interpolation_holds_event_signals_and_preserves_exact_transition_times() {
    let result = "time,V(x),D(clk),E(sample)\n0,0,0,2\n2,2,1,6\n4,4,0.5,10\n";
    let held = "time,V(x),D(clk),E(sample)\n0,0,0,2\n1,1,0,2\n2,2,1,6\n3,3,1,6\n4,4,0.5,10\n";
    let output = compare(
        result,
        held,
        &["--interpolate", "--abstol", "0", "--reltol", "0"],
    );
    assert!(output.status.success(), "{output:?}");
    let ramped =
        "time,V(x),D(clk),E(sample)\n0,0,0,2\n1,1,0.5,4\n2,2,1,6\n3,3,0.75,8\n4,4,0.5,10\n";
    failed(compare(result, ramped, &["--interpolate"]));
}

#[test]
fn interpolation_holds_explicitly_typed_logic_without_a_name_prefix() {
    let dir = test_dir("comparison_logic");
    let result_path = dir.join("result.json");
    let golden_path = dir.join("golden.json");
    for (path, times, values) in [
        (&result_path, vec![0, 2, 4], vec![0, 1, 0]),
        (&golden_path, vec![0, 1, 2, 3, 4], vec![0, 0, 1, 1, 0]),
    ] {
        std::fs::write(
            path,
            serde_json::json!({
                "scale":{"name":"time", "type":"time", "values":times},
                "signals":[{"name":"clock", "type":"logic", "values":values}]
            })
            .to_string(),
        )
        .unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(result_path)
        .arg(golden_path)
        .arg("--interpolate")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn a_wide_result_can_compare_on_a_dense_golden_grid_within_the_input_budget() {
    let dir = test_dir("comparison_streaming");
    let result = dir.join("wide.csv");
    let golden = dir.join("dense.csv");
    let config = dir.join("limit.toml");
    // Both inputs fit in 5,000 values; materializing every result column on
    // the golden grid would unnecessarily allocate over a million values.
    std::fs::write(&config, "[resources]\nmax_external_data_values=5000\n").unwrap();
    let mut table = String::from("time");
    for i in 0..512 {
        table.push_str(&format!(",V(n{i})"));
    }
    for point in [0, 2047] {
        table.push_str(&format!("\n{point}"));
        for _ in 0..512 {
            table.push_str(&format!(",{point}"));
        }
    }
    table.push('\n');
    std::fs::write(&result, table).unwrap();
    let mut table = String::from("time,V(n0)\n");
    for i in 0..2048 {
        table.push_str(&format!("{i},{i}\n"));
    }
    std::fs::write(&golden, table).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--config"])
        .arg(&config)
        .arg("compare")
        .arg(&result)
        .arg(&golden)
        .args(["--interpolate", "--json", "--abstol", "0", "--reltol", "0"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["num_variables"], 2);
    assert_eq!(report["num_points"], 2048);
    assert_eq!(report["num_differences"], 0);
}
