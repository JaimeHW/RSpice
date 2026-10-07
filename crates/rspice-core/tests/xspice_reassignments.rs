use rspice_core::{Engine, Netlist, netlist::ElementKind};

#[test]
fn last_scalar_assignment_wins_in_root_and_scoped_instances() {
    for (fields, expected) in [
        ("gain=1 gain=2", 2.0),
        ("gain={later} GAIN=2", 2.0),
        ("gain=1 gain=2 gain={later}", 3.0),
        ("gain={missing} gain=2", 2.0),
    ] {
        for scoped in [false, true] {
            let mut body = format!("A1 in out gain {fields}");
            if scoped {
                body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
            }
            let source = format!("* repeated fields\nV1 in 0 1\n{body}\n.PARAM later=3\n.END\n");
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
fn reassignment_removes_previous_values_across_typed_storage() {
    // Check every old/new representation; field type validation belongs to
    // circuit construction, after the winning assignment has been selected.
    for name in ["real", "string", "string_array"] {
        for before in [
            "1",
            "{missing}",
            "\"text\"",
            "<1 2>",
            "<{missing} 2>",
            "[1 2]",
            "[1 {missing}]",
            "[\"a\" \"b\"]",
        ] {
            for after in [
                "2",
                "{later}",
                "\"new\"",
                "<3 4>",
                "<{later} 4>",
                "[3 4]",
                "[3 {later}]",
                "[\"c\"]",
            ] {
                let source = format!(
                    "* repeated typed fields\nA1 [in] print_param_types {name}={before} {}={after}\n.END\n",
                    name.to_uppercase()
                );
                let netlist = Netlist::parse(&source).unwrap();
                let ElementKind::Xspice {
                    params,
                    expr_params,
                    string_params,
                    string_expr_params,
                    string_vector_params,
                    string_vector_expr_params,
                    real_vector_params,
                    real_vector_expr_params,
                    ..
                } = &netlist.elements[0].kind
                else {
                    panic!("expected XSPICE");
                };
                let count = params.len()
                    + expr_params.len()
                    + string_params.len()
                    + string_expr_params.len()
                    + string_vector_params.len()
                    + string_vector_expr_params.len()
                    + real_vector_params.len()
                    + real_vector_expr_params.len();
                assert_eq!(count, 1, "{source}");
                let expected = Netlist::parse(&format!(
                    "* final typed field\nA1 [in] print_param_types {name}={after}\n.END\n"
                ))
                .unwrap();
                assert_eq!(
                    format!("{:?}", netlist.elements[0].kind),
                    format!("{:?}", expected.elements[0].kind),
                    "{source}"
                );
            }
        }
    }
}
