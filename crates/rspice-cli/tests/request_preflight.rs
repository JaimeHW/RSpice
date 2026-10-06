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
        ("control-dc", ".control\ndc V1 0 1 0\n.endc", "DC sweep"),
        ("control-dc-run", ".dc V1 0 1 0\n.control\nrun\n.endc", "DC sweep"),
        (
            "control-op-args",
            ".control\nop unexpected\n.endc",
            "does not accept arguments",
        ),
        ("control-tran", ".control\ntran 1u -1m\n.endc", "TSTOP"),
        (
            "control-ac",
            ".control\nac dec 1 100 1\n.endc",
            "stop frequency",
        ),
        ("control-run", ".tran 1u -1m\n.control\nrun\n.endc", "TSTOP"),
        (
            "control-no-analysis",
            ".control\nrun\n.endc",
            "no declarative analysis",
        ),
        (
            "control-run-args",
            ".op\n.control\nrun unexpected\n.endc",
            "does not accept arguments",
        ),
        (
            "control-unsupported",
            ".control\ntf V(in) V1\n.endc",
            "no electrical or presentation handler",
        ),
        (
            "control-unsupported-run",
            ".tf V(in) V1\n.control\nrun\n.endc",
            "no control-host execution handler",
        ),
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

#[test]
fn static_control_checks_report_deferred_values_without_executing_the_script() {
    for (name, cards, valid, deferred) in [
        ("replaced", ".tran 1u -1m\n.control\nop\n.endc", true, false),
        (
            "dynamic",
            ".control\nlet stop=1m\ntran 1u $&stop\n.endc",
            true,
            true,
        ),
        (
            "conditional",
            ".control\nif 0\ntran 1u -1m\nend\nop\n.endc",
            false,
            false,
        ),
    ] {
        let dir = common::test_dir(name);
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            format!("static control\nV1 in 0 1\nR1 in 0 1k\n{cards}\n.end\n"),
        )
        .unwrap();
        for strict in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command.args(["--quiet", "check"]).arg(&deck).arg("--json");
            if strict {
                command.arg("--strict");
            }
            let output = command.output().unwrap();
            assert_eq!(
                output.status.success(),
                valid && !(strict && deferred),
                "{name}: {output:?}"
            );
            let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(report["valid"], valid);
            assert_eq!(report["strict_valid"], valid && !deferred);
            if deferred {
                assert!(
                    report["warnings"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|warning| warning["code"] == "CONTROL_DYNAMIC_ANALYSIS")
                );
            }
        }
        assert_eq!(
            std::fs::read_dir(&dir).unwrap().count(),
            1,
            "check cannot publish control output"
        );
    }
}

#[test]
fn control_analysis_checks_keep_include_locations_and_configured_point_limits() {
    let dir = common::test_dir("control-check-limits");
    let deck = dir.join("deck.cir");
    let include = dir.join("commands.inc");
    let config = dir.join("config.toml");
    std::fs::write(
        &deck,
        "included requests\nV1 in 0 1\nR1 in 0 1k\n.include commands.inc\n.end\n",
    )
    .unwrap();
    std::fs::write(&include, "* included\n.control\nac lin 20 1 20\n.endc\n").unwrap();
    std::fs::write(&config, "[resources]\nmax_analysis_points=10\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(config)
        .args(["--quiet", "check"])
        .arg(deck)
        .arg("--json")
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let details = &report["errors"][0]["details"];
    assert_eq!(details["line"], 3);
    assert!(
        details["path"].as_str().unwrap().ends_with("commands.inc"),
        "{details}"
    );
    assert!(
        details["code"].as_str().unwrap().contains("resource"),
        "{details}"
    );
}
