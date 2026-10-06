mod common;
use std::process::Command;

#[test]
fn numeric_flags_cannot_silently_discard_trailing_input() {
    let dir = common::test_dir("numeric_flags");
    let deck = dir.join("deck.cir");
    let input = dir.join("wave.csv");
    let output = dir.join("output.csv");
    std::fs::write(&deck, "numeric\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    std::fs::write(&input, "time,V(out)\n0,1\n1,2\n").unwrap();
    for value in ["1+2", "1/2", "1u;", "1 extra", "1.2.3", "1e999"] {
        for args in [
            vec![
                "run",
                deck.to_str().unwrap(),
                "--temp",
                value,
                "-f",
                "csv",
                "-o",
                output.to_str().unwrap(),
            ],
            vec![
                "compare",
                input.to_str().unwrap(),
                input.to_str().unwrap(),
                "--abstol",
                value,
            ],
            vec![
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                "csv",
                "--stop",
                value,
            ],
        ] {
            std::fs::write(&output, "original bytes").unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", "json"])
                .args(&args)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(2), "{args:?}: {result:?}");
            let error: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
            assert_eq!(error["error"]["exit_code"], 2);
            assert_eq!(std::fs::read_to_string(&output).unwrap(), "original bytes");
        }
    }
}

#[test]
fn defines_require_a_complete_finite_number_in_every_netlist_command() {
    let dir = common::test_dir("numeric_defines");
    let deck = dir.join("deck.cir");
    std::fs::write(
        &deck,
        "numeric\n.param LOAD=1k\nV1 in 0 1\nR1 in 0 {LOAD}\n.op\n.end\n",
    )
    .unwrap();
    for value in ["2*3", "1+2", "1 extra", "1.2.3", "1e999"] {
        for command in ["run", "check", "info"] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", command])
                .arg(&deck)
                .arg("-D")
                .arg(format!("LOAD={value}"))
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{command} {value}: {result:?}"
            );
            assert!(
                String::from_utf8_lossy(&result.stderr).contains("--define"),
                "{result:?}"
            );
        }
    }
}
