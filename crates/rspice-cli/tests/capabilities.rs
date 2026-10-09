//! Capability discovery must distinguish execution, control, and result fidelity.
mod common;

use serde_json::Value;
use std::process::{Command, Output};

fn invoke(config: &std::path::Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(config)
        .args(arguments)
        .env_remove("RUST_LOG")
        .output()
        .expect("run rspice")
}

fn capabilities(config: &std::path::Path) -> Value {
    let output = invoke(config, &["--quiet", "capabilities", "--json"]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    serde_json::from_slice(&output.stdout).expect("single JSON document")
}

fn row<'a>(document: &'a Value, id: &str) -> &'a Value {
    document["analyses"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == id)
        .expect("analysis is inventoried")
}

#[test]
fn discovery_includes_unexposed_analyses_and_separates_control_support() {
    let dir = common::test_dir("capability-inventory");
    let config = dir.join("config.toml");
    // Discovery performs no parser, model-loading, or solver work.
    std::fs::write(&config, "[resources]\nmax_netlist_bytes = 1\n").unwrap();
    let document = capabilities(&config);
    assert_eq!(document["schema"], "rspice.capabilities");
    assert_eq!(document["schema_version"], 1);
    assert_eq!(document["tool"]["commit"], env!("RSPICE_BUILD_COMMIT"));
    let entries = document["analyses"].as_array().unwrap();
    let ids: std::collections::BTreeSet<_> = entries
        .iter()
        .map(|entry| entry["id"].as_str().unwrap())
        .collect();
    let expected = rspice_core::execution::AnalysisKind::ALL.map(|kind| kind.tag());
    assert_eq!(entries.len(), expected.len());
    assert_eq!(ids, expected.into_iter().collect());
    for id in ["soa", "optimize", "psp", "hbsp", "hbnoise"] {
        let entry = row(&document, id);
        assert_eq!(entry["execution"]["status"], "unsupported");
        assert!(
            entry["execution"]["reason"]
                .as_str()
                .is_some_and(|reason| !reason.is_empty())
        );
        assert!(entry["result_family"].is_null());
    }
    for id in [
        "op", "dc", "ac", "tran", "noise", "stb", "pz", "sens", "tf", "disto",
    ] {
        assert_eq!(row(&document, id)["execution"]["status"], "supported");
        assert_eq!(row(&document, id)["control"]["status"], "supported");
        assert_eq!(row(&document, id)["control_command"], id);
    }
    for id in ["pss", "hb", "pac", "pnoise", "mc", "qpss"] {
        assert_eq!(row(&document, id)["execution"]["status"], "supported");
        assert_eq!(row(&document, id)["control"]["status"], "unsupported");
    }
    for id in ["four", "fft"] {
        assert_eq!(row(&document, id)["control"]["status"], "partial");
        assert!(row(&document, id)["control_command"].is_null());
    }
}

#[test]
fn discovery_reuses_result_contracts_and_exposes_format_direction() {
    let dir = common::test_dir("capability-formats");
    let config = dir.join("config.toml");
    std::fs::write(&config, "").unwrap();
    let document = capabilities(&config);
    assert_eq!(
        document["result_document"]["schema"],
        rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA
    );
    assert_eq!(
        document["result_document"]["schema_version"],
        rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_VERSION
    );
    let families = document["result_document"]["families"].as_array().unwrap();
    for id in [
        "port-noise",
        "qpss",
        "qpxf",
        "pstb",
        "dcmatch",
        "monte-carlo",
    ] {
        let family = families.iter().find(|family| family["id"] == id).unwrap();
        for axis in ["scalar", "step", "temperature"] {
            assert_eq!(family[axis]["status"], "mapped");
        }
    }
    assert!(
        document["formats"]["input"]
            .as_array()
            .unwrap()
            .contains(&Value::from("touchstone"))
    );
    assert!(
        !document["formats"]["output"]
            .as_array()
            .unwrap()
            .contains(&Value::from("touchstone"))
    );
    assert!(
        document["formats"]["scope"]
            .as_str()
            .unwrap()
            .contains("representability")
    );
}

#[test]
fn declared_limitations_agree_with_command_admission() {
    let dir = common::test_dir("capability-admission");
    let config = dir.join("config.toml");
    std::fs::write(&config, "").unwrap();
    let document = capabilities(&config);
    let strict = document["workflows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "veriloga_strict_lrm")
        .unwrap();
    assert_eq!(strict["support"]["status"], "unsupported");
    let output = invoke(
        &config,
        &[
            "--quiet",
            "--error-format",
            "json",
            "compile-va",
            "not-read.va",
            "--strict",
        ],
    );
    assert_eq!(output.status.code(), Some(69), "{output:?}");
    let error: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["capability"], "veriloga.strict_lrm");
    let deck = dir.join("control.cir");
    std::fs::write(
        &deck,
        "Control capability\nV1 out 0 1\nR1 out 0 1k\n.control\npss 1k\n.endc\n.end\n",
    )
    .unwrap();
    let output = invoke(
        &config,
        &["--quiet", "check", deck.to_str().unwrap(), "--json"],
    );
    assert!(!output.status.success(), "{output:?}");
    let checked: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(checked["valid"], false);
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("no electrical or presentation handler")
    );
}

#[test]
fn capability_help_text_and_quiet_modes_are_discoverable() {
    let dir = common::test_dir("capability-console");
    let config = dir.join("config.toml");
    std::fs::write(&config, "").unwrap();
    for args in [["--help"].as_slice(), ["completions", "bash"].as_slice()] {
        let output = invoke(&config, args);
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("capabilities"));
    }
    let text = invoke(&config, &["capabilities"]);
    assert!(text.status.success());
    let text = String::from_utf8(text.stdout).unwrap();
    assert!(text.contains("Execution"));
    assert!(text.contains("Control"));
    assert!(text.contains("optimize"));
    assert!(text.contains("unsupported"));
    let quiet = invoke(&config, &["--quiet", "capabilities"]);
    assert!(quiet.status.success());
    assert!(quiet.stdout.is_empty());
}
