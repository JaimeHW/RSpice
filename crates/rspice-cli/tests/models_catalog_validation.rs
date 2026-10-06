mod common;

use std::process::Command;

const PACK: &str = "ok\tbasic\tok\tpermissive\tMIT\t1\t\t3\t0\t3\t0\t1\t100\tdiode\tOK\n";
const GOOD: &str = "PART_GOOD\tmodel\tdiode\tok\tparts.sp\t1\t0\ttop\n";

#[test]
fn invalid_pack_identity_and_paths_fail_before_any_listing() {
    let dir = common::test_dir("invalid_pack_identity");
    let mut cases = vec![format!("{PACK}{PACK}")];
    for (column, value) in [
        (0, ""),
        (2, "/outside"),
        (2, "../outside"),
        (6, "../outside.lib"),
        (6, "C:/outside.lib"),
        (5, "yes"),
    ] {
        let mut fields: Vec<_> = PACK.trim_end().split('\t').collect();
        fields[column] = value;
        cases.push(fields.join("\t"));
    }
    for content in cases {
        std::fs::write(dir.join("PACKS.tsv"), content).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "--error-format",
                "json",
                "models",
                "--models-dir",
            ])
            .arg(dir.path())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert!(
            error["error"]["message"]
                .as_str()
                .unwrap()
                .contains("PACKS.tsv"),
            "{error}"
        );
    }
}

#[test]
fn catalog_paths_cannot_be_reinterpreted_as_a_different_source() {
    let dir = common::test_dir("catalog_source_paths");
    std::fs::write(dir.join("PACKS.tsv"), PACK).unwrap();
    for source in [
        "/outside.lib",
        "../outside.lib",
        "C:/outside.lib",
        r"lib\parts.lib",
        "lib/parts.lib:stream",
    ] {
        std::fs::write(
            dir.join("CATALOG.tsv"),
            format!("PART\tmodel\tdiode\tok\t{source}\t1\t0\ttop\n"),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "--error-format",
                "json",
                "models",
                "--models-dir",
            ])
            .arg(dir.path())
            .args(["--part", "PART"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{source}: {output:?}");
        assert!(output.stdout.is_empty());
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        let message = error["error"]["message"].as_str().unwrap();
        assert!(
            message.contains("CATALOG.tsv: catalog line 1")
                && message.contains("invalid relative source path"),
            "{error}"
        );
    }
}

#[test]
fn catalog_errors_keep_their_location_and_never_print_a_partial_listing() {
    let dir = common::test_dir("malformed_catalog");
    std::fs::write(dir.join("PACKS.tsv"), PACK).unwrap();
    let row = "PART_BAD\tmodel\tdiode\tok\tparts.sp\t2\t0\ttop";
    for (column, value) in [
        (1, "unknown"),
        (3, "missing"),
        (5, "0"),
        (5, "bad"),
        (6, "yes"),
        (7, "private"),
    ] {
        let mut fields: Vec<_> = row.split('\t').collect();
        fields[column] = value;
        std::fs::write(
            dir.join("CATALOG.tsv"),
            format!("# header\n{GOOD}{}\n", fields.join("\t")),
        )
        .unwrap();
        for query in [
            ["--part", "PART_GOOD"],
            ["--device", "diode"],
            ["--search", "PART"],
        ] {
            for shippable_only in [false, true] {
                let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
                command
                    .args([
                        "--quiet",
                        "--error-format",
                        "json",
                        "models",
                        "--models-dir",
                    ])
                    .arg(dir.path())
                    .args(query);
                if shippable_only {
                    command.arg("--shippable-only");
                }
                let output = command.output().unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(2),
                    "{column}: {value}: {output:?}"
                );
                assert!(output.stdout.is_empty(), "{output:?}");
                let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                assert!(
                    error["error"]["message"]
                        .as_str()
                        .unwrap()
                        .contains("CATALOG.tsv: catalog line 3"),
                    "{error}"
                );
            }
        }
    }
}

#[test]
fn catalog_read_failures_name_the_index_and_keep_line_context() {
    let dir = common::test_dir("catalog_read_error");
    std::fs::write(dir.join("PACKS.tsv"), PACK).unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "--error-format",
                "json",
                "models",
                "--models-dir",
            ])
            .arg(dir.path())
            .args(["--search", "PART"])
            .output()
            .unwrap()
    };
    let missing = invoke();
    assert_eq!(missing.status.code(), Some(2));
    let error: serde_json::Value = serde_json::from_slice(&missing.stderr).unwrap();
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("CATALOG.tsv"),
        "{error}"
    );
    let mut bytes = format!("# header\n{GOOD}").into_bytes();
    bytes.extend_from_slice(&[0xff, b'\n']);
    std::fs::write(dir.join("CATALOG.tsv"), bytes).unwrap();
    let invalid = invoke();
    assert_eq!(invalid.status.code(), Some(2));
    assert!(invalid.stdout.is_empty());
    let error: serde_json::Value = serde_json::from_slice(&invalid.stderr).unwrap();
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("CATALOG.tsv: catalog line 3"),
        "{error}"
    );
}

#[test]
fn legacy_catalogs_remain_readable_and_declared_restrictions_still_apply() {
    let dir = common::test_dir("legacy_catalog");
    std::fs::write(dir.join("PACKS.tsv"), PACK).unwrap();
    std::fs::write(
        dir.join("CATALOG.tsv"),
        concat!(
            "PART_OLD\tmodel\tdiode\tok\tparts.sp\t1\n",
            "PART_BLOCKED\tmodel\tdiode\tok\tparts.sp\t2\t1\n",
            "PART_NESTED\tmodel\tdiode\tok\tparts.sp\t3\t0\tnested\n",
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "models", "--models-dir"])
        .arg(dir.path())
        .args(["--search", "PART", "--shippable-only"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("PART_OLD"), "{text}");
    assert!(
        !text.contains("PART_BLOCKED") && !text.contains("PART_NESTED"),
        "{text}"
    );
}
