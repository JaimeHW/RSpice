//! `--timings`: the JSON phase report of `run` and `check`, and the promise
//! that asking for it changes no result file.

mod common;

use common::test_dir;
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

const OP_DECK: &str =
    "* timings op\nV1 in 0 1\nR1 in out 1k\nD1 out 0 DX\n.model DX D(IS=1e-14)\n.op\n.end\n";

const TRAN_DECK: &str = "* timings tran\nV1 in 0 PULSE(0 1 0 1u 1u 5u 10u)\nR1 in out 1k\n\
C1 out 0 1n\nD1 out 0 DX\n.model DX D(IS=1e-14 CJO=1p)\n.tran 0.1u 20u\n.end\n";

fn rspice(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

fn passed(output: &Output) {
    assert!(
        output.status.success(),
        "rspice failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Run `deck` writing a rawfile to `raw`, with `--timings timings` when given.
fn run(deck: &Path, raw: &Path, timings: Option<&Path>) -> Output {
    let mut args = vec![
        "run".as_ref(),
        deck.as_os_str(),
        "-q".as_ref(),
        "-o".as_ref(),
        raw.as_os_str(),
    ];
    if let Some(timings) = timings {
        args.push("--timings".as_ref());
        args.push(timings.as_os_str());
    }
    rspice(&args)
}

fn read_report(path: &Path) -> Value {
    let text = std::fs::read_to_string(path).expect("timings report written");
    serde_json::from_str(&text).expect("timings report is JSON")
}

fn seconds(document: &Value, key: &str) -> f64 {
    let value = document[key]
        .as_f64()
        .unwrap_or_else(|| panic!("`{key}` is a number of seconds in {document:#}"));
    assert!(value >= 0.0, "`{key}` is negative in {document:#}");
    value
}

fn count(document: &Value, key: &str) -> u64 {
    document[key]
        .as_u64()
        .unwrap_or_else(|| panic!("`{key}` is a count in {document:#}"))
}

fn assert_front_end(report: &Value, command: &str) {
    assert_eq!(report["schema"], "rspice-timings/1");
    assert_eq!(report["command"], command);
    for key in [
        "total_wall_s",
        "expand_s",
        "parse_s",
        "preflight_s",
        "flatten_s",
        "build_s",
        "output_s",
    ] {
        seconds(report, key);
    }
    assert!(seconds(report, "total_wall_s") > 0.0);
}

#[test]
fn an_op_run_reports_its_phases_and_newton_iterations() {
    let dir = test_dir("timings_op");
    let deck = dir.join("op.cir");
    std::fs::write(&deck, OP_DECK).unwrap();
    let timings = dir.join("timings.json");
    let output = run(&deck, &dir.join("op.raw"), Some(&timings));
    passed(&output);

    let report = read_report(&timings);
    assert_front_end(&report, "run");
    assert!(count(&report, "circuit_builds") >= 1, "{report:#}");
    let op = &report["operating_point"];
    assert!(count(op, "solves") >= 1, "{report:#}");
    assert!(count(op, "newton_iterations") >= 1, "{report:#}");
    seconds(op, "wall_s");
    assert!(report["transient"].is_null(), "{report:#}");

    let analyses = report["analyses"].as_array().expect("analyses array");
    assert_eq!(analyses.len(), 1, "{report:#}");
    assert_eq!(analyses[0]["analysis"], "op");
    seconds(&analyses[0], "wall_s");
    assert!(seconds(&report, "output_s") > 0.0, "the rawfile was timed");
}

#[test]
fn a_transient_run_reports_its_stepping_phases_and_counts() {
    let dir = test_dir("timings_tran");
    let deck = dir.join("tran.cir");
    std::fs::write(&deck, TRAN_DECK).unwrap();
    let timings = dir.join("timings.json");
    passed(&run(&deck, &dir.join("tran.raw"), Some(&timings)));

    let report = read_report(&timings);
    assert_front_end(&report, "run");
    let transient = &report["transient"];
    assert_eq!(count(transient, "runs"), 1, "{report:#}");
    for key in [
        "wall_s",
        "device_evaluation_s",
        "matrix_assembly_s",
        "linear_solve_s",
        "factor_s",
        "solve_s",
        "certificate_s",
        "convergence_check_s",
        "residual_proof_s",
        "truncation_s",
        "output_s",
        "other_s",
    ] {
        seconds(transient, key);
    }
    assert!(seconds(transient, "wall_s") > 0.0, "{report:#}");
    assert!(count(transient, "accepted_steps") >= 20, "{report:#}");
    count(transient, "rejected_steps");
    assert!(count(transient, "newton_iterations") >= count(transient, "accepted_steps"));
    assert!(
        count(transient, "factorizations") + count(transient, "refactorizations") >= 1,
        "{report:#}"
    );
    // The transient's initial operating point is counted there.
    assert!(
        count(&report["operating_point"], "solves") >= 1,
        "{report:#}"
    );
    let analyses = report["analyses"].as_array().expect("analyses array");
    assert_eq!(analyses.len(), 1, "{report:#}");
    assert_eq!(analyses[0]["analysis"], "tran");
    assert_eq!(count(&analyses[0]["transient"], "runs"), 1);
    // The transient publishes its rawfile through its own path.
    assert!(seconds(&report, "output_s") > 0.0, "the rawfile was timed");
    assert!(seconds(&analyses[0], "output_s") > 0.0, "{report:#}");
}

#[test]
fn result_files_are_byte_identical_with_and_without_timings() {
    let dir = test_dir("timings_identical");
    for (name, text) in [("op", OP_DECK), ("tran", TRAN_DECK)] {
        let deck = dir.join(format!("{name}.cir"));
        std::fs::write(&deck, text).unwrap();
        let plain = dir.join(format!("{name}_plain.raw"));
        let timed = dir.join(format!("{name}_timed.raw"));
        passed(&run(&deck, &plain, None));
        passed(&run(&deck, &timed, Some(&dir.join(format!("{name}.json")))));
        assert_eq!(
            std::fs::read(&plain).unwrap(),
            std::fs::read(&timed).unwrap(),
            "{name}: --timings changed the rawfile"
        );
    }
}

#[test]
fn check_reports_the_front_end_without_analyses() {
    let dir = test_dir("timings_check");
    let deck = dir.join("op.cir");
    std::fs::write(&deck, OP_DECK).unwrap();
    let timings = dir.join("check.json");
    passed(&rspice(&[
        "check".as_ref(),
        deck.as_os_str(),
        "-q".as_ref(),
        "--timings".as_ref(),
        timings.as_os_str(),
    ]));
    let report = read_report(&timings);
    assert_front_end(&report, "check");
    assert!(report.get("analyses").is_none(), "{report:#}");
}

#[test]
fn a_dash_writes_the_report_to_stderr() {
    let dir = test_dir("timings_stderr");
    let deck = dir.join("op.cir");
    std::fs::write(&deck, OP_DECK).unwrap();
    let output = rspice(&[
        "run".as_ref(),
        deck.as_os_str(),
        "-q".as_ref(),
        "--timings".as_ref(),
        "-".as_ref(),
    ]);
    passed(&output);
    let stderr = String::from_utf8(output.stderr).expect("utf-8 stderr");
    let start = stderr.find('{').expect("a JSON document on stderr");
    let report: Value = serde_json::from_str(&stderr[start..]).expect("stderr report is JSON");
    assert_front_end(&report, "run");
}

#[test]
fn the_help_names_every_timing_key() {
    let output = rspice(&["run".as_ref(), "--help".as_ref()]);
    passed(&output);
    let help = String::from_utf8_lossy(&output.stdout);
    for key in [
        "--timings",
        "expand_s",
        "preflight_s",
        "device_evaluation_s",
        "residual_proof_s",
        "refactorizations",
    ] {
        assert!(help.contains(key), "`{key}` missing from run --help");
    }
}
