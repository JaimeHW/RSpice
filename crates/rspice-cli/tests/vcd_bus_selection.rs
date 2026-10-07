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
