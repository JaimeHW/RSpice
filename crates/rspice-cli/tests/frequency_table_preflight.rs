//! Checking a table request must use the executor's admission rules.
mod common;
use std::process::Command;

#[test]
fn check_rejects_invalid_tables_in_direct_explicit_and_declarative_control_requests() {
    let directory = common::test_dir("table-preflight");
    for (index, (table, command, diagnostic)) in [
        ("", "ac data=points", "unknown .DATA table"),
        ("", "noise V(out) V1 data=points", "unknown .DATA table"),
        (
            ".data points load\n1\n.enddata",
            "ac data=points",
            "no FREQ or HERTZ",
        ),
        (
            ".data points FREQ HERTZ\n1 2\n.enddata",
            "ac data=points",
            "ambiguous frequency",
        ),
        (
            ".data points FREQ\n-1\n.enddata",
            "ac data=points",
            "nonnegative",
        ),
        (
            ".data points FREQ\n0\n.enddata",
            "noise V(out) V1 data=points",
            "strictly positive",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        for (route, cards) in [
            format!(".{command}"),
            format!(".control\n{command}\n.endc"),
            format!(".{command}\n.control\nrun\n.endc"),
        ]
        .into_iter()
        .enumerate()
        {
            let input = directory.join(format!("{index}-{route}.cir"));
            std::fs::write(
                &input,
                format!("Table preflight\nV1 out 0 AC 1\nR1 out 0 1k\n{table}\n{cards}\n.end\n"),
            )
            .unwrap();
            let check = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "check", "--json"])
                .arg(&input)
                .output()
                .unwrap();
            assert!(!check.status.success(), "{index}/{route}: {check:?}");
            let report: serde_json::Value = serde_json::from_slice(&check.stdout).unwrap();
            assert_eq!(report["valid"], false);
            assert!(
                report["errors"].to_string().contains(diagnostic),
                "{report}"
            );
            let run = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&input)
                .output()
                .unwrap();
            assert!(!run.status.success(), "{run:?}");
            assert!(
                String::from_utf8_lossy(&run.stderr).contains(diagnostic),
                "{run:?}"
            );
        }
    }
}

#[test]
fn an_unused_declarative_table_does_not_invalidate_a_replacement_control_script() {
    let directory = common::test_dir("unused-table-preflight");
    let input = directory.join("input.cir");
    std::fs::write(
        &input,
        "Unused table\nV1 out 0 1\nR1 out 0 1k\n.ac data=missing\n.control\nop\n.endc\n.end\n",
    )
    .unwrap();
    for command in ["check", "run"] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", command])
            .arg(&input)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
}
