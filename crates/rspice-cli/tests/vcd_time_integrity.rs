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
        // Rounded to the nearest femtosecond, the change lands on the tick
        // zero already holds a different value at, and is refused there
        // rather than moved.
        assert!(
            String::from_utf8_lossy(&converted.stderr).contains("share femtosecond tick 0"),
            "{converted:?}"
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
}

#[test]
fn close_events_on_long_timelines_keep_distinct_ticks() {
    let dir = test_dir("vcd_long_time_precision");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    table(
        &input,
        "time",
        None,
        &[20.000_000_000_000_014, 20.000_000_000_000_018],
        &[0.0, 1.0],
    );
    let converted = convert(&input, &output, "vcd", &[]);
    assert!(converted.status.success(), "{converted:?}");
    let document = rspice_core::io::parse_vcd_file(&output).unwrap();
    assert_eq!(
        document.signals[0]
            .changes
            .iter()
            .map(|change| change.tick * document.timescale.femtoseconds())
            .collect::<Vec<_>>(),
        [20_000_000_000_000_014, 20_000_000_000_000_018],
    );
}

#[test]
fn long_duration_waveforms_use_the_available_vcd_tick_range() {
    let dir = test_dir("vcd_multi_day_timeline");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    for (times, scale, expected) in [
        ([0.0, 86_400.0, 172_800.0], "100 s", [0, 864, 1728]),
        ([0.0, 86_401.0, 86_402.0], "1 s", [0, 86_401, 86_402]),
        ([0.0, 86_400.5, 86_401.0], "100 ms", [0, 864_005, 864_010]),
    ] {
        table(&input, "time", None, &times, &[0.0, 1.0, 0.0]);
        let converted = convert(&input, &output, "vcd", &[]);
        assert!(converted.status.success(), "{times:?}: {converted:?}");
        let document = rspice_core::io::parse_vcd_file(&output).unwrap();
        assert_eq!(document.timescale.to_string(), scale);
        assert_eq!(
            document.signals[0]
                .changes
                .iter()
                .map(|change| change.tick)
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn incompatible_timeline_range_and_resolution_preserve_the_destination() {
    let dir = test_dir("vcd_timeline_range_and_resolution");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    table(
        &input,
        "time",
        None,
        &[0.0, 1e-15, 86_400.0],
        &[0.0, 1.0, 0.0],
    );
    std::fs::write(&output, "predecessor").unwrap();
    let converted = convert(&input, &output, "vcd", &[]);
    assert_eq!(converted.status.code(), Some(1), "{converted:?}");
    assert!(
        String::from_utf8_lossy(&converted.stderr).contains("64-bit tick range"),
        "{converted:?}"
    );
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
}

/// Two values one signal held at different times that round onto one tick
/// have no order in a dump and are refused; two signals changing at those
/// times are simultaneous on that tick.
fn rspice_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .current_dir(dir)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .output()
        .unwrap()
}

/// The tick of every change one dump writes for the variable named `name`.
fn change_ticks(dump: &str, name: &str) -> Vec<String> {
    let code = dump
        .lines()
        .find_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            (fields.first() == Some(&"$var") && fields.get(4) == Some(&name))
                .then(|| fields[3].to_string())
        })
        .unwrap_or_else(|| panic!("no variable {name}:\n{dump}"));
    let mut tick = String::new();
    let mut ticks = Vec::new();
    for line in dump.lines().map(str::trim) {
        if let Some(at) = line.strip_prefix('#') {
            tick = at.to_string();
        } else if line.len() > 1 && line[1..] == code && !line.starts_with('$') {
            ticks.push(tick.clone());
        }
    }
    ticks
}

