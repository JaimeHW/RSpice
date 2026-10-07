//! Compression may approximate waveforms, but never measurement evidence.

mod common;

use serde_json::Value;
use std::process::Command;

#[test]
fn continuous_measurements_and_verdicts_use_the_accepted_transient_before_compression() {
    let dir = common::test_dir("continuous-compression");
    for route in ["direct", "control", "checkpoint"] {
        let mut runs = Vec::new();
        for compressed in [false, true] {
            let deck = dir.join(format!("{route}-{compressed}.sp"));
            let control = if route == "control" {
                ".CONTROL\nRUN\n.ENDC\n"
            } else {
                ""
            };
            std::fs::write(
                &deck,
                format!(
                    "Continuous compression\n\
                     V1 in 0 SIN(0 1 1k)\nR1 in out 1k\nC1 out 0 100n\n\
                     .TRAN 100u 3m\n\
                     .MEASURE TRAN reference FIND V(out) AT=173u\n\
                     .MEASURE TRAN_CONT sample FIND V(out) AT=173u FAILVALUE=0.505\n\
                     .MEASURE TRAN_CONT crossings WHEN V(out)=0.25 CROSS=1\n\
                     {control}.END\n"
                ),
            )
            .unwrap();
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "run"])
                .arg(&deck)
                .args([
                    "--spice-dialect",
                    "xyce",
                    "--summary",
                    "-",
                    "-f",
                    "csv",
                    "-o",
                ])
                .arg(dir.join(format!("{route}-{compressed}.csv")));
            if compressed {
                command.args(["--compress", "--compress-tol", "0.1"]);
            }
            if route == "checkpoint" {
                command
                    .arg("--checkpoint")
                    .arg(dir.join(format!("{route}-{compressed}.checkpoint")));
            }
            let output = command.output().unwrap();
            assert_eq!(
                output.status.code(),
                Some(3),
                "{route}, compressed={compressed}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
            let rows = summary["runs"][0]["measurements"].as_array().unwrap();
            let named = |name: &str| {
                rows.iter()
                    .find(|row| row["name"].as_str().unwrap().eq_ignore_ascii_case(name))
                    .unwrap()
            };
            assert_eq!(named("sample")["value"], named("reference")["value"]);
            assert_eq!(named("sample")["failure_limit_exceeded"], true);
            assert!(rows.len() > 3, "retain every continuous crossing");
            let outputs = summary["outputs"].as_array().unwrap();
            assert_eq!(outputs.len(), 1);
            let stored_rows = std::fs::read_to_string(outputs[0].as_str().unwrap())
                .unwrap()
                .lines()
                .count();
            runs.push((rows.clone(), stored_rows));
        }
        assert!(
            runs[1].1 < runs[0].1,
            "{route} must actually discard samples"
        );
        assert_eq!(
            runs[0].0, runs[1].0,
            "{route}: compression changed measurement evidence"
        );
    }
}
