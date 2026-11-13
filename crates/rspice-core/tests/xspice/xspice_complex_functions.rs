use rspice_core::{
    Engine, Netlist,
    netlist::{ElementKind, flatten_netlist_with_models},
};

#[test]
fn complex_components_accept_function_commas_and_grouped_whitespace() {
    for (real, imag, definitions) in [
        ("min(2,3)", "max(4,5)", ""),
        ("min( 2 , 3 )", "max(2, max(4, 5))", ""),
        ("(1 + 1)", "(10 / 2)", ""),
        (
            "later(2, 3)",
            "max(4, later(5, 6))",
            ".FUNC later(x,y) {min(x,y)}",
        ),
    ] {
        for quoted in [false, true] {
            let vector = format!("[<{real} {imag}>]");
            let vector = if quoted {
                format!("\"{vector}\"")
            } else {
                vector
            };
            let body =
                format!("A1 [in] print_param_types complex=<{real} {imag}> complex_array={vector}");
            for scoped in [false, true] {
                let body = if scoped {
                    format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell")
                } else {
                    body.clone()
                };
                let source = format!("* complex function syntax\n{body}\n{definitions}\n.END\n");
                let netlist = Netlist::parse(&source).unwrap();
                Engine::default().build_circuit(&netlist).unwrap();
                // Forward root expressions intentionally stay deferred until construction.
                if definitions.is_empty() || scoped {
                    let flat = flatten_netlist_with_models(&netlist).unwrap();
                    let ElementKind::Xspice {
                        string_params,
                        string_vector_params,
                        ..
                    } = &flat.elements[0].kind
                    else {
                        panic!("expected XSPICE");
                    };
                    assert_eq!(string_params[0].1, "<2 5>", "{source}");
                    assert_eq!(string_vector_params[0].1, ["<2 5>"], "{source}");
                }
            }
        }
    }
}

#[test]
fn whitespace_must_not_join_separate_operands_inside_parentheses() {
    for expression in ["min(2 3)", "(2 3)", "min(2, 3 4)"] {
        for field in [
            format!("complex=<{expression} 5>"),
            format!("complex_array=[<{expression} 5>]"),
        ] {
            let source =
                format!("* invalid separated operands\nA1 [in] print_param_types {field}\n.END\n");
            let result = Netlist::parse(&source)
                .map_err(|e| e.to_string())
                .and_then(|netlist| {
                    Engine::default()
                        .build_circuit(&netlist)
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                });
            assert!(result.is_err(), "{source}");
        }
    }
}
