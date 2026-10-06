mod common;
use std::process::Command;

const CONFIG: &str = "[output]\nformat = 'csv'\n";
const DECK: &str = "config\nV1 in 0 1\nR1 in 0 1k\n.tran 1n 2n\n.end\n";

#[test]
fn explicit_and_project_configuration_files_are_protected_from_every_run_artifact() {
    for explicit in [false, true] {
        let dir = common::test_dir("config_run_ownership");
        let deck = dir.join("deck.cir");
        let config = dir.join(if explicit { "custom.toml" } else { ".rspicerc" });
        std::fs::write(&deck, DECK).unwrap();
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            std::fs::write(&config, CONFIG).unwrap();
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command.arg("--quiet").current_dir(&dir);
            if explicit {
                command.arg("--config").arg(&config);
            }
            let result = command
                .arg("run")
                .arg(&deck)
                .args(&flags)
                .arg(&config)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{explicit}, {flags:?}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("source"),
                "{result:?}"
            );
            assert_eq!(std::fs::read_to_string(&config).unwrap(), CONFIG);
        }
    }
}

#[test]
fn converters_compilers_and_blessing_cannot_replace_their_loaded_configuration() {
    let dir = common::test_dir("config_tool_ownership");
    let config = dir.join("config.csv");
    let wave = dir.join("wave.csv");
    let model = dir.join("resistor.va");
    std::fs::write(&wave, "time,V(out)\n0,1\n1,1\n").unwrap();
    std::fs::write(&model, "module resistor(p,n);\ninout p,n; electrical p,n;\nanalog I(p,n)<+V(p,n)/1000;\nendmodule\n").unwrap();
    for args in [
        vec![
            "convert",
            wave.to_str().unwrap(),
            config.to_str().unwrap(),
            "--to",
            "csv",
        ],
        vec![
            "compile-va",
            model.to_str().unwrap(),
            "-o",
            config.to_str().unwrap(),
        ],
        vec![
            "compare",
            wave.to_str().unwrap(),
            config.to_str().unwrap(),
            "--bless",
        ],
    ] {
        std::fs::write(&config, CONFIG).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "--config"])
            .arg(&config)
            .args(&args)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{args:?}: {result:?}");
        let error: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
        assert_eq!(error["error"]["exit_code"], 2);
        assert_eq!(std::fs::read_to_string(&config).unwrap(), CONFIG);
    }
}

#[test]
fn parallel_generated_names_preserve_loaded_configurations() {
    let dir = common::test_dir("config_derived_ownership");
    let deck = dir.join("deck.cir");
    let config = dir.join("result.ss.csv");
    std::fs::write(&deck, DECK).unwrap();
    std::fs::write(&config, CONFIG).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--config"])
        .arg(&config)
        .arg("run")
        .arg(&deck)
        .args(["--corners", "tt,ss", "-j", "2", "-o"])
        .arg(dir.join("result.csv"))
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&config).unwrap(), CONFIG);
}

#[test]
fn explicit_configuration_does_not_reserve_an_unloaded_project_file() {
    let dir = common::test_dir("unused_config_output");
    let deck = dir.join("deck.cir");
    let config = dir.join("selected.toml");
    let unused = dir.join(".rspicerc");
    std::fs::write(&deck, DECK).unwrap();
    std::fs::write(&config, CONFIG).unwrap();
    std::fs::write(&unused, "this file is not read as configuration\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--config"])
        .arg(&config)
        .arg("run")
        .arg(&deck)
        .arg("-o")
        .arg(&unused)
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    assert_eq!(std::fs::read_to_string(&config).unwrap(), CONFIG);
    assert!(
        std::fs::read_to_string(&unused)
            .unwrap()
            .starts_with("time,")
    );
}
