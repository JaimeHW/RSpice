use rspice_core::{Engine, Netlist};

fn assert_resistance(declarations: &str, devices: &str, expected: f64) {
    let source =
        format!("* model temperature dependencies\nV1 out 0 1\n{declarations}\n{devices}\n.END\n");
    let netlist = Netlist::parse(&source).unwrap();
    let current = Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .branch_current_named("V1")
        .unwrap();
    let actual = -1.0 / current;
    assert!(
        (actual - expected).abs() < expected.abs() * 1e-9,
        "expected {expected}, got {actual}: {source}"
    );
}

#[test]
fn hierarchy_keeps_instance_and_model_temperatures_symbolic() {
    for field in ["TEMP", "TNOM"] {
        for declaration in ["inside", "outside"] {
            let model = format!(".MODEL rm R(RSH={{{field}}} TNOM=47)");
            let (root, local) = if declaration == "inside" {
                ("", model.as_str())
            } else {
                (model.as_str(), "")
            };
            assert_resistance(
                &format!("{root}\n.SUBCKT cell a\n{local}\nR1 a 0 rm L=1 W=1 TEMP=47\n.ENDS"),
                "X1 out cell",
                47.0,
            );
        }
    }
}

#[test]
fn scoped_temperature_functions_capture_local_parameters() {
    for quantity in ["TEMP", "TNOM"] {
        assert_resistance(
            &format!(
                ".SUBCKT cell a PARAMS: scale=2\n.FUNC thermal() {{{quantity}*scale}}\n.MODEL rm R(RSH={{thermal()}} TNOM=47)\nR1 a 0 rm L=1 W=1 TEMP=47\n.ENDS"
            ),
            "X1 out cell scale=3",
            141.0,
        );
    }
}

#[test]
fn scoped_vector_functions_capture_local_values_and_keep_temperature() {
    let source = "* scoped vector model\nV1 input 0 .2\n.TEMP 47\n.SUBCKT cell a b PARAMS: scale=2\n.FUNC upper() {TEMP*scale}\n.MODEL lut pwl(X_ARRAY=[0 1] Y_ARRAY=[0 {upper()}])\nA1 a b lut\n.ENDS\nX1 input out cell scale=3\n.END\n";
    let result = Engine::default()
        .run_dc_op(&Netlist::parse(source).unwrap())
        .unwrap();
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        (result.node_voltages[index] - 28.2).abs() < 1e-9,
        "{:?}",
        result.node_voltages
    );
}
