use rspice_core::{Engine, Netlist};

fn resistance(source: &str) -> f64 {
    let netlist = Netlist::parse(&format!(
        "* model parameter scope\nV1 out 0 1\n{source}\n.END\n"
    ))
    .unwrap();
    -1.0 / Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .branch_current_named("V1")
        .unwrap()
}

#[test]
fn deck_bindings_do_not_change_with_model_field_order_or_resolution_time() {
    for directive in [".PARAM", ".GLOBAL_PARAM"] {
        for fields in [
            "RSH={100*DEFW} DEFW=2",
            "RSH={TEMP*0+100*DEFW} DEFW=2",
            "RSH={TEMP*0+100*DEFW} DEFW={TEMP*0+2}",
            "DEFW={TEMP*0+2} RSH={TEMP*0+100*DEFW}",
            "RSH={100*DEFW+later*0} DEFW={later}",
            "DEFW={later} RSH={100*DEFW+later*0}",
            "DEFW=2 RSH={100*DEFW+later*0}",
            "DEFW={TEMP*0+2} RSH={sheet()}",
        ] {
            let source = format!(
                "{directive} DEFW=1\n.FUNC sheet() {{TEMP*0+100*DEFW}}\n.MODEL rm R({fields})\n.PARAM later=2\nR1 out 0 rm L=2"
            );
            let actual = resistance(&source);
            assert!((actual - 100.0).abs() < 1e-8, "{fields}: {actual}");
        }
    }
}

#[test]
fn model_fields_fill_missing_bindings_without_leaking_to_the_deck() {
    for fields in [
        "RSH={100*DEFW} DEFW=2",
        "DEFW=2 RSH={100*DEFW}",
        "RSH={TEMP*0+100*DEFW} DEFW={TEMP*0+2}",
        "DEFW={TEMP*0+2} RSH={TEMP*0+100*DEFW}",
        "RSH={100*DEFW} DEFW={later}",
    ] {
        let source = format!(".MODEL rm R({fields})\n.PARAM later=2\nR1 out 0 rm L=2");
        let actual = resistance(&source);
        assert!((actual - 200.0).abs() < 1e-8, "{fields}: {actual}");
        let netlist = Netlist::parse(&format!("* scope\n{source}\n.END\n")).unwrap();
        assert_eq!(netlist.params.get("DEFW"), None);
    }
}

#[test]
fn model_self_references_keep_the_enclosing_parameter_binding() {
    for fields in ["RSH={RSH*2}", "RSH={TEMP*0+RSH*2}"] {
        let source = format!(".PARAM RSH=50\n.MODEL rm R({fields})\nR1 out 0 rm L=1 W=1");
        let actual = resistance(&source);
        assert!((actual - 100.0).abs() < 1e-8, "{fields}: {actual}");
    }
}

#[test]
fn scoped_model_fields_do_not_replace_instance_parameters() {
    let source = ".PARAM DEFW=1\n.SUBCKT cell a PARAMS: DEFW=3\n.MODEL rm R(DEFW={TEMP*0+2} RSH={TEMP*0+100*DEFW})\nR1 a 0 rm L=2\n.ENDS\nX1 out cell DEFW=4";
    assert!((resistance(source) - 400.0).abs() < 1e-8);
}

#[test]
fn xspice_nominal_temperature_still_comes_from_the_model_card() {
    let source = "* code model nominal temperature\nV1 input 0 1\n.MODEL gm gain(GAIN={TNOM} TNOM=47)\nA1 input out gm\n.END\n";
    let result = Engine::default()
        .run_dc_op(&Netlist::parse(source).unwrap())
        .unwrap();
    assert!((result.try_voltage_named("out").unwrap() - 47.0).abs() < 1e-8);
}
