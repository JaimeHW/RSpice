use rspice_core::{Engine, Netlist};

#[test]
fn scoped_instance_expressions_read_model_defaults_like_root_instances() {
    for (model, fields, enclosing, expected) in [
        ("gain=3", "gain={gain*2}", "", 6.0),
        ("in_offset=3", "gain={in_offset+1}", "", 16.0),
        ("gain=3", "gain={gain*2}", ".PARAM gain=9", 18.0),
        ("gain={later}", "gain={gain*2}", "", 6.0),
        ("gain={TEMP}", "gain={gain*2}", ".TEMP 47", 94.0),
        ("gain={TNOM} TNOM={later+30}", "gain={gain*2}", "", 66.0),
    ] {
        for scoped in [false, true] {
            for local_model in [false, true] {
                let card = format!(".MODEL alias gain({model})");
                let device = format!("A1 in out alias {fields}");
                let body = if scoped {
                    format!(
                        "{}\n.SUBCKT cell in out\n{}\n{device}\n.ENDS\nX1 in out cell",
                        if local_model { "" } else { &card },
                        if local_model { &card } else { "" }
                    )
                } else {
                    format!("{card}\n{device}")
                };
                let source = format!(
                    "* scoped defaults\n{enclosing}\nV1 in 0 1\n{body}\n.PARAM later=3\n.END\n"
                );
                let result = Engine::default()
                    .run_dc_op(&Netlist::parse(&source).unwrap())
                    .unwrap();
                let actual = result.try_voltage_named("out").unwrap();
                assert!(
                    (actual - expected).abs() < 1e-10,
                    "{source}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn scoped_vector_expressions_can_use_scalar_model_defaults() {
    for vector in ["[0 {input_domain}]", "\"[0 {input_domain}]\""] {
        for scoped in [false, true] {
            let mut body = format!("A1 in out lookup y_array={vector}");
            if scoped {
                body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
            }
            let source = format!(
                "* vector model fallback\nV1 in 0 .5\n.MODEL lookup pwl(x_array=[0 1] input_domain=.01 fraction=false)\n{body}\n.END\n"
            );
            let actual = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap()
                .try_voltage_named("out")
                .unwrap();
            assert!((actual - 0.005).abs() < 1e-12, "{source}: {actual}");
        }
    }
}
