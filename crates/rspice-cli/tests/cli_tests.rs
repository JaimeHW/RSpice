//! Integration tests for RSpice CLI
//!
//! These tests verify the CLI functionality by running the actual binary.

use assert_cmd::Command;
use predicates::prelude::*;
use rustyhdf5::{AttrValue, File as Hdf5File};
use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixture(name: &str) -> PathBuf {
    let path = fixtures_dir().join(name);
    assert!(
        path.exists(),
        "missing CLI test fixture: {}",
        path.display()
    );
    path
}

fn group_signal_names(file: &Hdf5File, group_name: &str) -> Vec<String> {
    let group = file.group(group_name).expect("group should exist");
    let attrs = group.attrs().expect("group attrs");
    let signal_count = match attrs.get("signal_count") {
        Some(AttrValue::I64(value)) => *value as usize,
        other => panic!("expected signal_count attr, got {:?}", other),
    };

    (0..signal_count)
        .map(|index| {
            let key = format!("signal_{index:04}_name");
            match attrs.get(&key) {
                Some(AttrValue::String(name)) => name.clone(),
                other => panic!("expected string attr for {key}, got {:?}", other),
            }
        })
        .collect()
}

// ============================================================================
// Help and Version Tests
// ============================================================================

#[test]
fn test_help_output() {
    Command::cargo_bin("rspice")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("RSpice"))
        .stdout(predicate::str::contains("run"))
        .stdout(predicate::str::contains("info"))
        .stdout(predicate::str::contains("check"));
}

#[test]
fn test_run_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Run a SPICE simulation"));
}

#[test]
fn test_info_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["info", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Display netlist information"));
}

#[test]
fn test_check_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Validate netlist syntax"));
}

// ============================================================================
// Run Command Tests
// ============================================================================

#[test]
fn test_run_resistor_divider() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "-q"])
        .assert()
        .success();
}

#[test]
fn test_run_missing_file() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "nonexistent_file.sp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error")));
}

// ============================================================================
// Info Command Tests
// ============================================================================

#[test]
fn test_info_resistor_divider() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["info", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Elements"))
        .stdout(predicate::str::contains("Resistors"));
}

#[test]
fn test_info_json_output() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["info", fixture_path.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"resistors\""));
}

// ============================================================================
// Check Command Tests
// ============================================================================

#[test]
fn test_check_valid_netlist() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_check_connectivity() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", fixture_path.to_str().unwrap(), "--connectivity"])
        .assert()
        .success();
}

#[test]
fn test_check_json_output() {
    let fixture_path = fixture("resistor_divider.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", fixture_path.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\""));
}

// ============================================================================
// Global Flag Tests
// ============================================================================

#[test]
fn test_verbose_flag() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["-v", "info", fixture_path.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_quiet_flag() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["-q", "check", fixture_path.to_str().unwrap()])
        .assert()
        .success();
}

// ============================================================================
// Advanced Analysis Argument Parsing Tests
// ============================================================================

#[test]
fn test_run_pss_args_parsing() {
    // Test that PSS arguments are recognized (parsing, not execution)
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--pss-freq"))
        .stdout(predicate::str::contains("--pss-harmonics"));
}

#[test]
fn test_run_hb_args_parsing() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--hb-freq"))
        .stdout(predicate::str::contains("--hb-harmonics"));
}

#[test]
fn test_run_pz_args_parsing() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--pz-input"))
        .stdout(predicate::str::contains("--pz-output"));
}

#[test]
fn test_run_sensitivity_args_parsing() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--sens-output"))
        .stdout(predicate::str::contains("--sens-param"));
}

#[test]
fn test_run_corner_args_parsing() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--corners"));
}

// ============================================================================
// Compare Command Tests
// ============================================================================

#[test]
fn test_compare_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["compare", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("golden"))
        .stdout(predicate::str::contains("tolerance"));
}

// ============================================================================
// Simulation Options Tests
// ============================================================================

#[test]
fn test_run_with_temperature() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "--temp", "85", "-q"])
        .assert()
        .success();
}

#[test]
fn test_run_with_tolerance() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--abstol",
            "1e-12",
            "-q",
        ])
        .assert()
        .success();
}

#[test]
fn test_run_with_maxiter() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--maxiter",
            "100",
            "-q",
        ])
        .assert()
        .success();
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_run_invalid_format_option() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--format",
            "invalid_format",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("error")));
}

#[test]
fn test_check_missing_file() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", "nonexistent_netlist.sp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error")));
}

#[test]
fn test_info_missing_file() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["info", "nonexistent_netlist.sp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error")));
}

#[test]
fn test_convert_missing_input() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["convert", "nonexistent.csv", "output.json", "--to", "json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error")));
}

