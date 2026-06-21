//! Integration tests for `rspice run` netlist-input flags:
//! `-D/--define` parameter overrides and `-I/--include` search directories.

use std::path::{Path, PathBuf};
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_run_flags_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

/// Read V(out) from a DC OP csv export (`signal,value` rows).
fn read_op_voltage(path: &Path, signal: &str) -> f64 {
    let text = std::fs::read_to_string(path).expect("read op csv");
    for line in text.lines().skip(1) {
        let mut fields = line.split(',');
        let name = fields.next().unwrap_or_default();
        if name.eq_ignore_ascii_case(signal) {
            return fields
                .next()
                .expect("value field")
                .trim()
                .parse()
                .expect("numeric value");
        }
    }
    panic!("signal {signal} not found in {}", path.display());
}

#[test]
fn define_overrides_netlist_param() {
    let dir = test_dir("define");
    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* divider with parametric lower leg\n\
         .param rbot=1k\n\
         v1 in 0 dc 10\n\
         r1 in out 1k\n\
         r2 out 0 {rbot}\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    // Nominal: 10V * 1k/2k = 5V
    let out_nom = dir.join("nom.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out_nom.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(output.status.success(), "nominal run failed");
    let v_nom = read_op_voltage(&out_nom, "V(OUT)");
    assert!(
        (v_nom - 5.0).abs() < 1e-6,
        "expected 5V nominal, got {v_nom}"
    );

    // Override: 10V * 3k/4k = 7.5V, with a SPICE-suffixed value
    let out_def = dir.join("def.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-D",
        "rbot=3k",
        "-o",
        out_def.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(output.status.success(), "define run failed");
    let v_def = read_op_voltage(&out_def, "V(OUT)");
    assert!(
        (v_def - 7.5).abs() < 1e-6,
        "expected 7.5V with -D rbot=3k, got {v_def}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn define_cascades_through_dependent_params() {
    let dir = test_dir("define_cascade");
    let deck = dir.join("cascade.sp");
    // r2 references rbot before the .param line; rtop is derived from rbot.
    // -D rbot=2k must flow through both: 10V * 2k/(4k+2k) = 3.333V.
    std::fs::write(
        &deck,
        "* derived params with use-before-define\n\
         v1 in 0 dc 10\n\
         r2 out 0 {rbot}\n\
         r1 in out {rtop}\n\
         .param rbot=1k rtop={rbot*2}\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    // Nominal: 10 * 1k/3k
    let out_nom = dir.join("nom.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out_nom.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "nominal run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v_nom = read_op_voltage(&out_nom, "V(OUT)");
    assert!(
        (v_nom - 10.0 / 3.0).abs() < 1e-6,
        "expected 3.333V nominal, got {v_nom}"
    );

    // Override cascades into rtop = 2*rbot
    let out_def = dir.join("def.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-D",
        "rbot=2k",
        "-o",
        out_def.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "define run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v_def = read_op_voltage(&out_def, "V(OUT)");
    assert!(
        (v_def - 10.0 / 3.0).abs() < 1e-6,
        "expected 3.333V with cascaded override, got {v_def}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_define_is_a_usage_error() {
    let dir = test_dir("define_bad");
    let deck = dir.join("trivial.sp");
    std::fs::write(&deck, "* trivial\nv1 1 0 1\nr1 1 0 1k\n.op\n.end\n").expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap(), "-D", "novalue"]);
    assert!(!output.status.success(), "malformed define must fail");
    assert_eq!(output.status.code(), Some(2), "usage errors exit with 2");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("NAME=VALUE"),
        "stderr should explain the expected form: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn invalid_solver_controls_are_usage_errors() {
    let dir = test_dir("invalid_solver_controls");
    let deck = dir.join("trivial.sp");
    let out = dir.join("out.csv");
    std::fs::write(&deck, "* trivial\nv1 1 0 1\nr1 1 0 1k\n.op\n.end\n").expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
        "--maxiter",
        "0",
        "--reltol=-1",
        "--abstol=-1",
    ]);
    assert_eq!(output.status.code(), Some(2), "usage errors exit with 2");
    assert!(
        !out.exists(),
        "invalid solver controls must not write an output file"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--maxiter") || stderr.contains("--reltol") || stderr.contains("--abstol"),
        "stderr should identify the invalid solver control: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn contradictory_min_and_max_step_are_usage_errors() {
    let dir = test_dir("contradictory_timesteps");
    let deck = dir.join("trivial.sp");
    let out = dir.join("out.csv");
    std::fs::write(
        &deck,
        "* trivial transient\nv1 1 0 pulse(0 1 0 1n 1n 5n 10n)\nr1 1 0 1k\n.tran 1n 20n\n.end\n",
    )
    .expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
        "--min-step",
        "1u",
        "--max-step",
        "1n",
    ]);

    assert_eq!(output.status.code(), Some(2), "usage errors exit with 2");
    assert!(
        !out.exists(),
        "contradictory timestep controls must not write an output file"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--min-step") && stderr.contains("--max-step"),
        "stderr should identify both contradictory timestep flags: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn include_search_path_resolves_models() {
    let dir = test_dir("include");
    let model_dir = dir.join("modellib");
    std::fs::create_dir_all(&model_dir).expect("create model dir");
    std::fs::write(
        model_dir.join("parts.inc"),
        "* shared parts\n.model dswitch d(is=1e-14)\n",
    )
    .expect("write include");

    let deck_dir = dir.join("decks");
    std::fs::create_dir_all(&deck_dir).expect("create deck dir");
    let deck = deck_dir.join("clamp.sp");
    std::fs::write(
        &deck,
        "* diode clamp using an out-of-tree include\n\
         .include parts.inc\n\
         v1 in 0 dc 5\n\
         r1 in out 1k\n\
         d1 out 0 dswitch\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    // Without -I the include cannot resolve
    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "run without -I should fail to resolve the include"
    );

    // With -I pointing at the model directory it parses and solves
    let out_csv = dir.join("clamp.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-I",
        model_dir.to_str().unwrap(),
        "-o",
        out_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "run with -I failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v_out = read_op_voltage(&out_csv, "V(OUT)");
    assert!(
        v_out > 0.2 && v_out < 2.0,
        "diode clamp should hold V(out) near a forward drop, got {v_out}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
