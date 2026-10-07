//! Cancellation must reach work inside a single control expression.
use rspice_core::abort_signal::CountingAbort;
use rspice_core::engine::{ControlCircuit, ControlExecutionError};
use rspice_core::execution::control::{
    ControlCommand, ControlErrorKind, ControlLimits, ControlProgram,
};
use rspice_core::{Engine, Netlist, NoAbort};

const SOURCE: &str = "* expression cancellation\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1u\n";

fn work_functions() -> String {
    let mut source = String::from(".FUNC f0() {1}\n");
    for index in 1..=14 {
        source.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    source.push_str(".FUNC work(x) {f14()}\n");
    source
}

#[test]
fn assignments_and_loop_tests_cancel_without_executing_the_next_command() {
    for script in [
        "let expensive=work(14)",
        "if work(14)\nend",
        "while work(14)\nend",
        "dowhile work(14)\nend",
        "repeat work(14)\nend",
    ] {
        let program = ControlProgram::parse_deck_with_abort(
            &format!(
                "{SOURCE}{}.control\n{script}\necho unreachable\n.endc\n.end\n",
                work_functions()
            ),
            ControlLimits::default(),
            &NoAbort,
        )
        .unwrap();
        let mut circuit =
            ControlCircuit::new(Netlist::parse(program.declarative_source()).unwrap()).unwrap();
        let mut session = program.start(circuit.netlist().params.clone());
        let abort = CountingAbort::new(128);
        let error = session.next_command(&mut circuit, &abort).unwrap_err();
        assert_eq!(error.kind, ControlErrorKind::Aborted, "{script}: {error}");
        assert_eq!(abort.count(), 129);
        assert_eq!(abort.polls_after_abort(), 0);
        assert!(session.variables().get("expensive").is_none());
        assert!(session.next_command(&mut circuit, &NoAbort).is_err());
    }
}

#[test]
fn command_arguments_cancel_before_publishing_changes_or_output() {
    let engine = Engine::default();
    let mut circuit = ControlCircuit::new(
        Netlist::parse(&format!("{SOURCE}{}.end\n", work_functions())).unwrap(),
    )
    .unwrap();
    let variables = circuit.netlist().params.clone();
    let original_elements = format!("{:?}", circuit.netlist().elements);
    let command = |name: &str, arguments: &str| ControlCommand {
        line: 12,
        name: name.into(),
        arguments: arguments.into(),
    };
    circuit
        .execute(
            &engine,
            &command("pz", "in 0 out 0 vol pz"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    for (name, arguments) in [
        ("set", "num_threads=work(14)"),
        ("alter", "R1 work(14)"),
        ("alter", "@V1[sin] [ 0 work(14) 1 ]"),
        ("plot", "pole(1) xlimit 0 work(14)"),
        ("print", "pole(work(14))"),
        ("settype", "frequency pole(work(14))"),
    ] {
        let abort = CountingAbort::new(128);
        let error = circuit
            .execute(&engine, &command(name, arguments), &variables, &abort)
            .unwrap_err();
        assert!(
            matches!(error, ControlExecutionError::Command(ref error) if error.kind == ControlErrorKind::Aborted),
            "{name} {arguments}: {error}"
        );
        assert_eq!(abort.count(), 129);
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(
            format!("{:?}", circuit.netlist().elements),
            original_elements
        );
        assert_eq!(circuit.settings().maximum_parallel_workers, None);
    }
}