/// An A/D connect module with hysteresis feeds an HDL input (`timescale
/// 1ns/1ps`). The analog solution dates the falling crossing at 188.5714286
/// ns, between two femtoseconds and between two of the HDL's ticks; the HDL
/// saw it at tick 188571 ps (`$realtime` prints 188.5710) and its own
/// `$dumpvars` writes it there. The run's dump and the dump converted from
/// its rawfile say the same: `$timescale 1 ps`, the edge at #188571.
#[test]
fn an_hdl_input_dumps_at_the_tick_the_hdl_saw_it_at() {
    let dir = test_dir("vcd_hdl_tick");
    std::fs::write(
        dir.join("rx.vams"),
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module rx(in, o);\n\
         \x20 input in; output o; reg o;\n\
         \x20 initial begin $dumpfile(\"hdl.vcd\"); $dumpvars(0, rx); o = 0; end\n\
         \x20 always @(in) o = in;\n\
         endmodule\n\
         connectmodule a2d_h(a, d);\n\
         \x20 input a; output d; electrical a; logic d; reg d;\n\
         \x20 parameter real vhi = 0.6, vlo = 0.4;\n\
         \x20 initial d = 0;\n\
         \x20 always @(cross(V(a) - vhi, +1)) d = 1;\n\
         \x20 always @(cross(V(a) - vlo, -1)) d = 0;\n\
         endmodule\n\
         connectmodule d2a_s(d, a);\n\
         \x20 input d; output a; logic d; electrical a;\n\
         \x20 analog V(a) <+ transition(d ? 2.0 : 0.0, 0, 3n);\n\
         endmodule\n\
         connectrules hyst;\n\
         \x20 connect a2d_h #(.vhi(0.7));\n\
         \x20 connect d2a_s;\n\
         endconnectrules\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("m10.cir"),
        "* authored connect modules with hysteresis\n\
         .hdl \"rx.vams\" module=rx\n\
         .options connectrules=hyst\n\
         v1 in 0 pwl(0 0 100n 1 150n 0.5 160n 0.65 200n 0.3 210n 0.65)\n\
         x1 in o rx\n\
         r1 o 0 1k\n\
         .tran 1n 250n\n\
         .end\n",
    )
    .unwrap();

    let run = rspice_in(&dir, &["run", "m10.cir", "-f", "vcd", "-o", "run.vcd"]);
    assert!(run.status.success(), "{run:?}");
    let hdl = std::fs::read_to_string(dir.join("hdl.vcd")).unwrap();
    let run = std::fs::read_to_string(dir.join("run.vcd")).unwrap();
    assert!(run.contains("$timescale\n\t1 ps\n$end"), "{run}");
    assert_eq!(change_ticks(&run, "IN"), ["0", "70000", "188571"], "{run}");
    // `o = in` runs on the tick the HDL saw `in` change at.
    assert_eq!(change_ticks(&run, "IN"), change_ticks(&hdl, "o"), "{hdl}");
    assert_eq!(change_ticks(&run, "O"), change_ticks(&hdl, "o"), "{hdl}");

    let raw = rspice_in(&dir, &["run", "m10.cir", "-f", "raw", "-o", "run.raw"]);
    assert!(raw.status.success(), "{raw:?}");
    let converted = convert(&dir.join("run.raw"), &dir.join("raw.vcd"), "vcd", &[]);
    assert!(converted.status.success(), "{converted:?}");
    assert_eq!(std::fs::read_to_string(dir.join("raw.vcd")).unwrap(), run);

    // `d2a_s` uses `transition`, which the JIT's CFG route does not lower, so
    // it compiles on the postfix plan. That is a route choice with the same
    // results, and it printed `WARN [JIT] Model 'd2a_s' takes the postfix
    // plan: ...` on every run.
    let loud = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", "m10.cir", "-f", "csv", "-o", "loud.csv"])
        .current_dir(&dir)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RUST_LOG")
        .output()
        .unwrap();
    assert!(loud.status.success(), "{loud:?}");
    let stderr = String::from_utf8_lossy(&loud.stderr);
    assert!(!stderr.contains("[JIT]"), "{stderr}");
}

#[test]
fn one_signal_cannot_hold_two_values_on_one_tick() {
    let dir = test_dir("vcd_distinct_time_collision");
    let input = dir.join("source.json");
    let output = dir.join("events.vcd");
    let first = 1e-9_f64;
    let next = first.next_up();
    let write = |signals: serde_json::Value| {
        std::fs::write(
            &input,
            serde_json::json!({
                "scale": {"name": "time", "type": "time", "values": [0.0, first, next]},
                "signals": signals,
            })
            .to_string(),
        )
        .unwrap();
    };

    write(serde_json::json!([
        {"name": "D(a)", "type": "digital", "values": [0.0, 1.0, 0.0]}
    ]));
    std::fs::write(&output, "predecessor").unwrap();
    let converted = convert(&input, &output, "vcd", &[]);
    assert_eq!(converted.status.code(), Some(1), "{converted:?}");
    assert!(
        String::from_utf8_lossy(&converted.stderr).contains("cannot order the two values"),
        "{converted:?}"
    );
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");

    write(serde_json::json!([
        {"name": "D(a)", "type": "digital", "values": [0.0, 1.0, 1.0]},
        {"name": "D(b)", "type": "digital", "values": [0.0, 0.0, 1.0]}
    ]));
    let converted = convert(&input, &output, "vcd", &[]);
    assert!(converted.status.success(), "{converted:?}");
    let document = rspice_core::io::parse_vcd_file(&output).unwrap();
    assert_eq!(document.timescale.to_string(), "1 ns");
    for signal in &document.signals {
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, [0, 1], "{signal:?}");
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
