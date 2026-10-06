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
