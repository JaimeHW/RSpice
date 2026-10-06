mod common;
use std::process::Command;

#[test]
fn check_and_run_reject_invalid_requests_before_publication() {
    for (name, cards, diagnostic) in [
        (
            "control",
            ".control\nforeach x 1 2\nop\n.endc",
            "control.syntax",
        ),
        ("tran", ".tran 1u -1m", "TSTOP"),
        ("ac", ".ac dec 1 100 1", "stop frequency"),
        ("dc", ".dc V1 0 1 0", "DC sweep"),
        (
            "stepped",
            ".step param stop list 1m -1m\n.tran 1u {stop}",
            "TSTOP",
        ),
    ] {
        let dir = common::test_dir(name);
        let path = dir.join("deck.cir");
        std::fs::write(
            &path,
            format!("preflight\n.param stop=1m\nV1 in 0 1\nR1 in 0 1k\n{cards}\n.end\n"),
        )
        .unwrap();
        let checked = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "check"])
            .arg(&path)
            .arg("--json")
            .output()
            .unwrap();
        assert!(!checked.status.success(), "{name}: {checked:?}");
        let report: serde_json::Value = serde_json::from_slice(&checked.stdout).unwrap();
        assert_eq!(report["valid"], false);
        assert_eq!(report["strict_valid"], false);
        assert!(
            String::from_utf8_lossy(&checked.stdout).contains(diagnostic),
            "{name}: {report}"
        );
        if name == "control" {
            assert!(report["errors"][0]["details"]["line"].as_u64().is_some());
        }
        let output = dir.join("out.csv");
        let ran = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&path)
            .args(["-f", "csv", "-o"])
            .arg(&output)
            .output()
            .unwrap();
        assert!(!ran.status.success(), "{name}: {ran:?}");
        assert!(!std::fs::read_dir(&dir).unwrap().any(|entry| {
            entry
                .unwrap()
                .path()
                .extension()
                .is_some_and(|ext| ext == "csv")
        }));
    }
}
