mod common;

use std::process::Command;

#[test]
fn explicit_complex_projection_preserves_the_selected_physical_value() {
    let directory = common::test_dir("projected_model");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        "* explicit projection\n.PARAM z={2+3j}\n.MODEL device R(RSH={IMG(z)})\n\
         V1 out 0 1\nR1 out 0 device L=1 W=1\n.OP\n.END\n",
    )
    .unwrap();
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
    let current: f64 = csv
        .lines()
        .filter_map(|line| line.split_once(','))
        .find(|(name, _)| name.eq_ignore_ascii_case("I(V1)"))
        .unwrap()
        .1
        .parse()
        .unwrap();
    assert!((current + 1.0 / 3.0).abs() < 1e-10, "{csv}");
}

#[test]
fn check_and_run_refuse_complex_real_model_fields_before_publication() {
    for (model, instance) in [
        ("D(IS=VALUE)", "D1 out 0 device"),
        ("R(RSH=VALUE)", "R1 out 0 device L=1 W=1"),
        ("gain(gain=VALUE)", "A1 out aux device"),
        ("pwl(x_array=[0 1] y_array=[0 VALUE])", "A1 out aux device"),
    ] {
        for value in ["{2+1e-300j}", "{TEMP*0+2+1e-300j}"] {
            let directory = common::test_dir("complex_real_model");
            let deck = directory.join("deck.cir");
            let result = directory.join("result.csv");
            std::fs::write(
                &deck,
                format!(
                    "* real model fields\nV1 out 0 .2\n{instance}\n.MODEL device {}\n.OP\n.END\n",
                    model.replace("VALUE", value)
                ),
            )
            .unwrap();
            for command in ["check", "run"] {
                std::fs::write(&result, "existing result").unwrap();
                let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
                process.args(["--quiet", command]).arg(&deck);
                if command == "run" {
                    process.args(["--format", "csv", "--output"]).arg(&result);
                }
                let output = process.output().unwrap();
                assert!(!output.status.success(), "{command}: {model}, {value}");
                let error = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(error.contains("real value"), "{command}: {error}");
                assert_eq!(std::fs::read_to_string(&result).unwrap(), "existing result");
            }
        }
    }
}

#[test]
fn complex_thermal_material_failure_does_not_publish_partial_results() {
    let directory = common::test_dir("complex_thermal_model");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        "* hot material validation\nV1 out 0 1\nR1 out 0 device L=1 A=1\n\
         .MODEL device R(LEVEL=2 RESISTIVITY=100 HEATCAPACITY={IF(TEMP>27,1u+1e-300j,1u)})\n\
         .TRAN 1m 3m\n.END\n",
    )
    .unwrap();
    std::fs::write(&result, "existing result").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--format", "csv", "--output"])
        .arg(&result)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("HEATCAPACITY") && error.contains("real value"),
        "{error}"
    );
    assert_eq!(std::fs::read_to_string(result).unwrap(), "existing result");
}
