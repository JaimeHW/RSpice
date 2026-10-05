mod common;
use std::process::Command;
fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args(["--quiet", "--error-format", "json"]);
    command
}
#[test]
fn corners_preserve_output_errors_in_each_report() {
    let dir = common::test_dir("corner_errors");
    let deck = dir.join("op.sp");
    std::fs::write(&deck, "* OP\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    for jobs in ["1", "2"] {
        let summary = dir.join(format!("errors{jobs}.json"));
        let output = command()
            .arg("run")
            .arg(&deck)
            .args(["--corners", "tt,ss", "--jobs", jobs, "-f", "csv", "-o"])
            .arg(dir.join("absent/result.csv"))
            .arg("--summary")
            .arg(&summary)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(73), "{output:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(diagnostic["error"]["exit_code"], 73);
        assert!(
            diagnostic["error"]["path"]
                .as_str()
                .unwrap_or_else(|| panic!("{diagnostic}"))
                .contains("absent")
        );
        let summary: serde_json::Value =
            serde_json::from_slice(&std::fs::read(summary).unwrap()).unwrap();
        assert_eq!(summary["counts"]["runs"], 2);
        assert_eq!(summary["counts"]["failed_runs"], 2);
        assert_eq!(
            summary["execution"]["workers"],
            jobs.parse::<usize>().unwrap()
        );
        for report in summary["runs"].as_array().unwrap() {
            assert_eq!(report["error_details"]["code"], "output_commit_failed");
        }
    }
}
#[test]
fn corner_admission_precedes_any_artifact_publication() {
    let dir = common::test_dir("corner_admission");
    let deck = dir.join("op.sp");
    std::fs::write(&deck, "* OP\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let config = dir.join("limit.toml");
    std::fs::write(&config, "[resources]\nmax_batch_runs=1\n").unwrap();
    let output = command()
        .arg("--config")
        .arg(config)
        .arg("run")
        .arg(&deck)
        .args(["--corners", "tt,ss", "-o"])
        .arg(dir.join("result.csv"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    assert!(!dir.join("result.tt.csv").exists());
    for corners in ["tt,TT", "../tt", "tt,,ss"] {
        let output = command()
            .arg("run")
            .arg(&deck)
            .args(["--corners", corners])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
    }
    std::fs::write(
        &deck,
        "* OP\nV1 in 0 1\nR1 in 0 1k\n.op\n.alter other\nR1 in 0 2k\n.end\n",
    )
    .unwrap();
    let config = dir.join("aggregate.toml");
    std::fs::write(&config, "[resources]\nmax_batch_runs=3\n").unwrap();
    let output = command()
        .arg("--config")
        .arg(config)
        .arg("run")
        .arg(&deck)
        .args(["--corners", "tt,ss", "-o"])
        .arg(dir.join("aggregate.csv"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["error"]["requested"], 4);
}
#[test]
fn corners_keep_root_overrides_when_reelaborating() {
    let dir = common::test_dir("corner_overrides");
    let deck = dir.join("op.sp");
    std::fs::write(
        &deck,
        "* OP\n.param level=1\nV1 in 0 {level}\nR1 in 0 1k\n.op\n.end\n",
    )
    .unwrap();
    let output = command()
        .arg("run")
        .arg(deck)
        .args([
            "--corners",
            "tt,ss",
            "-j",
            "2",
            "-D",
            "level=3",
            "-f",
            "csv",
            "-o",
        ])
        .arg(dir.join("result.csv"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    for corner in ["tt", "ss"] {
        let csv = std::fs::read_to_string(dir.join(format!("result.{corner}.csv"))).unwrap();
        assert!(
            csv.lines()
                .any(|line| line.to_ascii_uppercase().starts_with("V(IN),3")),
            "{csv}"
        );
    }
}
