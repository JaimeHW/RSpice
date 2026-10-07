use rspice_core::{Engine, Netlist};

fn deck(fields: &str, definitions: &str, scoped: bool) -> String {
    let mut body = format!("A1 in out gain {fields}");
    if scoped {
        body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
    }
    format!(
        "* retained sibling bindings\n.OPTIONS SEED=37\nV1 in 0 1\n{body}\n{definitions}\n.END\n"
    )
}

#[test]
fn retained_global_definitions_do_not_freeze_pending_sibling_bindings() {
    for fields in [
        "gain={in_offset+1} in_offset={later}",
        "gain={field_gain()} in_offset={later}",
        "in_offset={later} gain={in_offset+1}",
        "gain={in_offset+1} in_offset={in_offset+2}",
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3", scoped).replace(
                ".OPTIONS SEED=37",
                ".OPTIONS SEED=37\n.PARAM base=1\n.GLOBAL_PARAM in_offset={base}\n.FUNC field_gain() {in_offset+1}",
            );
            let actual = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap()
                .try_voltage_named("out")
                .unwrap();
            assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
        }
    }
}
