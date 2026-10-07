//! Retained report payloads obey the same value policy as waveform results.

mod common;

use common::test_dir;
use std::process::{Command, Output};

const CONTINUOUS: &str = "* continuous report payload\n\
    .PARAM rval=1\n\
    V1 out 0 0\n\
    R1 out 0 {rval}\n\
    .DC V1 -1 1 1\n\
    .MEASURE DC_CONT zero WHEN V(out)=0 CROSS=1 FAILVALUE=2\n\
    .MEASURE DC_CONT half WHEN V(out)=0.5 CROSS=1 FAILVALUE=2\n";

fn run(source: &str, limit: usize, flags: &[&str]) -> Output {
    let dir = test_dir("report_budget");
    let deck = dir.join("input.sp");
    let config = dir.join("config.toml");
    std::fs::write(&deck, source).unwrap();
    std::fs::write(
        &config,
        format!("[resources]\nmax_result_values = {limit}\n"),
    )
    .unwrap();
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "--config"])
        .arg(config)
        .arg("run")
        .arg(deck)
        .args(["--spice-dialect", "xyce", "--summary", "-"])
        .args(flags)
        .output()
        .unwrap()
}

fn assert_report_limit(output: &Output, limit: usize) {
    assert_limit_at(output, limit, "Report retention");
}

fn assert_limit_at(output: &Output, limit: usize, analysis: &str) {
    assert_eq!(
        output.status.code(),
        Some(75),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let fatal: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(fatal["error"]["resource"], "result_values");
    assert_eq!(fatal["error"]["limit"], limit);
    assert!(fatal["error"]["requested"].as_u64().unwrap() > limit as u64);
    assert_eq!(fatal["error"]["analysis"], analysis);
}

#[test]
fn step_reports_charge_every_retained_measurement_field() {
    let source = format!("{CONTINUOUS}.STEP PARAM rval 1 60 1\n.END\n");
    // Each coordinate retains one duration and four floats in each of two
    // continuous records: value, raw value, event axis and FAILVALUE.
    let exact = run(&source, 540, &[]);
    assert!(exact.status.success(), "{exact:?}");
    let summary: serde_json::Value = serde_json::from_slice(&exact.stdout).unwrap();
    assert_eq!(summary["counts"]["runs"], 60);
    assert_eq!(summary["counts"]["measurements"], 120);
    assert_report_limit(&run(&source, 500, &[]), 500);
}

#[test]
fn late_report_budget_failure_preserves_the_previous_complete_sweep() {
    let dir = test_dir("report_budget_rollback");
    let deck = dir.join("input.sp");
    let config = dir.join("config.toml");
    std::fs::write(
        &deck,
        format!("{CONTINUOUS}.STEP PARAM rval 1 60 1\n.END\n"),
    )
    .unwrap();
    let execute = |limit| {
        std::fs::write(
            &config,
            format!("[resources]\nmax_result_values = {limit}\n"),
        )
        .unwrap();
        Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "--config"])
            .arg(&config)
            .arg("run")
            .arg(&deck)
            .args(["--spice-dialect", "xyce", "-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .output()
            .unwrap()
    };
    let snapshot = || -> std::collections::BTreeMap<_, _> {
        std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("result")
            })
            .map(|path| {
                (
                    path.file_name().unwrap().to_owned(),
                    std::fs::read(&path).unwrap(),
                )
            })
            .collect()
    };
    let baseline = execute(540);
    assert!(
        baseline.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&baseline.stderr)
    );
    let before = snapshot();
    assert!(before.len() >= 60);
    assert_report_limit(&execute(500), 500);
    assert_eq!(
        snapshot(),
        before,
        "failed admission must discard all staged replacements"
    );
}

#[test]
fn one_run_cannot_accumulate_measurements_past_its_report_budget() {
    let statements: String = (0..60)
        .map(|index| format!(".MEAS DC value{index} PARAM='{index}'\n"))
        .collect();
    let source = format!(
        "* scalar report payload\nV1 out 0 0\nR1 out 0 1k\n.DC V1 0 1 1\n{statements}.END\n"
    );
    let exact = run(&source, 121, &[]);
    assert!(exact.status.success(), "{exact:?}");
    assert_report_limit(&run(&source, 100, &[]), 100);
}

