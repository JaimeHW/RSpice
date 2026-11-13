use rspice_core::{
    Engine, Netlist,
    netlist::{ElementKind, flatten_netlist_with_models},
};

#[test]
fn complex_instance_components_resolve_forward_parameters_and_functions() {
    for (real, imag, definitions) in [
        ("{r}", "{i}", ".PARAM r=2 i=3"),
        ("-r+4", "i+1", ".PARAM r=2 i=2"),
        ("{RE(z)}", "{IMG(z)}", ".PARAM z={2+3j}"),
        ("later(1)", "later(2)", ".FUNC later(x) {x+1}"),
    ] {
        let body = format!(
            "A1 [in] print_param_types complex=<{real} {imag}> complex_array=[<{real} {imag}> <1 0>]"
        );
        let netlist =
            Netlist::parse(&format!("* forward complex\n{body}\n{definitions}\n.END\n")).unwrap();
        Engine::default().build_circuit(&netlist).unwrap();
        let scoped = Netlist::parse(&format!(
            "* scoped complex\n.SUBCKT cell in\n{body}\n.ENDS\nX1 in cell\n{definitions}\n.END\n"
        ))
        .unwrap();
        let flattened = flatten_netlist_with_models(&scoped).unwrap();
        let ElementKind::Xspice {
            string_params,
            string_vector_params,
            ..
        } = &flattened.elements[0].kind
        else {
            panic!("expected XSPICE instance");
        };
        assert_eq!(string_params[0].1, "<2 3>", "{body}");
        assert_eq!(string_vector_params[0].1, ["<2 3>", "<1 0>"], "{body}");
        Engine::default().build_circuit(&scoped).unwrap();
    }
}

#[test]
fn deferred_complex_components_still_require_real_results() {
    for field in ["complex=<{z} 1>", "complex_array=[<1 {z}>]"] {
        let source = format!(
            "* nonreal component\nA1 [in] print_param_types {field}\n.PARAM z={{2+1e-300j}}\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .map(|_| ())
            .expect_err(&source)
            .to_string();
        assert!(error.contains("real value"), "{source}: {error}");
    }
}

#[test]
fn scoped_vectors_and_complex_fields_use_resolved_instance_overrides() {
    for model in ["print_param_types", "alias"] {
        let netlist = Netlist::parse(&format!(
            "* scoped field dependencies\n.SUBCKT cell in PARAMS: base=0\n\
             .MODEL alias print_param_types(real=1 integer=1)\n\
             A1 [in] {model} real={{base+1}} integer={{base+2}} \
             real_array=[{{real}} {{integer}}] complex=<{{real}} {{integer}}> \
             complex_array=[<{{real}} {{integer}}>]\n.ENDS\nX1 in cell base=3\n.END\n"
        ))
        .unwrap();
        let flattened = flatten_netlist_with_models(&netlist).unwrap();
        let ElementKind::Xspice {
            real_vector_params,
            string_params,
            string_vector_params,
            ..
        } = &flattened.elements[0].kind
        else {
            panic!("expected XSPICE instance");
        };
        assert_eq!(real_vector_params[0].1, [4.0, 5.0]);
        assert_eq!(string_params[0].1, "<4 5>");
        assert_eq!(string_vector_params[0].1, ["<4 5>"]);
        Engine::default().build_circuit(&netlist).unwrap();
    }
}

#[test]
fn deferred_complex_evaluation_observes_build_cancellation() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for field in [
        "complex=<f18() 1>",
        "complex=<1 f18()>",
        "complex_array=[<1 f18()>]",
    ] {
        let netlist = Netlist::parse(&format!(
            "* deferred cancellation\nA1 [in] print_param_types {field}\n{functions}.END\n"
        ))
        .unwrap();
        let abort = rspice_core::abort_signal::CountingAbort::new(1024);
        let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(rspice_core::SimulationError::Aborted)),
            "{field}: {:?}",
            result.err()
        );
        assert_eq!(abort.count(), 1025, "{field}");
        assert_eq!(abort.polls_after_abort(), 0, "{field}");
    }
}
