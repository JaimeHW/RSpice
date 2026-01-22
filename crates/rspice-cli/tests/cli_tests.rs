//! Integration tests for RSpice CLI
//!
//! These tests verify the CLI functionality by running the actual binary.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixture(name: &str) -> PathBuf {
    fixtures_dir().join(name)
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
    if !fixture_path.exists() {
        return; // Skip if fixture doesn't exist
    }

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
    if !fixture_path.exists() {
        return;
    }

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
    if !fixture_path.exists() {
        return;
    }

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
    if !fixture_path.exists() {
        return;
    }

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
    if !fixture_path.exists() {
        return;
    }

    Command::cargo_bin("rspice")
        .unwrap()
        .args(["check", fixture_path.to_str().unwrap(), "--connectivity"])
        .assert()
        .success();
}

#[test]
fn test_check_json_output() {
    let fixture_path = fixture("resistor_divider.sp");
    if !fixture_path.exists() {
        return;
    }

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
// HDF5 Output Test (Feature Disabled)
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
