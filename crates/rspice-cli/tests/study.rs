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

#[test]
fn study_run_publishes_validated_native_evidence_and_json_progress() {
    let root = common::test_dir("study-run");
    let mut document = fixture(&root);
    document["tasks"].as_array_mut().unwrap().extend([
        json!({"id": "spectrum", "depends_on": ["waveform"], "analysis": {"Fft": {"request": {
            "output": "V(out)", "points": 16, "window": "RECT", "start": 0.0, "stop": 0.001
        }}}}),
        json!({"id": "waveform", "analysis": {"Transient": {
            "step_time": 0.00001, "stop_time": 0.001, "start_time": 0.0, "max_timestep": 0.00001, "uic": false
        }}}),
    ]);
    let path = save(&root, &document);
    let destination = root.join("result.json");
    let output = invoke(
        &root,
        &[
            "study",
            "run",
            path.to_str().unwrap(),
            "--output",
            destination.to_str().unwrap(),
            "--json",
            "--progress-json",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["schema"], "rspice.study.run");
    assert_eq!(summary["task_count"], 4);
    let events: Vec<Value> = String::from_utf8(output.stderr)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let starts: Vec<&str> = events
        .iter()
        .filter(|event| event["event"] == "task_started")
        .map(|event| event["task"].as_str().unwrap())
        .collect();
    assert_eq!(starts, ["bias", "response", "waveform", "spectrum"]);
    let artifact: Value = serde_json::from_slice(&std::fs::read(&destination).unwrap()).unwrap();
    assert_eq!(artifact["schema"], "rspice.study.results");
    assert_eq!(
        artifact["preparation"]["snapshot_digest"],
        summary["snapshot_digest"]
    );
    assert_eq!(artifact["executed_netlists"].as_object().unwrap().len(), 4);
    let retained: rspice_formats::project_results::ProjectSimulationResults =
        serde_json::from_value(artifact["results"].clone()).unwrap();
    retained.validate().unwrap();
    let restored = retained
        .restore_with(rspice_formats::project_results::ProjectAnalysisResult::into_analysis)
        .unwrap();
    let run = &restored.runs[0];
    assert_eq!(run.analyses.len(), 4);
    assert!(run.success);
    run.validate_provenance().unwrap();
    let output_voltage = run.analyses[0]
        .dc_op
        .as_ref()
        .unwrap()
        .node_voltages
        .iter()
        .find(|value| value.name.eq_ignore_ascii_case("V(out)"))
        .unwrap_or_else(|| panic!("{:?}", run.analyses[0].dc_op));
    assert!((output_voltage.value - 0.5).abs() < 1e-12);
    assert_eq!(
        run.analyses[3].analysis_type,
        rspice_results::analysis_type::AnalysisType::Fourier
    );
}

#[test]
fn study_run_protects_inputs_and_preserves_outputs_on_storage_failure() {
    let root = common::test_dir("study-publication");
    let mut document = fixture(&root);
    let path = save(&root, &document);
    let included = root.join("circuits/resistors.inc");
    let original = std::fs::read(&included).unwrap();
    let output = invoke(
        &root,
        &[
            "study",
            "run",
            path.to_str().unwrap(),
            "--output",
            included.to_str().unwrap(),
            "--json",
        ],
    );
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert_eq!(std::fs::read(&included).unwrap(), original);
    document["save_policy"] = json!({
        "output_selection_mode": "save_all", "retained_dataset_limit": 1,
        "maximum_storage_bytes": 5000, "live_streaming_enabled": false,
        "retain_failure_diagnostics": false
    });
    save(&root, &document);
    let destination = root.join("result.json");
    std::fs::write(&destination, "previous result").unwrap();
    let names = || {
        std::fs::read_dir(&*root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<std::collections::BTreeSet<_>>()
    };
    let before = names();
    let output = invoke(
        &root,
        &[
            "study",
            "run",
            path.to_str().unwrap(),
            "--output",
            destination.to_str().unwrap(),
            "--json",
        ],
    );
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    assert_eq!(
        std::fs::read_to_string(&destination).unwrap(),
        "previous result"
    );
    assert_eq!(
        before,
        names(),
        "abandoned publication leaves no staging files"
    );
}

#[test]
fn failed_study_measurement_is_not_a_successful_run() {
    let root = common::test_dir("study-verification");
    let mut document = fixture(&root);
    document["tasks"] = json!([{ "id": "waveform", "analysis": {"Transient": {
        "step_time": 0.00001, "stop_time": 0.001, "start_time": 0.0, "max_timestep": 0.00001, "uic": false
    }}}]);
    std::fs::write(root.join("circuits/divider.cir"),
        "Study measurement\nV1 in 0 DC 1\nR1 in out 1k\nR2 out 0 1k\n.meas tran peak MAX V(out) GOAL=2 TOL=0.01\n.end\n").unwrap();
    let path = save(&root, &document);
    let destination = root.join("result.json");
    let output = invoke(
        &root,
        &[
            "study",
            "run",
            path.to_str().unwrap(),
            "-o",
            destination.to_str().unwrap(),
            "--json",
        ],
    );
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert!(!destination.exists());
    let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["success"], false);
}

#[test]
fn running_study_cancellation_joins_the_solver_and_keeps_stderr_json() {
    let root = common::test_dir("study-cancel-solver");
    let mut document = fixture(&root);
    document["tasks"] = json!([{ "id": "waveform", "analysis": {"Transient": {
        "step_time": 1e-6, "stop_time": 1.0, "start_time": 0.0, "max_timestep": 1e-6, "uic": false
    }}}]);
    let path = save(&root, &document);
    let destination = root.join("result.json");
    std::fs::write(&destination, "previous result").unwrap();
    // Deliberately omit quiet/error-format: progress-json must choose one
    // machine-readable stderr grammar for progress, logs, and terminal errors.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(root.join("config.toml"))
        .args(["study", "run"])
        .arg(&path)
        .arg("-o")
        .arg(&destination)
        .args(["--json", "--progress-json", "--timeout", "0.1"])
        .env_remove("RUST_LOG")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(124), "{output:?}");
    assert_eq!(
        std::fs::read_to_string(&destination).unwrap(),
        "previous result"
    );
    let events: Vec<Value> = String::from_utf8(output.stderr)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert!(
        events.iter().any(|event| event["event"] == "task_started"),
        "{events:?}"
    );
    assert!(
        events
            .iter()
            .any(|event| event["error"]["exit_code"] == 124)
    );
}

#[test]
fn study_run_refuses_untransported_limits_and_honors_deadlines() {
    let root = common::test_dir("study-runtime-policy");
    let document = fixture(&root);
    let path = save(&root, &document);
    let destination = root.join("result.json");
    std::fs::write(
        root.join("config.toml"),
        "[resources]\nmax_matrix_unknowns = 2\n",
    )
    .unwrap();
    for command in ["check", "plan"] {
        let output = invoke(&root, &["study", command, path.to_str().unwrap(), "--json"]);
        assert_eq!(output.status.code(), Some(69), "{output:?}");
        let error: Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(
            error["error"]["capability"],
            "study.execution_resource_overrides"
        );
    }
    std::fs::write(root.join("config.toml"), "").unwrap();
    let output = invoke(
        &root,
        &[
            "study",
            "run",
            path.to_str().unwrap(),
            "--output",
            destination.to_str().unwrap(),
            "--json",
            "--timeout",
            "0.000000001",
        ],
    );
    assert_eq!(output.status.code(), Some(124), "{output:?}");
    assert!(!destination.exists());
}
