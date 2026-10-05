mod common;
use std::process::Command;
#[test]
fn shippable_filter_applies_before_the_result_cap_in_every_query() {
    let dir = common::test_dir("models_filter");
    let packs = [
        "ok\tbasic\tok\tpermissive\tMIT\t1\t\t203\t0\t203\t0\t1\t100\tdiode\tOK",
        "blocked\tbasic\tblocked\tunknown\tUNKNOWN\t0\t\t1\t0\t1\t0\t1\t100\tdiode\tBlocked",
    ]
    .join("\n");
    std::fs::write(dir.join("PACKS.tsv"), packs).unwrap();
    let mut catalog = String::new();
    for n in 0..201 {
        catalog.push_str(&format!(
            "PART_RESTRICTED{n}\tmodel\tdiode\tok\tparts.sp\t1\t1\ttop\n"
        ));
    }
    catalog.push_str("PART_BLOCKED\tmodel\tdiode\tblocked\tparts.sp\t1\t0\ttop\n");
    catalog.push_str("PART_OK\tmodel\tdiode\tok\tparts.sp\t1\t0\ttop\n");
    std::fs::write(dir.join("CATALOG.tsv"), catalog).unwrap();
    for query in [
        ["--device", "DIODE"],
        ["--search", "PART"],
        ["--part", "PART_OK"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "models", "--models-dir"])
            .arg(dir.path())
            .arg("--shippable-only")
            .args(query)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("PART_OK"), "{text}");
        assert!(
            !text.contains("PART_RESTRICTED") && !text.contains("PART_BLOCKED"),
            "{text}"
        );
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "models", "--models-dir"])
        .arg(dir.path())
        .args(["--shippable-only", "--part", "PART_BLOCKED"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}
