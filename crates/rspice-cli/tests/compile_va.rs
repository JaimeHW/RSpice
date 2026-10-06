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
        assert!(error["error"]["path"].is_string(), "{error}");
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

#[test]
fn source_io_failures_keep_their_exit_category_and_path() {
    let dir = common::test_dir("source_io");
    let invalid = dir.join("invalid.va");
    let directory = dir.join("directory.va");
    let missing = dir.join("missing.va");
    let result = dir.join("model.json");
    std::fs::write(&invalid, [0xff, 0xfe]).unwrap();
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(&result, "previous interface").unwrap();
    for (path, exit, code) in [
        (&missing, 66, "input_not_found"),
        (&invalid, 74, "input_read_error"),
        (&directory, 74, "input_read_error"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "compile-va"])
            .arg(path)
            .arg("-o")
            .arg(&result)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(exit), "{path:?}: {output:?}");
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(error["error"]["code"], code, "{error}");
        assert_eq!(error["error"]["path"], path.to_str().unwrap());
        if path == &invalid {
            let cause = String::from_utf8(std::fs::read(path).unwrap()).unwrap_err();
            assert!(
                error["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains(&cause.to_string()),
                "{error}"
            );
            let text = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "compile-va"])
                .arg(path)
                .output()
                .unwrap();
            assert_eq!(text.status.code(), Some(74));
            assert!(
                String::from_utf8(text.stderr)
                    .unwrap()
                    .contains(&cause.to_string())
            );
        }
        assert_eq!(
            std::fs::read_to_string(&result).unwrap(),
            "previous interface"
        );
    }
}

#[test]
fn include_read_failures_identify_the_include_and_preserve_existing_output() {
    let dir = common::test_dir("include_io");
    let root = dir.join("model.va");
    let include = dir.join("value.vams");
    let result = dir.join("model.json");
    std::fs::write(&root, format!("`include \"value.vams\"\n{SOURCE}")).unwrap();
    std::fs::write(&include, [0xff, 0xfe]).unwrap();
    std::fs::write(&result, "previous interface").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "compile-va"])
        .arg(root)
        .arg("-o")
        .arg(&result)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(74), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["code"], "input_read_error", "{error}");
    assert_eq!(
        std::path::Path::new(error["error"]["path"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        include.canonicalize().unwrap()
    );
    assert_eq!(
        std::fs::read_to_string(result).unwrap(),
        "previous interface"
    );
}

#[cfg(windows)]
#[test]
fn an_exclusively_open_source_is_an_io_failure() {
    use std::os::windows::fs::OpenOptionsExt;

    let dir = common::test_dir("source_sharing");
    let root = dir.join("model.va");
    std::fs::write(&root, SOURCE).unwrap();
    let _held = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&root)
        .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "compile-va"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(74), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["code"], "input_read_error", "{error}");
    assert_eq!(error["error"]["path"], root.to_str().unwrap());
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains(&std::io::Error::from_raw_os_error(32).to_string()),
        "the sharing violation must remain visible: {error}"
    );
}

