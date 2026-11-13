use rspice_core::{Engine, Netlist};

fn thermal_state(declarations: &str, fields: &str) -> rspice_core::circuit::ThermalResistorState {
    let source = format!(
        "* thermal material evaluation\n.OPTIONS SEED=37\n{declarations}\n\
         .MODEL rm R(LEVEL=2 {fields})\nR1 a 0 rm L=1 A=1\n.END\n"
    );
    let circuit = Engine::default()
        .build_circuit(&Netlist::parse(&source).unwrap())
        .unwrap();
    circuit.resistor_storage().thermal[0].clone().unwrap()
}

#[test]
fn material_fields_retain_enclosing_bindings_after_temperature_updates() {
    for declaration in [".PARAM RESISTIVITY=2", ".GLOBAL_PARAM RESISTIVITY=2"] {
        for fields in [
            "RESISTIVITY={TEMP*0+3} HEATCAPACITY={TEMP*0+RESISTIVITY}",
            "HEATCAPACITY={TEMP*0+RESISTIVITY} RESISTIVITY={TEMP*0+3}",
        ] {
            let mut state = thermal_state(declaration, fields);
            for temperature in [27.0, 47.0] {
                state.update_material_at_temperature(temperature).unwrap();
                assert_eq!(state.resistivity, 3.0);
                assert_eq!(state.heat_capacity, 2.0, "{declaration}: {fields}");
            }
        }
    }
}

#[test]
fn nominal_temperature_is_not_applied_again_by_material_updates() {
    for fields in [
        "TNOM={TNOM+20} RESISTIVITY={TNOM} HEATCAPACITY=1",
        "RESISTIVITY={TNOM} HEATCAPACITY=1 TNOM={TNOM+20}",
    ] {
        let mut state = thermal_state("", fields);
        assert_eq!(state.tnom_celsius, 47.0);
        for temperature in [27.0, 47.0] {
            state.update_material_at_temperature(temperature).unwrap();
            assert_eq!(state.resistivity, 47.0, "{fields}");
        }
    }
}

#[test]
fn thermal_material_reads_share_the_instances_sample() {
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    let expected = rspice_core::netlist::expr::eval_expression("aunif(100,1)", &reference).unwrap();
    let state = thermal_state(
        "",
        "RESISTIVITY={TEMP*0+aunif(100,1)} HEATCAPACITY={RESISTIVITY}",
    );
    assert_eq!(state.resistivity, expected);
    assert_eq!(state.heat_capacity, expected);
}

#[test]
fn implicit_engine_defaults_do_not_shadow_model_fields_during_updates() {
    let mut state = thermal_state("", "GMIN={TEMP*0+2} RESISTIVITY={GMIN} HEATCAPACITY=1");
    assert_eq!(state.resistivity, 2.0);
    state.update_material_at_temperature(47.0).unwrap();
    assert_eq!(state.resistivity, 2.0);
}

#[test]
fn forward_material_dependencies_follow_the_updated_temperature() {
    let mut state = thermal_state("", "RESISTIVITY={HEATCAPACITY*2} HEATCAPACITY={TEMP+10}");
    assert_eq!(state.resistivity, 74.0);
    state.update_material_at_temperature(47.0).unwrap();
    assert_eq!(state.resistivity, 114.0);
    assert_eq!(state.heat_capacity, 57.0);
}
