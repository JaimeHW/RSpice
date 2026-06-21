//! The exit-status contract automation relies on:
//! 0 = success, 1 = simulation error, 2 = usage error, 3 = verification
//! failure (failed .MEAS), 65 = parse error, 66 = missing input.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_exit_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .output()
        .expect("run rspice")
}

fn run_rspice_with_output_format_env(args: &[&str], format: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .env("RSPICE_OUTPUT_FORMAT", format)
        .env_remove("RSPICE_TEMPERATURE")
        .output()
        .expect("run rspice")
}

fn run_rspice_with_envs(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rspice"));
    cmd.args(args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE");
    for (name, value) in envs {
        cmd.env(name, value);
    }
    cmd.output().expect("run rspice")
}

fn run_rspice_in_dir(args: &[&str], cwd: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .current_dir(cwd)
        .args(args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .output()
        .expect("run rspice")
}

const PASSING_DECK: &str = "* RC with a measurement that evaluates\n\
     V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
     R1 in out 1k\n\
     C1 out 0 100p\n\
     .TRAN 1n 2u\n\
     .MEAS TRAN vmax MAX V(out)\n\
     .END\n";

const FAILING_MEAS_DECK: &str = "* RC with a measurement that cannot trigger\n\
     V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
     R1 in out 1k\n\
     C1 out 0 100p\n\
     .TRAN 1n 2u\n\
     .MEAS TRAN never TRIG V(out) VAL=0.5 RISE=1 TARG V(out) VAL=9.9 RISE=1\n\
     .END\n";

#[test]
fn passing_measurements_exit_zero() {
    let dir = test_dir("pass");
    let deck = dir.join("pass.sp");
    std::fs::write(&deck, PASSING_DECK).expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn failed_measurement_exits_three() {
    let dir = test_dir("meas_fail");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "failed .MEAS must exit 3; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("NEVER"),
        "stderr should name the failed measurement: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn allow_failed_meas_restores_exit_zero() {
    let dir = test_dir("meas_allow");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-failed-meas",
    ]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Measurements for an analysis that never ran are failures, not skips.
#[test]
fn unevaluated_measurement_exits_three() {
    let dir = test_dir("unevaluated");
    let deck = dir.join("ac_meas.sp");
    std::fs::write(
        &deck,
        "* tran-only deck with an AC measurement\n\
         V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .TRAN 1n 2u\n\
         .MEAS AC gain MAX V(out)\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "unevaluated .MEAS must exit 3; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Explicit requested modes still have to fail loudly for `.MEAS`
/// statements that their analysis did not evaluate.
#[test]
fn requested_mode_unevaluated_measurement_exits_three() {
    let dir = test_dir("requested_mode_unevaluated");
    let deck = dir.join("sparam_meas.sp");
    let summary = dir.join("summary.json");
    std::fs::write(
        &deck,
        "* s-parameter request with an unrelated transient measurement\n\
         R1 in out 50\n\
         R2 out 0 50\n\
         .AC LIN 1 1k 1k\n\
         .MEAS TRAN SHOULD_FAIL MAX V(out)\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--sparam",
        "in,0,out,0",
        "--summary",
        summary.to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "requested mode must not skip failed .MEAS checks; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SHOULD_FAIL"),
        "stderr should name the unevaluated measurement: {stderr}"
    );

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).expect("summary file"))
            .expect("valid json");
    assert_eq!(
        json["passed"], false,
        "summary verdict must fail for requested-mode measurement escape: {json}"
    );
    assert_eq!(
        json["runs"][0]["passed"], false,
        "per-run verdict must match failed requested-mode measurement: {json}"
    );
    assert!(
        json["runs"][0]["duration_secs"]
            .as_f64()
            .unwrap_or_default()
            > 0.0,
        "requested-mode run duration must be measured, not hardcoded to zero: {json}"
    );
    assert_eq!(
        json["runs"][0]["measurements"][0]["passed"], false,
        "unevaluated measurement must be recorded: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn parallel_status_lines_mark_measurement_failures() {
    let dir = test_dir("parallel_meas_status");
    let deck = dir.join("data_fail.sp");
    std::fs::write(
        &deck,
        "* .DATA expansion where every concrete run misses its GOAL\n\
         .param rbot=1k\n\
         V1 in 0 10\n\
         R1 in out 1k\n\
         R2 out 0 {rbot}\n\
         .data pts rbot\n\
         1k\n\
         2k\n\
         .enddata\n\
         .dc V1 10 10 1 sweep data=pts\n\
         .meas dc vout MAX V(out) GOAL=1 TOL=0.1\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["run", deck.to_str().unwrap(), "--jobs", "2"]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "failed parallel measurements must exit 3; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("pts row 1:") && stdout.contains("pts row 2:"),
        "parallel status lines should mark each failed row, stdout: {stdout}"
    );
    assert!(
        stdout.contains("measurement(s) failed") && stdout.contains("VOUT"),
        "parallel status lines should name failed measurements, stdout: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Conflicting parallel voltage sources produce a singular system; the
/// NaN/Inf solution must be an error, not a quiet success.
#[test]
fn nonfinite_result_is_a_simulation_error() {
    let dir = test_dir("nonfinite");
    let deck = dir.join("vloop.sp");
    std::fs::write(
        &deck,
        "* conflicting parallel voltage sources\n\
         V1 a 0 5\n\
         V2 a 0 3\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "non-finite OP must exit 1; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("non-finite"),
        "stderr should explain the non-finite solution: {stderr}"
    );

    // The escape hatch restores the old behavior for debugging.
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-nonfinite",
    ]);
    assert_eq!(output.status.code(), Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fourier_ground_probe_exits_zero_without_panicking() {
    let dir = test_dir("four_ground");
    let deck = dir.join("four_ground.sp");
    std::fs::write(
        &deck,
        "* Fourier ground probe should be a zero-valued waveform\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in 0 1k\n\
         .four 1k V(0)\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "ground Fourier probe must not panic; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fourier_invalid_numeric_node_is_a_simulation_error() {
    let dir = test_dir("four_invalid_node");
    let deck = dir.join("four_invalid_node.sp");
    std::fs::write(
        &deck,
        "* Fourier invalid numeric node should fail cleanly\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in 0 1k\n\
         .four 1k V(999)\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "invalid Fourier node must be a simulation error, not a panic; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("V(999)") && stderr.contains("not available"),
        "stderr should name the unavailable Fourier output: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn parse_error_exits_sixty_five() {
    let dir = test_dir("parse");
    let deck = dir.join("broken.sp");
    std::fs::write(&deck, "* broken\nR1 in out\n.tran 1u 1m\n.end\n").expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(65));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_data_sweep_exits_sixty_five() {
    let dir = test_dir("bad_data");
    let deck = dir.join("bad_data.sp");
    std::fs::write(
        &deck,
        "* bad .DATA sweep\n\
         .param vdd=1 rl=1k\n\
         V1 in 0 {vdd}\n\
         R1 in 0 {rl}\n\
         .data corners vdd rl\n\
         1.0 1k\n\
         1.2\n\
         .enddata\n\
         .dc data=corners\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(65),
        "malformed .DATA should be a parse error; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(".data corners") && stderr.contains("does not fill 2 columns"),
        "stderr should explain the malformed .DATA table: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn failed_step_point_exits_one_without_truncated_success() {
    let dir = test_dir("bad_step_point");
    let deck = dir.join("bad_step_point.sp");
    std::fs::write(
        &deck,
        "* bad .STEP point\n\
         V1 in 0 10\n\
         R1 in out {rval}\n\
         R2 out 0 1k\n\
         .param rval=1k\n\
         .step param rval list 1k 0\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);

    assert_eq!(
        output.status.code(),
        Some(1),
        "failed .STEP point must fail the command; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr_upper = stderr.to_ascii_uppercase();
    assert!(
        stderr_upper.contains(".STEP PARAM RVAL = 0"),
        "stderr should identify the failed step point: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn invalid_env_output_format_exits_seventy_eight() {
    let dir = test_dir("bad_env_output_format");
    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    let out = dir.join("out.raw");

    let output = run_rspice_with_output_format_env(
        &[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ],
        "definitely-not-a-format",
    );

    assert_eq!(
        output.status.code(),
        Some(78),
        "bad RSPICE_OUTPUT_FORMAT must be a config error; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("output.format") && stderr.contains("definitely-not-a-format"),
        "stderr should name the invalid configured format: {stderr}"
    );
    assert!(
        !out.exists(),
        "invalid format must not write fallback raw output"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn invalid_env_temperature_exits_seventy_eight() {
    let dir = test_dir("bad_env_temperature");
    let deck = dir.join("divider.sp");
    let out = dir.join("out.raw");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");

    let output = run_rspice_with_envs(
        &[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ],
        &[("RSPICE_TEMPERATURE", "not-a-number")],
    );

    assert_eq!(
        output.status.code(),
        Some(78),
        "bad RSPICE_TEMPERATURE must be a config error; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("RSPICE_TEMPERATURE") && stderr.contains("not-a-number"),
        "stderr should name the invalid environment variable: {stderr}"
    );
    assert!(
        !out.exists(),
        "invalid temperature must stop before writing output"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn invalid_config_output_format_exits_seventy_eight() {
    let dir = test_dir("bad_config_output_format");
    let deck = dir.join("divider.sp");
    let config = dir.join("rspice.toml");
    let out = dir.join("out.raw");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[output]\nformat = 'definitely-not-a-format'\n")
        .expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(78),
        "bad config output.format must be a config error; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("output.format") && stderr.contains("definitely-not-a-format"),
        "stderr should name the invalid configured format: {stderr}"
    );
    assert!(
        !out.exists(),
        "invalid format must not write fallback raw output"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_project_config_exits_seventy_eight() {
    let dir = test_dir("bad_project_config");
    let deck = dir.join("divider.sp");
    let out = dir.join("out.csv");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    std::fs::write(dir.join(".rspicerc"), "[output\nformat = 'csv'\n").expect("write config");

    let output = run_rspice_in_dir(
        &["--quiet", "run", "divider.sp", "-o", "out.csv", "-f", "csv"],
        &dir,
    );

    assert_eq!(
        output.status.code(),
        Some(78),
        "malformed project .rspicerc must be a config error; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(".rspicerc"),
        "stderr should name the project config: {stderr}"
    );
    assert!(!out.exists(), "bad project config must not write output");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn explicit_config_unknown_key_exits_seventy_eight() {
    let dir = test_dir("unknown_config_key");
    let deck = dir.join("divider.sp");
    let config = dir.join("rspice.toml");
    let out = dir.join("out.raw");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[output]\nformt = 'csv'\n").expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(78),
        "unknown config keys must be config errors; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("formt"),
        "stderr should name the unknown key: {stderr}"
    );
    assert!(!out.exists(), "unknown config key must not write output");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn explicit_config_invalid_numeric_exits_seventy_eight() {
    let dir = test_dir("bad_config_numeric");
    let deck = dir.join("divider.sp");
    let config = dir.join("rspice.toml");
    let out = dir.join("out.raw");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[simulation]\nreltol = -1.0\n").expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(78),
        "invalid numeric config values must be config errors; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reltol"),
        "stderr should name the invalid numeric field: {stderr}"
    );
    assert!(
        !out.exists(),
        "invalid numeric config must not write output"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn explicit_config_still_applies_environment_overrides() {
    let dir = test_dir("explicit_config_env");
    let deck = dir.join("divider.sp");
    let config = dir.join("rspice.toml");
    let out = dir.join("result.txt");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[output]\nformat = 'raw'\n").expect("write config");

    let output = run_rspice_with_envs(
        &[
            "--config",
            config.to_str().unwrap(),
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ],
        &[("RSPICE_OUTPUT_FORMAT", "csv")],
    );

    assert_eq!(
        output.status.code(),
        Some(0),
        "env override over explicit --config should succeed; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let body = std::fs::read_to_string(&out).expect("read output");
    assert!(
        body.starts_with("signal,value"),
        "RSPICE_OUTPUT_FORMAT=csv must override explicit config raw format: {body}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn check_json_warning_only_valid_matches_non_strict_exit_code() {
    let dir = test_dir("check_warning_json");
    let deck = dir.join("current_only.sp");
    std::fs::write(
        &deck,
        "* current-source-only floating node warning\nI1 n 0 1m\n.op\n.end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "check", "--json", deck.to_str().unwrap()]);

    assert_eq!(
        output.status.code(),
        Some(0),
        "warning-only non-strict check should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
        panic!(
            "valid JSON stdout expected: {err}; stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });
    assert_eq!(
        json["valid"], true,
        "non-strict JSON verdict must match exit code: {json}"
    );
    assert_eq!(
        json["strict_valid"], false,
        "strict_valid should still reflect warnings: {json}"
    );
    assert!(
        json["warnings"]
            .as_array()
            .is_some_and(|warnings| !warnings.is_empty()),
        "warning-only fixture should report warnings: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn missing_input_exits_sixty_six() {
    let output = run_rspice(&["--quiet", "run", "definitely_missing_deck.sp"]);
    assert_eq!(output.status.code(), Some(66));
}

/// --timeout stops a long transient at the next safe point and exits with
/// the GNU timeout convention (124).
#[test]
fn timeout_exits_one_twenty_four() {
    let dir = test_dir("timeout");
    let deck = dir.join("slow.sp");
    // 100 simulated seconds of a 1kHz sine at 1ns steps: far longer than
    // the 1-second budget on any machine.
    std::fs::write(
        &deck,
        "* long transient\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .tran 1n 100\n\
         .end\n",
    )
    .expect("write deck");

    let start = std::time::Instant::now();
    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap(), "--timeout", "1"]);
    assert_eq!(
        output.status.code(),
        Some(124),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        start.elapsed() < std::time::Duration::from_secs(30),
        "timeout must stop the run promptly"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// The --summary JSON verdict agrees with the exit code.
#[test]
fn summary_json_carries_the_verdict() {
    let dir = test_dir("summary");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");
    let summary = dir.join("summary.json");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--summary",
        summary.to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(3));

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).expect("summary file"))
            .expect("valid json");
    assert_eq!(
        json["passed"], false,
        "summary verdict must match exit code"
    );
    assert_eq!(
        json["runs"][0]["passed"], false,
        "per-run verdict must match failed measurement"
    );
    assert_eq!(json["tool"]["name"], "rspice");
    assert_eq!(
        json["runs"][0]["measurements"][0]["passed"], false,
        "failed measurement recorded: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn output_write_failure_keeps_io_exit_and_not_success_summary() {
    let dir = test_dir("output_write_io");
    let deck = dir.join("op.sp");
    std::fs::write(
        &deck,
        "* op output failure\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");
    let unwritable_output = dir.join("output-directory");
    std::fs::create_dir(&unwritable_output).expect("create output directory");
    let summary = dir.join("summary.json");

    let output = run_rspice(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        unwritable_output.to_str().unwrap(),
        "--summary",
        summary.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(74),
        "output write failure must preserve I/O exit; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Simulation complete"),
        "failed output write must not print completion: {stdout}"
    );
    if summary.exists() {
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&summary).expect("summary file"))
                .expect("valid json");
        assert_eq!(json["passed"], false, "summary must not pass: {json}");
        assert!(
            json["outputs"]
                .as_array()
                .is_none_or(|outputs| outputs.is_empty()),
            "summary must not list unwritten artifacts: {json}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// Failed measurements still land in report files alongside the bad exit
/// status, and the run-level testcase carries the same failed verdict so CI
/// dashboards and shell checks agree.
#[test]
fn failed_measurement_marks_run_verdict_in_machine_reports() {
    let dir = test_dir("reports");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");
    let junit = dir.join("report.xml");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--report-format",
        "junit",
        "--report-file",
        junit.to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(3));

    let xml = std::fs::read_to_string(&junit).expect("read junit report");
    assert!(
        xml.contains("failures=\"2\""),
        "JUnit report should count the failed run verdict and measurement: {xml}"
    );
    assert!(
        xml.contains("Run verification failed"),
        "JUnit report should mark the run-level testcase failed: {xml}"
    );
    assert!(
        xml.contains("Measurement failed"),
        "JUnit report should carry the failure element: {xml}"
    );

    let tap = dir.join("report.tap");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--report-format",
        "tap",
        "--report-file",
        tap.to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(3));

    let tap = std::fs::read_to_string(&tap).expect("read tap report");
    assert!(
        tap.contains("not ok 1 - fail"),
        "TAP report should mark the run-level testcase failed: {tap}"
    );
    assert!(
        tap.contains("Run verification failed"),
        "TAP report should explain the run verdict failure: {tap}"
    );
    assert!(
        tap.contains("not ok 2 - NEVER"),
        "TAP report should still include the failed measurement: {tap}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
