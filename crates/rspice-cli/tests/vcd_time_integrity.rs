//! Timeline conversions preserve units and reject unrepresentable coordinates.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path, format: &str, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", format])
        .args(extra)
        .output()
        .unwrap()
}

fn table(path: &Path, quantity: &str, unit: Option<&str>, times: &[f64], values: &[f64]) {
    std::fs::write(
        path,
        serde_json::json!({
            "scale": {"name": "time", "type": quantity, "unit": unit, "values": times},
            "signals": [{"name": "D(clk)", "type": "digital", "values": values}]
        })
        .to_string(),
    )
    .unwrap();
}

#[test]
fn declared_time_units_survive_every_metadata_preserving_carrier() {
    let dir = test_dir("vcd_time_units");
    let input = dir.join("source.json");
    for (unit, femtoseconds) in [
        ("s", 1_000_000_000_000_000u64),
        ("ms", 1_000_000_000_000),
        ("us", 1_000_000_000),
        ("µs", 1_000_000_000),
        ("μs", 1_000_000_000),
        ("ns", 1_000_000),
        ("ps", 1_000),
        ("fs", 1),
    ] {
        table(&input, "time", Some(unit), &[0.0, 1.0], &[0.0, 1.0]);
        for (format, extension) in [
            ("json", "json"),
            ("raw", "raw"),
            ("ascii", "ascii"),
            ("hdf5", "h5"),
        ] {
            let source = dir.join(format!("converted.{extension}"));
            let written = convert(&input, &source, format, &[]);
            assert!(written.status.success(), "{unit} {format}: {written:?}");
            let output = dir.join("events.vcd");
            let converted = convert(&source, &output, "vcd", &[]);
            assert!(converted.status.success(), "{unit} {format}: {converted:?}");
            let document = rspice_core::io::parse_vcd_file(&output).unwrap();
            assert_eq!(
                document.signals[0].changes[1].tick * document.timescale.femtoseconds(),
                femtoseconds,
                "{unit} {format}: event time changed"
            );
        }
    }
}

#[test]
fn non_time_coordinates_and_unsupported_units_preserve_the_destination() {
    let dir = test_dir("vcd_non_time_axis");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    for (quantity, unit) in [
        ("frequency", Some("Hz")),
        ("frequency", Some("s")),
        ("index", None),
        ("voltage", None),
        ("value", None),
        ("time", Some("Hz")),
        ("time", Some("MS")),
        ("time", Some("fortnight")),
    ] {
        table(&input, quantity, unit, &[0.0, 1.0], &[0.0, 1.0]);
        std::fs::write(&output, "predecessor").unwrap();
        let converted = convert(&input, &output, "vcd", &[]);
        assert_eq!(
            converted.status.code(),
            Some(1),
            "{quantity} {unit:?}: {converted:?}"
        );
        assert!(
            String::from_utf8_lossy(&converted.stderr).contains("coordinate"),
            "{converted:?}"
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
}

#[test]
fn redundant_samples_cannot_hide_invalid_timeline_coordinates() {
    let dir = test_dir("vcd_grid_time_order");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    for (times, values) in [
        (vec![0.0, -1.0, 2.0], vec![0.0, 0.0, 1.0]),
        (vec![0.0, 2.0, 1.0, 3.0], vec![0.0, 0.0, 0.0, 1.0]),
    ] {
        table(&input, "time", None, &times, &values);
        std::fs::write(&output, "predecessor").unwrap();
        let converted = convert(&input, &output, "vcd", &[]);
        assert_eq!(converted.status.code(), Some(1), "{converted:?}");
        assert!(
            String::from_utf8_lossy(&converted.stderr).contains("coordinate"),
            "{converted:?}"
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
}

#[test]
fn positive_sub_femtosecond_events_are_never_moved_to_zero() {
    let dir = test_dir("vcd_tiny_time");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    for time in [f64::from_bits(1), 1e-30, 1e-25, 0.25e-15] {
        table(&input, "time", None, &[0.0, time], &[0.0, 1.0]);
        std::fs::write(&output, "predecessor").unwrap();
        let converted = convert(&input, &output, "vcd", &[]);
        assert_eq!(converted.status.code(), Some(1), "{time}: {converted:?}");
        assert!(
            String::from_utf8_lossy(&converted.stderr).contains("femtoseconds"),
            "{converted:?}"
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
}

#[test]
fn vcd_clipping_uses_seconds_after_converting_the_declared_time_unit() {
    let dir = test_dir("vcd_scaled_time_clipping");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    // An explicit unit can supply the time dimension for an otherwise untyped coordinate.
    table(
        &input,
        "value",
        Some("ms"),
        &[0.0, 1.0, 2.0],
        &[0.0, 1.0, 0.0],
    );
    let converted = convert(
        &input,
        &output,
        "vcd",
        &["--start", "500u", "--stop", "1.5m"],
    );
    assert!(converted.status.success(), "{converted:?}");
    let document = rspice_core::io::parse_vcd_file(&output).unwrap();
    assert_eq!(
        document.signals[0]
            .changes
            .iter()
            .map(|change| change.tick * document.timescale.femtoseconds())
            .collect::<Vec<_>>(),
        [500_000_000_000, 1_000_000_000_000]
    );
}
