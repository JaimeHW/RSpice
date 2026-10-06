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

#[test]
fn corner_parameters_are_available_to_sources_before_validation() {
    let dir = common::test_dir("corner_source_parameter");
    let deck = dir.join("deck.sp");
    let lib = dir.join("corners.lib");
    std::fs::write(
        &deck,
        "* corner source\nV1 in 0 {level}\nR1 in 0 1k\n.op\n.end\n",
    )
    .unwrap();
    std::fs::write(
        &lib,
        "* library\n.lib tt\n.param level=1\n.endl\n.lib ss\n.param level=2\n.endl\n",
    )
    .unwrap();
    let output = command()
        .arg("run")
        .arg(deck)
        .args(["--corners", "tt,ss", "--corner-lib"])
        .arg(lib)
        .args(["-o"])
        .arg(dir.join("result.csv"))
        .args(["-f", "csv"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    for (corner, voltage) in [("tt", "1"), ("ss", "2")] {
        let csv = std::fs::read_to_string(dir.join(format!("result.{corner}.csv"))).unwrap();
        assert!(
            csv.lines().any(|line| line
                .to_ascii_uppercase()
                .starts_with(&format!("V(IN),{voltage}"))),
            "{csv}"
        );
    }
}

#[test]
fn default_control_outputs_remain_distinct_for_parallel_corners() {
    let dir = common::test_dir("corner_default_control");
    let deck = dir.join("deck.sp");
    let lib = dir.join("corners.lib");
    std::fs::write(&deck, "* control divider\nV1 in 0 10\nR1 in out 1k\nR2 out 0 {rload}\n.control\nop\nprint v(out)\n.endc\n.end\n").unwrap();
    std::fs::write(
        &lib,
        "* library\n.lib tt\n.param rload=1k\n.endl\n.lib ss\n.param rload=3k\n.endl\n",
    )
    .unwrap();
    let output = command()
        .arg("run")
        .arg(&deck)
        .args(["--corners", "tt,ss", "--corner-lib"])
        .arg(lib)
        .args(["--jobs", "2", "--summary", "-"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let outputs = summary["outputs"].as_array().unwrap();
    assert_eq!(outputs.len(), 2, "{summary}");
    assert_ne!(outputs[0], outputs[1]);
    let documents: Vec<String> = outputs
        .iter()
        .map(|path| std::fs::read_to_string(path.as_str().unwrap()).unwrap())
        .collect();
    assert_ne!(
        documents[0], documents[1],
        "distinct corner solutions must survive publication"
    );
}

#[test]
fn corner_library_axes_contribute_to_the_whole_invocations_budget() {
    let dir = common::test_dir("corner_axes_budget");
    let deck = dir.join("deck.sp");
    let lib = dir.join("corners.lib");
    let config = dir.join("limit.toml");
    std::fs::write(&config, "[resources]\nmax_batch_runs=7\n").unwrap();
    std::fs::write(
        &deck,
        "* nested sweep\nV1 in 0 1\nR1 in 0 1k\n.op\n.alter second\nR1 in 0 2k\n.end\n",
    )
    .unwrap();
    std::fs::write(&lib, "* library\n.lib tt\n.step param p list 1 2\n.endl\n.lib ss\n.step param p list 1 2\n.endl\n").unwrap();
    let output = command()
        .arg("--config")
        .arg(config)
        .arg("run")
        .arg(&deck)
        .args(["--corners", "tt,ss", "--corner-lib"])
        .arg(lib)
        .args(["--jobs", "2", "-o"])
        .arg(dir.join("result.csv"))
        .args(["-f", "csv"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["requested"], 8);
    assert!(!std::fs::read_dir(&dir).unwrap().any(|entry| {
        entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("result")
    }));
}
