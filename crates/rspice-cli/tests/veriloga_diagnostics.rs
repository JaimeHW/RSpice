mod common;

use std::process::Command;

fn fixture(name: &str) -> (common::TestDirectory, std::path::PathBuf) {
    let root = common::test_dir(name);
    std::fs::write(root.join("model.va"), "`include \"child.va\"\n").unwrap();
    std::fs::write(root.join("child.va"), "module chatty(p,n);\ninout p,n; electrical p,n; parameter string label = \"x\";\nanalog begin\n $display(\"%s\", label);\n I(p,n) <+ V(p,n);\nend\nendmodule\n").unwrap();
    let deck = root.join("op.cir");
    std::fs::write(
        &deck,
        "Compiler warnings\n.va \"model.va\"\nV1 p 0 1\nX1 p 0 chatty\n.op\n.op\n.end\n",
    )
    .unwrap();
    (root, deck)
}

#[test]
fn runtime_warnings_survive_disk_cache_hits_and_are_not_repeated_per_analysis() {
    let (root, deck) = fixture("runtime_compiler_warnings");
    for cache_hit in [false, true] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--error-format",
                "json",
                "--log-level",
                if cache_hit { "debug" } else { "off" },
                "run",
            ])
            .arg(&deck)
            .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stderr).unwrap();
        let records: Vec<serde_json::Value> = text
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        if cache_hit {
            assert!(
                records.iter().any(|record| record["message"]
                    .as_str()
                    .is_some_and(|message| message.contains("Verilog-A cache hit (disk)"))),
                "{records:?}"
            );
        }
        let warnings: Vec<_> = records
            .iter()
            .filter(|record| record["schema"] == "rspice.diagnostic")
            .collect();
        assert_eq!(warnings.len(), 1, "{text}");
        let warning = &warnings[0]["diagnostic"];
        assert_eq!(warning["code"], "VA-SEM-NO-EFFECT-SYSTEM-TASK");
        assert_eq!(warning["line"], 4);
        assert_eq!(warning["column"], 2);
        assert_eq!(
            std::path::Path::new(warning["path"].as_str().unwrap())
                .canonicalize()
                .unwrap(),
            root.join("child.va").canonicalize().unwrap()
        );
    }
    assert!(std::fs::read_dir(root.join("cache")).unwrap().any(|entry| {
        entry
            .unwrap()
            .path()
            .extension()
            .is_some_and(|extension| extension == "json")
    }));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
}

#[test]
fn check_strict_includes_veriloga_compiler_warnings() {
    let (root, deck) = fixture("check_compiler_warnings");
    for strict in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command
            .args(["--error-format", "json", "check", "--json"])
            .arg(&deck);
        if strict {
            command.arg("--strict");
        }
        let output = command
            .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
            .output()
            .unwrap();
        assert_eq!(output.status.success(), !strict, "{output:?}");
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["valid"], true, "{result}");
        assert_eq!(result["strict_valid"], false, "{result}");
        assert_eq!(result["warnings"].as_array().unwrap().len(), 1, "{result}");
        assert_eq!(
            result["warnings"][0]["code"],
            "VA-SEM-NO-EFFECT-SYSTEM-TASK"
        );
        let diagnostic = &result["warnings"][0]["diagnostic"];
        if strict {
            let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(
                error["error"]["diagnostics"],
                serde_json::json!([diagnostic])
            );
            assert_eq!(error["error"]["line"], diagnostic["line"]);
            assert_eq!(error["error"]["path"], diagnostic["path"]);
        }
        assert_eq!(diagnostic["line"], 4);
        assert_eq!(diagnostic["column"], 2);
        assert_eq!(
            std::path::Path::new(diagnostic["path"].as_str().unwrap())
                .canonicalize()
                .unwrap(),
            root.join("child.va").canonicalize().unwrap()
        );
    }
}

