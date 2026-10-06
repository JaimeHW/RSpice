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
