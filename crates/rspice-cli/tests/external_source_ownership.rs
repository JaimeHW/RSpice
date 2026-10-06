mod common;
use std::path::Path;
use std::process::Command;

const MODEL: &str = "`include \"disciplines.vams\"\n`include \"gain.vh\"\nmodule resistor(p,n);\ninout p,n; electrical p,n;\nanalog I(p,n) <+ V(p,n)*`GAIN;\nendmodule\n";

fn expect_protected(deck: &Path, target: &Path, flags: &[&str]) {
    let before = std::fs::read(target).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .args(["-f", "csv"])
        .args(flags)
        .arg(target)
        .current_dir(deck.parent().unwrap())
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{flags:?}: {result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read(target).unwrap(), before);
}

#[test]
fn all_artifact_roles_preserve_pwl_inputs() {
    let dir = common::test_dir("pwl_source_ownership");
    let data = dir.join("source.dat");
    let deck = dir.join("deck.cir");
    for nested in [false, true] {
        let body = if nested {
            ".subckt source in\nV1 in 0 DC {bias} PWL FILE=\"source.dat\"\n.ends\n.param bias=1\nX1 in source\n"
        } else {
            "V1 in 0 PWL FILE=\"source.dat\"\n"
        };
        std::fs::write(
            &deck,
            format!("data\n{body}R1 in 0 1k\n.tran 1n 2n\n.end\n"),
        )
        .unwrap();
        for flags in [
            vec!["-o"],
            vec!["--checkpoint"],
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            std::fs::write(&data, "0 1\n1 1\n").unwrap();
            expect_protected(&deck, &data, &flags);
        }
    }
}

#[test]
fn verilog_models_and_active_nested_headers_are_protected() {
    let dir = common::test_dir("verilog_source_ownership");
    let deck = dir.join("deck.cir");
    let model = dir.join("resistor.va");
    let header = dir.join("gain.vh");
    let nested = dir.join("constants.vh");
    std::fs::write(
        &deck,
        "model\n.va resistor.va\nV1 in 0 1\nX1 in 0 resistor\n.op\n.end\n",
    )
    .unwrap();
    std::fs::write(&model, MODEL).unwrap();
    std::fs::write(&header, "`include \"constants.vh\"\n").unwrap();
    std::fs::write(&nested, "`define GAIN 1e-3\n").unwrap();
    for target in [&model, &header, &nested] {
        for flags in [
            vec!["-o"],
            vec!["--summary"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            expect_protected(&deck, target, &flags);
        }
    }
}

#[test]
fn parallel_derived_outputs_preserve_external_inputs() {
    for veriloga in [false, true] {
        let dir = common::test_dir("derived_external_source_ownership");
        let deck = dir.join("deck.cir");
        let source = dir.join("result.ss.csv");
        let contents = if veriloga {
            "`define GAIN 1e-3\n"
        } else {
            "0 1\n1 1\n"
        };
        let body = if veriloga {
            std::fs::write(
                dir.join("resistor.va"),
                MODEL.replace("gain.vh", "result.ss.csv"),
            )
            .unwrap();
            ".va resistor.va\nV1 in 0 1\nX1 in 0 resistor\n"
        } else {
            "V1 in 0 PWL FILE=\"result.ss.csv\"\nR1 in 0 1k\n"
        };
        std::fs::write(&source, contents).unwrap();
        std::fs::write(&deck, format!("derived\n{body}.op\n.end\n")).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{veriloga}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("source"),
            "{result:?}"
        );
        assert_eq!(std::fs::read_to_string(&source).unwrap(), contents);
    }
}

#[test]
fn failed_model_compilation_does_not_replace_its_headers_with_failure_reports() {
    let dir = common::test_dir("invalid_external_source_ownership");
    let deck = dir.join("deck.cir");
    let model = dir.join("broken.va");
    let header = dir.join("broken.vh");
    std::fs::write(
        &deck,
        "broken\n.va broken.va\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
    )
    .unwrap();
    std::fs::write(&model, "`include \"broken.vh\"\n").unwrap();
    std::fs::write(&header, "not a valid Verilog-A module\n").unwrap();
    expect_protected(
        &deck,
        &header,
        &["--report-format", "junit", "--report-file"],
    );
}
