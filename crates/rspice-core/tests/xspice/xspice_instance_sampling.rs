use rspice_core::Netlist;
use rspice_core::netlist::{ParamContext, expr::eval_expression};

#[test]
fn failed_forward_instance_probes_do_not_advance_the_parameter_stream() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for (instance, declaration) in [
        ("A1 in out gain gain={aunif(100,1)+later}", ".PARAM later=0"),
        ("A1 in out gain gain=aunif(100,1)+later", ".PARAM later=0"),
        (
            "A1 in out gain gain={aunif(100,1)+later()}",
            ".FUNC later() {0}",
        ),
        (
            "A1 [in] print_param_types real_array=[0 {aunif(100,1)+later}]",
            ".PARAM later=0",
        ),
    ] {
        let source = format!(
            "* forward instance sample\n.OPTIONS SEED=37\n{instance}\n{declaration}\n\
             .PARAM marker={{aunif(100,1)}}\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(
            netlist.params.get("marker").unwrap(),
            expected,
            "{instance}"
        );
    }
}

#[test]
fn abandoned_complex_grammar_probes_do_not_sample_literal_strings() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for value in [
        "[<sample()>]",
        "[<sample() 1]",
        "[<sample() sample()oops>]",
        r#"["<sample()>"]"#,
        r#"["<alpha beta>"]"#,
    ] {
        let source = format!(
            "* literal strings\n.OPTIONS SEED=37\n.FUNC sample() {{aunif(100,1)}}\n\
             A1 [in] print_param_types string_array={value}\n\
             .PARAM marker={{aunif(100,1)}}\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.params.get("marker").unwrap(), expected, "{value}");
        rspice_core::Engine::default()
            .build_circuit(&netlist)
            .unwrap();
    }
}

#[test]
fn accepted_complex_pairs_sample_each_component_once() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let real = eval_expression("aunif(100,1)", &reference).unwrap();
    let imag = eval_expression("aunif(100,1)", &reference).unwrap();
    let marker = eval_expression("aunif(100,1)", &reference).unwrap();
    let netlist = Netlist::parse(
        "* accepted complex pair\n.OPTIONS SEED=37\n.FUNC sample() {aunif(100,1)}\n\
         A1 [in] print_param_types complex_array=[<sample() sample()>]\n\
         .PARAM marker={aunif(100,1)}\n.END\n",
    )
    .unwrap();
    assert_eq!(netlist.params.get("marker").unwrap(), marker);
    let rspice_core::netlist::ElementKind::Xspice {
        string_vector_params,
        ..
    } = &netlist.elements[0].kind
    else {
        panic!("expected XSPICE instance");
    };
    assert_eq!(string_vector_params[0].1, [format!("<{real} {imag}>")]);
}

#[test]
fn independent_bare_vectors_keep_root_sampling_order_and_self_references() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let vector_sample = eval_expression("aunif(100,1)", &reference).unwrap();
    let scalar_sample = eval_expression("aunif(100,1)", &reference).unwrap();
    let marker = eval_expression("aunif(100,1)", &reference).unwrap();
    for (vector, expected_first) in [
        ("[known {aunif(100,1)}]", 2.0),
        ("\"[known {aunif(100,1)}]\"", 2.0),
        ("[real_array {aunif(100,1)}]", 3.0),
    ] {
        let netlist = Netlist::parse(&format!(
            "* bare vector samples\n.OPTIONS SEED=37\n.PARAM known=2 real_array=3\n\
             A1 [in] print_param_types real_array={vector} real={{aunif(100,1)}}\n\
             .PARAM marker={{aunif(100,1)}}\n.END\n"
        ))
        .unwrap();
        assert_eq!(netlist.params.get("marker"), Some(marker));
        let rspice_core::netlist::ElementKind::Xspice {
            params,
            real_vector_params,
            ..
        } = &netlist.elements[0].kind
        else {
            panic!("expected XSPICE instance");
        };
        assert_eq!(params[0].1, scalar_sample);
        assert_eq!(real_vector_params[0].1, [expected_first, vector_sample]);
    }
}
