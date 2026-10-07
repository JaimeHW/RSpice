use rspice_core::{Engine, Netlist, netlist::ElementKind};

fn decks(body: &str) -> [String; 3] {
    [
        format!("* root\n.PARAM z=2\n{body}\n.END\n"),
        format!("* forward\n{body}\n.PARAM z=2\n.END\n"),
        format!(
            "* scoped\n.SUBCKT cell in out PARAMS: z=0\n{body}\n.ENDS\nX1 in out cell z=2\n.END\n"
        ),
    ]
}

#[test]
fn scalar_signs_preserve_expression_precedence() {
    for (expression, expected) in [
        ("-z+3", 1.0),
        ("-z-3", -5.0),
        ("-sin(z)+3", 3.0 - 2.0_f64.sin()),
        ("-(z)+3", 1.0),
        ("-{z}+3", 1.0),
        ("-{z+3}", -5.0),
        ("- z+3", 1.0),
        ("+z+3", 5.0),
        ("-z", -2.0),
        ("-true", -1.0),
        ("-2+3", 1.0),
    ] {
        for source in decks(&format!("V1 in 0 1\nA1 in out gain gain={expression}")) {
            let netlist = Netlist::parse(&source).unwrap();
            let result = Engine::default().run_dc_op(&netlist).unwrap();
            let actual = result.try_voltage_named("out").unwrap();
            assert!(
                (actual - expected).abs() < 1e-12,
                "{source}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn vector_and_complex_component_signs_preserve_expression_precedence() {
    let [root, _, scoped] = decks(
        "A1 [in] print_param_types real_array=[-z+3 -{z+3} - z+3 -true] \
         complex=<-z+3 -{z}+3> complex_array=[<-sin(z)+3 -{z+3}>]",
    );
    for source in [root, scoped] {
        let netlist = Netlist::parse(&source).unwrap();
        let flattened = rspice_core::netlist::flatten_netlist_with_models(&netlist).unwrap();
        let ElementKind::Xspice {
            real_vector_params,
            string_params,
            string_vector_params,
            ..
        } = &flattened.elements[0].kind
        else {
            panic!("expected XSPICE element");
        };
        assert_eq!(real_vector_params[0].1, [1.0, -5.0, 1.0, -1.0], "{source}");
        assert_eq!(string_params[0].1, "<1 1>", "{source}");
        assert_eq!(
            string_vector_params[0].1,
            [format!("<{} -5>", 3.0 - 2.0_f64.sin())],
            "{source}"
        );
        Engine::default().build_circuit(&netlist).unwrap();
    }
}
