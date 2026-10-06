mod common;

use std::process::Command;

fn fixture(name: &str) -> (common::TestDirectory, std::path::PathBuf) {
    let root = common::test_dir(name);
    std::fs::write(root.join("model.va"), "`include \"child.va\"\n").unwrap();
    std::fs::write(root.join("child.va"), "module chatty(p,n);\ninout p,n; electrical p,n;\nanalog begin\n $display(\"ignored\");\n I(p,n) <+ V(p,n);\nend\nendmodule\n").unwrap();
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
    assert!(
        result["errors"][0]["message"]
            .as_str()
            .unwrap()
            .contains("preparation failed"),
        "{result}"
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
