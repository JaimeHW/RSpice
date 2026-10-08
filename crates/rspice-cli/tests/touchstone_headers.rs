//! Header interpretation must preserve the meaning of imported RF samples.
mod common;

use common::{read_json, test_dir};
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", "json"])
        .output()
        .unwrap()
}

fn signal<'a>(table: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == name)
        .unwrap_or_else(|| panic!("missing {name} in {table}"))
}

#[test]
fn later_option_lines_cannot_reinterpret_network_samples() {
    let dir = test_dir("touchstone_first_options");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    for source in [
        "# Hz S RI R 75\n# GHz S MA R 50\n1 0.2 0.3\n",
        "# Hz S RI R 75\n1 0.2 0.3\n# GHz S MA R 50\n",
        "# Hz S RI R 75\n# ignored additional option line\n1 0.2 0.3\n",
    ] {
        std::fs::write(&input, source).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{source}: {result:?}");
        let table = read_json(&output);
        assert_eq!(table["scale"]["values"], serde_json::json!([1.0]));
        assert_eq!(signal(&table, "S11")["real"], serde_json::json!([0.2]));
        assert_eq!(signal(&table, "S11")["imag"], serde_json::json!([0.3]));
        assert_eq!(signal(&table, "Z0(1)")["values"], serde_json::json!([75.0]));
    }
}

#[test]
fn default_and_reordered_options_preserve_units_values_and_references() {
    let dir = test_dir("touchstone_option_defaults");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    // Exercise every independently omitted field. Defaults are GHz, S, MA, 50 ohm.
    for mask in 0..16 {
        let mut fields = Vec::new();
        for (index, field) in ["R 75", "RI", "S", "MHz"].iter().enumerate() {
            if mask & (1 << index) != 0 {
                fields.push(*field);
            }
        }
        let source = format!("# {}\n1 0.25 90\n", fields.join(" "));
        std::fs::write(&input, &source).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{source}: {result:?}");
        let table = read_json(&output);
        let frequency = if mask & 8 != 0 { 1e6 } else { 1e9 };
        let reference = if mask & 1 != 0 { 75.0 } else { 50.0 };
        assert_eq!(table["scale"]["values"], serde_json::json!([frequency]));
        let (real, imag) = if mask & 2 != 0 {
            (0.25, 90.0)
        } else {
            (0.0, 0.25)
        };
        assert!((signal(&table, "S11")["real"][0].as_f64().unwrap() - real).abs() < 1e-14);
        assert!((signal(&table, "S11")["imag"][0].as_f64().unwrap() - imag).abs() < 1e-14);
        assert_eq!(
            signal(&table, "Z0(1)")["values"],
            serde_json::json!([reference])
        );
    }
    for options in ["RI R 75 Hz S", "s ri hz r 75", "R 75 S Hz RI"] {
        std::fs::write(&input, format!("# {options}\n1 0.2 0.3\n")).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{options}: {result:?}");
        let table = read_json(&output);
        assert_eq!(table["scale"]["values"], serde_json::json!([1.0]));
        assert_eq!(signal(&table, "S11")["imag"], serde_json::json!([0.3]));
    }
}

#[test]
fn conflicting_or_unknown_option_fields_preserve_the_destination() {
    let dir = test_dir("touchstone_invalid_options");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    for options in [
        "Hz GHz S RI",
        "Hz S RI MA",
        "Hz S S RI",
        "Hz S RI R 50 R 75",
        "Hz S RI R",
        "Hz S RI R 0",
        "Hz Y RI",
        "Hz S RI unknown",
    ] {
        std::fs::write(&input, format!("# {options}\n1 0.2 0.3\n")).unwrap();
        std::fs::write(&output, "previous result").unwrap();
        let result = convert(&input, &output);
        assert!(!result.status.success(), "{options}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "previous result");
    }
}
