//! Wide comparisons retain exact identity, selection order and mismatch locations.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn compare(result: &Path, golden: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(result)
        .arg(golden)
        .args(["--json", "--abstol", "0", "--reltol", "0"])
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn wide_reordered_signals_and_explicit_selections_keep_their_identity_and_order() {
    const COUNT: usize = 1500;
    let dir = test_dir("wide_signal_index");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    let table = |reverse: bool, changed: bool| {
        let indices: Vec<_> = if reverse {
            (0..COUNT).rev().collect()
        } else {
            (0..COUNT).collect()
        };
        let mut header = String::from("time");
        let mut row = String::from("0");
        for index in indices {
            let name = if reverse {
                format!("v(N{index})")
            } else {
                format!("V(n{index})")
            };
            header.push_str(&format!(",{name}"));
            let value = index + usize::from(changed && matches!(index, 17 | 1498));
            row.push_str(&format!(",{value}"));
        }
        format!("{header}\n{row}\n")
    };
    std::fs::write(&result, table(false, false)).unwrap();
    std::fs::write(&golden, table(true, false)).unwrap();
    let selectors = (0..COUNT)
        .rev()
        .flat_map(|index| ["--variables".to_owned(), format!("n{index}")])
        .collect::<Vec<_>>();
    let selected: Vec<_> = selectors.iter().map(String::as_str).collect();
    for flags in [vec![], selected.clone()] {
        let output = compare(&result, &golden, &flags);
        assert!(output.status.success(), "{output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["num_variables"], COUNT + 1);
        assert_eq!(report["num_differences"], 0);
    }
    std::fs::write(&golden, table(true, true)).unwrap();
    let mut selected_fast = selected;
    selected_fast.push("--fail-fast");
    for (flags, first) in [(vec!["--fail-fast"], "V(n17)"), (selected_fast, "V(n1498)")] {
        let output = compare(&result, &golden, &flags);
        assert_eq!(output.status.code(), Some(3), "{output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["num_differences"], 1);
        assert_eq!(report["differences"][0]["variable"], first);
    }
}

#[test]
fn qualified_names_do_not_select_a_different_signals_alias() {
    let dir = test_dir("qualified_alias_index");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(x),V(V(x))\n0,1,2\n").unwrap();
    std::fs::write(&golden, "time,V(x),V(V(x))\n0,1,9\n").unwrap();
    for flags in [
        vec!["--variables", "V(x)"],
        vec![
            "--variables",
            "x",
            "--variables",
            "V(x)",
            "--variables",
            "x",
        ],
    ] {
        let output = compare(&result, &golden, &flags);
        assert!(output.status.success(), "{output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["num_variables"], 2);
    }
    assert_eq!(
        compare(&result, &golden, &["--variables", "V(V(x))"])
            .status
            .code(),
        Some(3)
    );
}
