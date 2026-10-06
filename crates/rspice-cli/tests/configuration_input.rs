//! Configuration inputs must fail explicitly rather than fall back silently.
mod common;

use std::ffi::OsString;
use std::process::Command;

#[cfg(windows)]
fn non_unicode_value() -> OsString {
    use std::os::windows::ffi::OsStringExt;
    OsString::from_wide(&[0xd800])
}

#[cfg(unix)]
fn non_unicode_value() -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(vec![0xff])
}

#[cfg(any(windows, unix))]
#[test]
fn non_unicode_configuration_values_never_disable_requested_settings() {
    let directory = common::test_dir("config_encoding");
    let config = directory.join("empty.toml");
    std::fs::write(&config, "").unwrap();
    for variable in [
        "RSPICE_TEMPERATURE",
        "RSPICE_OUTPUT_FORMAT",
        "RSPICE_MAX_NETLIST_BYTES",
        "RSPICE_MAX_NETLIST_LINES",
        "RSPICE_MAX_EXPANDED_SOURCE_BYTES",
        "RSPICE_MAX_DEPENDENCY_SOURCE_BYTES",
        "RSPICE_MAX_EXTERNAL_DATA_BYTES",
        "RSPICE_MAX_EXTERNAL_DATA_VALUES",
        "RSPICE_MAX_SHARED_CACHE_BYTES",
        "RSPICE_MAX_INCLUDE_DEPTH",
        "RSPICE_MAX_HIERARCHY_DEPTH",
        "RSPICE_MAX_FLATTENED_ELEMENTS",
        "RSPICE_MAX_CIRCUIT_NODES",
        "RSPICE_MAX_MATRIX_UNKNOWNS",
        "RSPICE_MAX_ANALYSIS_POINTS",
        "RSPICE_MAX_RESULT_VALUES",
        "RSPICE_MAX_PARALLEL_WORKERS",
        "RSPICE_MAX_BATCH_RUNS",
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "--config"])
            .arg(&config)
            .args(["health", "--mode", "liveness", "--json"])
            .env(variable, non_unicode_value())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(78), "{variable}: {output:?}");
        assert!(output.stdout.is_empty(), "{variable}: {output:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        let message = diagnostic["error"]["message"].as_str().unwrap();
        assert!(
            message.contains(variable) && message.contains("Unicode"),
            "{message}"
        );
    }
}

#[cfg(any(windows, unix))]
#[test]
fn non_unicode_logging_values_report_configuration_errors() {
    let directory = common::test_dir("logging_encoding");
    let config = directory.join("empty.toml");
    std::fs::write(&config, "").unwrap();
    for variable in ["RUST_LOG", "RUST_LOG_STYLE"] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--error-format", "json", "--config"])
            .arg(&config)
            .args(["health", "--mode", "liveness", "--json"])
            .env_remove("RUST_LOG")
            .env_remove("RUST_LOG_STYLE")
            .env(variable, non_unicode_value())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(78), "{variable}: {output:?}");
        assert!(output.stdout.is_empty(), "{variable}: {output:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        let message = diagnostic["error"]["message"].as_str().unwrap();
        assert!(
            message.contains(variable) && message.contains("Unicode"),
            "{message}"
        );
    }
}

#[test]
fn configuration_read_errors_retain_the_filesystem_reason() {
    let directory = common::test_dir("config_read_error");
    for path in [directory.join("missing.toml"), directory.to_path_buf()] {
        let reason = std::fs::read_to_string(&path).unwrap_err().to_string();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "--config"])
            .arg(&path)
            .args(["health", "--mode", "liveness", "--json"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(78), "{output:?}");
        assert!(output.stdout.is_empty(), "{output:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        let message = diagnostic["error"]["message"].as_str().unwrap();
        assert!(message.contains(path.to_str().unwrap()), "{message}");
        assert!(
            message.contains(&reason),
            "missing {reason:?} in {message:?}"
        );
    }
}

#[cfg(any(windows, unix))]
#[test]
fn a_dangling_project_configuration_is_not_treated_as_absent() {
    let directory = common::test_dir("config_symlink");
    let target = directory.join("missing.toml");
    let config = directory.join(".rspicerc");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &config).unwrap();
    #[cfg(windows)]
    if let Err(error) = std::os::windows::fs::symlink_file(&target, &config) {
        if error.raw_os_error() == Some(1314) {
            eprintln!("Windows symlink privilege unavailable; skipping runtime assertion");
            return;
        }
        panic!("create configuration symlink: {error}");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .current_dir(&directory)
        .args([
            "--quiet",
            "--error-format",
            "json",
            "health",
            "--mode",
            "liveness",
            "--json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(78), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    let message = diagnostic["error"]["message"].as_str().unwrap();
    assert!(message.contains(".rspicerc"), "{message}");
}
