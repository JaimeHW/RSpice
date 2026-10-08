mod common;

use std::process::Command;

fn run(body: &str, analysis: &str) -> String {
    let directory = common::test_dir("resistor_evaluation");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        format!("* resistor evaluation\n.OPTIONS SEED=37\n{body}\n{analysis}\n.END\n"),
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
        "{body}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::read_to_string(result).unwrap()
}

#[test]
fn resistor_instances_preserve_seeded_samples_across_construction_paths() {
    let mut context = rspice_core::netlist::ParamContext::new();
    context.set_random_seed(37);
    let expected: Vec<_> = (0..2)
        .map(|_| rspice_core::netlist::expr::eval_expression("aunif(100,1)", &context).unwrap())
        .collect();
    for (model, instance) in [
        ("RSH={TEMP*0+aunif(100,1)}", "rm L=1 W=1"),
        ("RSH={TEMP*0+aunif(100,1)} KF={RSH}", "rm L=1 W=1"),
        ("R={TEMP*0+aunif(100,1)}", "R={1+0*V(a)} rm"),
        (
            "LEVEL=2 RESISTIVITY={TEMP*0+aunif(100,1)} HEATCAPACITY={RESISTIVITY}",
            "rm L=1 A=1",
        ),
    ] {
        for tolerance in [0, 1000] {
            let csv = run(
                &format!(
                    ".OPTIONS DEVICE ZERORESISTANCETOL={tolerance}\n.MODEL rm R({model})\n\
                     V1 a 0 1\nV2 b 0 1\nR1 a 0 {instance}\nR2 b 0 {instance}"
                ),
                ".OP",
            );
            for (name, expected) in ["I(V1)", "I(V2)"].into_iter().zip(&expected) {
                let current = common::delimited_data_text(&csv, b',')
                    .lines()
                    .filter_map(|line| line.split_once(','))
                    .find(|(signal, _)| signal.eq_ignore_ascii_case(name))
                    .unwrap_or_else(|| panic!("{csv}"))
                    .1
                    .parse::<f64>()
                    .unwrap();
                let actual = -1.0 / current;
                assert!(
                    (actual - expected).abs() < 1e-8,
                    "{model}, {instance}, {name}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn thermal_transient_retains_the_resolved_nominal_temperature() {
    let csv = run(
        "V1 out 0 1\nR1 out 0 rm L=1 A=1\n\
         .MODEL rm R(LEVEL=2 TNOM={TNOM+20} RESISTIVITY={TNOM} HEATCAPACITY=1)",
        ".TRAN 1m 3m",
    );
    let mut lines = common::delimited_data_text(&csv, b',').lines();
    let headers: Vec<_> = lines.next().unwrap().split(',').collect();
    let column = |signal: &str| {
        headers
            .iter()
            .position(|name| name.eq_ignore_ascii_case(signal))
            .unwrap_or_else(|| panic!("{csv}"))
    };
    let source_column = column("I(V1)");
    let resistor_column = column("I(R1)");
    let mut samples = 0;
    for row in lines {
        let values: Vec<_> = row.split(',').collect();
        let source_current = values[source_column].parse::<f64>().unwrap();
        let resistor_current = values[resistor_column].parse::<f64>().unwrap();
        assert!((source_current + 1.0 / 47.0).abs() < 1e-10, "{row}");
        assert!((resistor_current - 1.0 / 47.0).abs() < 1e-10, "{row}");
        samples += 1;
    }
    assert!(samples > 2, "{csv}");
}
