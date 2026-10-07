//! Retained TF scalars participate in the shared control expression language.
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::ControlCircuit;
use rspice_core::execution::control::{
    ControlCommand, ControlErrorKind, ControlLimits, ControlProgram, ControlScalarEvaluator,
};
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::{Engine, Netlist, NoAbort};

fn circuit(source: &str, probe: &str) -> ControlCircuit {
    let netlist = Netlist::parse(source).unwrap();
    let variables = netlist.params.clone();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    circuit
        .execute(
            &Engine::default(),
            &ControlCommand {
                name: "tf".into(),
                arguments: probe.into(),
                line: 8,
            },
            &variables,
            &NoAbort,
        )
        .unwrap();
    circuit
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn assignments_conditions_and_loops_read_retained_transfer_results() {
    let source = "TF scalar workflow\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.control\n\
        tf V(out) V1\nlet old_gain = tf1.transfer_function\nalter R1 2k\n\
        tf V(out) V1\nlet ratio = old_gain/transfer_gain\n\
        if ratio > 1.3\nlet accepted = 1\nelse\nlet accepted = 0\nend\n\
        let n = 0\nwhile n < tf2.transfer_gain*4\nlet n = n+1\nend\n.endc\n.end\n";
    let program =
        ControlProgram::parse_deck_with_abort(source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        circuit
            .execute(&Engine::default(), &command, session.variables(), &NoAbort)
            .unwrap();
    }
    assert_eq!(circuit.datasets().len(), 2);
    assert!((session.variables().get("ratio").unwrap() - 4.0 / 3.0).abs() < 1e-9);
    assert_eq!(session.variables().get("accepted"), Some(1.0));
    assert_eq!(session.variables().get("n"), Some(2.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unbounded_transfer_reads_fail_only_when_evaluated_and_respect_function_scope() {
    let mut circuit = circuit("Open\nV1 in 0 1\nR1 out 0 1k\n.end\n", "I(V1) V1");
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let mut variables = ParamContext::new();
        variables.set_expression_dialect(dialect);
        variables.define_function(
            "shadow",
            vec!["input_impedance".into()],
            "input_impedance+1",
        );
        variables.define_function("broken", vec![], "(");
        variables.define_function("saved", vec![], "tf1.input_impedance");
        for (expression, expected) in [
            ("if(0,input_impedance,4)", 4.0),
            ("if(1,7,tf99.transfer_function)", 7.0),
            ("if(1,3,broken())", 3.0),
            ("shadow(4)", 5.0),
            ("transfer_function", 0.0),
        ] {
            assert_eq!(
                circuit.evaluate_scalar(expression, &variables, 11).unwrap(),
                expected,
                "{dialect:?}: {expression}"
            );
        }
        // Never turn an unbounded circuit determination into Xyce's finite
        // expression-overflow sentinel, even when a local parameter collides.
        variables.set("input_impedance", 42.0);
        for expression in ["input_impedance", "tf1.V1#input_impedance", "saved()"] {
            let error = circuit
                .evaluate_scalar(expression, &variables, 17)
                .unwrap_err();
            assert_eq!(error.kind, ControlErrorKind::Expression);
            assert_eq!(error.line, 17);
            assert!(error.to_string().contains("unbounded"), "{error}");
        }
        assert!(
            circuit
                .evaluate_scalar("tf99.transfer_gain", &variables, 19)
                .is_err()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn transfer_bindings_preserve_expression_dialects_and_random_evaluation_order() {
    let mut circuit = circuit("Gain\nI1 0 out 0\nR1 out 0 3k\n.end\n", "V(out) I1");
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let mut variables = ParamContext::new();
        variables.set_expression_dialect(dialect);
        variables.set_random_seed(1742);
        variables.define_function("scaled", vec!["x".into()], "x*tf1.transfer_function");
        assert_eq!(
            circuit.evaluate_scalar("scaled(2)", &variables, 8).unwrap(),
            6000.0
        );
        let mut reference = ParamContext::new();
        reference.set_expression_dialect(dialect);
        reference.set_random_seed(1742);
        for expression in [
            "log(100)",
            "if(0,aunif(0,1),aunif(0,1))",
            "aunif(0,1)+2*aunif(0,1)",
            "if(1,5,unavailable)",
        ] {
            assert_eq!(
                circuit.evaluate_scalar(expression, &variables, 8).unwrap(),
                eval_expression(expression, &reference).unwrap(),
                "{dialect:?}: {expression}"
            );
        }
    }
}
