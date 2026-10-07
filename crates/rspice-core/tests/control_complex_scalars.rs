//! Complex scalar control flow and the boundaries of real circuit commands.
use rspice_core::engine::{ControlCircuit, ControlCommandEffect, ControlPresentationKind};
use rspice_core::execution::control::{ControlErrorKind, ControlLimits, ControlProgram};
use rspice_core::netlist::expr::ParamContext;
use rspice_core::{ComplexValue, Engine, Netlist, NoAbort};

fn program(script: &str) -> ControlProgram {
    ControlProgram::parse_deck_with_abort(
        &format!("complex control\nV1 in 0 1\nR1 in 0 1k\n.control\n{script}\n.endc\n.end\n"),
        ControlLimits::default(),
        &NoAbort,
    )
    .unwrap()
}

fn circuit(program: &ControlProgram) -> ControlCircuit {
    ControlCircuit::new(Netlist::parse(program.declarative_source()).unwrap()).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn assignments_and_substitutions_preserve_both_components_and_precedence() {
    let program = program(
        "let z = 2+3j\nlet square = z*z\nlet doubled = 2*$&z\n\
         let negated = -$z\nlet tiny = 1+1e-300j\nlet tiny_copy = $&tiny\n\
         let reused = z\nlet reused = 7\nlet resumed = reused+4j",
    );
    let mut circuit = circuit(&program);
    let mut session = program.start(ParamContext::new());
    assert!(
        session
            .next_command(&mut circuit, &NoAbort)
            .unwrap()
            .is_none()
    );
    for (name, expected) in [
        ("z", ComplexValue::new(2.0, 3.0)),
        ("square", ComplexValue::new(-5.0, 12.0)),
        ("doubled", ComplexValue::new(4.0, 6.0)),
        ("negated", ComplexValue::new(-2.0, -3.0)),
        ("tiny_copy", ComplexValue::new(1.0, 1e-300)),
        ("reused", ComplexValue::from(7.0)),
        ("resumed", ComplexValue::new(7.0, 4.0)),
    ] {
        assert_eq!(
            session.variables().get_complex(name),
            Some(expected),
            "{name}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn imaginary_conditions_drive_if_while_and_dowhile() {
    let program = program(
        "let selected = 0\nif 1e-300j\nlet selected = 1\nend\n\
         let z = 2j\nlet n = 0\nwhile z\nlet n = n+1\nlet z = z-1j\nend\n\
         let z = 2j\nlet m = 0\ndowhile z\nlet m = m+1\nlet z = z-1j\nend\n\
         if z\nlet selected = 99\nend",
    );
    let mut circuit = circuit(&program);
    let mut session = program.start(ParamContext::new());
    assert!(
        session
            .next_command(&mut circuit, &NoAbort)
            .unwrap()
            .is_none()
    );
    assert_eq!(session.variables().get("selected"), Some(1.0));
    assert_eq!(session.variables().get("n"), Some(2.0));
    assert_eq!(session.variables().get("m"), Some(2.0));
    assert_eq!(
        session.variables().get_complex("z"),
        Some(ComplexValue::ZERO)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn repeat_requires_an_exactly_real_integer_without_partial_execution() {
    for count in ["2+1j", "2+1e-300j", "1j", "-1", "1.5"] {
        let program = program(&format!("let n = 0\nrepeat {count}\nlet n = n+1\nend"));
        let mut circuit = circuit(&program);
        let mut session = program.start(ParamContext::new());
        let error = session.next_command(&mut circuit, &NoAbort).unwrap_err();
        assert_eq!(error.line, 6);
        assert!(error.message.contains("integer"), "{count}: {error}");
        assert_eq!(session.variables().get("n"), Some(0.0));
        assert!(session.next_command(&mut circuit, &NoAbort).is_err());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn imaginary_nonfinite_values_fail_before_assignment_or_command_publication() {
    for expression in ["1+bad", "$&bad", "bad"] {
        let program = program(&format!("let stored = {expression}\nop"));
        let mut circuit = circuit(&program);
        let mut variables = ParamContext::new();
        variables.set_complex("bad", ComplexValue::new(2.0, f64::INFINITY));
        let mut session = program.start(variables);
        let error = session.next_command(&mut circuit, &NoAbort).unwrap_err();
        assert_eq!(error.kind, ControlErrorKind::Expression);
        assert_eq!(error.line, 5);
        assert!(error.message.contains("nonfinite"), "{error}");
        assert_eq!(session.variables().get_complex("stored"), None);
        assert!(circuit.datasets().is_empty());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn real_commands_refuse_complex_values_without_mutating_the_circuit() {
    for command in [
        "alter R1 z",
        "alter R1 $&z",
        "alter @V1[sin] [ 0 1 z ]",
        "set num_threads=z",
        "option reltol=0.02 abstol=z",
        "option reltol=0.02 abstol={z}",
        "option abstol=2j",
        "option abstol=2.0j",
        "option abstol={2+1j}",
        "ac lin 2 1 $&z",
    ] {
        let program = program(&format!("let z = 2+1e-300j\n{command}"));
        let mut circuit = circuit(&program);
        let elements = format!("{:?}", circuit.netlist().elements);
        let options = format!("{:?}", circuit.netlist().options);
        let settings = circuit.settings().clone();
        let mut session = program.start(ParamContext::new());
        let command = session
            .next_command(&mut circuit, &NoAbort)
            .unwrap()
            .unwrap();
        assert!(
            circuit
                .execute(&Engine::default(), &command, session.variables(), &NoAbort)
                .is_err(),
            "{} {} silently accepted a complex value",
            command.name,
            command.arguments
        );
        assert_eq!(format!("{:?}", circuit.netlist().elements), elements);
        assert_eq!(format!("{:?}", circuit.netlist().options), options);
        assert_eq!(circuit.settings(), &settings);
        assert!(circuit.datasets().is_empty());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn complex_print_samples_survive_scalar_and_substitution_bindings() {
    let program = program("let z = 2-3j\nop\nprint z 2*$&z");
    let mut circuit = circuit(&program);
    let mut session = program.start(ParamContext::new());
    let mut printed = false;
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        if let ControlCommandEffect::Presentation(request) = circuit
            .execute(&Engine::default(), &command, session.variables(), &NoAbort)
            .unwrap()
        {
            let ControlPresentationKind::Print(traces) = request.kind else {
                panic!("print")
            };
            assert_eq!(traces.len(), 2);
            assert_eq!(traces[0].y.samples, [ComplexValue::new(2.0, -3.0)]);
            assert_eq!(traces[1].y.samples, [ComplexValue::new(4.0, -6.0)]);
            printed = true;
        }
    }
    assert!(printed);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn substitutions_roundtrip_subnormal_and_maximum_finite_components() {
    for value in [
        ComplexValue::new(f64::MAX, f64::from_bits(1)),
        ComplexValue::new(-f64::MAX, -f64::from_bits(1)),
        ComplexValue::new(f64::from_bits(1), f64::MAX),
    ] {
        let program = program("let copy = $&edge");
        let mut circuit = circuit(&program);
        let mut variables = ParamContext::new();
        variables.set_complex("edge", value);
        let mut session = program.start(variables);
        assert!(
            session
                .next_command(&mut circuit, &NoAbort)
                .unwrap()
                .is_none()
        );
        assert_eq!(session.variables().get_complex("copy"), Some(value));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn complex_plot_limits_are_rejected_without_replacing_retained_data() {
    let program = program("let z = 2+1e-300j\nop\nplot v(in) ylimit 0 z");
    let mut circuit = circuit(&program);
    let mut session = program.start(ParamContext::new());
    let op = session
        .next_command(&mut circuit, &NoAbort)
        .unwrap()
        .unwrap();
    circuit
        .execute(&Engine::default(), &op, session.variables(), &NoAbort)
        .unwrap();
    let identity = circuit.datasets()[0].analysis_id;
    let plot = session
        .next_command(&mut circuit, &NoAbort)
        .unwrap()
        .unwrap();
    let error = circuit
        .execute(&Engine::default(), &plot, session.variables(), &NoAbort)
        .unwrap_err();
    assert!(error.to_string().contains("real value"), "{error}");
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(circuit.datasets()[0].analysis_id, identity);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_real_projections_and_option_random_draw_order_are_preserved() {
    let program = program(
        "let z = 1000+1j\nalter R1 real(z)\nset num_threads=real(2+3j)\n\
         option reltol={aunif(0.01,0.001)} abstol={imag(z)*1e-12}\n\
         let next_draw = aunif(0,1)\nlet n = 0\nrepeat real(2+3j)\nlet n = n+1\nend\nop",
    );
    let mut circuit = circuit(&program);
    let mut variables = ParamContext::new();
    variables.set_random_seed(1742);
    let mut reference = ParamContext::new();
    reference.set_random_seed(1742);
    let expected_tolerance =
        rspice_core::netlist::expr::eval_expression("aunif(0.01,0.001)", &reference).unwrap();
    let expected_draw =
        rspice_core::netlist::expr::eval_expression("aunif(0,1)", &reference).unwrap();
    let mut session = program.start(variables);
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        circuit
            .execute(&Engine::default(), &command, session.variables(), &NoAbort)
            .unwrap();
    }
    assert_eq!(circuit.netlist().options.reltol, Some(expected_tolerance));
    assert_eq!(circuit.netlist().options.abstol, Some(1e-12));
    assert_eq!(circuit.settings().maximum_parallel_workers, Some(2));
    assert_eq!(session.variables().get("next_draw"), Some(expected_draw));
    assert_eq!(session.variables().get("n"), Some(2.0));
    assert_eq!(circuit.datasets().len(), 1);
}