#[test]
fn control_runs_retain_compiler_warnings_and_check_refuses_broken_models() {
    let (root, deck) = fixture("control_compiler_warnings");
    let source = std::fs::read_to_string(&deck)
        .unwrap()
        .replace(".op\n.op", ".control\nop\nop\n.endc");
    std::fs::write(&deck, source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "run"])
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let text = String::from_utf8(output.stderr).unwrap();
    let records: Vec<serde_json::Value> = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(
        records[0]["diagnostic"]["code"],
        "VA-SEM-NO-EFFECT-SYSTEM-TASK"
    );

    std::fs::write(root.join("child.va"), "module broken(\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "check", "--json"])
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["valid"], false, "{result}");
    assert_eq!(
        result["errors"][0]["diagnostic"]["phase"], "Parser",
        "{result}"
    );
    assert_eq!(result["errors"][0]["diagnostic"]["line"], 1, "{result}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    let diagnostics = result["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|issue| (!issue["diagnostic"].is_null()).then_some(issue["diagnostic"].clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        error["error"]["diagnostics"],
        serde_json::json!(diagnostics)
    );
}

#[test]
fn check_preserves_source_resource_failures_with_cold_and_warm_caches() {
    for (setting, resource, limit) in [
        ("max_include_depth", "include_depth", 1),
        ("max_expanded_source_bytes", "expanded_source_bytes", 256),
        (
            "max_dependency_source_bytes",
            "dependency_source_bytes",
            256,
        ),
    ] {
        for warm in [false, true] {
            let (root, deck) = fixture(&format!("check_{setting}_{warm}"));
            let child = root.join("child.va");
            let source = std::fs::read_to_string(&child).unwrap();
            let parameters = (0..128)
                .map(|index| format!("parameter real q{index}=1;\n"))
                .collect::<String>();
            std::fs::write(
                &child,
                source.replace(
                    "module chatty(p,n);",
                    &format!("module chatty(p,n);\n{parameters}"),
                ),
            )
            .unwrap();
            let cache = root.join("cache");
            if warm {
                let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "run"])
                    .arg(&deck)
                    .env("RSPICE_VERILOGA_CACHE_DIR", &cache)
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
                assert!(std::fs::read_dir(&cache).unwrap().any(|entry| {
                    entry
                        .unwrap()
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "json")
                }));
            }
            let config = root.join("config.toml");
            std::fs::write(&config, format!("[resources]\n{setting}={limit}\n")).unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", "json", "--config"])
                .arg(&config)
                .args(["check", "--json"])
                .arg(&deck)
                .env("RSPICE_VERILOGA_CACHE_DIR", &cache)
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(75),
                "{setting}, warm={warm}: {output:?}"
            );
            let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(error["error"]["resource"], resource, "{error}");
            assert_eq!(error["error"]["limit"], limit, "{error}");
            assert!(error["error"]["requested"].as_u64().unwrap() > limit);
            let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(report["valid"], false, "{report}");
            assert_eq!(
                report["errors"][0]["details"]["resource"], resource,
                "{report}"
            );
        }
    }
}

