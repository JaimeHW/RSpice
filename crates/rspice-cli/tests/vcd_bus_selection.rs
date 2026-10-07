//! Bus selection keeps scope, declaration order, aliases and bit identity.
mod common;

use common::test_dir;
use rspice_core::io::{VcdDocument, parse_vcd_file};
use std::path::Path;
use std::process::{Command, Output};

const BUSES: &str = "$timescale 1 ns $end
$scope module top $end
$scope module left $end
$var wire 4 ! bus [-2:1] $end
$var wire 4 ! alias [3:0] $end
$upscope $end
$scope module right $end
$var wire 4 @ bus [3:0] $end
$upscope $end
$scope module simple $end
$var wire 4 # implicit $end
$upscope $end
$upscope $end
$enddefinitions $end
#0
b0101 !
b1010 @
b0011 #
#5
b1100 !
b0101 @
b1010 #
";

fn convert(source: &Path, destination: &Path, selector: &str, expanded: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", "vcd", "--variables", selector]);
    if expanded {
        command.arg("--expand-buses");
    }
    command.output().unwrap()
}

fn only_scope(document: &VcdDocument, scope: &str) {
    assert!(!document.signals.is_empty());
    for signal in &document.signals {
        assert!(
            signal
                .variables
                .iter()
                .all(|variable| variable.scope == ["top", scope])
        );
    }
}

#[test]
fn scoped_qualified_and_alias_bus_names_resolve_the_same_vector() {
    let dir = test_dir("scoped_bus_selection");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for selector in [
        "left.bus",
        "top.left.bus",
        "left.bus[-2:1]",
        "top.left.bus [-2:1]",
        "D(left.bus)",
        "D(left.bus[-2:1])",
        "D(top.left.bus [-2:1])",
        "D(left.alias[3:0])",
    ] {
        let output = convert(&source, &destination, selector, false);
        assert!(output.status.success(), "{selector}: {output:?}");
        assert!(output.stderr.is_empty(), "{selector}: {output:?}");
        let document = parse_vcd_file(&destination).unwrap();
        only_scope(&document, "left");
        assert_eq!(document.signals.len(), 1);
        assert_eq!(document.signals[0].variables.len(), 2);
        assert_eq!(document.signals[0].width, 4);
        assert_eq!(document.signals[0].changes.len(), 2);
    }
}

#[test]
fn scoped_bit_selections_keep_the_vector_and_report_widening() {
    let dir = test_dir("scoped_bus_bits");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for (selector, scope) in [
        ("left.bus[-1]", "left"),
        ("D(top.left.bus[1])", "left"),
        ("D(left.alias[2])", "left"),
        ("D(simple.implicit[2])", "simple"),
    ] {
        let output = convert(&source, &destination, selector, false);
        assert!(output.status.success(), "{selector}: {output:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(selector) && stderr.contains("whole"),
            "{stderr}"
        );
        let document = parse_vcd_file(&destination).unwrap();
        only_scope(&document, scope);
        assert_eq!(document.signals.len(), 1);
        assert_eq!(document.signals[0].width, 4);
    }
}

#[test]
fn ambiguous_invalid_and_wrong_quantity_bus_selections_preserve_outputs() {
    let dir = test_dir("invalid_bus_selection");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for selector in [
        "bus",
        "D(bus)",
        "left.bus[2]",
        "left.bus[1:-2]",
        "D(simple.implicit[4])",
        "E(left.bus)",
    ] {
        std::fs::write(&destination, "preserve output").unwrap();
        let output = convert(&source, &destination, selector, false);
        assert_eq!(output.status.code(), Some(2), "{selector}: {output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "preserve output"
        );
    }
}

