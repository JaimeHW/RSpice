mod common;
use std::process::Command;

const SOURCE: &str = "`include \"disciplines.vams\"\nmodule resistor(p,n);\ninout p,n; electrical p,n;\nparameter real R=1000;\nanalog I(p,n) <+ V(p,n)/R;\nendmodule\n";

#[test]
fn interface_output_cannot_replace_the_source_or_its_includes() {
    let dir = common::test_dir("source_collision");
    let input = dir.join("model.va");
    let include = dir.join("value.vams");
    let source = format!(
        "`include \"value.vams\"\n{}",
        SOURCE.replace("R=1000", "R=`VALUE")
    );
    let included = "`define VALUE 42\n";
    std::fs::write(&input, &source).unwrap();
    std::fs::write(&include, included).unwrap();
    std::fs::create_dir(dir.join("child")).unwrap();
    let destinations = [
        input.clone(),
        include.clone(),
        dir.join("child/../model.va"),
        #[cfg(windows)]
        dir.join("MODEL.VA"),
    ];
    for destination in destinations {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "compile-va"])
            .arg(&input)
            .arg("-o")
            .arg(&destination)
            .output()
            .unwrap();
        assert!(!output.status.success(), "{destination:?}: {output:?}");
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert!(
            error["error"]["message"]
                .as_str()
                .unwrap()
                .contains("source"),
            "{error}"
        );
        assert_eq!(std::fs::read_to_string(&input).unwrap(), source);
        assert_eq!(std::fs::read_to_string(&include).unwrap(), included);
    }
    // An ordinary existing summary remains replaceable.
    let summary = dir.join("model.json");
    std::fs::write(&summary, "old summary").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compile-va"])
        .arg(&input)
        .arg("-o")
        .arg(&summary)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(common::read_json(&summary)["model"], "resistor");
}

#[test]
fn source_admission_preserves_typed_resource_failures() {
    for (setting, resource) in [
        ("max_netlist_bytes", "netlist_bytes"),
        ("max_netlist_lines", "netlist_lines"),
        ("max_dependency_source_bytes", "dependency_source_bytes"),
        ("max_expanded_source_bytes", "expanded_source_bytes"),
        ("max_include_depth", "include_depth"),
    ] {
        let dir = common::test_dir(setting);
        let config = dir.join("config.toml");
        let input = dir.join("model.va");
        let result = dir.join("model.json");
        std::fs::write(&config, format!("[resources]\n{setting}=1\n")).unwrap();
        std::fs::write(&input, SOURCE).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "--config"])
            .arg(&config)
            .arg("compile-va")
            .arg(&input)
            .arg("-o")
            .arg(&result)
            .output()
            .unwrap();
        assert!(!output.status.success(), "{setting}: {output:?}");
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(error["error"]["resource"], resource, "{error}");
        assert_eq!(error["error"]["limit"], 1);
        assert!(!result.exists());
    }
}

#[test]
fn module_selection_quiet_and_versioned_interface_work_together() {
    let dir = common::test_dir("selection");
    let input = dir.join("models.va");
    let result = dir.join("selected.json");
    std::fs::write(
        &input,
        format!(
            "{SOURCE}\n{}",
            SOURCE.replace("module resistor", "module second")
        ),
    )
    .unwrap();
    let invoke = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "compile-va"])
            .arg(&input)
            .args(args)
            .arg("-o")
            .arg(&result)
            .output()
            .unwrap()
    };
    for args in [&[][..], &["--module", "missing"][..]] {
        let output = invoke(args);
        assert!(!output.status.success(), "{output:?}");
        assert!(!result.exists());
    }
    let output = invoke(&["--module", "second", "--detailed", "--show-usage"]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let json = common::read_json(&result);
    assert_eq!(json["model"], "second");
    assert_eq!(json["schema"], "rspice.compile_va");
    assert_eq!(json["schema_version"], 1);
    assert!(json["run_id"].is_string());
    assert_eq!(json["tool"]["name"], "rspice");
}

#[test]
fn strict_is_an_explicit_capability_refusal() {
    let dir = common::test_dir("strict");
    let input = dir.join("model.va");
    std::fs::write(&input, SOURCE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "compile-va"])
        .arg(&input)
        .arg("--strict")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(69));
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["capability"], "veriloga.strict_lrm");
}

#[test]
fn including_source_directory_precedes_cli_and_config_paths() {
    let dir = common::test_dir("include_precedence");
    let local = dir.join("local");
    let extra = dir.join("extra");
    std::fs::create_dir(&local).unwrap();
    std::fs::create_dir(&extra).unwrap();
    std::fs::write(local.join("value.vams"), "`define RESISTANCE 42\n").unwrap();
    std::fs::write(extra.join("value.vams"), "`define RESISTANCE 999\n").unwrap();
    let input = local.join("model.va");
    std::fs::write(
        &input,
        format!(
            "`include \"value.vams\"\n{}",
            SOURCE.replace("R=1000", "R=`RESISTANCE")
        ),
    )
    .unwrap();
    let result = dir.join("model.json");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compile-va"])
        .arg(&input)
        .arg("-I")
        .arg(&extra)
        .arg("-o")
        .arg(&result)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(common::read_json(&result)["parameters"][0]["default"], 42.0);
}
