use super::*;
use crate::{Netlist, NoAbort};

fn collect(source: &str) -> Vec<ControlCommand> {
    let program =
        ControlProgram::parse_deck_with_abort(source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params);
    let mut commands = Vec::new();
    while let Some(command) = session
        .next_command(&mut ParameterScalarEvaluator, &NoAbort)
        .unwrap()
    {
        commands.push(command);
    }
    commands
}

#[test]
fn control_original_scripts_preserve_every_ordered_analysis_and_alteration() {
    let bjt = include_str!("../../../../../tests/paranoia/control_structs/foreach_bjt_ft.sp");
    let commands = collect(bjt);
    assert_eq!(commands.len(), 13);
    for (pair, expected) in commands[..12]
        .chunks_exact(2)
        .zip([0.5e-3, 1e-3, 5e-3, 10e-3, 50e-3, 100e-3])
    {
        assert_eq!(pair[0].name, "alter");
        let (target, value) = pair[0].arguments.split_once('=').unwrap();
        assert_eq!(target.trim(), "ic");
        assert_eq!(
            crate::netlist::expr::eval_simple(value.trim()).unwrap(),
            expected
        );
        assert_eq!(pair[1].name, "ac");
        assert_eq!(pair[1].arguments, "dec 10 10k 5g");
    }
    assert_eq!(commands[12].name, "plot");
    assert!(commands[12].arguments.contains("ac6.vgain#branch"));

    let memristor = include_str!("../../../../../tests/paranoia/memristor/memristor.sp");
    let commands = collect(memristor);
    assert_eq!(
        commands
            .iter()
            .map(|command| command.name.as_str())
            .collect::<Vec<_>>(),
        [
            "tran", "alter", "tran", "alter", "tran", "plot", "settype", "plot", "plot", "plot"
        ]
    );
    for (command, factor) in commands
        .iter()
        .filter(|command| command.name == "tran")
        .zip([1.0, 1.1, 1.4])
    {
        let parts = command.arguments.split_whitespace().collect::<Vec<_>>();
        let step: f64 = parts[0].parse().unwrap();
        let stop: f64 = parts[1].parse().unwrap();
        assert!((stop * factor / 1e-8 - 1.0).abs() < 1e-14);
        assert!((stop / step - 100.0).abs() < 1e-12);
        assert_eq!(parts[2], "uic");
    }
    for (command, expected_frequency) in commands
        .iter()
        .filter(|command| command.name == "alter")
        .zip([1.1e8, 1.4e8])
    {
        let parts = command.arguments.split_whitespace().collect::<Vec<_>>();
        assert_eq!(parts[0], "@V1[sin]");
        assert_eq!(parts[2], "0");
        assert_eq!(parts[3].parse::<f64>().unwrap(), 3.0);
        assert!((parts[4].parse::<f64>().unwrap() / expected_frequency - 1.0).abs() < 1e-14);
    }
}

#[test]
fn control_nested_loops_resume_with_assignments_break_continue_and_branches() {
    let commands = collect(
        "control flow\nV1 in 0 1\nR1 in 0 1k\n.control\n\
        let n = 0\nforeach label 'left side' right\nrepeat 3\nlet n = n+1\n\
        if n == 2\ncontinue\nend\necho $label $&n\nif n == 4\nbreak\nend\nend\nend\n\
        let k = 0\nwhile k < 2\nlet k = k+1\nend\ndowhile k < 3\nlet k = k+1\nend\n\
        if k == 3\necho done\nelse\necho wrong\nend\nquit\necho unreachable\n.endc\n.end\n",
    );
    assert_eq!(commands.len(), 4);
    for (command, prefix, number) in [
        (&commands[0], "left side", 1.0),
        (&commands[1], "left side", 3.0),
        (&commands[2], "right", 4.0),
    ] {
        let (label, value) = command.arguments.rsplit_once(' ').unwrap();
        assert_eq!(label, prefix);
        assert_eq!(value.parse::<f64>().unwrap(), number);
    }
    assert_eq!(commands[3].arguments, "done");
    assert_eq!(
        collect("empty\n.control\nforeach x ''\necho word=$x\nend\n.endc\n.end\n")[0].arguments,
        "word="
    );
}

#[test]
fn control_empty_infinite_loop_is_bounded_and_cancellation_is_typed() {
    let program = ControlProgram::parse_deck_with_abort(
        "loop\n.control\nwhile 1\nend\n.endc\n.end\n",
        ControlLimits {
            max_steps: 17,
            ..ControlLimits::default()
        },
        &NoAbort,
    )
    .unwrap();
    let mut session = program.start(ParamContext::new());
    let error = session
        .next_command(&mut ParameterScalarEvaluator, &NoAbort)
        .unwrap_err();
    assert_eq!(error.kind, ControlErrorKind::ResourceLimit);
    assert!(error.message.contains("17"));

    struct Stopped;
    impl AbortSignal for Stopped {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    let mut session = program.start(ParamContext::new());
    assert_eq!(
        session
            .next_command(&mut ParameterScalarEvaluator, &Stopped)
            .unwrap_err()
            .kind,
        ControlErrorKind::Aborted
    );
    assert!(
        ControlProgram::parse_deck_with_abort(
            "bad\n.control\nif 1\n.endc\n.end\n",
            ControlLimits::default(),
            &NoAbort
        )
        .is_err()
    );
}
