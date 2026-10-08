mod common;

use std::process::Command;

fn source(fields: &str, scoped: bool) -> String {
    let mut body = format!("A1 %hd[out 0] capmod {fields}");
    if scoped {
        body = format!(".SUBCKT cell out\n{body}\n.ENDS\nX1 out cell");
    }
    format!(
        "* native binding publication\n.PARAM IC=7 C=1e-9\nV1 in 0 AC 1\nR1 in out 1k\n{body}\n\
         .PARAM later=2\n.GLOBAL_PARAM dependent={{IC}}\n.FUNC read_ic() {{IC}}\n\
         .MODEL capmod capacitor(c=1n ic=9)\n.AC LIN 1 100k 100k\n.END\n"
    )
}

#[test]
fn native_capacitor_sibling_values_reach_ac_csv_in_both_scopes() {
    for fields in [
        "C={IC*1e-9} IC={later}",
        "C={dependent*1e-9} IC={later}",
        "IC={later} C={read_ic()*1e-9}",
    ] {
        for scoped in [false, true] {
            let directory = common::test_dir("native_binding_ac");
            let deck = directory.join("deck.cir");
            let result = directory.join("result.csv");
            std::fs::write(&deck, source(fields, scoped)).unwrap();
            for command in ["check", "run"] {
                let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
                process.args(["--quiet", command]).arg(&deck);
                if command == "run" {
                    process.args(["--format", "csv", "--output"]).arg(&result);
                }
                let output = process.output().unwrap();
                assert!(
                    output.status.success(),
                    "{fields}, scoped={scoped}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            let csv = std::fs::read_to_string(&result).unwrap();
            let mut lines = csv.lines();
            let columns = lines.next().unwrap().split(',').collect::<Vec<_>>();
            let values = lines.next().unwrap().split(',').collect::<Vec<_>>();
            let x = 2.0 * std::f64::consts::PI * 1e5 * 1e3 * 2e-9;
            for (name, expected) in [
                ("Re(V(OUT))", 1.0 / (1.0 + x * x)),
                ("Im(V(OUT))", -x / (1.0 + x * x)),
            ] {
                let column = columns
                    .iter()
                    .position(|value| value.eq_ignore_ascii_case(name))
                    .unwrap_or_else(|| panic!("{csv}"));
                let actual: f64 = values[column].parse().unwrap();
                assert!(
                    (actual - expected).abs() < 1e-9,
                    "{fields}, scoped={scoped}, {name}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn native_dependency_failures_keep_json_causes_and_existing_results() {
    for (fields, expected) in [
        ("C={IC*1e-9} IC={C/1e-9}", "CYCLIC"),
        ("C={IC*1e-9} IC={missing_leaf}", "MISSING_LEAF"),
    ] {
        for scoped in [false, true] {
            let directory = common::test_dir("native_binding_failure");
            let deck = directory.join("deck.cir");
            let result = directory.join("result.csv");
            std::fs::write(&deck, source(fields, scoped)).unwrap();
            for command in ["check", "run"] {
                std::fs::write(&result, "existing result").unwrap();
                let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
                process
                    .args(["--quiet", "--error-format", "json", command])
                    .arg(&deck);
                if command == "run" {
                    process.args(["--format", "csv", "--output"]).arg(&result);
                }
                let output = process.output().unwrap();
                assert!(
                    !output.status.success(),
                    "{fields}, scoped={scoped}, {command}"
                );
                let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                assert!(
                    error["error"]["message"]
                        .as_str()
                        .unwrap()
                        .to_ascii_uppercase()
                        .contains(expected),
                    "{error}"
                );
                assert_eq!(error["error"]["exit_code"], output.status.code().unwrap());
                assert_eq!(std::fs::read_to_string(&result).unwrap(), "existing result");
            }
        }
    }
}