#[test]
fn usage_examples_run_with_nested_paths_and_explicit_module_selection() {
    for selected in [None, Some("second")] {
        let dir = common::test_dir("usage_example");
        let models = dir.join("vendor library/nested");
        let examples = dir.join("examples");
        std::fs::create_dir_all(&models).unwrap();
        std::fs::create_dir(&examples).unwrap();
        let source = match selected {
            None => SOURCE.to_owned(),
            Some(_) => format!(
                "{SOURCE}\n{}",
                SOURCE
                    .replace("module resistor", "module second")
                    .replace("R=1000", "R=2000")
            ),
        };
        std::fs::write(models.join("two terminal model.va"), source).unwrap();
        let mut compile = Command::new(env!("CARGO_BIN_EXE_rspice"));
        compile.current_dir(&dir).args([
            "compile-va",
            "vendor library/nested/two terminal model.va",
            "--show-usage",
        ]);
        if let Some(module) = selected {
            compile.args(["--module", module]);
        }
        let output = compile.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        let cards = text
            .lines()
            .filter(|line| line.starts_with("  .va ") || line.starts_with("  X1 "))
            .map(str::trim)
            .collect::<Vec<_>>();
        assert_eq!(cards.len(), 2, "{text}");
        let deck = examples.join("example.sp");
        std::fs::write(
            &deck,
            format!(
                "* printed usage example\n{}\nVdrive p 0 1\nRload n 0 1k\n.op\n.end\n",
                cards.join("\n")
            ),
        )
        .unwrap();
        let result = examples.join("result.json");
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .current_dir(&examples)
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--format", "json", "--output"])
            .arg(&result)
            .output()
            .unwrap();
        assert!(output.status.success(), "{text}\n{output:?}");
        let document = common::read_json(&result);
        let voltage = document["signals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|signal| signal["descriptor"]["canonicalName"] == "v(n)")
            .unwrap()["values"]["samples"][0]
            .as_f64()
            .unwrap();
        let expected = if selected.is_some() { 1.0 / 3.0 } else { 0.5 };
        assert!((voltage - expected).abs() < 1e-8, "{document}");
    }
}

#[test]
fn preprocessing_diagnostics_name_the_authored_include_and_line() {
    let dir = common::test_dir("preprocessor_location");
    let root = dir.join("model.va");
    let header = dir.join("bad.vams");
    std::fs::write(&root, format!("`include \"bad.vams\"\n{SOURCE}")).unwrap();
    std::fs::write(&header, "// invalid conditional\n`else\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "compile-va"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["code"], "veriloga_error");
    assert_eq!(error["error"]["line"], 2, "{error}");
    assert_eq!(
        std::path::Path::new(error["error"]["path"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        header.canonicalize().unwrap()
    );
}

#[test]
fn compiler_diagnostics_identify_original_include_and_macro_locations() {
    let dir = common::test_dir("compiler_locations");
    let root = dir.join("root.va");
    let child = dir.join("child.va");
    let output_path = dir.join("interface.json");
    std::fs::write(
        &root,
        "`include \"disciplines.vams\"\n`include \"child.va\"\n",
    )
    .unwrap();
    for (body, phase, expected_line, token) in [
        ("analog I(p,n) <+ @;", "Parser", 4, "@"),
        ("analog I(p,n) <+ \u{a3};", "Lexer", 4, "\u{a3}"),
        (
            "parameter real R=1;\nparameter real R=2;",
            "Semantic",
            5,
            "R",
        ),
        ("`define BAD @\nanalog I(p,n) <+ `BAD;", "Parser", 5, "`BAD"),
    ] {
        let source = format!(
            "// original child\nmodule selected(p,n);\ninout p,n; electrical p,n;\n{body}\nendmodule\n"
        );
        std::fs::write(&child, &source).unwrap();
        std::fs::write(&output_path, "previous interface").unwrap();
        for format in ["text", "json"] {
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", format, "compile-va"])
                .arg(&root)
                .arg("-o")
                .arg(&output_path)
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(1), "{output:?}");
            let stderr = String::from_utf8(output.stderr).unwrap();
            if format == "text" {
                assert!(!stderr.contains("at offset"), "{stderr}");
                assert!(
                    stderr.contains(&format!("child.va:{expected_line}:")),
                    "{stderr}"
                );
            } else {
                let error: serde_json::Value = serde_json::from_str(&stderr).unwrap();
                assert_eq!(error["error"]["line"], expected_line, "{error}");
                assert!(
                    !error["error"]["message"]
                        .as_str()
                        .unwrap()
                        .contains("at offset"),
                    "{error}"
                );
                let diagnostics = error["error"]["diagnostics"].as_array().unwrap();
                assert_eq!(diagnostics.len(), 1, "{error}");
                let diagnostic = &diagnostics[0];
                assert_eq!(diagnostic["phase"], phase, "{error}");
                assert_eq!(diagnostic["line"], expected_line, "{error}");
                assert_eq!(
                    std::path::Path::new(diagnostic["path"].as_str().unwrap())
                        .canonicalize()
                        .unwrap(),
                    child.canonicalize().unwrap()
                );
                let start = diagnostic["byte_start"].as_u64().unwrap() as usize;
                let end = diagnostic["byte_end"].as_u64().unwrap() as usize;
                assert!(source[start..end].contains(token), "{error}");
                if token == "`BAD" {
                    assert_eq!(diagnostic["column"], 1, "{error}");
                }
                assert!(diagnostic["code"].as_str().unwrap().starts_with("VA-"));
                assert!(diagnostic.get("source").is_none(), "{error}");
            }
            assert_eq!(
                std::fs::read_to_string(&output_path).unwrap(),
                "previous interface"
            );
        }
    }
}
