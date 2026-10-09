mod common;

use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output};

fn invoke(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(root.join("config.toml"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .env_remove("RUST_LOG")
        .output()
        .unwrap()
}

fn fixture(root: &Path) -> Value {
    std::fs::write(root.join("config.toml"), "").unwrap();
    std::fs::create_dir(root.join("circuits")).unwrap();
    std::fs::write(
        root.join("circuits/divider.cir"),
        "Study circuit\n.include \"resistors.inc\"\nV1 in 0 DC 1 AC 1\n.end\n",
    )
    .unwrap();
    std::fs::write(
        root.join("circuits/resistors.inc"),
        "R1 in out 1k\nR2 out 0 1k\n",
    )
    .unwrap();
    json!({
        "schema_version": 1,
        "id": "54dac9d3-e7c4-46d3-b2ae-9db058426b19",
        "circuit": {"path": "circuits/divider.cir"},
        "tasks": [
            {"id": "response", "depends_on": ["bias"], "analysis": {"Ac": {
                "start_freq": 1.0, "stop_freq": 1000.0, "points_per_unit": 5, "sweep": "Decade"
            }}},
            {"id": "bias", "analysis": "DcOp"}
        ]
    })
}

fn save(root: &Path, document: &Value) -> std::path::PathBuf {
    let path = root.join("study.json");
    std::fs::write(&path, serde_json::to_vec_pretty(document).unwrap()).unwrap();
    path
}

#[test]
fn study_check_and_plan_capture_relative_sources_and_stable_execution_order() {
    let root = common::test_dir("study-plan");
    let mut document = fixture(&root);
    let path = save(&root, &document);
    let output = invoke(&root, &["study", "plan", path.to_str().unwrap(), "--json"]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let first: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(first["schema"], "rspice.study.plan");
    assert_eq!(first["valid"], true);
    assert_eq!(first["task_count"], 2);
    assert_eq!(first["tasks"][0]["id"], "bias");
    assert_eq!(
        first["tasks"][1]["dependency_ids"][0],
        first["tasks"][0]["instance_id"]
    );
    document["tasks"].as_array_mut().unwrap().reverse();
    save(&root, &document);
    let output = invoke(&root, &["study", "plan", path.to_str().unwrap(), "--json"]);
    assert!(output.status.success(), "{output:?}");
    let reordered: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(first["snapshot_digest"], reordered["snapshot_digest"]);
    assert_eq!(first["tasks"], reordered["tasks"]);
    let output = invoke(&root, &["study", "check", path.to_str().unwrap(), "--json"]);
    assert!(output.status.success(), "{output:?}");
    let checked: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(checked["schema"], "rspice.study.check");
    assert_eq!(first["snapshot_digest"], checked["snapshot_digest"]);
    assert!(checked["tasks"].is_null());
    std::fs::write(
        root.join("circuits/resistors.inc"),
        "R1 in out 2k\nR2 out 0 1k\n",
    )
    .unwrap();
    let output = invoke(&root, &["study", "plan", path.to_str().unwrap(), "--json"]);
    let changed: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_ne!(changed["source_digest"], first["source_digest"]);
}

#[test]
fn study_refusals_are_structured_and_keep_source_locations() {
    let root = common::test_dir("study-errors");
    let mut document = fixture(&root);
    document["tasks"][0]["analysis"]["Ac"]["pointz"] = json!(12);
    let path = save(&root, &document);
    let output = invoke(&root, &["study", "check", path.to_str().unwrap(), "--json"]);
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let error: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["code"], "study.invalid_input");
    assert!(String::from_utf8_lossy(&output.stdout).contains("pointz"));
    document["tasks"][0]["analysis"]["Ac"]
        .as_object_mut()
        .unwrap()
        .remove("pointz");
    document["tasks"][1]["depends_on"] = json!(["response"]);
    save(&root, &document);
    let output = invoke(&root, &["study", "plan", path.to_str().unwrap(), "--json"]);
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    document["tasks"][1]
        .as_object_mut()
        .unwrap()
        .remove("depends_on");
    save(&root, &document);
    let included = root.join("circuits/resistors.inc");
    std::fs::write(&included, "R1 in out 1k\nRbroken\n").unwrap();
    let output = invoke(&root, &["study", "check", path.to_str().unwrap(), "--json"]);
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let error: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["line"], 2);
    // Legacy Syntax carries its source path in its message; typed parser
    // failures use the separate location fields. Neither may lose the owner.
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("resistors.inc")
    );
}

#[test]
fn study_inspection_honors_input_budgets_quiet_mode_and_discovery() {
    let root = common::test_dir("study-budget");
    let document = fixture(&root);
    let path = save(&root, &document);
    let output = invoke(&root, &["study", "check", path.to_str().unwrap()]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty());
    let output = invoke(&root, &["capabilities", "--json"]);
    let capabilities: Value = serde_json::from_slice(&output.stdout).unwrap();
    let study = capabilities["workflows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == "study_files")
        .unwrap();
    assert_eq!(study["support"]["status"], "partial");
    std::fs::write(
        root.join("config.toml"),
        "[resources]\nmax_netlist_bytes = 1\n",
    )
    .unwrap();
    let output = invoke(&root, &["study", "check", path.to_str().unwrap(), "--json"]);
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let error: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["resource"], "netlist_bytes");
}
