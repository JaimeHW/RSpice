//! Noise point-event measurements reach reports through every execution route.

mod common;

use serde_json::Value;
use std::process::Command;

const NETWORK: &str = "Noise continuous measurements\n\
    I1 0 out DC 0 AC 1\n\
    R1 out 0 1k\n\
    .DATA points FREQ\n10\n32.5\n55\n77.5\n100\n.ENDDATA\n";

fn row<'a>(rows: &'a [Value], name: &str) -> &'a Value {
    rows.iter()
        .find(|row| row["name"].as_str().unwrap().eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing measurement {name}: {rows:?}"))
}

#[test]
fn noise_continuous_measurements_are_evaluated_in_direct_control_and_table_runs() {
    let dir = common::test_dir("noise-continuous-routes");
    for sweep in ["lin 5 10 100", "data=points"] {
        let command = format!("noise V(out) I1 {sweep}");
        for cards in [
            format!(".{command}"),
            format!(".control\n{command}\n.endc"),
            format!(".{command}\n.control\nrun\n.endc"),
        ] {
            let deck = dir.join("input.cir");
            let measurements = dir.join("measurements.json");
            std::fs::write(
                &deck,
                format!(
                    "{NETWORK}{cards}\n\
                     .MEASURE NOISE scalar FIND ONOISE AT=55\n\
                     .MEASURE NOISE_CONT density FIND ONOISE AT=55\n\
                     .MEASURE NOISE_CONT crossing WHEN HERTZ=55 CROSS=1\n\
                     .END\n"
                ),
            )
            .unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["--spice-dialect", "xyce", "--summary", "-", "--meas-file"])
                .arg(&measurements)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{cards}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let report: Value =
                serde_json::from_slice(&std::fs::read(&measurements).unwrap()).unwrap();
            let rows = report["measurements"].as_array().unwrap();
            assert_eq!(rows.len(), 3, "{cards}: {rows:?}");
            let scalar = row(rows, "scalar");
            let density = row(rows, "density");
            let crossing = row(rows, "crossing");
            assert!(scalar["value"].as_f64().unwrap() > 0.0);
            assert_eq!(density["value"], scalar["value"]);
            assert_eq!(crossing["value"], 55.0);
            for event in [density, crossing] {
                assert_eq!(event["passed"], true);
                assert_eq!(event["record_index"], 0);
                assert_eq!(event["event_axis"], 55.0);
                assert_eq!(event["raw_value"], event["value"]);
                assert_eq!(event["aggregate_policy"], "all_records_must_pass");
            }
            let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(summary["counts"]["measurements"], 3);
            assert_eq!(summary["counts"]["failed_measurements"], 0);
            assert_eq!(summary["passed"], true);
        }
    }
}

#[test]
fn noise_continuous_failvalue_preserves_the_record_and_controls_the_exit_status() {
    let dir = common::test_dir("noise-continuous-verdict");
    let deck = dir.join("input.cir");
    std::fs::write(
        &deck,
        format!(
            "{NETWORK}.NOISE V(out) I1 lin 5 10 100\n\
             .MEASURE NOISE_CONT crossing WHEN HERTZ=55 CROSS=1 FAILVALUE=1\n\
             .END\n"
        ),
    )
    .unwrap();
    for allowed in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command.args(["--quiet", "run"]).arg(&deck).args([
            "--spice-dialect",
            "xyce",
            "--summary",
            "-",
        ]);
        if allowed {
            command.arg("--allow-failed-meas");
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(if allowed { 0 } else { 3 }));
        let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
        let rows = summary["runs"][0]["measurements"].as_array().unwrap();
        assert_eq!(rows.len(), 1);
        let crossing = &rows[0];
        assert_eq!(crossing["value"], 55.0);
        assert_eq!(crossing["raw_value"], 55.0);
        assert_eq!(crossing["event_axis"], 55.0);
        assert_eq!(crossing["record_index"], 0);
        assert_eq!(crossing["failure_limit"], 1.0);
        assert_eq!(crossing["failure_limit_exceeded"], true);
        assert_eq!(crossing["passed"], false);
        assert_eq!(summary["runs"][0]["passed"], false);
        assert_eq!(summary["passed"], allowed);
    }
}
