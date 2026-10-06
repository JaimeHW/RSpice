mod common;
use rspice_core::io::{VcdBit, VcdValue, parse_vcd_file};
use std::process::Command;

const VCD: &str = "$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! changing $end\n$var wire 1 \" held $end\n$var wire 4 # bus [3:0] $end\n$var real 64 $ real $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n1\"\nb10xz #\nr1.25 $\n#10\n1!\nr2.5 $\n#20\n0!\n";

#[test]
fn selected_raw_sections_keep_the_same_clipped_state() {
    let dir = common::test_dir("selected_section");
    let input = dir.join("input.json");
    let raw = dir.join("input.raw");
    let output = dir.join("output.vcd");
    let table = serde_json::json!({
        "analysis": "transient", "plot_name": "events",
        "scale": {"name":"time", "type":"time", "values":[0.0, 1e-9, 10e-9]},
        "signals": [{"name":"D(d)", "type":"digital", "values":[0.0, 0.0, 1.0]}]
    });
    std::fs::write(&input, serde_json::to_vec(&table).unwrap()).unwrap();
    let converted = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&input)
        .arg(&raw)
        .args(["--to", "raw"])
        .output()
        .unwrap();
    assert!(converted.status.success(), "{converted:?}");
    let clipped = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&raw)
        .arg(&output)
        .args([
            "--to",
            "vcd",
            "--section",
            "1",
            "--start",
            "5n",
            "--stop",
            "15n",
        ])
        .output()
        .unwrap();
    assert!(clipped.status.success(), "{clipped:?}");
    let document = parse_vcd_file(&output).unwrap();
    let changes = &document.signals[0].changes;
    assert_eq!(changes[0].value, VcdValue::Logic(vec![VcdBit::Zero]));
    assert_eq!(changes[0].tick as f64 * document.timescale.seconds(), 5e-9);
    assert_eq!(changes[1].value, VcdValue::Logic(vec![VcdBit::One]));
    assert_eq!(changes[1].tick as f64 * document.timescale.seconds(), 10e-9);
}

#[test]
fn clipping_keeps_the_state_at_the_start_including_constant_signals() {
    let dir = common::test_dir("held_state");
    let input = dir.join("input.vcd");
    std::fs::write(&input, VCD).unwrap();
    for (start, stop, boundary, changing) in [
        ("5n", "15n", 5, vec![(5, VcdBit::Zero), (10, VcdBit::One)]),
        (
            "5.25n",
            "15n",
            525,
            vec![(525, VcdBit::Zero), (1000, VcdBit::One)],
        ),
        ("10n", "15n", 10, vec![(10, VcdBit::One)]),
        ("5n", "5n", 5, vec![(5, VcdBit::Zero)]),
    ] {
        let output = dir.join("output.vcd");
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&output)
            .args(["--to", "vcd", "--start", start, "--stop", stop])
            .output()
            .unwrap();
        assert!(result.status.success(), "{start}: {result:?}");
        let document = parse_vcd_file(&output).unwrap();
        let observed: Vec<_> = document.signals[0]
            .changes
            .iter()
            .map(|change| (change.tick, change.value.clone()))
            .collect();
        assert_eq!(
            observed,
            changing
                .into_iter()
                .map(|(tick, bit)| (tick, VcdValue::Logic(vec![bit])))
                .collect::<Vec<_>>()
        );
        assert_eq!(document.signals[1].changes.len(), 1);
        assert_eq!(document.signals[1].changes[0].tick, boundary);
        assert_eq!(
            document.signals[1].changes[0].value,
            VcdValue::Logic(vec![VcdBit::One])
        );
        assert_eq!(document.signals[2].changes[0].tick, boundary);
        assert_eq!(
            document.signals[2].changes[0].value,
            VcdValue::Logic(vec![
                VcdBit::One,
                VcdBit::Zero,
                VcdBit::Unknown,
                VcdBit::HighImpedance
            ])
        );
        assert_eq!(document.signals[3].changes[0].tick, boundary);
        assert_eq!(
            document.signals[3].changes[0].value,
            VcdValue::Real(if start == "10n" { 2.5 } else { 1.25 })
        );
    }
}

#[test]
fn clipping_compares_integer_ticks_without_rounding_distinct_events_together() {
    let dir = common::test_dir("large_ticks");
    let input = dir.join("input.vcd");
    let output = dir.join("output.vcd");
    std::fs::write(&input, "$timescale 1 s $end\n$var wire 1 ! d $end\n$enddefinitions $end\n#0\n0!\n#9007199254740992\n1!\n#9007199254740993\n0!\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(input)
        .arg(&output)
        .args(["--to", "vcd", "--stop", "9007199254740992"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let document = parse_vcd_file(&output).unwrap();
    assert_eq!(
        document.signals[0]
            .changes
            .iter()
            .map(|change| change.tick)
            .collect::<Vec<_>>(),
        [0, 9007199254740992]
    );
}

#[test]
fn invalid_conversion_ranges_leave_destinations_unchanged() {
    let dir = common::test_dir("invalid_range");
    let input = dir.join("input.vcd");
    std::fs::write(&input, VCD).unwrap();
    for format in ["vcd", "csv", "json", "hdf5", "raw"] {
        for flags in [
            vec!["--start", "10n", "--stop", "5n"],
            vec!["--stop", "1e999"],
        ] {
            let output = dir.join(format!("output.{format}"));
            std::fs::write(&output, "original bytes").unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(&input)
                .arg(&output)
                .args(["--to", format])
                .args(&flags)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{format} {flags:?}: {result:?}"
            );
            assert_eq!(std::fs::read_to_string(output).unwrap(), "original bytes");
        }
    }
}

#[test]
fn unrepresentable_clip_boundaries_and_tick_overflow_preserve_existing_output() {
    let dir = common::test_dir("clip_precision");
    let input = dir.join("input.vcd");
    let output = dir.join("output.vcd");
    std::fs::write(&input, "$timescale 1 s $end\n$var wire 1 ! d $end\n$enddefinitions $end\n#0\n0!\n#18446744073709551615\n1!\n").unwrap();
    for start in ["0.5", "5e-16", "18446744073709551616"] {
        std::fs::write(&output, "original bytes").unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&output)
            .args(["--to", "vcd", "--start", start])
            .output()
            .unwrap();
        assert!(!result.status.success(), "{start}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "original bytes");
    }
    // Discarding the late event avoids overflow and keeps the held state at 0.5 s.
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&input)
        .arg(&output)
        .args(["--to", "vcd", "--start", "0.5", "--stop", "1"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let document = parse_vcd_file(&output).unwrap();
    assert_eq!(document.timescale.to_string(), "100 ms");
    assert_eq!(document.signals[0].changes[0].tick, 5);
    assert_eq!(
        document.signals[0].changes[0].value,
        VcdValue::Logic(vec![VcdBit::Zero])
    );
}
