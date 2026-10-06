//! Topology checking uses the same hierarchy and resource policy as execution.
mod common;
use common::test_dir;
use std::process::Command;

#[test]
fn strict_check_rejects_a_voltage_loop_inside_a_subcircuit_as_json() {
    let dir = test_dir("hierarchical_loop");
    let deck = dir.join("loop.cir");
    std::fs::write(&deck, "Nested loop\n.subckt bad p n\nV1 p n 1\nV2 p n 2\n.ends\nX1 in 0 bad\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("check")
        .arg(deck)
        .args(["--strict", "--connectivity", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["valid"], false);
    assert!(
        json["errors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|error| error["message"].as_str().unwrap().contains("closes a loop"))
    );
}

#[test]
fn check_does_not_ignore_hierarchy_admission_failures() {
    let dir = test_dir("hierarchy_limit");
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_flattened_elements=1\n").unwrap();
    let deck = dir.join("nested.cir");
    std::fs::write(
        &deck,
        "Limited hierarchy\n.subckt load p n\nR1 p n 1k\nR2 p n 2k\n.ends\nX1 in 0 load\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(config)
        .args(["--error-format", "json", "check"])
        .arg(deck)
        .args(["--connectivity", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["valid"], false);
}

#[test]
fn validation_errors_take_precedence_over_strict_warnings() {
    let dir = test_dir("strict_error_precedence");
    let deck = dir.join("deck.sp");
    std::fs::write(
        &deck,
        "* error and warning\n.option foobar=1\nV1 in 0 1\nV2 in 0 2\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "check"])
        .arg(deck)
        .args(["--strict", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!report["warnings"].as_array().unwrap().is_empty());
    assert!(!report["errors"].as_array().unwrap().is_empty());
}

#[test]
fn check_validates_alter_variants_and_each_step_coordinate() {
    let dir = test_dir("check_complete_plan");
    for (name, source) in [
        (
            "alter",
            "* ALTER validation\nV1 in 0 1\nR1 in 0 1k\n.op\n.alter bad\nX1 in 0 MISSING\n.end\n",
        ),
        (
            "step",
            "* conditional STEP validation\n.param p=1\nV1 in 0 1\nR1 in 0 1k\n.if (p > 1)\nV2 in 0 2\n.endif\n.step param p list 1 2\n.op\n.end\n",
        ),
    ] {
        let deck = dir.join(format!("{name}.sp"));
        std::fs::write(&deck, source).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "check"])
            .arg(deck)
            .args(["--strict", "--json"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(65), "{output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["valid"], false);
        assert!(!report["errors"].as_array().unwrap().is_empty());
        assert!(
            report["errors"][0]["message"]
                .as_str()
                .unwrap()
                .contains(if name == "alter" {
                    "bad"
                } else {
                    "closes a loop"
                }),
            "{report}"
        );
    }
}

#[test]
fn xspice_check_honors_the_selected_device_dialect() {
    let dir = test_dir("check_dialect");
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, "Check dialect\nV1 in 0 1\nA1 in out buf\n.model buf gain(gain=1)\nR1 out 0 1k\nC1 out 0 cm\n.model cm C(C=1p)\n.op\n.end\n").unwrap();
    for (dialect, valid) in [("ngspice", true), ("xyce", false)] {
        for command in ["check", "run"] {
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", command])
                .arg(&deck)
                .args(["--spice-dialect", dialect])
                .output()
                .unwrap();
            assert_eq!(
                output.status.success(),
                valid,
                "{command}, {dialect}: {output:?}"
            );
            if !valid {
                let diagnostic = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(diagnostic.contains("requires L"), "{diagnostic}");
            }
        }
    }
}

#[test]
fn xspice_check_resolves_config_deck_and_swept_temperatures() {
    let dir = test_dir("check_temperatures");
    let config = dir.join("hot.toml");
    std::fs::write(&config, "[simulation]\ntemperature=200\n").unwrap();
    let body = "Check temperature\nV1 in 0 1\nA1 in out buf\n.model buf gain(gain=1)\nR1 out 0 1k\nC1 out 0 cm\n.model cm C(C=1p TC1=-0.01 TNOM=27)\n.op\n";
    for (mode, options, configured, valid) in [
        ("nominal", "", false, true),
        ("config", "", true, false),
        ("deck", ".options temp=200\n", false, false),
        ("override", ".options temp=27\n", true, true),
        ("sweep", ".temp 27 200\n", false, false),
    ] {
        let deck = dir.join(format!("{mode}.cir"));
        std::fs::write(&deck, format!("{body}{options}.end\n")).unwrap();
        for command in ["check", "run"] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
            process.arg("--quiet");
            if configured {
                process.arg("--config").arg(&config);
            }
            process.arg(command).arg(&deck);
            if command == "check" {
                process.arg("--json");
            }
            let output = process.output().unwrap();
            assert_eq!(
                output.status.success(),
                valid,
                "{command}, {mode}: {output:?}"
            );
            if !valid {
                let diagnostic = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(diagnostic.contains("negative capacitance"), "{diagnostic}");
            }
        }
    }
}