#[test]
fn fatal_compiler_findings_keep_original_include_locations_in_run_and_check() {
    for (body, phase, line, token) in [
        ("analog I(p,n) <+ @;", "Parser", 4, "@"),
        ("analog I(p,n) <+ \u{a3};", "Lexer", 4, "\u{a3}"),
        (
            "parameter real R=1;\nparameter real R=2;",
            "Semantic",
            5,
            "R",
        ),
        ("`define BAD @\nanalog I(p,n) <+ `BAD;", "Parser", 5, "`BAD"),
        (
            "analog I(p,n) <+ V(p,n) + missing;",
            "Semantic",
            4,
            "missing",
        ),
        (
            "`define BAD missing\nanalog I(p,n) <+ `BAD;",
            "Semantic",
            5,
            "`BAD",
        ),
    ] {
        let (root, deck) = fixture("fatal_compiler_locations");
        let child = root.join("child.va");
        let source = format!(
            "// original child\nmodule chatty(p,n);\ninout p,n; electrical p,n;\n{body}\nendmodule\n"
        );
        std::fs::write(&child, &source).unwrap();
        for command in ["run", "check", "compile-va"] {
            for format in ["text", "json"] {
                let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
                process.args([
                    "--quiet",
                    "--log-level",
                    "off",
                    "--error-format",
                    format,
                    command,
                ]);
                if command == "check" && format == "json" {
                    process.arg("--json");
                }
                let summary = root.join("summary.json");
                if command == "run" {
                    process.arg("--summary").arg(&summary);
                }
                let output = process
                    .arg(if command == "compile-va" {
                        root.join("model.va")
                    } else {
                        deck.clone()
                    })
                    .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
                    .output()
                    .unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(if command == "compile-va" { 1 } else { 65 }),
                    "{output:?}"
                );
                let stderr = String::from_utf8(output.stderr).unwrap();
                let stdout = String::from_utf8(output.stdout).unwrap();
                assert!(!stderr.contains("at offset"), "{stderr}");
                assert!(!stdout.contains("at offset"), "{stdout}");
                if format == "text" {
                    assert!(
                        format!("{stdout}{stderr}").contains(&format!("child.va:{line}:")),
                        "{stdout}{stderr}"
                    );
                    continue;
                }
                let fatal: serde_json::Value = serde_json::from_str(&stderr).unwrap();
                let report: serde_json::Value = if command == "check" {
                    serde_json::from_str(&stdout).unwrap()
                } else if command == "run" {
                    common::read_json(&summary)
                } else {
                    serde_json::Value::Null
                };
                let diagnostic = if command == "check" {
                    assert_eq!(report["valid"], false);
                    assert_eq!(report["errors"].as_array().unwrap().len(), 1, "{report}");
                    &report["errors"][0]["diagnostic"]
                } else {
                    let diagnostics = fatal["error"]["diagnostics"].as_array().unwrap();
                    assert_eq!(diagnostics.len(), 1, "{fatal}");
                    if command == "run" {
                        assert_eq!(
                            report["runs"][0]["error_details"]["diagnostics"],
                            fatal["error"]["diagnostics"]
                        );
                    }
                    &diagnostics[0]
                };
                assert_eq!(diagnostic["phase"], phase);
                assert_eq!(diagnostic["line"], line);
                assert_eq!(
                    std::path::Path::new(diagnostic["path"].as_str().unwrap())
                        .canonicalize()
                        .unwrap(),
                    child.canonicalize().unwrap()
                );
                let start = diagnostic["byte_start"].as_u64().unwrap() as usize;
                let end = diagnostic["byte_end"].as_u64().unwrap() as usize;
                assert!(source[start..end].contains(token), "{diagnostic}");
            }
        }
    }
}

#[test]
fn preprocessor_failures_keep_original_include_locations() {
    let (root, deck) = fixture("preprocessor_failure_locations");
    let child = root.join("child.va");
    std::fs::write(&child, "// invalid conditional\n`else\n").unwrap();
    for command in ["run", "check", "compile-va"] {
        let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
        process.args(["--quiet", "--error-format", "json", command]);
        if command == "check" {
            process.arg("--json");
        }
        let output = process
            .arg(if command == "compile-va" {
                root.join("model.va")
            } else {
                deck.clone()
            })
            .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(if command == "compile-va" { 1 } else { 65 }),
            "{output:?}"
        );
        let document: serde_json::Value = serde_json::from_slice(if command == "check" {
            &output.stdout
        } else {
            &output.stderr
        })
        .unwrap();
        let diagnostic = if command == "check" {
            &document["errors"][0]["diagnostic"]
        } else {
            &document["error"]["diagnostics"][0]
        };
        assert_eq!(diagnostic["code"], "VA-INPUT-PREPROCESS", "{document}");
        assert_eq!(diagnostic["line"], 2);
        assert!(diagnostic["column"].is_null());
        assert!(diagnostic["byte_start"].is_null());
        assert_eq!(
            std::path::Path::new(diagnostic["path"].as_str().unwrap())
                .canonicalize()
                .unwrap(),
            child.canonicalize().unwrap()
        );
    }
}

