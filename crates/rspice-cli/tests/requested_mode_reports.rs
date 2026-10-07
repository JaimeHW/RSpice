//! Command-line analysis modes retain failures in the same reports as cards.

mod common;

use common::{read_json, test_dir};
use std::process::Command;

#[test]
fn a_failed_requested_mode_does_not_skip_a_successful_later_variant() {
    for jobs in ["1", "2"] {
        let dir = test_dir("requested_partial_success");
        let deck = dir.join("variants.sp");
        let artifact = dir.join("results.json");
        std::fs::write(
            &deck,
            "* line variant fails PZ, lumped RC variant succeeds\n\
             .PARAM use_line=1\n\
             V1 in 0 1\n\
             .IF (use_line > 0)\n\
             T1 in 0 out 0 Z0=50 TD=1n\n\
             .ELSE\n\
             Rseries in out 1k\n\
             .ENDIF\n\
             Rload out 0 50\n\
             C1 out 0 1n\n\
             .OP\n\
             .ALTER supported\n\
             .PARAM use_line=0\n\
             .END\n",
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "run"])
            .arg(&deck)
            .args([
                "--pz-input",
                "in",
                "--pz-output",
                "out",
                "--jobs",
                jobs,
                "--summary",
                "-",
                "-f",
                "json",
                "-o",
            ])
            .arg(&artifact)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(69), "{output:?}");
        let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(summary["passed"], false);
        assert_eq!(summary["counts"]["runs"], 2);
        assert_eq!(summary["counts"]["failed_runs"], 1);
        assert_eq!(summary["counts"]["passed_runs"], 1);
        assert_eq!(summary["runs"][0]["passed"], false);
        assert_eq!(summary["runs"][1]["passed"], true);
        let outputs = summary["outputs"].as_array().unwrap();
        assert_eq!(outputs.len(), 1);
        let published_path = std::path::Path::new(outputs[0].as_str().unwrap());
        assert_eq!(published_path, dir.join("results.supported.json"));
        let document = read_json(published_path);
        assert_eq!(document["analysis"]["tag"], "pz-001");
        assert!(!dir.join("results.base.json").exists());
    }
}

#[test]
fn requested_analysis_failures_are_reported_for_every_outer_run() {
    for jobs in ["1", "2"] {
        for requested_mode in [false, true] {
            let dir = test_dir("requested_failure_reports");
            let deck = dir.join("line.sp");
            let summary_path = dir.join("summary.json");
            let meas_path = dir.join("measurements.json");
            let report_path = dir.join("report.xml");
            let artifact = dir.join("results.json");
            let analysis = if requested_mode {
                ".OP"
            } else {
                ".PZ in 0 out 0 vol pz"
            };
            std::fs::write(
                &deck,
                format!(
                    "* pole-zero refuses a distributed line\n\
                     V1 in 0 1\n\
                     T1 in 0 out 0 Z0=50 TD=1n\n\
                     R1 out 0 50\n\
                     {analysis}\n\
                     .MEAS TRAN skipped MAX V(out)\n\
                     .ALTER second\n\
                     V1 in 0 2\n\
                     .END\n"
                ),
            )
            .unwrap();
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "--error-format", "json", "run"])
                .arg(&deck)
                .args(["--jobs", jobs, "-f", "json", "-o"])
                .arg(&artifact)
                .arg("--summary")
                .arg(&summary_path)
                .arg("--meas-file")
                .arg(&meas_path)
                .args(["--report-format", "junit", "--report-file"])
                .arg(&report_path);
            if requested_mode {
                command.args(["--pz-input", "in", "--pz-output", "out"]);
            }
            let output = command.output().unwrap();
            assert_eq!(output.status.code(), Some(69), "{output:?}");
            let fatal: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(fatal["error"]["capability"], "analysis.pz.device");

            let summary = read_json(&summary_path);
            assert_eq!(summary["passed"], false);
            assert_eq!(summary["counts"]["runs"], 2);
            assert_eq!(summary["counts"]["failed_runs"], 2);
            assert_eq!(summary["counts"]["failed_measurements"], 2);
            assert_eq!(summary["outputs"], serde_json::json!([]));
            let runs = summary["runs"].as_array().unwrap();
            assert_ne!(runs[0]["name"], runs[1]["name"]);
            for run in runs {
                assert_eq!(run["passed"], false);
                assert_eq!(run["error_details"]["category"], fatal["error"]["category"]);
                assert_eq!(run["error_details"]["code"], fatal["error"]["code"]);
                assert_eq!(run["error_details"]["capability"], "analysis.pz.device");
                assert_eq!(run["measurements"][0]["name"], "SKIPPED");
                assert_eq!(run["measurements"][0]["passed"], false);
            }
            let measurements = read_json(&meas_path);
            assert_eq!(measurements["total"], 2);
            assert_eq!(measurements["failed"], 2);
            let xml = std::fs::read_to_string(&report_path).unwrap();
            let junit = roxmltree::Document::parse(&xml).unwrap();
            assert_eq!(junit.root_element().attribute("failures"), Some("4"));
            assert_eq!(
                junit
                    .descendants()
                    .filter(|node| node.has_tag_name("testsuite"))
                    .count(),
                2
            );
            assert!(
                !std::fs::read_dir(&dir).unwrap().any(|entry| entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("results")),
                "a failed command-line mode must not fall back to the deck's successful OP"
            );
        }
    }
}
