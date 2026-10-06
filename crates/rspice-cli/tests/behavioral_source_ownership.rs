mod common;
use std::process::Command;

#[test]
fn every_file_lookup_function_preserves_its_input() {
    for function in [
        "table",
        "tablefile",
        "fasttable",
        "fasttablefile",
        "cubic",
        "cubicfile",
        "akima",
        "akimafile",
        "wodicka",
        "wodickafile",
        "bli",
        "blifile",
    ] {
        let dir = common::test_dir("behavioral_file_ownership");
        let source = dir.join("source.dat");
        let original = "0 1\n1e-9 1\n2e-9 1\n3e-9 1\n4e-9 1\n";
        std::fs::write(&source, original).unwrap();
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            format!("table\nB1 out 0 V={function}(\"source.dat\")\nR1 out 0 1k\n.op\n.end\n"),
        )
        .unwrap();
        for (output, expected) in [(dir.join("result.csv"), 0), (source.clone(), 2)] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv", "-o"])
                .arg(output)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(expected),
                "{function}: {result:?}"
            );
            if expected == 2 {
                assert!(
                    String::from_utf8_lossy(&result.stderr).contains("source"),
                    "{result:?}"
                );
            }
        }
        assert_eq!(std::fs::read_to_string(&source).unwrap(), original);
    }
}

#[test]
fn scoped_functions_and_passive_expressions_protect_inputs_from_results_and_reports() {
    for body in [
        "B1 out 0 I=tablefile(\"source.dat\")\nR1 out 0 1k",
        "V1 out 0 1\nR1 out 0 R={1000*tablefile(\"source.dat\")}",
        "V1 out 0 1\nR1 out 0 1k\nC1 out 0 {1u*tablefile(\"source.dat\")}",
        ".subckt child out\n.func waveform(x) {tablefile(\"source.dat\")+x}\nB1 out 0 V={waveform(0)}\n.ends\nX1 out child\nR1 out 0 1k",
    ] {
        let dir = common::test_dir("behavioral_scoped_inputs");
        let source = dir.join("source.dat");
        std::fs::write(&source, "0 1\n1e-9 1\n").unwrap();
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, format!("table\n{body}\n.op\n.end\n")).unwrap();
        let normal = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .output()
            .unwrap();
        assert!(normal.status.success(), "{body}: {normal:?}");
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv"])
                .args(&flags)
                .arg(&source)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{body}, {flags:?}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("source"),
                "{result:?}"
            );
            assert_eq!(std::fs::read_to_string(&source).unwrap(), "0 1\n1e-9 1\n");
        }
    }
}

#[test]
fn failure_reports_preserve_malformed_lookup_inputs() {
    let dir = common::test_dir("behavioral_invalid_input");
    let source = dir.join("source.dat");
    let original = "unparseable lookup data\n";
    std::fs::write(&source, original).unwrap();
    let deck = dir.join("deck.cir");
    std::fs::write(
        &deck,
        "invalid table\nB1 out 0 V=tablefile(\"source.dat\")\nR1 out 0 1k\n.op\n.end\n",
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--report-format", "junit", "--report-file"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&source).unwrap(), original);
}
