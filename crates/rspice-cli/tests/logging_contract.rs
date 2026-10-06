//! Logging configuration must respect CLI overrides and the stderr grammar.
mod common;

use std::process::{Command, Output};

fn simulate(flags: &[&str], environment: &[(&str, &str)]) -> Output {
    let directory = common::test_dir("logging");
    let config = directory.join("empty.toml");
    let deck = directory.join("op.cir");
    let result = directory.join("result.json");
    std::fs::write(&config, "").unwrap();
    std::fs::write(&deck, "Logging\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let previous = b"existing result must survive invalid logging configuration";
    std::fs::write(&result, previous).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "--config"])
        .arg(&config)
        .args(flags)
        .arg("run")
        .arg(&deck)
        .args(["--format", "json", "--output"])
        .arg(&result)
        .env_remove("RUST_LOG")
        .env_remove("RUST_LOG_STYLE")
        .envs(environment.iter().copied())
        .output()
        .unwrap();
    if output.status.success() {
        assert_ne!(std::fs::read(&result).unwrap(), previous);
    } else {
        assert_eq!(std::fs::read(&result).unwrap(), previous);
    }
    output
}

fn records(output: &Output) -> Vec<serde_json::Value> {
    assert!(output.status.success(), "{output:?}");
    std::str::from_utf8(&output.stderr)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).expect("each stderr line must be JSON"))
        .collect()
}

fn assert_loading_logged(records: &[serde_json::Value]) {
    assert!(
        records.iter().any(|record| record["level"] == "INFO"
            && record["message"]
                .as_str()
                .unwrap()
                .contains("Loading netlist")),
        "{records:?}"
    );
}

#[test]
fn explicit_levels_override_environment_directives_and_regexes() {
    for filter in ["trace", "rspice=trace", "trace/Loading"] {
        let output = simulate(&["--log-level", "off"], &[("RUST_LOG", filter)]);
        assert!(records(&output).is_empty(), "{output:?}");
    }
    for filter in ["off", "rspice=off", "trace/never-match", "info/["] {
        let output = simulate(&["--log-level", "info"], &[("RUST_LOG", filter)]);
        assert_loading_logged(&records(&output));
    }
}

#[test]
fn verbose_overrides_environment_but_yields_to_explicit_level() {
    let output = simulate(&["--verbose"], &[("RUST_LOG", "off")]);
    assert!(
        records(&output)
            .iter()
            .any(|record| record["level"] == "DEBUG")
    );
    let output = simulate(
        &["--verbose", "--log-level", "off"],
        &[("RUST_LOG", "trace")],
    );
    assert!(records(&output).is_empty(), "{output:?}");
}

#[test]
fn environment_filters_and_regexes_apply_without_cli_overrides() {
    let output = simulate(&[], &[("RUST_LOG", "rspice=info/Loading netlist")]);
    let logs = records(&output);
    assert_loading_logged(&logs);
    assert!(logs.iter().all(|record| {
        record["target"].as_str().unwrap().starts_with("rspice")
            && record["message"]
                .as_str()
                .unwrap()
                .contains("Loading netlist")
    }));
    let output = simulate(&[], &[("RUST_LOG", "off")]);
    assert!(records(&output).is_empty());
}

#[test]
fn malformed_environment_logging_fails_with_one_json_diagnostic() {
    for (variable, value) in [
        ("RUST_LOG", "rspice=wrong"),
        ("RUST_LOG", "info/["),
        ("RUST_LOG", "info/x/y"),
        ("RUST_LOG_STYLE", "sometimes"),
    ] {
        let output = simulate(&[], &[(variable, value)]);
        assert_eq!(output.status.code(), Some(78), "{output:?}");
        assert!(output.stdout.is_empty(), "{output:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(diagnostic["error"]["category"], "configuration");
        let message = diagnostic["error"]["message"].as_str().unwrap();
        assert!(message.contains(variable), "{message}");
    }
}

#[test]
fn quiet_skips_unused_logging_configuration() {
    let output = simulate(
        &["--quiet"],
        &[("RUST_LOG", "info/["), ("RUST_LOG_STYLE", "sometimes")],
    );
    assert!(records(&output).is_empty(), "{output:?}");
}

#[test]
fn forced_text_color_does_not_corrupt_json_logs() {
    let output = simulate(&["--log-level", "info"], &[("RUST_LOG_STYLE", "always")]);
    assert_loading_logged(&records(&output));
    assert!(!output.stderr.contains(&0x1b));
}

#[test]
fn veriloga_native_compilation_obeys_quiet_levels_and_json_logging() {
    let directory = common::test_dir("veriloga_logging");
    let model = directory.join("model.va");
    let deck = directory.join("op.cir");
    let result = directory.join("result.json");
    std::fs::write(
        &model,
        "module resistor(p,n); inout p,n; electrical p,n; analog I(p,n) <+ V(p,n); endmodule\n",
    )
    .unwrap();
    std::fs::write(
        &deck,
        "Logging\n.va \"model.va\"\nV1 p 0 1\nX1 p 0 resistor\n.op\n.end\n",
    )
    .unwrap();
    for flags in [
        &["--quiet"][..],
        &["--log-level", "off"],
        &["--log-level", "info"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--error-format", "json"])
            .args(flags)
            .arg("run")
            .arg(&deck)
            .args(["--format", "json", "--output"])
            .arg(&result)
            .env_remove("RUST_LOG")
            .env_remove("RUST_LOG_STYLE")
            .output()
            .unwrap();
        let logs = records(&output);
        if flags == ["--log-level", "info"] {
            assert!(
                logs.iter().any(|record| record["message"]
                    .as_str()
                    .unwrap()
                    .contains("Loaded Verilog-A model")),
                "{logs:?}"
            );
        } else {
            assert!(logs.is_empty(), "{logs:?}");
        }
        if flags == ["--quiet"] {
            assert!(output.stdout.is_empty(), "{output:?}");
        }
    }
}
