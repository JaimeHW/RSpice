//! Event carriers must retain the declared coordinate and all value components.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn malformed_event_axes_and_complex_values_preserve_existing_outputs() {
    let dir = test_dir("raw_event_admission");
    let input = dir.join("events.raw");
    let output = dir.join("events.vcd");
    for (plot, title, variable) in [
        (
            "Digital Events (rspice-digital-events/1)",
            "digital",
            "D(clk) digital",
        ),
        ("Real Events (rspice-real-events/1)", "real", "E(ctrl) real"),
        (
            "Digital Bus (rspice-digital-bus/1)",
            "bus[0:0]",
            "D(clk) digital",
        ),
    ] {
        for (axis, flags, metadata, values, expected) in [
            (
                "frequency frequency",
                "real",
                "",
                "0 0 0\n1 1 1",
                "time coordinate",
            ),
            (
                "time time",
                "real",
                "Command: RSpiceTableV2 {\"real_variables\":[],\"units\":[\"ms\",null]}\n",
                "0 0 0\n1 1 1",
                "seconds",
            ),
            (
                "time time",
                "complex",
                "",
                "0 0,0 0,1\n1 1,0 1,2",
                "real columns",
            ),
            ("time time", "real", "", "0 1 0\n1 0 1", "decreases"),
            ("time time", "real", "", "0 -1 0\n1 0 1", "non-negative"),
        ] {
            std::fs::write(
                &input,
                format!("Title: {title}\nPlotname: {plot}\n{metadata}Flags: {flags}\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 {axis}\n1 {variable}\nValues:\n{values}\n"),
            ).unwrap();
            std::fs::write(&output, "predecessor").unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(&input)
                .arg(&output)
                .args(["--to", "vcd"])
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(1),
                "{plot} {expected}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains(expected),
                "{plot}: {result:?}"
            );
            assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        }
    }
}

#[test]
fn event_seconds_metadata_and_same_time_changes_remain_lossless() {
    let dir = test_dir("raw_event_seconds");
    let input = dir.join("events.raw");
    let output = dir.join("events.vcd");
    for unit in ["s", "sec", "second", "seconds"] {
        std::fs::write(&input, format!(
            "Title: digital\nPlotname: Digital Events (rspice-digital-events/1)\nCommand: RSpiceTableV2 {{\"real_variables\":[],\"units\":[\"{unit}\",null]}}\nFlags: real\nNo. Variables: 2\nNo. Points: 3\nVariables:\n0 time time\n1 D(clk) digital\nValues:\n0 0 0\n1 1 1\n2 1 0\n"
        )).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&output)
            .args(["--to", "vcd"])
            .output()
            .unwrap();
        assert!(result.status.success(), "{unit}: {result:?}");
        let document = rspice_core::io::parse_vcd_file(&output).unwrap();
        let changes = &document.signals[0].changes;
        assert_eq!(changes.len(), 3);
        assert_eq!(changes[1].tick, changes[2].tick);
        assert_eq!(
            document.timescale.seconds_at_tick(changes[1].tick),
            Some(1.0)
        );
    }
}
