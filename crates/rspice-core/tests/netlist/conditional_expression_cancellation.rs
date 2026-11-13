use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::{Netlist, ParseWithAbortError};

fn source(body: &str) -> String {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    format!("* conditional cancellation\n{functions}V1 out 0 1\nR1 out 0 1k\n{body}\n.OP\n.END\n")
}

#[test]
fn active_conditional_expressions_cancel_without_becoming_provisional_errors() {
    for body in [
        ".IF {f18()}\nR2 out 0 1k\n.ENDIF",
        ".IF 0\n.ELSEIF {f18()}\nR2 out 0 1k\n.ENDIF",
        ".SUBCKT cell a\n.IF {f18()}\nR2 a 0 1k\n.ENDIF\n.ENDS\nX1 out cell",
    ] {
        let abort = CountingAbort::new(1024);
        let result = Netlist::parse_with_abort(&source(body), &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "{body}: {} polls",
            abort.count()
        );
        assert_eq!(abort.count(), 1025);
        assert_eq!(abort.polls_after_abort(), 0);
    }
}

#[test]
fn suppressed_conditions_do_not_evaluate_their_expressions() {
    let abort = CountingAbort::new(1024);
    let parsed = Netlist::parse_with_abort(&source(
        ".IF 0\n.IF {f18()}\nR2 out 0 1k\n.ENDIF\n.ELSEIF 1\nR3 out 0 2k\n.ELSEIF {f18()}\nR4 out 0 1k\n.ENDIF"
    ), &abort).unwrap();
    assert_eq!(parsed.elements.len(), 3);
    assert_eq!(parsed.elements[2].name, "R3");
    assert!(abort.count() < 1024);
}