#[test]
fn control_and_parallel_step_failures_retain_compiler_metadata() {
    for mode in ["control", "step"] {
        let (root, deck) = fixture(&format!("fatal_{mode}_compiler_locations"));
        let child = root.join("child.va");
        std::fs::write(&child, "// model\nmodule chatty(p,n);\ninout p,n; electrical p,n;\nanalog I(p,n) <+ @;\nendmodule\n").unwrap();
        let source = std::fs::read_to_string(&deck).unwrap();
        std::fs::write(
            &deck,
            source.replace(
                ".op\n.op",
                if mode == "control" {
                    ".control\nop\n.endc"
                } else {
                    ".param gain=1\n.step param gain list 1 2\n.op"
                },
            ),
        )
        .unwrap();
        let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
        process.args([
            "--quiet",
            "--log-level",
            "off",
            "--error-format",
            "json",
            "run",
        ]);
        if mode == "step" {
            process.args(["--jobs", "2"]);
        }
        let output = process
            .arg(&deck)
            .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(65), "{output:?}");
        let document: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        let diagnostics = document["error"]["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics.len(), 1, "{document}");
        assert_eq!(diagnostics[0]["line"], 4);
        assert_eq!(
            std::path::Path::new(diagnostics[0]["path"].as_str().unwrap())
                .canonicalize()
                .unwrap(),
            child.canonicalize().unwrap()
        );
    }
}

/// An analog `$strobe` prints once per accepted point on standard output, and
/// a severity task reports where it was written on standard error; neither
/// prints from a Newton iteration the solver went past.
#[test]
fn analog_output_tasks_print_once_per_accepted_point() {
    let root = common::test_dir("analog_output_tasks");
    std::fs::write(
        root.join("printer.va"),
        "module printer(a);\ninout a; electrical a;\nanalog begin\n V(a) <+ 1.5;\n $strobe(\"STROBE V(a)=%g\", V(a));\n $warning(\"careful at %g\", V(a));\nend\nendmodule\n",
    )
    .unwrap();
    let deck = root.join("strobe.cir");
    std::fs::write(
        &deck,
        "Analog output\n.hdl \"printer.va\"\nX1 a printer\nRa a 0 1k\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("STROBE V(a)=1.5\n").count(), 1, "{stdout}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stderr.matches("careful at 1.5").count(), 1, "{stderr}");
    assert!(stderr.contains("printer.va:6, X1)"), "{stderr}");
    assert!(!stderr.contains("VA-SEM-NO-EFFECT-SYSTEM-TASK"), "{stderr}");
}

