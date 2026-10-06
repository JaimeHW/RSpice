mod common;
use std::process::Command;

#[test]
fn nested_execution_preserves_resource_failures() {
    for (name, source, command, setting) in [
        (
            "step",
            "* budget\nV1 in 0 1\nR1 in 0 1k\nR2 in out 1k\nR3 out 0 1k\n.step param p list 1 2\n.end\n",
            "run",
            "max_result_values=2",
        ),
        (
            "xspice",
            "* XSPICE\nV1 in 0 1\nA1 in out g\n.model g gain(gain=2)\nR1 out 0 1k\n.op\n.end\n",
            "check",
            "max_circuit_nodes=1",
        ),
    ] {
        let dir = common::test_dir(name);
        let deck = dir.join("deck.sp");
        let config = dir.join("config.toml");
        std::fs::write(&deck, source).unwrap();
        std::fs::write(&config, format!("[resources]\n{setting}\n")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--config")
            .arg(config)
            .args(["--quiet", "--error-format", "json", command])
            .arg(deck)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(75), "{output:?}");
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert!(error.to_string().contains("resource_limit"), "{error}");
    }
}

#[test]
fn hdf5_publication_failure_is_an_output_error() {
    let dir = common::test_dir("hdf_publication");
    let input = dir.join("input.csv");
    let output_path = dir.join("existing-directory.h5");
    std::fs::write(&input, "time,V(x)\n0,1\n").unwrap();
    std::fs::create_dir(&output_path).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(&output_path)
        .args(["--to", "hdf5"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(73), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert!(error.to_string().contains("output"), "{error}");
    assert!(output_path.is_dir());
}

#[test]
fn fatal_behavioral_diagnostic_retains_every_reported_error_detail() {
    let dir = common::test_dir("behavioral_diagnostic");
    let deck = dir.join("deck.sp");
    let summary = dir.join("summary.json");
    std::fs::write(
        &deck,
        "behavioral reference\nB1 1 0 I={1m}\nR1 1 0 1\nB2 2 0 I={I(b1)*20}\nR2 2 0 1\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(&deck)
        .args(["--spice-dialect", "xyce", "--summary"])
        .arg(&summary)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(80), "{output:?}");
    let fatal: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(fatal["error"]["code"], "behavioral_reference_error");
    assert_eq!(fatal["error"]["instance_name"], "B2");
    assert_eq!(fatal["error"]["canonical_instance_name"], "B2");
    assert_eq!(fatal["error"]["missing_dependency"], "B1");
    assert_eq!(
        fatal["error"]["reason"],
        "lead_current_not_solution_variable"
    );
    let report = common::read_json(&summary);
    for (field, value) in report["runs"][0]["error_details"].as_object().unwrap() {
        assert_eq!(fatal["error"][field], *value, "lost {field}: {fatal}");
    }
}

#[test]
fn fatal_elaboration_diagnostic_identifies_the_instance_and_refusal() {
    let dir = common::test_dir("elaboration_diagnostic");
    std::fs::write(
        dir.join("resistor.va"),
        "`include \"disciplines.vams\"\nmodule ares(p,n);\ninout p,n; electrical p,n;\nanalog I(p,n) <+ V(p,n)/1000;\nendmodule\n",
    )
    .unwrap();
    let deck = dir.join("deck.sp");
    std::fs::write(
        &deck,
        "wrong terminal count\nV1 a 0 1\nX1 a 0 b ares\nR1 a 0 1k\n.va \"resistor.va\" ares\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .current_dir(&dir)
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(&deck)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let fatal: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(fatal["error"]["code"], "netlist_error");
    assert_eq!(fatal["error"]["instance_name"], "X1");
    assert_eq!(fatal["error"]["reason"], "port_count");
}
