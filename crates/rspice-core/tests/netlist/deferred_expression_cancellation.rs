use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::{Netlist, ParseWithAbortError};

#[test]
fn forward_source_and_model_expressions_honor_cancellation() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for body in [
        "V2 a 0 SIN(0 1 1 {later+f18()})\nR2 a 0 1k\n.PARAM later=1",
        "I2 out 0 {later+f18()}\n.PARAM later=1",
        ".MODEL dd D(IS={later+f18()})\nD1 out 0 dd\n.PARAM later=1",
    ] {
        let source = format!(
            "* deferred expression cancellation\n{functions}V1 out 0 1\nR1 out 0 1k\n{body}\n.OP\n.END\n"
        );
        let abort = CountingAbort::new(1024);
        let result = Netlist::parse_with_abort(&source, &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "{body}: {} polls; {:?}",
            abort.count(),
            result.as_ref().err()
        );
        assert_eq!(abort.count(), 1025, "{body}");
        assert_eq!(abort.polls_after_abort(), 0, "{body}");
    }
}
