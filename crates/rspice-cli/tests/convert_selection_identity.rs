//! Conversion selects named quantities without silently including alias collisions.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn convert(source: &Path, output: &Path, format: &str, selectors: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(output)
        .args(["--to", format]);
    for selector in selectors {
        command.args(["--variables", selector]);
    }
    command.output().unwrap()
}

fn signal_names(path: &Path) -> Vec<String> {
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    json["signals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|signal| signal["name"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn qualified_table_names_and_repeated_aliases_select_only_the_requested_column() {
    let dir = test_dir("exact_table_selection");
    let source = dir.join("source.csv");
    std::fs::write(&source, "time,V(x),V(V(x)),I(y)\n0,1,2,3\n").unwrap();
    for (index, selectors) in [vec!["V(x)"], vec!["x", "v(X)", "x"], vec![" V(x) "]]
        .iter()
        .enumerate()
    {
        let output = dir.join(format!("selected-{index}.json"));
        let result = convert(&source, &output, "json", selectors);
        assert!(result.status.success(), "{result:?}");
        assert_eq!(signal_names(&output), ["V(x)"]);
    }
}

#[test]
fn ambiguous_table_aliases_cannot_replace_an_output_or_the_input() {
    let dir = test_dir("ambiguous_table_selection");
    for header in ["V(x),I(x)", "VDB(x),MAG(x)"] {
        let source = dir.join("source.csv");
        let bytes = format!("time,{header}\n0,1,2\n");
        std::fs::write(&source, &bytes).unwrap();
        for role in ["missing", "existing", "input"] {
            let output = if role == "input" {
                source.clone()
            } else {
                dir.join(format!("{role}.csv"))
            };
            if role == "existing" {
                std::fs::write(&output, "preserve output").unwrap();
            }
            let result = convert(&source, &output, "csv", &["x"]);
            assert_eq!(result.status.code(), Some(2), "{result:?}");
            assert!(String::from_utf8_lossy(&result.stderr).contains("ambiguous"));
            if role == "missing" {
                assert!(!output.exists());
            } else {
                assert_eq!(
                    std::fs::read_to_string(&output).unwrap(),
                    if role == "input" {
                        bytes.as_str()
                    } else {
                        "preserve output"
                    }
                );
            }
        }
    }
}

#[test]
fn qualified_event_names_are_exact_for_grid_and_vcd_sources() {
    let dir = test_dir("exact_event_selection");
    let grid = dir.join("source.csv");
    let dump = dir.join("source.vcd");
    std::fs::write(
        &grid,
        "time,D(clk),D(D(clk)),E(level),E(E(level))\n0,0,1,2,3\n1e-9,1,0,4,5\n",
    )
    .unwrap();
    let result = convert(&grid, &dump, "vcd", &[]);
    assert!(result.status.success(), "{result:?}");
    for source in [&grid, &dump] {
        for selector in ["D(clk)", "E(level)"] {
            let selected = dir.join("selected.vcd");
            let table = dir.join("selected.json");
            let result = convert(source, &selected, "vcd", &[selector]);
            assert!(result.status.success(), "{result:?}");
            let result = convert(&selected, &table, "json", &[]);
            assert!(result.status.success(), "{result:?}");
            assert_eq!(signal_names(&table), [selector]);
        }
    }
}

#[test]
fn ambiguous_vcd_scope_aliases_require_a_full_name() {
    let dir = test_dir("ambiguous_event_selection");
    let source = dir.join("source.vcd");
    let output = dir.join("selected.vcd");
    std::fs::write(&source, "$timescale 1 ns $end\n$scope module left $end\n$var wire 1 ! clk $end\n$upscope $end\n$scope module right $end\n$var wire 1 @ clk $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n1@\n").unwrap();
    std::fs::write(&output, "preserve output").unwrap();
    let result = convert(&source, &output, "vcd", &["clk"]);
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(String::from_utf8_lossy(&result.stderr).contains("ambiguous"));
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "preserve output");
    let result = convert(&source, &output, "vcd", &["D(left.clk)"]);
    assert!(result.status.success(), "{result:?}");
    let document = rspice_core::io::parse_vcd_file(&output).unwrap();
    assert_eq!(document.signals.len(), 1);
    assert_eq!(document.signals[0].variables[0].scope, ["left"]);
}

const SCALAR_ALIASES: &str = "$timescale 1 ns $end
$scope module top $end
$scope module left $end
$var wire 1 ! clk $end
$var wire 1 ! alias $end
$var real 1 @ level $end
$var real 1 @ analog_alias $end
$upscope $end
$scope module right $end
$var real 1 # alias $end
$upscope $end
$upscope $end
$scope module external $end
$var wire 1 ! remote_clk $end
$upscope $end
$enddefinitions $end
#0
0!
r2 @
r3 #
#5
1!
r4 @
";

#[test]
fn scalar_vcd_selectors_resolve_scoped_and_relative_aliases_with_quantity() {
    let dir = test_dir("scalar_vcd_aliases");
    let source = dir.join("source.vcd");
    let output = dir.join("selected.vcd");
    std::fs::write(&source, SCALAR_ALIASES).unwrap();
    let original = rspice_core::io::parse_vcd_file(&source).unwrap();
    for (selector, index) in [
        ("D(top.left.clk)", 0),
        ("D(top.left.alias)", 0),
        ("D(left.alias)", 0),
        ("left.alias", 0),
        ("D(alias)", 0),
        ("D(external.remote_clk)", 0),
        ("external.remote_clk", 0),
        ("E(top.left.level)", 1),
        ("E(top.left.analog_alias)", 1),
        ("E(left.analog_alias)", 1),
        ("left.analog_alias", 1),
        ("E(alias)", 2),
    ] {
        let result = convert(&source, &output, "vcd", &[selector]);
        assert!(result.status.success(), "{selector}: {result:?}");
        let selected = rspice_core::io::parse_vcd_file(&output).unwrap();
        assert_eq!(selected.signals.len(), 1);
        assert_eq!(
            selected.signals[0].variables,
            original.signals[index].variables
        );
        assert_eq!(selected.signals[0].changes, original.signals[index].changes);
        assert_eq!(selected.signals[0].kind, original.signals[index].kind);
    }
}

#[test]
fn scalar_vcd_quantity_mismatches_and_ambiguous_aliases_preserve_output() {
    let dir = test_dir("scalar_vcd_alias_errors");
    let source = dir.join("source.vcd");
    let output = dir.join("selected.vcd");
    std::fs::write(&source, SCALAR_ALIASES).unwrap();
    std::fs::write(&output, "preserve output").unwrap();
    for selector in [
        "alias",
        "D(left.level)",
        "E(left.clk)",
        "D(left.remote_clk)",
    ] {
        let result = convert(&source, &output, "vcd", &[selector]);
        assert_eq!(result.status.code(), Some(2), "{selector}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "preserve output");
        if selector == "alias" {
            assert!(String::from_utf8_lossy(&result.stderr).contains("ambiguous"));
        }
    }
}
