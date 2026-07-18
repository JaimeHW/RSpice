use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let directory =
        std::env::temp_dir().join(format!("rspice_health_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&directory).expect("create health test directory");
    directory
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .env_remove("RUST_LOG")
        .output()
        .expect("run rspice health")
}

#[test]
fn readiness_json_exercises_parser_and_solver() {
    let directory = test_dir("ready");
    let config = directory.join("rspice.toml");
    std::fs::write(&config, "").expect("write empty explicit config");

    let output = run_rspice(&["--config", config.to_str().unwrap(), "health", "--json"]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("versioned JSON health response");
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["status"], "ready");
    assert_eq!(json["ready"], true);
    assert_eq!(json["mode"], "readiness");
    assert_eq!(json["checks"]["configuration"]["status"], "pass");
    assert_eq!(json["checks"]["parser"]["element_count"], 2);
    assert_eq!(json["checks"]["solver"]["node_count"], 1);
    assert_eq!(json["checks"]["solver"]["branch_count"], 1);
    assert_eq!(json["checks"]["solver"]["output_voltage"], 1.0);
    assert!(json["run_id"].is_string());

    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn liveness_skips_workload_admission_checks() {
    let directory = test_dir("live");
    let config = directory.join("rspice.toml");
    std::fs::write(&config, "[resources]\nmax_netlist_bytes = 1\n")
        .expect("write restrictive config");

    let output = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "health",
        "--mode",
        "liveness",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("liveness JSON");
    assert_eq!(json["ready"], true);
    assert_eq!(json["mode"], "liveness");
    assert_eq!(json["checks"]["parser"]["status"], "skipped");
    assert_eq!(json["checks"]["solver"]["status"], "skipped");

    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn readiness_failure_is_structured_and_nonzero() {
    let directory = test_dir("not_ready");
    let config = directory.join("rspice.toml");
    std::fs::write(&config, "[resources]\nmax_netlist_bytes = 1\n")
        .expect("write restrictive config");

    let output = run_rspice(&["--config", config.to_str().unwrap(), "health", "--json"]);
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("not-ready JSON response");
    assert_eq!(json["status"], "not_ready");
    assert_eq!(json["ready"], false);
    assert_eq!(json["error"]["code"], "resource_limit");
    assert_eq!(json["error"]["resource"], "netlist_bytes");
    assert_eq!(json["error"]["limit"], 1);

    let _ = std::fs::remove_dir_all(directory);
}
