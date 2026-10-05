//! Every deck command shares includes, dialect and parameter override semantics.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn includes_and_strict_parameter_overrides_agree_across_commands() {
    let dir = test_dir("input_contract");
    let library = dir.join("library");
    std::fs::create_dir(&library).unwrap();
    std::fs::write(
        library.join("res.inc"),
        ".param rbot=1k\nR1 in out 1k\nR2 out 0 {rbot}\n",
    )
    .unwrap();
    let deck = dir.join("deck.cir");
    std::fs::write(
        &deck,
        "Shared input\nV1 in 0 1\n.include res.inc\n.op\n.end\n",
    )
    .unwrap();
    let config = dir.join("paths.toml");
    std::fs::write(
        &config,
        format!(
            "[paths]\ninclude_paths = [{}]\n",
            serde_json::to_string(&library.to_string_lossy()).unwrap()
        ),
    )
    .unwrap();
    for command in ["run", "check", "info"] {
        for configured in [true, false] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
            if configured {
                process.arg("--config").arg(&config);
            }
            process.args(["--quiet", command]).arg(&deck);
            if !configured {
                process.arg("-I").arg(&library);
            }
            process.args([
                "-D",
                "rbot=3k",
                "--redefined-params",
                "error",
                "--spice-dialect",
                "xyce",
            ]);
            if command == "run" {
                process
                    .args(["-f", "csv", "-o"])
                    .arg(dir.join("result.csv"));
            }
            let result = process.output().unwrap();
            assert!(
                result.status.success(),
                "{command}, configured={configured}: {result:?}"
            );
            if command == "run" {
                let result = std::fs::read_to_string(dir.join("result.csv")).unwrap();
                let output = result
                    .lines()
                    .find(|line| line.starts_with("V(OUT),"))
                    .unwrap()
                    .split(',')
                    .nth(1)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                assert!((output - 0.75).abs() < 1e-9, "{result}");
            }
        }
    }
}

#[test]
fn overrides_are_visible_to_conditionals_and_do_not_change_subcircuit_local_parameters() {
    let dir = test_dir("conditional_override");
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, "Override binding\n.param mode=0\n.if (mode > 0)\nV1 in 0 1\n.else\nV1 in 0 9\n.endif\n.subckt load p n\n.param rbot=2k\nR1 p n {rbot}\n.ends\nX1 in 0 load\n.op\n.end\n").unwrap();
    let output = dir.join("out.csv");
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .args([
            "-D",
            "mode=1",
            "-D",
            "rbot=3k",
            "--redefined-params",
            "error",
            "-f",
            "csv",
            "-o",
        ])
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let csv = std::fs::read_to_string(output).unwrap();
    let current = csv
        .lines()
        .find(|line| line.starts_with("I(V1),"))
        .unwrap()
        .split(',')
        .nth(1)
        .unwrap()
        .parse::<f64>()
        .unwrap();
    assert!((current + 0.0005).abs() < 1e-9, "{csv}");
}
