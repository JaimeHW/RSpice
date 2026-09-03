//! The exit-status contract automation relies on.
//!
//! Every nonzero status is derived from one failure category, and for
//! anything the engine produced that category is the engine's own — so no
//! typed engine failure exits 1, which would tell automation nothing. The
//! published table lives in `crates/rspice-cli/README.md`.
//!
//! These tests drive a real deck or flag per category through the built
//! binary rather than constructing errors, because a code nothing reaches is
//! a code nobody can rely on.

mod common;

use common::test_dir;

use std::process::Command;

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .env_remove("RUST_LOG")
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

/// Conflicting parallel voltage sources are a topology error. This must remain
/// a simulation failure even when the non-finite result escape hatch is enabled.
#[test]
fn conflicting_voltage_sources_are_a_simulation_error() {
    let dir = test_dir("singular");
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
        Some(80),
        "singular OP must exit with the simulation category; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("matrix is singular") || stderr.contains("duplicate constraints"),
        "stderr should explain the singular topology: {stderr}"
    );

    // The non-finite export escape hatch must not mask topology failures.
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-nonfinite",
    ]);
    assert_eq!(output.status.code(), Some(80));

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
         .tran 10u 2m\n\
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
fn fourier_invalid_numeric_node_is_rejected_before_simulation() {
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
        Some(65),
        "invalid Fourier node must be a semantic input error, not a panic; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(".FOUR node '999' via V") && stderr.contains("undefined output symbols"),
        "stderr should retain the typed unavailable-output context: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn parse_error_exits_sixty_five() {
    let dir = test_dir("parse");
    let deck = dir.join("broken.sp");
    std::fs::write(&deck, "* broken\nV1 in 0 1\n.tran nope 1m\n.end\n").expect("write deck");

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
fn configured_batch_limit_rejects_multi_run_before_execution() {
    let dir = test_dir("bounded_multi_run");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* bounded multi-run\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .op\n\
         .alter second\n\
         V1 in 0 2\n\
         .end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[resources]\nmax_batch_runs = 1\n").expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(65),
        "resource rejection must be an input-policy error; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("batch_runs limit exceeded: requested 2, limit 1"),
        "typed batch limit must reach the CLI: {stderr}"
    );
    assert!(
        stderr.contains("resources.max_batch_runs"),
        "operator guidance must name the config key: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn configured_worker_limit_rejects_excessive_explicit_parallelism() {
    let dir = test_dir("bounded_workers");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* bounded workers\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .op\n\
         .alter second\n\
         V1 in 0 2\n\
         .alter third\n\
         V1 in 0 3\n\
         .end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[resources]\nmax_parallel_workers = 2\n").expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--error-format",
        "json",
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--jobs",
        "3",
    ]);

    assert_eq!(
        output.status.code(),
        Some(75),
        "worker-policy rejection is a resource-limit outcome; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("structured diagnostic");
    assert_eq!(diagnostic["error"]["code"], "resource_limit");
    assert_eq!(diagnostic["error"]["resource"], "parallel_workers");
    assert_eq!(diagnostic["error"]["requested"], 3);
    assert_eq!(diagnostic["error"]["limit"], 2);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn automatic_parallelism_clamps_to_configured_worker_limit() {
    let dir = test_dir("automatic_worker_cap");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    let summary = dir.join("summary.json");
    std::fs::write(
        &deck,
        "* automatic worker cap\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .op\n\
         .alter second\n\
         V1 in 0 2\n\
         .alter third\n\
         V1 in 0 3\n\
         .end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[resources]\nmax_parallel_workers = 1\n").expect("write config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--jobs",
        "0",
        "--summary",
        summary.to_str().unwrap(),
    ]);

    assert_eq!(
        output.status.code(),
        Some(0),
        "automatic parallelism must clamp instead of failing; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).expect("automatic worker summary"))
            .expect("valid summary");
    assert_eq!(report["execution"]["requested_jobs"], 0);
    assert_eq!(report["execution"]["workers"], 1);
    assert_eq!(report["resource_limits"]["max_parallel_workers"], 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn resource_policy_applies_to_run_check_and_info_ingestion() {
    let dir = test_dir("bounded_frontends");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(&deck, "* exceeds eight bytes\nV1 in 0 1\n.op\n.end\n").expect("write deck");
    std::fs::write(&config, "[resources]\nmax_netlist_bytes = 8\n").expect("write config");

    for subcommand in ["run", "check", "info"] {
        let output = run_rspice(&[
            "--config",
            config.to_str().unwrap(),
            "--quiet",
            subcommand,
            deck.to_str().unwrap(),
        ]);
        assert_eq!(
            output.status.code(),
            Some(65),
            "{subcommand} must apply the same ingestion policy; stdout: {}; stderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("netlist_bytes limit exceeded"),
            "{subcommand} must preserve typed resource diagnostics"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn resource_environment_override_takes_precedence_over_config_file() {
    let dir = test_dir("bounded_env_override");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* bounded multi-run\nV1 in 0 1\nR1 in 0 1k\n.op\n.alter second\nV1 in 0 2\n.end\n",
    )
    .expect("write deck");
    std::fs::write(&config, "[resources]\nmax_batch_runs = 4\n").expect("write config");

    let output = run_rspice_with_envs(
        &[
            "--config",
            config.to_str().unwrap(),
            "--quiet",
            "run",
            deck.to_str().unwrap(),
        ],
        &[("RSPICE_MAX_BATCH_RUNS", "1")],
    );

    assert_eq!(output.status.code(), Some(65));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("batch_runs limit exceeded: requested 2, limit 1")
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn failed_step_point_fails_the_command_without_truncated_success() {
    let dir = test_dir("bad_step_point");
    let deck = dir.join("bad_step_point.sp");
    std::fs::write(
        &deck,
        "* bad .STEP point\n\
         V1 in 0 1\n\
         D1 in 0 dmod\n\
         .model dmod D\n\
         .step D1 list 1 2\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);

    assert_eq!(
        output.status.code(),
        Some(80),
        "failed .STEP point must fail the command; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr_upper = stderr.to_ascii_uppercase();
    assert!(
        stderr_upper.contains(".STEP RUN-") && stderr_upper.contains("(DEVICE D1 = 1) PREFLIGHT"),
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

#[test]
fn json_errors_publish_stable_automation_contract() {
    let output = run_rspice(&[
        "--quiet",
        "--error-format",
        "json",
        "run",
        "definitely_missing_json_error_deck.sp",
    ]);
    assert_eq!(output.status.code(), Some(66));
    assert!(output.stdout.is_empty(), "fatal JSON belongs on stderr");

    let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap_or_else(|error| {
        panic!(
            "stderr must be exactly one JSON document: {error}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    assert_eq!(json["schema_version"], 2);
    assert_eq!(json["tool"]["name"], "rspice");
    assert_eq!(json["tool"]["commit"], env!("RSPICE_BUILD_COMMIT"));
    assert_eq!(json["error"]["code"], "input_not_found");
    assert_eq!(json["error"]["category"], "input_not_found");
    assert_eq!(json["error"]["retryable"], false);
    assert_eq!(json["error"]["exit_code"], 66);
}

#[test]
fn json_logs_are_correlated_newline_delimited_records() {
    let dir = test_dir("json_logs");
    let deck = dir.join("op.sp");
    std::fs::write(
        &deck,
        "* observable run\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");

    let output = run_rspice(&[
        "--log-level",
        "info",
        "--log-format",
        "json",
        "run",
        deck.to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let records = String::from_utf8(output.stderr)
        .expect("UTF-8 logs")
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("one JSON log record"))
        .collect::<Vec<_>>();
    assert!(
        !records.is_empty(),
        "info-level run must emit diagnostic logs"
    );
    assert!(records.iter().all(|record| record["run_id"].is_string()));
    assert!(records.iter().all(|record| record["timestamp"].is_string()));
    assert!(records.iter().any(|record| {
        record["message"]
            .as_str()
            .is_some_and(|message| message.contains("Loading netlist"))
    }));

    let _ = std::fs::remove_dir_all(&dir);
}

/// --timeout stops a long transient at the next safe point and exits with
/// the GNU timeout convention (124).
#[test]
fn timeout_exits_one_twenty_four() {
    let dir = test_dir("timeout");
    let deck = dir.join("slow.sp");
    let config = dir.join("rspice.toml");
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
    // This fixture intentionally exceeds production defaults so it can test
    // runtime cancellation rather than point-count admission control.
    std::fs::write(
        &config,
        "[resources]\nmax_analysis_points = 100000000001\nmax_result_values = 500000000005\n",
    )
    .expect("write config");

    let start = std::time::Instant::now();
    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--timeout",
        "1",
    ]);
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
    assert_eq!(json["tool"]["commit"], env!("RSPICE_BUILD_COMMIT"));
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["status"], "failed");
    assert_eq!(json["counts"]["runs"], 1);
    assert_eq!(json["counts"]["failed_measurements"], 1);
    assert_eq!(json["execution"]["workers"], 1);
    assert!(json["resource_limits"]["max_result_values"].is_number());
    assert_eq!(
        json["runs"][0]["measurements"][0]["passed"], false,
        "failed measurement recorded: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn output_write_failure_is_an_output_commit_failure_and_not_a_success_summary() {
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
        Some(73),
        "a run whose results could not be published must exit with the output-commit code; stdout={} stderr={}",
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

//=============================================================================
// One exit code per failure category
//
// The categories below are reachable from a deck or a flag, so each gets an
// end-to-end test through the built binary. Three are not covered here:
//
// - `materialization` is reachable — a deck whose analysis card set is
//   coordinate-dependent produces one — and is driven end to end by
//   `tests/step_cartesian.rs`, where the `.STEP` fixtures already live.
// - `result_schema` is an internal consistency failure no authored deck can
//   produce; it is covered by `rspice-core`'s `simulation_error_contract`,
//   and its exit code by `exit_code_for`'s own table test.
// - `solver` and `convergence` need a deck that defeats every convergence aid
//   the engine applies, which is a moving target: pinning one here would make
//   this suite fail for the wrong reason whenever the aids improve. Their
//   descriptors are pinned in `rspice-core` and their codes in `cli::error`.
//
// `cancellation` needs a real interrupt; its sibling path is covered by
// `--timeout` rather than by sending a signal from a test.
//=============================================================================

/// The machine-readable diagnostic a failing invocation printed.
fn fatal_diagnostic(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_slice(&output.stderr).unwrap_or_else(|error| {
        panic!(
            "stderr must be exactly one JSON document: {error}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

/// Run one deck and return its fatal diagnostic with the observed status.
fn run_json(dir: &std::path::Path, name: &str, deck: &str, extra: &[&str]) -> serde_json::Value {
    let path = dir.join(name);
    std::fs::write(&path, deck).expect("write deck");
    let path_text = path.to_str().expect("utf-8 deck path").to_owned();
    let mut args = vec!["--error-format", "json", "--quiet", "run", &path_text];
    args.extend_from_slice(extra);
    let output = run_rspice(&args);
    let mut diagnostic = fatal_diagnostic(&output);
    diagnostic["observed_exit_code"] = serde_json::json!(output.status.code());
    diagnostic
}

#[test]
fn capability_refusals_exit_sixty_nine_with_their_token() {
    let dir = test_dir("category_capability");

    // A netlist construct the grammar recognizes and declines to lower.
    let parsed = run_json(
        &dir,
        "ydevice.sp",
        "* Y-device family with no model program\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         YDELAY delay1 2 0 1 0 TD=10N\n\
         .op\n\
         .end\n",
        &[],
    );
    assert_eq!(parsed["observed_exit_code"], 69);
    assert_eq!(parsed["error"]["category"], "capability");
    assert_eq!(parsed["error"]["code"], "unsupported_capability");
    assert_eq!(
        parsed["error"]["capability"],
        "netlist.xyce.ydevice.no_model_program"
    );
    assert_eq!(
        parsed["error"]["line"], 4,
        "a refusal must carry the line it was authored on: {parsed}"
    );

    // A device the elaborator understands and this build cannot stamp.
    let elaborated = run_json(
        &dir,
        "ltra.sp",
        "* RLGC LTRA with shunt conductance\n\
         V1 in 0 1\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=1n C=1p LEN=1\n\
         Rl out 0 50\n\
         .op\n\
         .end\n",
        &[],
    );
    assert_eq!(elaborated["observed_exit_code"], 69);
    assert_eq!(elaborated["error"]["category"], "capability");
    assert_eq!(
        elaborated["error"]["capability"],
        "device.ltra.rlgc_conductance"
    );

    // An analysis that understands the device and does not cover it.
    let analysis = run_json(
        &dir,
        "pz.sp",
        "* pole-zero with a distributed line\n\
         V1 in 0 1\n\
         T1 in 0 out 0 Z0=50 TD=1n\n\
         R1 out 0 50\n\
         .pz in 0 out 0 vol pz\n\
         .end\n",
        &[],
    );
    assert_eq!(analysis["observed_exit_code"], 69);
    assert_eq!(analysis["error"]["capability"], "analysis.pz.device");

    let _ = std::fs::remove_dir_all(&dir);
}

/// The same refusal reached through a `.DC` card rather than a `.OP` one.
///
/// The DC sweep used to stringify whatever the engine returned, so this deck
/// exited 80 while the `.OP` spelling of it exited 69 - one refusal, two
/// statuses, decided by which card happened to name it.
#[test]
fn a_dc_sweep_keeps_the_engine_category_of_what_refused_it() {
    let dir = test_dir("dc_sweep_capability");
    let diagnostic = run_json(
        &dir,
        "ltra_dc.sp",
        "* RLGC LTRA with shunt conductance, swept\n\
         V1 in 0 1\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=1n C=1p LEN=1\n\
         Rl out 0 50\n\
         .dc V1 0 1 0.5\n\
         .end\n",
        &[],
    );

    assert_eq!(
        diagnostic["observed_exit_code"], 69,
        "a refusal a DC sweep raised is still a capability refusal: {diagnostic}"
    );
    assert_eq!(diagnostic["error"]["category"], "capability");
    assert_eq!(diagnostic["error"]["code"], "unsupported_capability");
    assert_eq!(
        diagnostic["error"]["capability"],
        "device.ltra.rlgc_conductance"
    );
    assert_eq!(
        diagnostic["error"]["analysis"], "DC Sweep",
        "the refusal must still name the analysis that raised it: {diagnostic}"
    );
    // The engine's own detail text is what the user reads, before and after
    // the category stopped being re-decided here.
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message
                .contains("neither ngspice nor Xyce defines an RLGC line with shunt conductance")),
        "the engine's detail must survive to the diagnostic: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Every advanced analysis shares one error map, so typing it types them all:
/// HB, PSS, Monte Carlo, and both S-parameter routes. `.HB` is the spelling
/// driven here because its refusal is a device the analysis declines to stamp.
#[test]
fn an_advanced_analysis_keeps_the_engine_category_of_what_refused_it() {
    let dir = test_dir("hb_capability");
    let diagnostic = run_json(
        &dir,
        "hb_bjt.sp",
        "* HB has no stamp for the complete Gummel-Poon equations\n\
         V1 in 0 SIN(0 0.5 1e6)\n\
         Vcc vcc 0 5\n\
         R1 in b 1k\n\
         Rc vcc c 1k\n\
         Q1 c b 0 npnmod\n\
         .model npnmod NPN IS=1e-16 BF=100\n\
         .hb 1e6\n\
         .end\n",
        &[],
    );

    assert_eq!(
        diagnostic["observed_exit_code"], 69,
        "a device HB declines to stamp is a capability refusal: {diagnostic}"
    );
    assert_eq!(diagnostic["error"]["category"], "capability");
    assert_eq!(diagnostic["error"]["code"], "unsupported_capability");
    assert_eq!(diagnostic["error"]["capability"], "analysis.hb.device");
    assert_eq!(
        diagnostic["error"]["analysis"], "HB",
        "the refusal must still name the analysis that raised it: {diagnostic}"
    );
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("HB runtime does not yet support")),
        "the engine's detail must survive to the diagnostic: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// The periodic family - `.PSS`, `.PAC`, `.PNOISE`, `.ENVELOPE` - shares one
/// error map, so typing it types all four. `.PSS` is the spelling driven here
/// because it is the carrier the other three linearize around, so it is the
/// one card of the family a deck can raise on its own.
#[test]
fn a_periodic_analysis_keeps_the_engine_category_of_what_refused_it() {
    let dir = test_dir("pss_capability");
    let diagnostic = run_json(
        &dir,
        "ltra_pss.sp",
        "* RLGC LTRA with shunt conductance, under a periodic steady state\n\
         V1 in 0 SIN(0 1 1e6)\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=1n C=1p LEN=1\n\
         Rl out 0 50\n\
         .pss FUND=1e6\n\
         .end\n",
        &[],
    );

    assert_eq!(
        diagnostic["observed_exit_code"], 69,
        "a refusal a periodic analysis raised is still a capability refusal: {diagnostic}"
    );
    assert_eq!(diagnostic["error"]["category"], "capability");
    assert_eq!(diagnostic["error"]["code"], "unsupported_capability");
    assert_eq!(
        diagnostic["error"]["capability"],
        "device.ltra.rlgc_conductance"
    );
    assert_eq!(
        diagnostic["error"]["analysis"], "PSS",
        "the refusal must still name the analysis that raised it: {diagnostic}"
    );
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message
                .contains("neither ngspice nor Xyce defines an RLGC line with shunt conductance")),
        "the engine's detail must survive to the diagnostic: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// The same refusal reached while resolving a `--node`-style flag rather than
/// while running an analysis.
///
/// A flag is resolved by elaborating the deck, which is where an authored
/// construct this build declines is refused; stringifying that refusal exited
/// 80 purely because a flag asked for the node map first.
#[test]
fn a_resolved_node_flag_keeps_the_engine_category_of_what_refused_it() {
    let dir = test_dir("node_resolution_capability");
    let diagnostic = run_json(
        &dir,
        "ltra_sens.sp",
        "* RLGC LTRA with shunt conductance, behind a node-resolving flag\n\
         V1 in 0 1\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=1n C=1p LEN=1\n\
         Rl out 0 50\n\
         .op\n\
         .end\n",
        &["--sens-output", "out", "--sens-param", "R1"],
    );

    assert_eq!(
        diagnostic["observed_exit_code"], 69,
        "a refusal raised while resolving a flag is still a capability refusal: {diagnostic}"
    );
    assert_eq!(diagnostic["error"]["category"], "capability");
    assert_eq!(diagnostic["error"]["code"], "unsupported_capability");
    assert_eq!(
        diagnostic["error"]["capability"],
        "device.ltra.rlgc_conductance"
    );
    assert_eq!(
        diagnostic["error"]["analysis"], "Node Resolution",
        "the refusal must still name the step that raised it: {diagnostic}"
    );
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message
                .contains("neither ngspice nor Xyce defines an RLGC line with shunt conductance")),
        "the engine's detail must survive to the diagnostic: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_unreadable_checkpoint_version_exits_seventy_six() {
    let dir = test_dir("category_persistence");
    let checkpoint = dir.join("future.chk");
    std::fs::write(
        &checkpoint,
        "RSPICE-CHECKPOINT 4294967295\nfingerprint 0x0\ntime 0\n",
    )
    .expect("write checkpoint");

    let diagnostic = run_json(
        &dir,
        "resume.sp",
        "* resume from an incompatible checkpoint\n\
         V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .tran 1n 2u\n\
         .end\n",
        &["--resume", checkpoint.to_str().unwrap()],
    );

    assert_eq!(diagnostic["observed_exit_code"], 76);
    assert_eq!(diagnostic["error"]["category"], "persistence");
    assert_eq!(diagnostic["error"]["code"], "persistence_incompatible");
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("4294967295")),
        "the diagnostic must name the version it found: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_output_symbol_the_result_cannot_supply_exits_eighty_three() {
    let dir = test_dir("category_signal");
    let diagnostic = run_json(
        &dir,
        "acsave.sp",
        "* AC result has no device-observable registry\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .ac dec 5 1 1meg\n\
         .save @Mdriver[Id]\n\
         .end\n",
        &[],
    );

    assert_eq!(diagnostic["observed_exit_code"], 83);
    assert_eq!(diagnostic["error"]["category"], "signal_unavailable");
    assert_eq!(diagnostic["error"]["code"], "requested_signal_unavailable");
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("@Mdriver[Id]")),
        "the authored spelling must survive to the diagnostic: {diagnostic}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn every_reachable_category_has_a_distinct_status_and_never_exits_one() {
    // A cross-check over the codes this suite proves reachable: they must all
    // differ, and none may be the general-error 1 automation cannot act on.
    let reachable = [
        ("netlist", 65),
        ("input_not_found", 66),
        ("capability", 69),
        ("output_commit", 73),
        ("resource_limit", 75),
        ("persistence", 76),
        ("configuration", 78),
        ("simulation", 80),
        ("signal_unavailable", 83),
        ("materialization", 85),
        ("timeout", 124),
    ];
    let mut seen = std::collections::BTreeMap::new();
    for (category, code) in reachable {
        assert_ne!(code, 1, "category {category} must not exit 1");
        assert!(
            seen.insert(code, category).is_none(),
            "category {category} shares exit {code} with {:?}",
            seen.get(&code)
        );
    }
}