#[test]
fn test_compare_missing_files() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["compare", "result.csv", "golden.csv"])
        .assert()
        .failure();
}

// ============================================================================
// Output Format Tests
// ============================================================================

#[test]
fn test_run_csv_output() {
    let fixture_path = fixture("rc_transient.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "csv",
            "-q",
        ])
        .assert()
        .success();

    // Verify output file was created
    assert!(output_path.exists(), "CSV output file should be created");
}

#[test]
fn test_run_json_output() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("output.json");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "json",
            "-q",
        ])
        .assert()
        .success();

    assert!(output_path.exists(), "JSON output file should be created");
}

#[test]
fn test_run_tsv_output() {
    let fixture_path = fixture("rc_transient.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("output.tsv");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "tsv",
            "-q",
        ])
        .assert()
        .success();

    assert!(output_path.exists(), "TSV output file should be created");
}

#[test]
fn test_run_raw_ascii_output() {
    let fixture_path = fixture("rc_transient.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("output.raw");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "ascii",
            "-q",
        ])
        .assert()
        .success();

    assert!(
        output_path.exists(),
        "RAW ASCII output file should be created"
    );
}

// ============================================================================
// Report Generation Tests
// ============================================================================

#[test]
fn test_run_junit_report() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let report_path = temp_dir.path().join("report.xml");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--report-file",
            report_path.to_str().unwrap(),
            "--report-format",
            "junit",
            "-q",
        ])
        .assert()
        .success();

    assert!(report_path.exists(), "JUnit report should be created");
    let content = std::fs::read_to_string(&report_path).unwrap();
    assert!(
        content.contains("testsuite"),
        "JUnit XML should contain testsuite element"
    );
}

#[test]
fn test_run_tap_report() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let report_path = temp_dir.path().join("report.tap");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--report-file",
            report_path.to_str().unwrap(),
            "--report-format",
            "tap",
            "-q",
        ])
        .assert()
        .success();

    assert!(report_path.exists(), "TAP report should be created");
    let content = std::fs::read_to_string(&report_path).unwrap();
    assert!(
        content.contains("TAP version") || content.contains("1.."),
        "TAP format should have header"
    );
}

#[test]
fn test_run_meas_json_output() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let meas_path = temp_dir.path().join("meas.json");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--meas-file",
            meas_path.to_str().unwrap(),
            "-q",
        ])
        .assert()
        .success();

    assert!(meas_path.exists(), "Measurement JSON should be created");
    let content = std::fs::read_to_string(&meas_path).unwrap();
    assert!(
        content.starts_with("{") || content.starts_with("["),
        "Should be valid JSON"
    );
}

// ============================================================================
// Simulation Result Tests
// ============================================================================

#[test]
fn test_run_dc_simulation_output() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete").or(predicate::str::contains("DC")));
}

#[test]
fn test_run_transient_simulation() {
    let fixture_path = fixture("rc_transient.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Transient").or(predicate::str::contains("complete")));
}

#[test]
fn test_run_ac_simulation() {
    let fixture_path = fixture("rc_lowpass.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("AC").or(predicate::str::contains("complete")));
}

#[test]
fn test_run_with_meas_flag() {
    let fixture_path = fixture("rc_transient.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "--meas"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Measurement")
                .or(predicate::str::contains("MEAS").or(predicate::str::contains("complete"))),
        );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_run_batch_mode() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "-b", "-q"])
        .assert()
        .success();
}

#[test]
fn test_run_with_define() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "-D", "TEMP=27", "-q"])
        .assert()
        .success();
}

#[test]
fn test_run_multiple_flags() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "--temp",
            "50",
            "--maxiter",
            "200",
            "--abstol",
            "1e-10",
            "-q",
        ])
        .assert()
        .success();
}

#[test]
fn test_info_element_count() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["info", fixture_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Elements").or(predicate::str::contains("element")));
}

#[test]
fn test_check_valid_no_errors() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", fixture_path.to_str().unwrap()])
        .assert()
        .success();
}

// ============================================================================
// Convert Command Tests
// ============================================================================

#[test]
fn test_convert_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["convert", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Convert"));
}

// ============================================================================
// Monte Carlo and Compression Tests
// ============================================================================

#[test]
fn test_run_monte_carlo_args() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    // Just test argument parsing, not full MC run
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("monte-carlo"));
}

#[test]
fn test_run_compression_args() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("compress"));
}

// ============================================================================
// Compile-VA Command Tests
// ============================================================================

#[test]
fn test_compile_va_help() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["compile-va", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Verilog-A").or(predicate::str::contains("Compile")));
}

