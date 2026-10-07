//! A run summary lists artifacts produced by this invocation, including on failure.

mod common;

use common::{read_json, test_dir};
use std::process::Command;

#[test]
fn summary_includes_special_format_and_companion_artifacts_exactly_once() {
    let cases: &[(&str, &str, &str, &[&str])] = &[
        (
            "results.csv",
            "csv",
            "* implicit step\n.PARAM rval=1k\nV1 in 0 1\nR1 in 0 {rval}\n.STEP PARAM rval LIST 1k 2k\n.END\n",
            &[],
        ),
        (
            "results.s2p",
            "raw",
            "* two-port\nR1 in out 50\nR2 out 0 50\n.AC LIN 2 1 10\n.END\n",
            &["--sparam", "in,0,out,0"],
        ),
        (
            "results.json",
            "json",
            "* transient and FFT\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.TRAN 10u 2m\n.FFT V(out) NP=8 WINDOW=RECT FORMAT=UNORM\n.END\n",
            &[],
        ),
    ];
    for &(filename, format, deck_text, flags) in cases {
        let dir = test_dir("published_summary");
        let deck = dir.join("input.sp");
        std::fs::write(&deck, deck_text).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", format, "-o"])
            .arg(dir.join(filename))
            .args(flags)
            .args(["--summary", "-"])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let listed = summary["outputs"].as_array().unwrap();
        let listed_paths: std::collections::BTreeSet<_> = listed
            .iter()
            .map(|path| std::path::PathBuf::from(path.as_str().unwrap()))
            .collect();
        let actual: std::collections::BTreeSet<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("results")
            })
            .collect();
        assert!(!actual.is_empty());
        assert_eq!(listed_paths, actual, "{filename}");
        assert_eq!(listed.len(), actual.len(), "no duplicate manifest entries");
        assert_eq!(summary["counts"]["outputs"], actual.len());
    }
}

#[test]
fn rejected_json_publication_does_not_claim_a_new_or_existing_destination() {
    for existing in [false, true] {
        for flags in [
            vec![],
            vec!["--pz-input", "in", "--pz-output", "out"],
            vec!["--monte-carlo", "3"],
        ] {
            let dir = test_dir("unpublished_summary");
            let deck = dir.join("input.sp");
            let config = dir.join("config.toml");
            let artifact = dir.join("result.json");
            std::fs::write(
                &deck,
                "* bounded JSON result\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.OP\n.END\n",
            )
            .unwrap();
            std::fs::write(&config, "[resources]\nmax_external_data_bytes = 200\n").unwrap();
            if existing {
                std::fs::write(&artifact, b"previous result").unwrap();
            }
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--config"])
                .arg(&config)
                .arg("run")
                .arg(&deck)
                .args(&flags)
                .args(["-f", "json", "-o"])
                .arg(&artifact)
                .args(["--summary", "-"])
                .output()
                .unwrap();
            assert!(!output.status.success(), "{output:?}");
            assert!(String::from_utf8_lossy(&output.stderr).contains("200-byte limit"));
            let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(summary["passed"], false);
            assert_eq!(summary["counts"]["outputs"], 0);
            assert_eq!(summary["outputs"], serde_json::json!([]));
            if existing {
                assert_eq!(std::fs::read(&artifact).unwrap(), b"previous result");
            } else {
                assert!(!artifact.exists());
            }
        }
    }
}

#[test]
fn a_later_publication_failure_preserves_earlier_artifacts_in_the_summary() {
    let dir = test_dir("partly_published_summary");
    let deck = dir.join("input.sp");
    let config = dir.join("config.toml");
    let artifact = dir.join("result.json");
    let summary_path = dir.join("summary.json");
    std::fs::write(
        &deck,
        "* OP fits the budget, AC exceeds it\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1n\n.OP\n.AC LIN 100 1 1000\n.END\n",
    )
    .unwrap();
    let run = |config: Option<&std::path::Path>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command.arg("--quiet");
        if let Some(config) = config {
            command.arg("--config").arg(config);
        }
        command
            .arg("run")
            .arg(&deck)
            .args(["-f", "json", "-o"])
            .arg(&artifact)
            .arg("--summary")
            .arg(&summary_path)
            .output()
            .unwrap()
    };
    let baseline = run(None);
    assert!(baseline.status.success(), "{baseline:?}");
    let op = dir.join("result.op-001.json");
    let ac = dir.join("result.ac-001.json");
    let op_bytes = std::fs::metadata(&op).unwrap().len();
    assert!(std::fs::metadata(&ac).unwrap().len() > op_bytes);
    std::fs::write(
        &config,
        format!("[resources]\nmax_external_data_bytes = {op_bytes}\n"),
    )
    .unwrap();
    std::fs::write(&ac, b"previous AC result").unwrap();
    let rejected = run(Some(&config));
    assert!(!rejected.status.success(), "{rejected:?}");
    let summary = read_json(&summary_path);
    assert_eq!(summary["passed"], false);
    assert_eq!(summary["counts"]["outputs"], 1);
    assert_eq!(summary["outputs"], serde_json::json!([op]));
    assert_eq!(std::fs::read(&ac).unwrap(), b"previous AC result");
    assert_eq!(read_json(&op)["analysis"]["tag"], "op-001");
}
