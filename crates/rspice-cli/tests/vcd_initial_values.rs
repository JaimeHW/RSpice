//! A real signal has no numeric sample before its first recorded assignment.
mod common;

use serde_json::json;
use std::path::Path;
use std::process::Command;

const SOURCE: &str = "$timescale 1 ns $end
$scope module test $end
$var wire 1 ! clock $end
$var wire 1 % late_bit $end
$var real 1 @ never $end
$var real 1 # later $end
$upscope $end
$enddefinitions $end
#0
0!
#5
1!
1%
r3 #
#9
0!
r4 #
";

fn convert(input: &Path, output: &Path, format: &str) {
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", format])
        .output()
        .unwrap();
    assert!(result.status.success(), "{format}: {result:?}");
}

#[test]
fn undefined_real_prefixes_survive_table_and_vcd_round_trips() {
    let directory = common::test_dir("vcd_initial_values");
    let source = directory.join("source.vcd");
    std::fs::write(&source, SOURCE).unwrap();
    for (format, extension) in [
        ("json", "json"),
        ("csv", "csv"),
        ("tsv", "tsv"),
        ("raw", "raw"),
        ("ascii", "ascii.raw"),
        ("hdf5", "h5"),
    ] {
        let table = directory.join(format!("table.{extension}"));
        let decoded = directory.join("decoded.json");
        convert(&source, &table, format);
        convert(&table, &decoded, "json");
        let document = common::read_json(&decoded);
        for (name, expected) in [
            ("D(clock)", json!([0.0, 1.0, 0.0])),
            ("D(late_bit)", json!([0.5, 1.0, 1.0])),
            ("E(never)", json!([null, null, null])),
            ("E(later)", json!([null, 3.0, 4.0])),
        ] {
            let signal = document["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|signal| signal["name"] == name)
                .unwrap();
            assert_eq!(signal["values"], expected, "{format}: {name}");
        }
        let output = directory.join("roundtrip.vcd");
        convert(&table, &output, "vcd");
        let dump = rspice_core::io::parse_vcd_file(&output).unwrap();
        assert_eq!(dump.signals.len(), 4);
        let real = |name: &str| {
            dump.signals
                .iter()
                .find(|signal| signal.variables[0].name == name)
                .unwrap()
        };
        assert!(real("never").changes.is_empty(), "{format}");
        let later = &real("later").changes;
        assert_eq!(later.len(), 2, "{format}");
        for (change, tick_ns, value) in [(&later[0], 5, 3.0), (&later[1], 9, 4.0)] {
            assert_eq!(
                change.tick * dump.timescale.femtoseconds(),
                tick_ns * 1_000_000,
                "{format}"
            );
            assert_eq!(change.value, rspice_core::io::VcdValue::Real(value));
        }
    }
}

#[test]
fn comparison_cannot_accept_fabricated_values_for_unassigned_reals() {
    let directory = common::test_dir("vcd_missing_real_comparison");
    let source = directory.join("source.vcd");
    let golden = directory.join("golden.csv");
    std::fs::write(&source, SOURCE).unwrap();
    let fabricated = "time,D(clock),D(late_bit),E(never),E(later)\n0,0,0.5,0.5,3\n5e-9,1,1,0.5,3\n9e-9,0,1,0.5,4\n";
    std::fs::write(&golden, fabricated).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "compare"])
        .arg(&source)
        .arg(&golden)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    let message = diagnostic["error"]["message"].as_str().unwrap();
    assert!(message.contains("E(never)"), "{diagnostic}");
    assert!(message.contains("undefined"), "{diagnostic}");
    assert_eq!(std::fs::read_to_string(&golden).unwrap(), fabricated);
}
