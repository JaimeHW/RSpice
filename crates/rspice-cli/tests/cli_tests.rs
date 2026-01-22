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
