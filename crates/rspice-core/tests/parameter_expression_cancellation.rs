use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::{Netlist, ParseWithAbortError};

#[test]
fn parameter_declarations_cancel_within_one_expression() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=14 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    functions.push_str(".FUNC work(x) {f14()}\n");
    for declaration in [
        ".PARAM expensive={work(14)}",
        ".PARAM expensive='work(14)'",
        ".PARAM expensive=work(14)+1",
        ".GLOBAL_PARAM expensive={work(14)}",
        ".SUBCKT cell a\n.PARAM expensive={work(14)}\nR1 a 0 {expensive}\n.ENDS cell\nX1 out cell",
    ] {
        let source = format!(
            "* parameter cancellation\n{functions}{declaration}\nV1 out 0 1\nR1 out 0 1k\n.OP\n.END\n"
        );
        let abort = CountingAbort::new(1024);
        assert!(
            matches!(
                Netlist::parse_with_abort(&source, &abort),
                Err(ParseWithAbortError::Aborted)
            ),
            "{declaration}: only {} polls",
            abort.count()
        );
        assert_eq!(abort.count(), 1025);
        assert_eq!(abort.polls_after_abort(), 0);
    }
}
