use rspice_core::{Engine, Netlist};

const REAL_MODELS: [(&str, &str); 6] = [
    ("R(RSH=VALUE)", "R1 out 0 device L=1 W=1"),
    ("C(CJ=VALUE)", "C1 out 0 device L=1 W=1"),
    ("L(L=VALUE)", "L1 out 0 device"),
    ("SW(RON=VALUE)", "S1 out 0 out 0 device"),
    ("gain(gain=VALUE)", "A1 out aux device"),
    ("pwl(x_array=[0 1] y_array=[0 VALUE])", "A1 out aux device"),
];

#[test]
fn deferred_real_model_fields_reject_even_tiny_imaginary_components() {
    for (model, instance) in REAL_MODELS {
        for expression in ["{TEMP*0+2+sqrt(-1)}", "{TEMP*0+2+1e-300j}"] {
            let source = format!(
                "* real model fields\nV1 out 0 1\n{instance}\n.MODEL device {}\n.END\n",
                model.replace("VALUE", expression)
            );
            let netlist = Netlist::parse(&source).unwrap();
            let error = Engine::default().build_circuit(&netlist).err();
            assert!(error.is_some(), "accepted {model}: {expression}");
            let message = error.unwrap().to_string();
            assert!(message.contains("real value"), "{model}: {message}");
        }
    }
}

#[test]
fn real_model_fields_accept_explicit_complex_projections() {
    for (model, instance) in REAL_MODELS {
        for expression in ["{TEMP*0+IMG(1+2j)}", "{TEMP*0+RE(2+1j)}"] {
            let source = format!(
                "* explicit projection\nV1 out 0 1\n{instance}\n.MODEL device {}\n.END\n",
                model.replace("VALUE", expression)
            );
            let netlist = Netlist::parse(&source).unwrap();
            Engine::default().build_circuit(&netlist).unwrap();
        }
    }
}

#[test]
fn thermal_updates_reject_complex_material_without_changing_accepted_values() {
    let source = "* thermal material validation\n\
        .MODEL rm R(LEVEL=2 RESISTIVITY={IF(TEMP>27,2+sqrt(-1),100)} HEATCAPACITY=1)\n\
        R1 out 0 rm L=1 A=1\n.END\n";
    let netlist = Netlist::parse(source).unwrap();
    let circuit = Engine::default().build_circuit(&netlist).unwrap();
    let mut state = circuit.resistor_storage().thermal[0].clone().unwrap();
    let original_resistivity = state.resistivity;
    let original_temperature = state.temperature_celsius;
    let error = state.update_material_at_temperature(47.0).unwrap_err();
    assert!(
        error.contains("RESISTIVITY") && error.contains("real value"),
        "{error}"
    );
    assert_eq!(state.resistivity, original_resistivity);
    assert_eq!(state.temperature_celsius, original_temperature);
}
