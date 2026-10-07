//! Execute deferred native model cards through the production CLI.
mod common;

use std::process::Command;

#[test]
fn a_temperature_dependent_diode_matches_the_equivalent_numeric_card() {
    let directory = common::test_dir("native_model_expression");
    let run = |value: &str, name: &str| {
        let deck = directory.join(format!("{name}.cir"));
        let result = directory.join(format!("{name}.csv"));
        std::fs::write(&deck, format!("* native model expression\nV1 out 0 .2\nD1 out 0 dd\n.MODEL dd D(IS={value})\n.OP\n.END\n")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--format", "csv", "--output"])
            .arg(&result)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let csv = std::fs::read_to_string(result).unwrap();
        csv.lines()
            .filter_map(|line| line.split_once(','))
            .find(|(signal, _)| signal.eq_ignore_ascii_case("I(V1)"))
            .unwrap_or_else(|| panic!("{csv}"))
            .1
            .parse::<f64>()
            .unwrap()
    };
    let expected = run("1u", "literal");
    let actual = run("{TEMP*0+1u}", "expression");
    assert!(expected.abs() > 1e-3);
    assert!(
        (actual - expected).abs() < expected.abs() * 1e-10,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn unresolved_native_model_parameters_fail_with_a_diagnostic() {
    let directory = common::test_dir("invalid_native_model_expression");
    let deck = directory.join("invalid.cir");
    std::fs::write(&deck, "* invalid native model expression\nV1 out 0 .2\nD1 out 0 dd\n.MODEL dd D(IS={TEMP+missing})\n.OP\n.END\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(&deck)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    let diagnostic = error.to_string().to_ascii_lowercase();
    assert!(
        diagnostic.contains("missing") && diagnostic.contains("parameter 'is'"),
        "{error}"
    );
}
