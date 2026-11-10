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

#[test]
fn control_sensitivity_publishes_direct_documents_and_complex_derivative_prints() {
    let dir = common::test_dir("control-sensitivity");
    let config = dir.join("empty.toml");
    std::fs::write(&config, "").unwrap();
    for (domain, sweep) in [("dc", ""), ("ac", " AC LIN 1 1000 1000")] {
        let mut documents = Vec::new();
        for (route, cards) in [
            ("direct", format!(".sens I(V1) R1{sweep}")),
            (
                "explicit",
                format!(
                    ".control\nsens I(V1) R1{sweep}\nlet saved = sens1.R1\nprint saved R1 output\n.endc"
                ),
            ),
            (
                "run",
                format!(".sens I(V1) R1{sweep}\n.control\nrun\nprint R1 output\n.endc"),
            ),
        ] {
            let stem = format!("{domain}-{route}");
            let deck = dir.join(format!("{stem}.sp"));
            std::fs::write(
                &deck,
                format!("Sensitivity\nV1 in 0 DC 1 AC 1\nR1 in 0 1k\n{cards}\n.end\n"),
            )
            .unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .arg("--config")
                .arg(&config)
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "json", "-o"])
                .arg(dir.join(format!("{stem}.json")))
                .output()
                .unwrap();
            assert!(output.status.success(), "{output:?}");
            let suffix = if route == "direct" { "" } else { ".sens-001" };
            let document = rspice_core::execution::AnalysisResultDocument::from_json(
                &std::fs::read_to_string(dir.join(format!("{stem}{suffix}.json"))).unwrap(),
            )
            .unwrap();
            documents.push(document);
            if route != "direct" {
                let print: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(dir.join(format!("{stem}.control-001.json"))).unwrap(),
                )
                .unwrap();
                let traces = print["traces"].as_array().unwrap();
                assert!((traces[0]["y"]["samples"][0][0].as_f64().unwrap() - 1e-6).abs() < 1e-14);
                assert_eq!(traces.last().unwrap()["y"]["unit"], "A");
            }
        }
        // Control execution attaches run coordinates; compare the physical payload.
        assert_eq!(documents[0].payload(), documents[1].payload());
        assert_eq!(documents[0].payload(), documents[2].payload());
    }
}

