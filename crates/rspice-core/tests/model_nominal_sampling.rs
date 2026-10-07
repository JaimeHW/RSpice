use rspice_core::{Engine, Netlist};

fn resistance(fields: &str) -> f64 {
    let source = format!(
        "* model nominal sampling\n.OPTIONS SEED=37\nV1 out 0 1\n.MODEL rm R({fields})\nR1 out 0 rm L=1 W=1\n.END\n"
    );
    -1.0 / Engine::default()
        .run_dc_op(&Netlist::parse(&source).unwrap())
        .unwrap()
        .branch_current_named("V1")
        .unwrap()
}

#[test]
fn unused_nominal_temperature_does_not_resample_model_values() {
    let expected = resistance("RSH={TEMP*0+aunif(100,1)}");
    for nominal in ["27", "47", "{TEMP*0+47}", "{TNOM+20}"] {
        let actual = resistance(&format!("RSH={{TEMP*0+aunif(100,1)}} TNOM={nominal}"));
        assert_eq!(actual, expected, "TNOM={nominal}");
    }
}

#[test]
fn nominal_expressions_are_bound_once_before_other_model_fields() {
    for fields in [
        "RSH={TNOM} TNOM={TNOM+20}",
        "TNOM={TNOM+20} RSH={TNOM}",
        "RSH={TNOM} TNOM={DEFW+20} DEFW={TEMP}",
        "DEFW={TEMP} TNOM={DEFW+20} RSH={TNOM}",
    ] {
        let actual = resistance(fields);
        assert!((actual - 47.0).abs() < 1e-8, "{fields}: {actual}");
    }
}

#[test]
fn nominal_dependencies_are_not_sampled_again_when_other_fields_resolve() {
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    let expected = rspice_core::netlist::expr::eval_expression("aunif(40,1)", &reference).unwrap();
    for fields in [
        "TNOM={DEFW} DEFW={TEMP*0+aunif(40,1)} RSH={DEFW}",
        "RSH={DEFW} TNOM={DEFW} DEFW={TEMP*0+aunif(40,1)}",
    ] {
        let actual = resistance(fields);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{fields}: {actual} != {expected}"
        );
    }
}

#[test]
fn forward_model_references_do_not_replay_an_earlier_statistical_call() {
    let expected = resistance("RSH={TEMP*0+aunif(100,1)}");
    for fields in [
        "RSH={TEMP*0+aunif(100,1)+0*DEFW} DEFW={TEMP}",
        "DEFW={TEMP} RSH={TEMP*0+aunif(100,1)+0*DEFW}",
    ] {
        assert_eq!(resistance(fields), expected, "{fields}");
    }
}
