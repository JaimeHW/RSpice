use rspice_core::abort_signal::CountingAbort;
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::NetlistParseOptions;
use rspice_core::netlist::{Netlist, ParseWithAbortError, flatten_netlist_with_models_with_abort};

// Parse inexpensive definitions first, then install an expensive function graph
// so every deadline below measures hierarchy evaluation, not initial parsing.
fn hierarchy_fixture(body: &str, dialect: ExpressionDialect) -> Netlist {
    let mut netlist = Netlist::parse_with_options(
        &format!("* hierarchy expression cancellation\n.FUNC work() {{1}}\nV1 out 0 1\nR1 out 0 1k\n{body}\n.OP\n.END\n"),
        NetlistParseOptions { expression_dialect: dialect, ..Default::default() },
    ).unwrap();
    netlist.params.define_function("f0", vec![], "1");
    for index in 1..=18 {
        netlist.params.define_function(
            &format!("f{index}"),
            vec![],
            &format!("f{}()+f{}()", index - 1, index - 1),
        );
    }
    netlist.params.define_function("work", vec![], "f18()");
    netlist
}

#[test]
fn subcircuit_argument_and_local_parameter_evaluations_stop_at_cancellation() {
    for dialect in [ExpressionDialect::default(), ExpressionDialect::Xyce] {
        for body in [
            ".SUBCKT cell a PARAMS: value=1\nR2 a 0 {value}\n.ENDS\nX1 out cell value={work()}",
            ".SUBCKT cell a PARAMS: value={work()}\nR2 a 0 {value}\n.ENDS\nX1 out cell",
            ".SUBCKT cell a\n.PARAM value={work()}\nR2 a 0 {value}\n.ENDS\nX1 out cell",
        ] {
            let netlist = hierarchy_fixture(body, dialect);
            let abort = CountingAbort::new(1024);
            let result = flatten_netlist_with_models_with_abort(&netlist, &abort);
            assert!(
                matches!(result, Err(ParseWithAbortError::Aborted)),
                "{dialect:?}: {body}: {} polls; {:?}",
                abort.count(),
                result.as_ref().err()
            );
            assert_eq!(abort.count(), 1025, "{dialect:?}: {body}");
            assert_eq!(abort.polls_after_abort(), 0, "{dialect:?}: {body}");
        }
    }
}