/// A model's `$finish` names its instance, file, line and time, and the
/// transient headline says the run ended before the requested stop.
#[test]
fn analog_finish_reports_its_caller_and_the_early_end() {
    let root = common::test_dir("analog_finish_report");
    std::fs::write(
        root.join("finish.va"),
        "module finish_t(a);\ninout a; electrical a;\nanalog begin\n V(a) <+ 1;\n if ($abstime > 5n) $finish;\nend\nendmodule\n",
    )
    .unwrap();
    let deck = root.join("finish.cir");
    std::fs::write(
        &deck,
        "Analog finish\n.hdl \"finish.va\"\nX1 a finish_t\nRa a 0 1k\n.tran 1n 20n\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("Transient complete"), "{stdout}");
    assert!(
        stdout.contains("Transient ended early at 5.") && stdout.contains("of the requested 20 ns"),
        "{stdout}"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr.matches("finished by $finish at 5.").count(),
        1,
        "{stderr}"
    );
    assert!(stderr.contains("finish.va:5, X1)"), "{stderr}");
}

/// A contribution that evaluates to NaN is named by the file, line and text
/// it was written as, not only by an index the author never sees.
#[test]
fn a_non_finite_contribution_names_its_source_line() {
    let root = common::test_dir("non_finite_contribution_source");
    std::fs::write(
        root.join("sq.va"),
        "`include \"disciplines.vams\"\nmodule sq(p,n); inout p,n; electrical p,n;\nanalog I(p,n) <+ 1m*sqrt(V(p,n));\nendmodule\n",
    )
    .unwrap();
    let deck = root.join("sq.cir");
    std::fs::write(
        &deck,
        "Square root of a negative bias\n.hdl \"sq.va\"\nV1 a 0 -1\nR1 a b 1k\nX1 b 0 sq\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(&deck)
        .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("contribution 0 (sq.va:3: I(p,n) <+ 1m*sqrt(V(p,n))"),
        "{stderr}"
    );
}

#[test]
fn module_selection_and_misspelled_masters_say_what_to_write() {
    let root = common::test_dir("module_selection_advice");
    let source = root.join("two.va");
    std::fs::write(
        &source,
        "`include \"disciplines.vams\"\n\
         module res(p,n);\ninout p,n; electrical p,n;\nparameter real r = 1k;\n\
         analog I(p,n) <+ V(p,n)/r;\nendmodule\n\
         module top(a,b);\ninout a,b; electrical a,b;\nres #(.r(2k)) r1 (a, b);\nendmodule\n",
    )
    .unwrap();
    let run = |name: &str, directive: &str, master: &str| {
        let deck = root.join(name);
        std::fs::write(
            &deck,
            format!("Module selection\n{directive}\nV1 a 0 2\nX1 a 0 {master}\n.op\n.end\n"),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--log-level", "off", "run"])
            .arg(&deck)
            .env("RSPICE_VERILOGA_CACHE_DIR", root.join("cache"))
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        (output.status.code(), stdout, stderr)
    };

    // Two modules and none named: the refusal said "select one by name",
    // naming neither the file nor how a deck selects, after "Simulation
    // complete".
    let (code, stdout, stderr) = run("unselected.cir", ".hdl \"two.va\"", "top");
    assert_eq!(code, Some(65), "{stdout}{stderr}");
    assert!(
        stderr.contains("two.va: [VA-MODULE-SELECTION]")
            && stderr.contains(
                "declares multiple modules: res, top; \
                 select one with module=<name> on the line that loads it"
            ),
        "{stderr}"
    );
    assert!(!stdout.contains("Simulation complete"), "{stdout}");

    let (code, stdout, stderr) = run("selected.cir", ".hdl \"two.va\" module=top", "top");
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert!(stdout.contains("Simulation complete"), "{stdout}");

    // A misspelled master is refused as an undefined subcircuit, now with
    // the module it most likely means: one the directive selects, and one
    // only compiling the source finds.
    std::fs::write(
        root.join("one.va"),
        "`include \"disciplines.vams\"\nmodule okres(p,n);\ninout p,n; electrical p,n;\n\
         analog I(p,n) <+ V(p,n)/1k;\nendmodule\n",
    )
    .unwrap();
    for (directive, master, intended) in [
        (".hdl \"two.va\" module=res", "rse", "res"),
        (".hdl \"one.va\"", "okress", "okres"),
    ] {
        let (code, stdout, stderr) = run("misspelled.cir", directive, master);
        assert_eq!(code, Some(65), "{stdout}{stderr}");
        assert!(
            stderr.contains(&format!(
                "Subcircuit {} has not been defined for instance X1 (did you mean '{intended}'?)",
                master.to_ascii_uppercase()
            )),
            "{stderr}"
        );
        assert!(!stdout.contains("Simulation complete"), "{stdout}");
    }

    // compile-va names its own option.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--log-level", "off", "compile-va"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("two.va: [VA-MODULE-SELECTION]")
            && stderr.contains("select one with --module <name>"),
        "{stderr}"
    );
}