#[test]
fn test_compile_va_missing_file() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["compile-va", "nonexistent.va"])
        .assert()
        .failure();
}

// ============================================================================
// HDF5 Tests
// ============================================================================

#[test]
fn test_hdf5_format_listed() {
    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hdf5"));
}

#[test]
fn test_run_hdf5_output_creates_operating_point_file() {
    let fixture_path = fixture("resistor_divider.sp");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("output.h5");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "hdf5",
            "-q",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("HDF5 output should open");
    let root = file.root();
    let attrs = root.attrs().expect("root attrs");
    assert!(
        matches!(attrs.get("title"), Some(AttrValue::String(title)) if title == "DC Operating Point")
    );

    let mut groups = root.groups().expect("root groups");
    groups.sort();
    assert!(groups.contains(&"operating_point".to_string()));

    let operating_point = file
        .group("operating_point")
        .expect("operating point group");
    let mut datasets = operating_point.datasets().expect("datasets");
    datasets.sort();
    assert!(datasets.contains(&"independent".to_string()));
    assert!(datasets.contains(&"signal_0000".to_string()));
}

#[test]
fn test_run_hdf5_operating_point_preserves_named_nodes_and_currents() {
    let fixture_path = fixture("named_divider.sp");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("named_op.h5");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "hdf5",
            "-q",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("HDF5 output should open");
    let signal_names = group_signal_names(&file, "operating_point");
    assert!(signal_names.contains(&"V(IN)".to_string()));
    assert!(signal_names.contains(&"V(OUT)".to_string()));
    assert!(signal_names.contains(&"I(V1)".to_string()));
}

#[test]
fn test_run_hdf5_transient_preserves_named_nodes() {
    let fixture_path = fixture("named_rc_transient.sp");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("named_tran.h5");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "hdf5",
            "-q",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("HDF5 output should open");
    let signal_names = group_signal_names(&file, "transient");
    assert!(signal_names.contains(&"V(IN)".to_string()));
    assert!(signal_names.contains(&"V(OUT)".to_string()));
}

#[test]
fn test_run_hdf5_ac_preserves_named_nodes_and_branch_currents() {
    let fixture_path = fixture("named_rc_lowpass.sp");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("named_ac.h5");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "hdf5",
            "-q",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("HDF5 output should open");
    let signal_names = group_signal_names(&file, "ac");
    assert!(signal_names.contains(&"V(IN)".to_string()));
    assert!(signal_names.contains(&"V(OUT)".to_string()));
    assert!(signal_names.contains(&"I(V1)".to_string()));
}

#[test]
fn test_run_hdf5_dc_sweep_exports_all_named_nodes() {
    let fixture_path = fixture("ladder_dc_sweep.sp");

    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("ladder_dc.h5");

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "run",
            fixture_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--format",
            "hdf5",
            "-q",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("HDF5 output should open");
    let signal_names = group_signal_names(&file, "dc_sweep");
    assert_eq!(signal_names.len(), 6);
    assert!(signal_names.contains(&"V(IN)".to_string()));
    assert!(signal_names.contains(&"V(N5)".to_string()));
}

#[test]
fn test_run_meas_uses_named_transient_node_aliases() {
    let fixture_path = fixture("named_rc_transient.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "--meas"])
        .assert()
        .success()
        .stdout(predicate::str::contains("vout_max").or(predicate::str::contains("VOUT_MAX")));
}

#[test]
fn test_run_meas_delay_statement_reports_success() {
    let fixture_path = fixture("delay_measurement.sp");

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["run", fixture_path.to_str().unwrap(), "--meas"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prop_delay").or(predicate::str::contains("PROP_DELAY")))
        .stdout(predicate::str::contains("FAILED").not());
}

#[test]
fn test_convert_csv_to_hdf5_creates_waveform_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("input.csv");
    let output_path = temp_dir.path().join("output.h5");

    std::fs::write(&input_path, "time,V(out)\n0.0,0.0\n1.0,1.25\n").unwrap();

    Command::cargo_bin("rspice")
        .unwrap()
        .args([
            "convert",
            input_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
            "--to",
            "hdf5",
        ])
        .assert()
        .success();

    let file = Hdf5File::open(&output_path).expect("converted HDF5 should open");
    let transient = file.group("transient").expect("transient group");
    let attrs = transient.attrs().expect("transient attrs");
    assert!(
        matches!(attrs.get("independent_name"), Some(AttrValue::String(name)) if name == "time")
    );
    let values = transient
        .dataset("signal_0000")
        .expect("signal dataset")
        .read_f64()
        .expect("signal values");
    assert_eq!(values, vec![0.0, 1.25]);
}