#[test]
fn invalid_sensitivity_print_keeps_existing_exports() {
    let dir = common::test_dir("control-sensitivity-rollback");
    let deck = dir.join("invalid.sp");
    std::fs::write(
        &deck,
        "Sensitivity\nV1 in 0 1\nR1 in 0 1k\n.control\nsens I(V1) R1\nprint missing\n.endc\n.end\n",
    )
    .unwrap();
    let existing = dir.join("result.sens-001.json");
    std::fs::write(&existing, "previous result\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["-f", "json", "-o"])
        .arg(dir.join("result.json"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read_to_string(existing).unwrap(),
        "previous result\n"
    );
    assert!(!dir.join("result.control-001.json").exists());
}

/// A diode divider at its operating point. Xyce 7.10 writes `.SENS.prn` as
/// `Index V(2) I(V1) {I(V1)} d_{I(V1)}/d_R1:R_adj ... d_{I(V1)}/d_DM:N_adj`:
/// the `.PRINT SENS` columns, the objective, then the adjoint derivatives
/// Xyce prints by default. Its default Newton tolerances leave the
/// derivatives up to 1% off the converged tangent (d/dR2 = 9.01884554e-09);
/// with `.options nonlin deltaxtol=1e-12 reltol=1e-12` it prints the values
/// asserted here, which are the closed-form derivatives of the divider.
/// `DM:N` is a default the model does not author, so its column is omitted
/// with a warning.
#[test]
fn a_xyce_sens_card_writes_xyce_s_table_of_the_objective_and_its_derivatives() {
    let dir = common::test_dir("xyce_sensitivity_table");
    let deck = dir.join("divider.cir");
    std::fs::write(
        &deck,
        "* diode divider\nV1 1 0 DC 3\nR1 1 2 2k\nR2 2 0 1k\nD1 2 0 DM\n.model DM D(IS=1e-12)\n\
         .op\n.print sens v(2) i(v1)\n.SENS objfunc={I(V1)} param=R1:R,R2:R,V1:DCV0,DM:IS,DM:N\n\
         .end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .env("RUST_LOG", "warn")
        .args(["run", "--spice-dialect", "xyce"])
        .arg(&deck)
        .arg("-o")
        .arg(dir.join("out.csv"))
        .args(["-f", "csv"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("DM:N"), "{stderr}");
    let table = std::fs::read_to_string(dir.join("out.sens-001.csv")).unwrap();
    let mut rows = table.lines().filter(|line| !line.starts_with('#'));
    let header = rows.next().unwrap().split(',').collect::<Vec<_>>();
    assert_eq!(
        header,
        [
            "Index",
            "v(2)",
            "i(v1)",
            "{I(V1)}",
            "d_{I(V1)}/d_R1:R_adj",
            "d_{I(V1)}/d_R2:R_adj",
            "d_{I(V1)}/d_V1:DCV0_adj",
            "d_{I(V1)}/d_DM:IS_adj",
        ]
    );
    let values = rows
        .next()
        .unwrap()
        .split(',')
        .map(|value| value.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    assert!(rows.next().is_none(), "an operating point is one row");
    let expected = [
        0.0,
        5.27108047e-01,
        -1.23644598e-03,
        -1.23644598e-03,
        6.07536513e-07,
        9.11148068e-09,
        -4.91357103e-04,
        -1.22614687e+07,
    ];
    for ((name, actual), expected) in header.iter().zip(&values).zip(expected) {
        assert!(
            (actual - expected).abs() <= 1e-6 * expected.abs(),
            "{name}: expected {expected:e}, got {actual:e}"
        );
    }
}

/// A swept, two-objective `.SENS` table is one table in JSON too: the JSON
/// carries the CSV's Index scale and columns, one row per `.DC` point, the
/// way a `.STEP` sweep's JSON does, and Xyce writes the same rows and columns
/// to `.SENS.prn`. On this divider I(V1) = -V1/(R1+R2) and V(2) = V1 R2/(R1+R2),
/// so dI(V1)/dR1 = V1/9e6 and dV(2)/dR1 = -V1/9000 per ohm.
#[test]
fn a_swept_xyce_sens_table_with_two_objectives_writes_json() {
    let dir = common::test_dir("xyce_sensitivity_json");
    let deck = dir.join("divider.cir");
    std::fs::write(
        &deck,
        "* divider\nV1 1 0 DC 3\nR1 1 2 2k\nR2 2 0 1k\n.dc V1 1 3 1\n\
         .SENS objfunc={I(V1)},{V(2)} param=R1:R\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run", "--spice-dialect", "xyce"])
        .arg(&deck)
        .arg("-o")
        .arg(dir.join("out.json"))
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let document: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("out.sens-001.json")).unwrap())
            .unwrap();
    assert_eq!(document["scale"]["name"], "Index");
    assert_eq!(
        document["scale"]["values"],
        serde_json::json!([0.0, 1.0, 2.0])
    );
    let column = |name: &str| {
        document["signals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|signal| signal["name"] == name)
            .unwrap_or_else(|| panic!("no column {name} in {document}"))["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>()
    };
    let sources = [1.0, 2.0, 3.0];
    for (name, expected) in [
        ("{I(V1)}", sources.map(|v1| -v1 / 3.0e3)),
        ("d_{I(V1)}/d_R1:R_adj", sources.map(|v1| v1 / 9.0e6)),
        ("{V(2)}", sources.map(|v1| v1 / 3.0)),
        ("d_{V(2)}/d_R1:R_adj", sources.map(|v1| -v1 / 9.0e3)),
    ] {
        let actual = column(name);
        assert_eq!(actual.len(), expected.len(), "{name}");
        for (actual, expected) in actual.iter().zip(expected) {
            assert!(
                (actual - expected).abs() <= 1e-9 * expected.abs(),
                "{name}: expected {expected:e}, got {actual:e}"
            );
        }
    }
}
