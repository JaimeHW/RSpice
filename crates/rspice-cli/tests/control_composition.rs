//! Composition of script execution with the ordinary transient and axis policy.
mod common;
use common::{read_json, test_dir};
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn run(deck: &Path, output: &Path, format: &str, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(deck)
        .arg("-o")
        .arg(output)
        .args(["-f", format])
        .args(extra)
        .output()
        .unwrap()
}
fn passed(output: Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn documents(dir: &Path, prefix: &str) -> Vec<Value> {
    std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(prefix)
                && path.extension().is_some_and(|ext| ext == "json")
        })
        .map(|path| read_json(&path))
        .collect()
}
fn last_csv(path: &Path, column: &str) -> (f64, f64) {
    let content = std::fs::read_to_string(path).unwrap();
    let index = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .position(|name| name.eq_ignore_ascii_case(column))
        .unwrap();
    let row = content
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|value| value.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    (row[0], row[index])
}

#[test]
fn control_checkpoint_resume_and_compression_follow_the_analytic_rc_response() {
    let dir = test_dir("control-segments");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, "control RC\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1u ic=0\n.control\ntran 10u 2m uic\n.endc\n.end\n").unwrap();
    let checkpoint = dir.join("state.chk");
    passed(run(
        &deck,
        &dir.join("first.csv"),
        "csv",
        &[
            "--checkpoint",
            checkpoint.to_str().unwrap(),
            "--tran-stop",
            "1m",
            "--compress",
        ],
    ));
    assert!(dir.join("state.tran-001.chk").is_file());
    let (time, voltage) = last_csv(&dir.join("first.tran-001.csv"), "V(out)");
    assert!((time - 1e-3).abs() < 1e-14);
    assert!(
        (voltage - (1.0 - (-1.0f64).exp())).abs() < 0.003,
        "{voltage}"
    );
    passed(run(
        &deck,
        &dir.join("resume.csv"),
        "csv",
        &["--resume", checkpoint.to_str().unwrap(), "--compress"],
    ));
    let (time, voltage) = last_csv(&dir.join("resume.tran-001.csv"), "V(out)");
    assert!((time - 2e-3).abs() < 1e-14);
    assert!(
        (voltage - (1.0 - (-2.0f64).exp())).abs() < 0.003,
        "{voltage}"
    );
}

#[test]
fn repeated_control_runs_publish_unique_fft_and_fourier_children_before_compression() {
    let dir = test_dir("control-postprocess");
    let deck = dir.join("sine.cir");
    std::fs::write(&deck, "control sine\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 1u 2m\n.four 1k V(out)\n.fft V(out) np=64 freq=1k\n.control\nop\nrun\nrun\n.endc\n.end\n").unwrap();
    passed(run(&deck, &dir.join("full.json"), "json", &[]));
    passed(run(
        &deck,
        &dir.join("compressed.json"),
        "json",
        &["--compress"],
    ));
    let full = documents(&dir, "full.");
    let compressed = documents(&dir, "compressed.");
    for ordinal in 1..=2 {
        let parent = format!("tran-{ordinal:03}");
        let child = format!("four-{ordinal:03}");
        let four = full
            .iter()
            .find(|doc| doc["analysis"]["tag"] == child)
            .unwrap();
        assert_eq!(four["parentAnalysis"]["tag"], parent);
        let compressed_four = compressed
            .iter()
            .find(|doc| doc["analysis"]["tag"] == child)
            .unwrap();
        assert_eq!(
            four["signals"], compressed_four["signals"],
            "Fourier uses accepted samples"
        );
        let fft = full
            .iter()
            .find(|doc| doc["parent_analysis_id"] == parent)
            .unwrap();
        assert_eq!(
            fft["results"][0]["analysis_id"],
            format!("fft-{ordinal:03}")
        );
        let compressed_fft = compressed
            .iter()
            .find(|doc| doc["parent_analysis_id"] == parent)
            .unwrap();
        assert_eq!(
            fft["results"], compressed_fft["results"],
            "FFT uses accepted samples"
        );
    }
}

#[test]
fn a_dynamic_control_transient_binds_declarative_postprocessing_without_a_tran_card() {
    let dir = test_dir("control-dynamic-postprocess");
    let deck = dir.join("sine.cir");
    std::fs::write(&deck, "dynamic sine\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.fft V(out) np=64 freq=1k\n.control\ntran 1u 2m\n.endc\n.end\n").unwrap();
    passed(run(
        &deck,
        &dir.join("result.json"),
        "json",
        &["--compress"],
    ));
    let docs = documents(&dir, "result.");
    assert!(
        docs.iter().any(|doc| doc["analysis"]["tag"] == "four-001"
            && doc["parentAnalysis"]["tag"] == "tran-001")
    );
    assert!(
        docs.iter()
            .any(|doc| doc["parent_analysis_id"] == "tran-001"
                && doc["results"][0]["analysis_id"] == "fft-001")
    );
}

