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
            ".SUBCKT cell a\nR2 a 0 1k\n.ENDS\nX1 out cell M={work()}",
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

#[test]
fn scoped_devices_models_sources_and_startup_expressions_stop_at_cancellation() {
    for body in [
        "R2 a 0 {work()}",
        "R2 a 0 1k TEMP={work()}",
        "C2 a 0 {work()}",
        "L2 a 0 {work()}",
        "V2 a 0 SIN(0 1 1 {work()})",
        "I2 a 0 PULSE(0 1 {work()})",
        "B2 a 0 V={work()}",
        "E2 a 0 a 0 {work()}",
        "P2 a 0 PORT=1 Z0={work()}",
        "A2 a 0 gain gain={work()}",
        "A2 a 0 print_param_types complex=<{work()} 1>",
        "A2 a 0 print_param_types complex_array=[<{work()} 1>]",
        "A2 a 0 print_param_types real_array=[{work()} 1]",
        ".MODEL cmp print_param_types(complex=<{work()} 1>)\nA2 a 0 cmp",
        ".MODEL cmp print_param_types(complex_array=[<{work()} 1>])\nA2 a 0 cmp",
        ".MODEL osc d_osc(cntl_array=[-1 1] freq_array=[1 {work()}])\nA2 null [a] osc",
        ".MODEL dd D(IS={work()})\nD2 a 0 dd",
        ".MODEL dd D(IS=1p)\nD2 a 0 dd AREA={work()}",
        ".IC V(a)={work()}\nR2 a 0 1k",
        ".NODESET V(a)={work()}\nR2 a 0 1k",
    ] {
        let netlist = hierarchy_fixture(
            &format!(".SUBCKT cell a\n{body}\n.ENDS\nX1 out cell"),
            ExpressionDialect::default(),
        );
        let abort = CountingAbort::new(1024);
        let result = flatten_netlist_with_models_with_abort(&netlist, &abort);
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

#[test]
fn hierarchy_cancellation_is_typed_at_every_poll_and_retry_keeps_values() {
    let mut netlist = hierarchy_fixture(
        ".SUBCKT cell a PARAMS: k=1\n.PARAM v={work()+k}\nR2 a 0 {v}\n.MODEL dd D(IS={v*1p})\nD2 a 0 dd\nI2 a 0 SIN(0 1 1 {v})\n.IC V(a)={v}\n.ENDS\nX1 out cell k=2",
        ExpressionDialect::default(),
    );
    netlist.params.define_function("work", vec![], "3");
    let count = CountingAbort::new(usize::MAX);
    let expected = flatten_netlist_with_models_with_abort(&netlist, &count).unwrap();
    for limit in 0..count.count() {
        let abort = CountingAbort::new(limit);
        let result = flatten_netlist_with_models_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "limit {limit}: {:?}",
            result.as_ref().err()
        );
        assert_eq!(abort.count(), limit + 1);
        assert_eq!(abort.polls_after_abort(), 0);
    }
    let actual = flatten_netlist_with_models_with_abort(&netlist, &rspice_core::NoAbort).unwrap();
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
