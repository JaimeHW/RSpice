mod common;
use std::process::Command;

#[test]
fn explicit_scoped_and_default_pem_tables_are_preserved() {
    for mode in ["explicit", "scoped", "default"] {
        let dir = common::test_dir("pem_source_ownership");
        let working = dir.join("working");
        std::fs::create_dir(&working).unwrap();
        let (positive, negative, body) = match mode {
            "explicit" => (
                "positive.csv",
                "negative.csv",
                ".model pem memristor level=4 fxpdata=\"positive.csv\" fxmdata=\"negative.csv\"\nYMEMRISTOR mr1 in 0 pem xo=1\n",
            ),
            "scoped" => (
                "positive.csv",
                "negative.csv",
                ".subckt cell in tablep=\"positive.csv\" tablem=\"negative.csv\"\n.model pem memristor level=4 fxpdata=tablep fxmdata=tablem\nYMEMRISTOR mr1 in 0 pem xo=1\n.ends\nX1 in cell\n",
            ),
            "default" => (
                "filep.dat",
                "filem.dat",
                ".model pem memristor level=4\nYMEMRISTOR mr1 in 0 pem xo=1\n",
            ),
            _ => unreachable!(),
        };
        for filename in [positive, negative] {
            std::fs::write(dir.join(filename), "0,1\n1,1\n").unwrap();
        }
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            format!("PEM inputs\nV1 in 0 0.005\n{body}.op\n.end\n"),
        )
        .unwrap();
        let ordinary = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .current_dir(&working)
            .output()
            .unwrap();
        assert!(ordinary.status.success(), "{mode}: {ordinary:?}");
        for filename in [positive, negative] {
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
                    .arg(dir.join(filename))
                    .current_dir(&working)
                    .output()
                    .unwrap();
                assert_eq!(
                    result.status.code(),
                    Some(2),
                    "{mode}, {filename}, {flags:?}: {result:?}"
                );
                assert!(
                    String::from_utf8_lossy(&result.stderr).contains("source"),
                    "{result:?}"
                );
                assert_eq!(
                    std::fs::read_to_string(dir.join(filename)).unwrap(),
                    "0,1\n1,1\n"
                );
            }
        }
    }
}

#[test]
fn malformed_pem_tables_are_not_replaced_by_failure_reports() {
    let dir = common::test_dir("pem_failure_report_input");
    let source = dir.join("filep.dat");
    std::fs::write(&source, "invalid model data\n").unwrap();
    std::fs::write(dir.join("filem.dat"), "0,1\n1,1\n").unwrap();
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, "PEM inputs\nV1 in 0 0.005\n.model pem memristor level=4\nYMEMRISTOR mr1 in 0 pem xo=1\n.op\n.end\n").unwrap();
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
    assert_eq!(
        std::fs::read_to_string(&source).unwrap(),
        "invalid model data\n"
    );
}
