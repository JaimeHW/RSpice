use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::{Netlist, ParseWithAbortError, simulation_options_with_overrides};

fn work_functions() -> String {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    functions.push_str(".FUNC work(x) {f18()}\n");
    functions
}

#[test]
fn numeric_cards_cancel_inside_eager_optional_and_deferred_operands() {
    let functions = work_functions();
    for card in [
        ".OPTIONS reltol={work(18)}\n.OP",
        ".TRAN {work(18)} 1",
        ".TRAN 1 {work(18)}",
        ".TRAN 1 100 {work(18)}",
        ".AC LIN {work(18)} 1 100",
        ".DC V1 0 {work(18)} 1",
        ".IC V(out)={work(18)}\n.OP",
        ".NODESET V(out)={work(18)}\n.OP",
        ".TRAN {later+work(18)} 1\n.PARAM later=1",
        ".IC V(out)={later+work(18)}\n.PARAM later=1\n.OP",
        ".OPTIONS TEMP={later+work(18)}\n.PARAM later=1\n.OP",
    ] {
        let source = format!(
            "* numeric card cancellation\n{functions}V1 out 0 1\nR1 out 0 1k\n{card}\n.END\n"
        );
        let abort = CountingAbort::new(1024);
        let result = Netlist::parse_with_abort(&source, &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "{card}: {} polls; {:?}",
            abort.count(),
            result.as_ref().err()
        );
        assert_eq!(abort.count(), 1025, "{card}");
        assert_eq!(abort.polls_after_abort(), 0, "{card}");
    }
}

#[test]
fn option_overrides_cancel_without_publishing_partial_assignments() {
    let netlist = Netlist::parse(&format!(
        "* options\n{}R1 out 0 1k\n.END\n",
        work_functions()
    ))
    .unwrap();
    let abort = CountingAbort::new(128);
    let result = simulation_options_with_overrides(
        "abstol=1n reltol={work(18)}",
        &netlist.params,
        &netlist.options,
        100_000,
        &abort,
    );
    assert!(
        matches!(result, Err(ParseWithAbortError::Aborted)),
        "{result:?}"
    );
    assert_eq!(abort.count(), 129);
    assert_eq!(abort.polls_after_abort(), 0);
    assert_eq!(netlist.options.abstol, None);
}

#[test]
fn grammar_retries_preserve_cancellation_at_every_poll_boundary() {
    for card in [
        ".TRAN {later+1} {later+2}\n.PARAM later=1",
        ".IC V(out)={later+1}\n.PARAM later=1\n.OP",
        ".OPTIONS TEMP={later+27}\n.PARAM later=1\n.OP",
    ] {
        let source = format!("* numeric retries\nV1 out 0 1\nR1 out 0 1k\n{card}\n.END\n");
        let mut completed = false;
        for limit in 0..1024 {
            let abort = CountingAbort::new(limit);
            let result = Netlist::parse_with_abort(&source, &abort);
            assert_eq!(abort.polls_after_abort(), 0, "{card} at {limit}");
            match result {
                Err(ParseWithAbortError::Aborted) => {}
                Ok(_) => {
                    completed = true;
                    break;
                }
                Err(error) => panic!("{card} at {limit}: {error}"),
            }
        }
        assert!(completed, "{card} never completed");
    }
}