#[test]
fn continuous_record_counts_are_charged_after_event_selection() {
    let source = "* multiple continuous records per statement\n\
        V1 in 0 0\n\
        B1 out 0 V=cos(3.141592653589793*V(in))\n\
        R1 out 0 1k\n\
        .DC V1 0 100 1\n\
        .MEASURE DC_CONT zero WHEN V(out)=0 CROSS=1 FAILVALUE=1000\n\
        .MEASURE DC_CONT high WHEN V(out)=0.5 CROSS=1 FAILVALUE=1000\n\
        .MEASURE DC_CONT low WHEN V(out)=-0.5 CROSS=1 FAILVALUE=1000\n\
        .MEASURE DC_CONT middle WHEN V(out)=0.25 CROSS=1 FAILVALUE=1000\n\
        .END\n";
    let exact = run(source, 1601, &[]);
    assert!(
        exact.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&exact.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&exact.stdout).unwrap();
    assert_eq!(summary["counts"]["measurements"], 400);
    assert_report_limit(&run(source, 1600, &[]), 1600);
    assert_limit_at(
        &run(source, 1500, &[]),
        1500,
        "DC continuous measurement projection",
    );
}

#[test]
fn continuous_evaluator_limits_reach_every_cli_analysis_route() {
    for (analysis, command, at) in [
        ("DC", "dc V1 0 1 1", "0.5"),
        ("AC", "ac lin 3 1 3", "1.5"),
        ("NOISE", "noise V(out) V1 lin 3 1 3", "1.5"),
        ("TRAN", "tran 0.01 0.1", "0.05"),
    ] {
        let measurements: String = (0..300)
            .map(|index| format!(".MEAS {analysis}_CONT sample{index} FIND TIME AT={at}\n"))
            .collect();
        for control in [false, true] {
            let execution = if control {
                format!(".CONTROL\n{command}\n.ENDC")
            } else {
                format!(".{command}")
            };
            let source = format!(
                "* continuous evaluator budget\nV1 out 0 DC 1 AC 1\nR1 out 0 1k\n{measurements}{execution}\n.END\n"
            );
            let exact = run(&source, 901, &[]);
            assert!(
                exact.status.success(),
                "{analysis}, control={control}: {}",
                String::from_utf8_lossy(&exact.stderr)
            );
            let summary: serde_json::Value = serde_json::from_slice(&exact.stdout).unwrap();
            assert_eq!(summary["counts"]["measurements"], 300);
            assert_limit_at(
                &run(&source, 800, &[]),
                800,
                &format!("{analysis} continuous measurement projection"),
            );
            if analysis == "TRAN" && !control {
                let compressed = run(&source, 901, &["--compress"]);
                assert!(
                    compressed.status.success(),
                    "{}",
                    String::from_utf8_lossy(&compressed.stderr)
                );
                assert_limit_at(&run(&source, 800, &["--compress"]), 800, "Transient");
            }
        }
    }
}

#[test]
fn control_datasets_share_the_enclosing_report_budget() {
    let statements: String = (0..15)
        .map(|index| format!(".MEAS DC value{index} PARAM='{index}'\n"))
        .collect();
    let source = format!(
        "* control report payload\nV1 out 0 0\nR1 out 0 1k\n{statements}.CONTROL\nrepeat 10\ndc V1 0 1 1\nend\n.ENDC\n.END\n"
    );
    let exact = run(&source, 301, &[]);
    assert!(
        exact.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&exact.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&exact.stdout).unwrap();
    assert_eq!(summary["counts"]["measurements"], 150);
    assert_report_limit(&run(&source, 250, &[]), 250);
}

#[test]
fn empty_failed_rows_cannot_bypass_admission_or_be_allowed_past_the_limit() {
    let statements: String = (0..101)
        .map(|index| format!(".MEAS OP value{index} PARAM='{index}'\n"))
        .collect();
    let source = format!("* unevaluated measurements\n{statements}.END\n");
    let exact = run(&source, 102, &["--allow-failed-meas"]);
    assert!(
        exact.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&exact.stderr)
    );
    assert_report_limit(&run(&source, 100, &["--allow-failed-meas"]), 100);
}

#[test]
fn serial_and_parallel_outer_runs_share_the_aggregate_report_budget() {
    let variants: String = (1..60)
        .map(|index| format!(".ALTER variant{index}\n.PARAM rval={}\n", index + 1))
        .collect();
    let source = format!("{CONTINUOUS}{variants}.END\n");
    for jobs in ["1", "2"] {
        let exact = run(&source, 540, &["--jobs", jobs]);
        assert!(exact.status.success(), "{exact:?}");
        assert_report_limit(&run(&source, 500, &["--jobs", jobs]), 500);
    }
}

#[test]
fn corner_runs_share_the_aggregate_report_budget() {
    let names = (0..60)
        .map(|index| format!("corner{index}"))
        .collect::<Vec<_>>()
        .join(",");
    let source = format!("{CONTINUOUS}.END\n");
    for jobs in ["1", "2"] {
        let flags = ["--corners", names.as_str(), "--jobs", jobs];
        let exact = run(&source, 540, &flags);
        assert!(exact.status.success(), "{exact:?}");
        assert_report_limit(&run(&source, 500, &flags), 500);
    }
}