#[test]
fn expanded_bus_names_keep_every_member_and_each_alias() {
    let dir = test_dir("expanded_bus_names");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for selector in [
        "left.bus",
        "D(left.bus[-2:1])",
        "top.left.alias",
        "D(left.alias[3:0])",
    ] {
        let output = convert(&source, &destination, selector, true);
        assert!(output.status.success(), "{selector}: {output:?}");
        assert!(output.stderr.is_empty(), "{output:?}");
        let document = parse_vcd_file(&destination).unwrap();
        only_scope(&document, "left");
        assert_eq!(document.signals.len(), 4);
        for (position, signal) in document.signals.iter().enumerate() {
            assert_eq!(signal.width, 1);
            assert_eq!(
                signal.variables[0].name,
                format!("bus[{}]", position as i64 - 2)
            );
            assert_eq!(signal.variables[1].name, format!("alias[{}]", 3 - position));
            for (tick, bits) in [(0, [0, 1, 0, 1]), (5, [1, 1, 0, 0])] {
                let actual = signal
                    .changes
                    .iter()
                    .take_while(|change| change.tick <= tick)
                    .last()
                    .unwrap();
                let expected = if bits[position] == 0 {
                    rspice_core::io::VcdBit::Zero
                } else {
                    rspice_core::io::VcdBit::One
                };
                assert_eq!(
                    actual.value,
                    rspice_core::io::VcdValue::Logic(vec![expected])
                );
            }
        }
    }
}

#[test]
fn expanded_bus_bit_aliases_reach_one_correct_member_without_widening() {
    let dir = test_dir("expanded_bus_bits");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for (selector, name) in [
        ("left.bus[-1]", "bus[-1]"),
        ("D(left.bus[-1])", "bus[-1]"),
        ("D(top.left.alias[2])", "bus[-1]"),
        ("left.alias[0]", "bus[1]"),
    ] {
        let output = convert(&source, &destination, selector, true);
        assert!(output.status.success(), "{selector}: {output:?}");
        assert!(output.stderr.is_empty(), "{output:?}");
        let document = parse_vcd_file(&destination).unwrap();
        only_scope(&document, "left");
        assert_eq!(document.signals.len(), 1);
        assert_eq!(document.signals[0].width, 1);
        assert_eq!(document.signals[0].variables[0].name, name);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&source)
        .arg(&destination)
        .args([
            "--to",
            "vcd",
            "--expand-buses",
            "--variables",
            "D(left.bus[-2])",
            "--start",
            "3n",
            "--stop",
            "5n",
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let document = parse_vcd_file(&destination).unwrap();
    let changes = &document.signals[0].changes;
    assert_eq!(changes.len(), 2);
    assert_eq!(
        changes[0].tick * document.timescale.femtoseconds(),
        3_000_000
    );
    assert_eq!(
        changes[1].tick * document.timescale.femtoseconds(),
        5_000_000
    );
    assert_eq!(
        changes[0].value,
        rspice_core::io::VcdValue::Logic(vec![rspice_core::io::VcdBit::Zero])
    );
    assert_eq!(
        changes[1].value,
        rspice_core::io::VcdValue::Logic(vec![rspice_core::io::VcdBit::One])
    );
}

#[test]
fn expanded_ambiguities_and_missing_bits_preserve_destinations() {
    let dir = test_dir("expanded_bus_errors");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, BUSES).unwrap();
    for selector in ["bus", "D(bus)", "bus[0]", "D(left.bus[2])", "E(left.bus)"] {
        std::fs::write(&destination, "preserve output").unwrap();
        let output = convert(&source, &destination, selector, true);
        assert_eq!(output.status.code(), Some(2), "{selector}: {output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "preserve output"
        );
    }
    std::fs::write(&source, BUSES.replace("alias [3:0]", "bus [1:-2]")).unwrap();
    let output = convert(&source, &destination, "D(left.bus[0])", true);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("ambiguous"));
    assert_eq!(
        std::fs::read_to_string(&destination).unwrap(),
        "preserve output"
    );
}

#[test]
fn one_bit_declared_ranges_select_without_a_widening_note() {
    let dir = test_dir("one_bit_declared_range");
    let source = dir.join("source.vcd");
    let destination = dir.join("selected.vcd");
    std::fs::write(&source, "$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! bus [4:4] $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n#5\n1!\n").unwrap();
    for expanded in [false, true] {
        for selector in ["bus", "D(bus[4:4])", "D(top.bus[4])", "bus[4]"] {
            let output = convert(&source, &destination, selector, expanded);
            assert!(
                output.status.success(),
                "{selector}, expanded={expanded}: {output:?}"
            );
            assert!(output.stderr.is_empty(), "{output:?}");
            let document = parse_vcd_file(&destination).unwrap();
            assert_eq!(document.signals.len(), 1);
            assert_eq!(document.signals[0].width, 1);
            assert_eq!(document.signals[0].changes.len(), 2);
        }
    }
}
