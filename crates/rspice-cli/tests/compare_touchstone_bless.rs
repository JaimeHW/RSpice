//! A promoted Touchstone file must retain its network under the destination name.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn compare(source: &Path, golden: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(source)
        .arg(golden)
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn bless_refuses_a_port_count_change_even_with_selected_variables() {
    let dir = test_dir("bless_ports");
    let source = dir.join("source.s2p");
    // These bytes are also valid as three one-port samples at 1, 2 and 3 Hz.
    std::fs::write(&source, "# Hz S RI R 50\n1 0 0 2 0 0 3 0 0\n").unwrap();
    for existing in [false, true] {
        for selected in [false, true] {
            for json in [false, true] {
                let golden = dir.join(format!("golden-{existing}-{selected}-{json}.s1p"));
                let original = "# Hz S RI R 50\n1 10 0\n";
                if existing {
                    std::fs::write(&golden, original).unwrap();
                }
                let mut args = vec!["--bless"];
                if selected {
                    args.extend(["--variables", "Re(S11)"]);
                }
                if json {
                    args.push("--json");
                }
                let output = compare(&source, &golden, &args);
                assert_eq!(output.status.code(), Some(2), "{output:?}");
                assert!(
                    String::from_utf8_lossy(&output.stderr).contains("Touchstone"),
                    "{output:?}"
                );
                if existing {
                    assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
                } else {
                    assert!(!golden.exists());
                }
            }
        }
    }
}

#[test]
fn bless_refuses_invalid_or_ambiguous_destination_interpretations() {
    let dir = test_dir("bless_ambiguous_ports");
    let source = dir.join("source.s2p");
    std::fs::write(&source, "# Hz S RI R 50\n1 0 0 2 0 0 3 0 0\n").unwrap();
    for extension in ["s4p", "ts"] {
        let golden = dir.join(format!("golden.{extension}"));
        let output = compare(&source, &golden, &["--bless"]);
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(!golden.exists());
    }
}

#[test]
fn compatible_touchstone_names_bless_and_replay_without_data_loss() {
    let dir = test_dir("bless_port_aliases");
    for (tag, bytes, extension) in [
        ("alias", "# Hz S RI R 50\n1 0 0 2 0 0 3 0 0\n", "S02P"),
        ("inferred", "# Hz S RI R 50\n1 0 0 -2 0 0 -3 0 0\n", "ts"),
        (
            "explicit",
            "[Version] 2.0\n# Hz S RI R 50\n[Number of Ports] 2\n[Number of Frequencies] 1\n[Two-Port Data Order] 21_12\n[Network Data]\n1 0 0 2 0 0 3 0 0\n[End]\n",
            "ts",
        ),
    ] {
        let source = dir.join(format!("{tag}.s2p"));
        let golden = dir.join(format!("{tag}-golden.{extension}"));
        std::fs::write(&source, bytes).unwrap();
        let output = compare(&source, &golden, &["--bless", "--json"]);
        assert!(output.status.success(), "{tag}: {output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["blessed"], true);
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), bytes);
        let output = compare(&source, &golden, &["--abstol", "0", "--reltol", "0"]);
        assert!(output.status.success(), "{tag}: {output:?}");
    }
}
