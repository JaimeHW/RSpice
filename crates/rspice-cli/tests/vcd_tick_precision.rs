//! Flattening integer VCD ticks must not change their event times.
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

fn dump(path: &Path, timescale: &str, ticks: &[u64]) {
    let mut text =
        format!("$timescale {timescale} $end\n$var wire 1 ! d $end\n$enddefinitions $end\n");
    for (index, tick) in ticks.iter().enumerate() {
        text.push_str(&format!("#{tick}\n{}!\n", index % 2));
    }
    std::fs::write(path, text).unwrap();
}

#[test]
fn imprecise_vcd_ticks_cannot_be_flattened_compared_or_blessed() {
    let dir = test_dir("vcd_tick_precision");
    let input = dir.join("source.vcd");
    let golden = dir.join("golden.vcd");
    dump(&golden, "1 s", &[0, 9_007_199_254_740_992]);
    let original = std::fs::read(&golden).unwrap();
    for (timescale, tick) in [
        ("1 s", 9_007_199_254_740_993),
        ("1 s", u64::MAX),
        // The integer itself is exact, but scaling it to binary64 seconds loses a tick.
        ("1 fs", 9_007_199_254_740_892),
    ] {
        dump(&input, timescale, &[0, tick]);
        for format in ["csv", "tsv", "json", "raw", "ascii", "hdf5"] {
            let output = dir.join(format!("out.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            let converted = convert(&input, &output, format, &[]);
            assert_eq!(
                converted.status.code(),
                Some(1),
                "{tick} {format}: {converted:?}"
            );
            assert!(
                String::from_utf8_lossy(&converted.stderr).contains("tick"),
                "{converted:?}"
            );
            assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        }
        for bless in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "compare"])
                .arg(&input)
                .arg(&golden)
                .args(["--abstol", "0", "--reltol", "0"]);
            if bless {
                command.arg("--bless");
            }
            let compared = command.output().unwrap();
            assert_eq!(
                compared.status.code(),
                Some(1),
                "{tick} bless={bless}: {compared:?}"
            );
            assert_eq!(std::fs::read(&golden).unwrap(), original);
        }
        // Staying in VCD retains the exact integer event time.
        let output = dir.join("exact.vcd");
        assert!(convert(&input, &output, "vcd", &[]).status.success());
        assert_eq!(
            rspice_core::io::parse_vcd_file(&output).unwrap().signals[0].changes[1].tick,
            tick
        );
    }
}

#[test]
fn exactly_representable_large_ticks_remain_supported() {
    let dir = test_dir("vcd_exact_large_ticks");
    let input = dir.join("source.vcd");
    let output = dir.join("table.json");
    for tick in [
        9_007_199_254_740_992,
        9_007_199_254_740_994,
        u64::MAX - 2047,
    ] {
        dump(&input, "1 s", &[0, tick]);
        let converted = convert(&input, &output, "json", &[]);
        assert!(converted.status.success(), "{tick}: {converted:?}");
        let table: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
        assert_eq!(
            table["scale"]["values"][1].as_f64().unwrap() as u128,
            u128::from(tick)
        );
    }
}
