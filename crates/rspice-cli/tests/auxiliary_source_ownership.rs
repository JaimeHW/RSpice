mod common;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(directory: &Path, family: &str, filename: &str) -> (PathBuf, PathBuf, PathBuf) {
    let decks = directory.join("decks");
    let working = directory.join("working");
    std::fs::create_dir(&decks).unwrap();
    std::fs::create_dir(&working).unwrap();
    let (data, body) = match family {
        "initcond" => (
            "C1 IC=1\n",
            format!(".INITCOND FILE \"{filename}\"\nC1 out 0 1u\nR1 out 0 1k\n.tran 1n 2n uic\n"),
        ),
        "measurement" => (
            "time,value\n0,1\n1e-9,1\n2e-9,1\n",
            format!(
                "V1 out 0 1\nR1 out 0 1k\n.tran 1n 2n\n.meas tran fit ERROR V(out) FILE=\"{filename}\" COMP_FUNCTION=L2NORM INDEPVARCOL=0 DEPVARCOL=1\n"
            ),
        ),
        "spef" => (
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*D_NET in 1\n*CAP\n1 in 1\n*END\n",
            format!(".spef_include \"{filename}\"\nV1 in 0 1\nR1 in 0 1k\n.op\n"),
        ),
        _ => unreachable!(),
    };
    // INITCOND explicitly resolves against execution directory; SPEF and
    // measurement references resolve against the root deck's directory.
    let source = if family == "initcond" {
        &working
    } else {
        &decks
    }
    .join(filename);
    let deck = decks.join("deck.cir");
    std::fs::write(&source, data).unwrap();
    std::fs::write(&deck, format!("auxiliary\n{body}.end\n")).unwrap();
    (deck, source, working)
}

#[test]
fn initial_conditions_parasitics_and_measurement_references_cannot_be_replaced() {
    for family in ["initcond", "measurement", "spef"] {
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let dir = common::test_dir("auxiliary_ownership");
            let (deck, source, working) = fixture(&dir, family, "source.csv");
            let original = std::fs::read(&source).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv"])
                .args(&flags)
                .arg(&source)
                .current_dir(&working)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{family}, {flags:?}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("source"),
                "{result:?}"
            );
            assert_eq!(std::fs::read(&source).unwrap(), original);
        }
    }
}

#[test]
fn parallel_generated_outputs_preserve_auxiliary_inputs() {
    for family in ["initcond", "measurement", "spef"] {
        let dir = common::test_dir("auxiliary_parallel_ownership");
        let (deck, source, working) = fixture(&dir, family, "result.ss.csv");
        let original = std::fs::read(&source).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
            .arg(source.parent().unwrap().join("result.csv"))
            .current_dir(&working)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{family}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("source"),
            "{result:?}"
        );
        assert_eq!(std::fs::read(&source).unwrap(), original);
    }
}

#[test]
fn noncolliding_auxiliary_inputs_still_run() {
    for family in ["initcond", "measurement", "spef"] {
        let dir = common::test_dir("auxiliary_normal_output");
        let (deck, source, working) = fixture(&dir, family, "source.csv");
        let original = std::fs::read(&source).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .current_dir(&working)
            .output()
            .unwrap();
        assert!(result.status.success(), "{family}: {result:?}");
        assert_eq!(std::fs::read(&source).unwrap(), original);
        assert!(dir.join("result.csv").is_file());
    }
}