#[test]
fn control_scripts_execute_at_every_step_and_temperature_coordinate() {
    let dir = test_dir("control-axes");
    let deck = dir.join("divider.cir");
    std::fs::write(&deck, "control axes\n.param rval=1k\nV1 in 0 1\nR1 in out {rval}\nR2 out 0 1k\n.step param rval list 1k 2k\n.temp 20 60\n.control\nop\n.endc\n.end\n").unwrap();
    passed(run(&deck, &dir.join("result.json"), "json", &[]));
    let docs = documents(&dir, "result.");
    let ops = docs
        .iter()
        .filter(|doc| doc["resultKind"] == "op")
        .collect::<Vec<_>>();
    assert_eq!(ops.len(), 4, "{docs:#?}");
    let mut halves = 0;
    let mut thirds = 0;
    for doc in ops {
        assert!(!doc["coordinate"].is_null());
        assert_eq!(
            doc["coordinate"]["assignments"].as_array().unwrap().len(),
            2
        );
        let voltage = doc["signals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|signal| signal["descriptor"]["canonicalName"] == "v(out)")
            .unwrap()["values"]["samples"][0]
            .as_f64()
            .unwrap();
        if (voltage - 0.5).abs() < 1e-8 {
            halves += 1;
        }
        if (voltage - 1.0 / 3.0).abs() < 1e-8 {
            thirds += 1;
        }
    }
    assert_eq!((halves, thirds), (2, 2));
}

#[test]
fn failed_control_scripts_preserve_checkpoints_and_result_files_together() {
    let dir = test_dir("control-rollback");
    let deck = dir.join("fail.cir");
    std::fs::write(&deck, "control failure\nV1 out 0 1\nR1 out 0 1k\n.control\ntran 1u 10u\nprint V(missing)\n.endc\n.end\n").unwrap();
    let checkpoint = dir.join("state.chk");
    let state = dir.join("state.tran-001.chk");
    let result = dir.join("result.tran-001.csv");
    std::fs::write(&state, b"previous checkpoint").unwrap();
    std::fs::write(&result, b"previous result").unwrap();
    let output = run(
        &deck,
        &dir.join("result.csv"),
        "csv",
        &["--checkpoint", checkpoint.to_str().unwrap()],
    );
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .to_ascii_lowercase()
            .contains("missing"),
        "{output:?}"
    );
    assert_eq!(std::fs::read(&state).unwrap(), b"previous checkpoint");
    assert_eq!(std::fs::read(&result).unwrap(), b"previous result");
}

#[test]
fn scheduled_restart_checkpoints_are_qualified_for_each_control_transient() {
    let dir = test_dir("control-restart");
    let deck = dir.join("restart.cir");
    std::fs::write(&deck, "control restart\nV1 out 0 1\nR1 out 0 1k\n.options restart job=state initial_interval=5u\n.control\ntran 1u 10u\ntran 1u 10u\n.endc\n.end\n").unwrap();
    passed(run(&deck, &dir.join("result.csv"), "csv", &["--compress"]));
    let files = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("state"))
        .collect::<Vec<_>>();
    assert!(
        files.iter().any(|name| name.ends_with("tran-001")),
        "{files:?}"
    );
    assert!(
        files.iter().any(|name| name.ends_with("tran-002")),
        "{files:?}"
    );
}

#[test]
fn control_axis_checkpoints_resume_their_own_coordinate_with_compression() {
    let dir = test_dir("control-axis-segments");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, "control axis RC\n.param rval=1k\nV1 in 0 1\nR1 in out {rval}\nC1 out 0 1u ic=0\n.step param rval list 1k 2k\n.temp 20 60\n.control\ntran 10u 2m uic\n.endc\n.end\n").unwrap();
    let checkpoint = dir.join("state.chk");
    passed(run(
        &deck,
        &dir.join("first.csv"),
        "csv",
        &[
            "--checkpoint",
            checkpoint.to_str().unwrap(),
            "--tran-stop",
            "1m",
            "--compress",
        ],
    ));
    let states = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "chk"))
        .count();
    assert_eq!(states, 4);
    passed(run(
        &deck,
        &dir.join("resume.csv"),
        "csv",
        &["--resume", checkpoint.to_str().unwrap(), "--compress"],
    ));
    let results = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("resume.")
                && path.extension().is_some_and(|ext| ext == "csv")
        })
        .collect::<Vec<_>>();
    assert_eq!(results.len(), 4);
    let mut fast = 0;
    let mut slow = 0;
    for result in results {
        let (time, voltage) = last_csv(&result, "V(out)");
        assert!((time - 2e-3).abs() < 1e-14);
        if (voltage - (1.0 - (-2.0f64).exp())).abs() < 0.003 {
            fast += 1;
        }
        if (voltage - (1.0 - (-1.0f64).exp())).abs() < 0.003 {
            slow += 1;
        }
    }
    assert_eq!((fast, slow), (2, 2));
}

#[test]
fn a_later_control_coordinate_failure_discards_all_staged_checkpoints_and_results() {
    let dir = test_dir("control-axis-rollback");
    let deck = dir.join("fail.cir");
    std::fs::write(&deck, "axis control failure\n.param rval=1k\nV1 out 0 1\nR1 out 0 {rval}\n.step param rval list 1k 2k\n.control\ntran 1u 10u\nif rval > 1500\nprint V(missing)\nend\n.endc\n.end\n").unwrap();
    let checkpoint = dir.join("state.chk");
    let output = run(
        &deck,
        &dir.join("result.csv"),
        "csv",
        &["--checkpoint", checkpoint.to_str().unwrap()],
    );
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .to_ascii_lowercase()
            .contains("missing"),
        "{output:?}"
    );
    let leftovers = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(leftovers, ["fail.cir"]);
}

#[test]
fn unused_transient_options_and_postprocessing_fail_without_publishing_an_op() {
    let dir = test_dir("control-unused-transient");
    let deck = dir.join("op.cir");
    let source = "unused transient policy\nV1 n 0 1\nR1 n 0 1k\n.control\nop\n.endc\n.end\n";
    std::fs::write(&deck, source).unwrap();
    let output = run(
        &deck,
        &dir.join("result.csv"),
        "csv",
        &["--tran-stop", "1m"],
    );
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(!dir.join("result.op-001.csv").exists());
    std::fs::write(&deck, source.replace(".control", ".four 1k V(n)\n.control")).unwrap();
    let output = run(&deck, &dir.join("result.csv"), "csv", &[]);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(!dir.join("result.op-001.csv").exists());
}
