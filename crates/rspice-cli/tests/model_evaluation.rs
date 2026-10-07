mod common;

use std::process::Command;

fn resistance(declarations: &str, fields: &str, dimensions: &str) -> f64 {
    let directory = common::test_dir("model_evaluation");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    let source = format!(
        "* model evaluation\n.OPTIONS SEED=37\nV1 out 0 1\n{declarations}\n.MODEL rm R({fields})\n.PARAM later=2\n.FUNC late() {{2}}\nR1 out 0 rm {dimensions}\n.OP\n.END\n"
    );
    std::fs::write(&deck, &source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--format", "csv", "--output"])
        .arg(&result)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{source}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let csv = std::fs::read_to_string(result).unwrap();
    let current: f64 = csv
        .lines()
        .filter_map(|line| line.split_once(','))
        .find(|(name, _)| name.eq_ignore_ascii_case("I(V1)"))
        .unwrap_or_else(|| panic!("{csv}"))
        .1
        .parse()
        .unwrap();
    -1.0 / current
}

#[test]
fn model_field_order_and_resolution_time_preserve_deck_bindings() {
    for declaration in [".PARAM DEFW=1", ".GLOBAL_PARAM DEFW=1"] {
        for fields in [
            "RSH={100*DEFW} DEFW=2",
            "RSH={TEMP*0+100*DEFW} DEFW=2",
            "RSH={TEMP*0+100*DEFW} DEFW={TEMP*0+2}",
            "DEFW={TEMP*0+2} RSH={TEMP*0+100*DEFW}",
            "DEFW={later} RSH={100*DEFW+later*0}",
            "RSH={100*DEFW+later*0} DEFW={later}",
        ] {
            let actual = resistance(declaration, fields, "L=2");
            assert!(
                (actual - 100.0).abs() < 1e-8,
                "{declaration}: {fields}: {actual}"
            );
        }
    }
}

#[test]
fn nominal_temperature_is_bound_once() {
    for fields in [
        "TNOM={TNOM+20} RSH={TNOM}",
        "RSH={TNOM} TNOM={TNOM+20}",
        "RSH={TNOM} TNOM={DEFW+20} DEFW={TEMP}",
    ] {
        let actual = resistance("", fields, "L=1 W=1");
        assert!((actual - 47.0).abs() < 1e-8, "{fields}: {actual}");
    }
}

#[test]
fn model_dependencies_and_nominal_temperature_preserve_seeded_samples() {
    let expected = resistance("", "RSH={TEMP*0+aunif(100,1)}", "L=1 W=1");
    for fields in [
        "RSH={TEMP*0+aunif(100,1)} TNOM=47",
        "RSH={TEMP*0+aunif(100,1)} TNOM={TNOM+20}",
        "RSH={TEMP*0+aunif(100,1)+0*DEFW} DEFW={TEMP}",
        "DEFW={TEMP} RSH={TEMP*0+aunif(100,1)+0*DEFW}",
        "RSH={aunif(100,1)+0*DEFW} DEFW={later}",
        "DEFW={later} RSH={aunif(100,1)+0*DEFW}",
        "RSH={aunif(100,1)+0*late()}",
    ] {
        assert_eq!(resistance("", fields, "L=1 W=1"), expected, "{fields}");
    }
}
