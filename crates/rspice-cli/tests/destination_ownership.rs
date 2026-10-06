mod common;
use std::process::Command;

#[test]
fn results_checkpoints_and_reports_cannot_replace_deck_sources() {
    for nested in [false, true] {
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let dir = common::test_dir("source_collision");
            let deck = dir.join("deck.cir");
            let include = dir.join("values.inc");
            let source = if nested {
                "* source\n.include values.inc\nV1 in 0 1\nR1 in 0 {load}\n.tran 1n 2n\n.end\n"
            } else {
                "* source\nV1 in 0 1\nR1 in 0 1k\n.tran 1n 2n\n.end\n"
            };
            let included = ".param load=1000\n";
            std::fs::write(&deck, source).unwrap();
            std::fs::write(&include, included).unwrap();
            let target = if nested { &include } else { &deck };
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv"])
                .args(&flags)
                .arg(target)
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(2),
                "{nested}, {flags:?}: {output:?}"
            );
            assert!(
                String::from_utf8_lossy(&output.stderr).contains("source"),
                "{output:?}"
            );
            assert_eq!(std::fs::read_to_string(&deck).unwrap(), source);
            assert_eq!(std::fs::read_to_string(&include).unwrap(), included);
            assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
        }
    }
}

#[test]
fn derived_parallel_outputs_cannot_replace_included_sources() {
    let dir = common::test_dir("derived_source_collision");
    let deck = dir.join("deck.cir");
    let include = dir.join("result.ss.csv");
    let source = "* source\n.include result.ss.csv\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
    let included = "* an otherwise empty included source\n";
    std::fs::write(&deck, source).unwrap();
    std::fs::write(&include, included).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
        .arg(dir.join("result.csv"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("source"),
        "{output:?}"
    );
    assert_eq!(std::fs::read_to_string(&deck).unwrap(), source);
    assert_eq!(std::fs::read_to_string(&include).unwrap(), included);
}

#[test]
fn default_output_and_stdin_includes_preserve_sources() {
    use std::io::Write;
    use std::process::Stdio;
    let dir = common::test_dir("default_source_collision");
    let deck = dir.join("deck.raw");
    let source = "* source\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
    std::fs::write(&deck, source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .output()
        .unwrap();
    // With no output path requested, this run only reports to the console.
    assert!(output.status.success(), "{output:?}");
    assert_eq!(std::fs::read_to_string(&deck).unwrap(), source);

    let include = dir.join("empty.inc");
    std::fs::write(&include, "* source include\n").unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run", "-", "-f", "csv", "-o"])
        .arg(&include)
        .current_dir(&dir)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"* stdin\n.include empty.inc\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert_eq!(
        std::fs::read_to_string(include).unwrap(),
        "* source include\n"
    );
}

#[test]
fn overlapping_result_checkpoint_and_report_paths_are_refused_without_overwriting() {
    let dir = common::test_dir("destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(
        &deck,
        "* collision\nV1 in 0 1\nR1 in 0 1k\n.tran 1n 2n\n.end\n",
    )
    .unwrap();
    for flags in [
        vec!["--meas-file"],
        vec!["--summary"],
        vec!["--report-format", "junit", "--report-file"],
        vec!["--checkpoint"],
    ] {
        let destination = dir.join("result.csv");
        std::fs::write(&destination, "original bytes").unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(&destination)
            .args(flags)
            .arg(dir.join("./result.csv"))
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert_eq!(
            std::fs::read_to_string(destination).unwrap(),
            "original bytes"
        );
    }
}

#[test]
fn derived_corner_paths_cannot_overwrite_reports_even_on_workers() {
    let dir = common::test_dir("derived_destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* collision\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let report = dir.join("result.ss.csv");
    std::fs::write(&report, "original bytes").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
        .arg(dir.join("result.csv"))
        .arg("--meas-file")
        .arg(&report)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("collides"));
    assert_eq!(std::fs::read_to_string(report).unwrap(), "original bytes");
}

#[test]
fn reports_cannot_share_a_destination_with_each_other() {
    let dir = common::test_dir("report_destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* OP\nV1 in 0 1\nR1 in 0 1k\n.end\n").unwrap();
    let report = dir.join("report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("--summary")
        .arg(&report)
        .arg("--meas-file")
        .arg(&report)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(!report.exists());
}
