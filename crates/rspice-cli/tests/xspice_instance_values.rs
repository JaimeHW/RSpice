mod common;

use std::{path::Path, process::Command};

fn csv_voltage(path: &Path, node: &str) -> f64 {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| line.split_once(','))
        .find(|(name, _)| name.eq_ignore_ascii_case(&format!("V({node})")))
        .unwrap()
        .1
        .parse()
        .unwrap()
}

#[test]
fn check_and_run_reject_non_real_instance_fields_without_replacing_results() {
    for instance in [
        "A1 in out gain gain={z}",
        "A1 in out gain gain=1j",
        "A1 [in] print_param_types real_array=[0 {z}]",
        "A1 [in] print_param_types complex=<{z} 1>",
        "A1 [in] print_param_types complex_array=[<1 {z}>]",
    ] {
        for scoped in [false, true] {
            let directory = common::test_dir("real_instance_fields");
            let deck = directory.join("deck.cir");
            let result = directory.join("result.csv");
            let body = if scoped {
                format!(
                    ".SUBCKT cell in PARAMS: z=0\n{instance}\n.ENDS\nX1 in cell z={{2+1e-300j}}"
                )
            } else {
                format!(".PARAM z={{2+1e-300j}}\n{instance}")
            };
            std::fs::write(
                &deck,
                format!("* real fields\nV1 in 0 1\n{body}\n.OP\n.END\n"),
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
                assert!(
                    !output.status.success(),
                    "{instance}: {command}, scoped={scoped}"
                );
                let diagnostic = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(diagnostic.contains("real value"), "{diagnostic}");
                assert_eq!(std::fs::read_to_string(&result).unwrap(), "existing result");
            }
        }
    }
}

#[test]
fn forward_xspice_reference_preserves_the_seeded_gain_of_another_instance() {
    let directory = common::test_dir("instance_forward_sample");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        "* independent sampled field\n.OPTIONS SEED=37\nV1 in 0 1\n\
         A1 in unused gain gain={aunif(100,1)+later}\n.PARAM later=0\n\
         .PARAM marker={aunif(100,1)}\nA2 in out gain gain={marker}\n.OP\n.END\n",
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
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    let expected = rspice_core::netlist::expr::eval_expression("aunif(100,1)", &reference).unwrap();
    let actual = csv_voltage(&result, "out");
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn signed_instance_expressions_preserve_precedence_in_published_results() {
    let body = "A1 in out gain gain=-z+3\nA2 in grouped gain gain=-{z+3}";
    for devices in [
        format!(".PARAM z=2\n{body}"),
        format!("{body}\n.PARAM z=2"),
        format!(
            ".SUBCKT cell in out grouped PARAMS: z=0\n{body}\n.ENDS\nX1 in out grouped cell z=2"
        ),
    ] {
        let directory = common::test_dir("signed_instance_fields");
        let deck = directory.join("deck.cir");
        let result = directory.join("result.csv");
        std::fs::write(
            &deck,
            format!("* signed expressions\nV1 in 0 1\n{devices}\n.OP\n.END\n"),
        )
        .unwrap();
        for command in ["check", "run"] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
            process.args(["--quiet", command]).arg(&deck);
            if command == "run" {
                process.args(["--format", "csv", "--output"]).arg(&result);
            }
            let output = process.output().unwrap();
            assert!(
                output.status.success(),
                "{devices}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        assert!((csv_voltage(&result, "out") - 1.0).abs() < 1e-12);
        assert!((csv_voltage(&result, "grouped") + 5.0).abs() < 1e-12);
    }
}
