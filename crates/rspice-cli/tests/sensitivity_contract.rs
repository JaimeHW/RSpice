mod common;

use std::process::Command;

#[test]
fn parameter_sensitivity_uses_one_nominal_point_in_every_presentation() {
    let dir = common::test_dir("sensitivity_nominal");
    let deck = dir.join("divider.sp");
    std::fs::write(&deck,
        "* analytic divider\n.param rload=1000\nV1 in 0 10\nR1 in out 1000\nR2 out 0 {rload}\n.op\n.end\n"
    ).unwrap();
    let config = dir.join("empty.toml");
    std::fs::write(&config, "").unwrap();
    for nominal in [None, Some(2000.0)] {
        let result = dir.join("sensitivity.json");
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command
            .arg("--config")
            .arg(&config)
            .args(["--verbose", "run"])
            .arg(&deck)
            .args([
                "--sens-output",
                "out",
                "--sens-param",
                "rload",
                "-f",
                "json",
                "-o",
            ])
            .arg(&result);
        if let Some(value) = nominal {
            command.args(["--sens-value", &value.to_string()]);
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        let document: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&result).unwrap()).unwrap();
        let resistance = nominal.unwrap_or(1000.0);
        let voltage = 10.0 * resistance / (1000.0 + resistance);
        let derivative = 10000.0 / (1000.0_f64 + resistance).powi(2);
        let relative = 1000.0 / (1000.0 + resistance);
        let entry = &document["payload"]["entries"][0];
        assert_eq!(entry["nominalValue"].as_f64(), Some(resistance));
        assert!((entry["absolute"].as_f64().unwrap() - derivative).abs() < 1e-10);
        assert!((entry["normalized"].as_f64().unwrap() - relative).abs() < 1e-8);
        let scalar = document["scalars"]
            .as_array()
            .unwrap()
            .iter()
            .find(|scalar| scalar["name"] == "output_value")
            .unwrap();
        assert!((scalar["value"]["value"].as_f64().unwrap() - voltage).abs() < 1e-8);
        assert_eq!(scalar["unit"]["unit"], "volt");
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains(&format!("Normalized: {relative:.6e}% change per 1%")),
            "{stdout}"
        );
    }
}

#[test]
fn sensitivity_rejects_an_undefined_nominal_parameter() {
    let dir = common::test_dir("sensitivity_undefined");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* op\nV1 in 0 1\nR1 in 0 1k\n.end\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(deck)
        .args(["--sens-output", "in", "--sens-param", "missing"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("undefined sensitivity parameter")
    );
}

#[test]
fn authored_sensitivity_documents_keep_voltage_and_current_output_units() {
    let dir = common::test_dir("sensitivity_units");
    let deck = dir.join("deck.sp");
    let path = dir.join("sensitivity.json");
    let config = dir.join("empty.toml");
    std::fs::write(&config, "").unwrap();
    for (probe, unit, expected) in [("V(in)", "volt", 1.0), ("I(V1)", "ampere", -0.001)] {
        for sweep in ["", " AC LIN 1 1000 1000"] {
            std::fs::write(
                &deck,
                format!(
                    "Nominal units\nV1 in 0 DC 1 AC 1\nR1 in 0 1k\n.sens {probe} R1{sweep}\n.end\n"
                ),
            )
            .unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .arg("--config")
                .arg(&config)
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "json", "-o"])
                .arg(&path)
                .output()
                .unwrap();
            assert!(output.status.success(), "{output:?}");
            let result: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let nominal = if sweep.is_empty() {
                assert_eq!(result["scalars"][0]["unit"]["unit"], unit);
                result["scalars"][0]["value"]["value"].as_f64().unwrap()
            } else {
                assert_eq!(result["signals"][0]["descriptor"]["unit"]["unit"], unit);
                result["signals"][0]["values"]["samples"][0]["real"]
                    .as_f64()
                    .unwrap()
            };
            assert!(
                (nominal - expected).abs() < 1e-10,
                "{nominal} != {expected}"
            );
        }
    }
}
