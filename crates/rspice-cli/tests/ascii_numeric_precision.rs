//! Text waveform admission must not turn a nonzero value into zero.
mod common;

use rspice_core::io::{VcdValue, parse_vcd_file};
use std::path::Path;
use std::process::{Command, Output};

fn raw(literal: &str, complex: bool) -> String {
    let (flags, first, second) = if complex {
        ("complex", format!("0,0 1,{literal} 0,0"), "1,0 2,3 1,0")
    } else {
        ("real", format!("0 {literal} 0"), "1 2 1")
    };
    format!(
        "Title: precision\nPlotname: Transient Analysis\nFlags: {flags}\nNo. Variables: 3\nNo. Points: 2\nVariables:\n0 time time\n1 V(out) voltage\n2 V(keep) voltage\nValues:\n0 {first}\n1 {second}\n"
    )
}

fn vcd(literal: &str) -> String {
    format!(
        "$timescale 1 s $end\n$var real 1 ! out $end\n$var wire 1 % keep $end\n$enddefinitions $end\n#0\nr{literal} !\n0%\n#1\nr2 !\n1%\n"
    )
}

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

fn assert_underflow(result: Output) {
    assert_eq!(result.status.code(), Some(1), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("underflow"),
        "{result:?}"
    );
}

#[test]
fn underflow_cannot_publish_conversions_or_comparison_baselines() {
    let dir = common::test_dir("ascii_underflow_publication");
    for (name, extension, source, baseline, selector) in [
        (
            "real",
            "raw",
            raw("1e-999", false),
            raw("0", false),
            "V(keep)",
        ),
        (
            "complex",
            "raw",
            raw("-2e-324", true),
            raw("0", true),
            "V(keep)",
        ),
        ("event", "vcd", vcd("1e-999"), vcd("0"), "D(keep)"),
    ] {
        let input = dir.join(format!("{name}.{extension}"));
        let golden = dir.join(format!("{name}.golden.{extension}"));
        let missing = dir.join(format!("{name}.missing.{extension}"));
        std::fs::write(&input, &source).unwrap();
        std::fs::write(&golden, &baseline).unwrap();
        for format in ["json", "raw", "ascii", "vcd"] {
            let output = dir.join(format!("protected.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            // Even selecting only the valid signal and clipping past the bad
            // sample must validate the entire source before publication.
            assert_underflow(convert(
                &input,
                &output,
                format,
                &["--variables", selector, "--start", "1"],
            ));
            assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        }
        assert_underflow(convert(&input, &input, extension, &[]));
        assert_eq!(std::fs::read_to_string(&input).unwrap(), source);
        for (destination, bless) in [(&golden, false), (&golden, true), (&missing, true)] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "compare"])
                .arg(&input)
                .arg(destination);
            if bless {
                command.arg("--bless");
            }
            assert_underflow(command.output().unwrap());
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), baseline);
            assert!(!missing.exists());
        }
        if extension == "raw" {
            let error =
                rspice_formats::spice_raw::decode_spice_raw(source.as_bytes(), Default::default())
                    .unwrap_err();
            assert!(error.to_string().contains("underflow"), "{error}");
        }
    }
}

#[test]
fn selecting_a_valid_raw_plot_cannot_hide_underflow_in_another_plot() {
    let dir = common::test_dir("ascii_underflow_plot");
    let input = dir.join("plots.raw");
    let output = dir.join("selected.json");
    std::fs::write(
        &input,
        format!("{}{}", raw("0", false), raw("1e-999", false)),
    )
    .unwrap();
    assert_underflow(convert(&input, &output, "json", &["--section", "1"]));
    assert!(!output.exists());
}

#[test]
fn text_conversion_preserves_authored_zero_and_representable_subnormals() {
    let dir = common::test_dir("ascii_precision_boundaries");
    for (literal, expected) in [
        ("0e999", 0.0_f64),
        ("-0e-999", -0.0_f64),
        ("5e-324", 5e-324_f64),
        ("-5e-324", -5e-324_f64),
    ] {
        for (extension, source, signal_name) in [
            ("raw", raw(literal, false), "V(out)"),
            ("vcd", vcd(literal), "E(out)"),
        ] {
            let input = dir.join(format!("source.{extension}"));
            let output = dir.join("table.json");
            std::fs::write(&input, source).unwrap();
            let result = convert(&input, &output, "json", &[]);
            assert!(result.status.success(), "{literal}/{extension}: {result:?}");
            let table = common::read_json(&output);
            let signal = table["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|signal| signal["name"] == signal_name)
                .unwrap();
            assert_eq!(
                signal["values"][0].as_f64().unwrap().to_bits(),
                expected.to_bits()
            );
            if extension == "vcd" {
                let output = dir.join("native.vcd");
                let result = convert(&input, &output, "vcd", &[]);
                assert!(result.status.success(), "{literal}: {result:?}");
                let document = parse_vcd_file(&output).unwrap();
                let VcdValue::Real(value) = document.signals[0].changes[0].value else {
                    panic!("expected real event")
                };
                assert_eq!(value.to_bits(), expected.to_bits());
            }
        }
    }
}
