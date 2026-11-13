use rspice_core::abort_signal::CountingAbort;
use rspice_core::analysis::{
    AcResult, evaluate_ac_continuous_measurements_with_limits_and_abort,
    evaluate_ac_measurements_with_abort,
};
use rspice_core::netlist::Netlist;
use rspice_core::{Complex64, ResourceLimits, SimulationError};

fn fixture(statement: &str) -> (Netlist, [AcResult; 1]) {
    let netlist = Netlist::parse(&format!(
        "* cancellation inside one expression\nV1 out 0 1\nR1 out 0 1k\n.FUNC work(x) {{IF(x<=0,1,work(x-1)+work(x-1))}}\n{statement}\n.END\n"
    ))
    .unwrap();
    let sweep = [AcResult {
        frequency: 10.0,
        node_names: vec!["out".into()],
        branch_names: Vec::new(),
        voltages: vec![Complex64::new(1.0, 0.0)],
        currents: Vec::new(),
    }];
    (netlist, sweep)
}

#[test]
fn scalar_and_equation_measurements_cancel_inside_a_single_row() {
    for statement in [
        ".MEAS AC expensive FIND {work(14)} AT=10",
        ".MEAS AC expensive EQN {work(14)}",
        ".MEAS AC expensive PARAM='work(14)'",
    ] {
        let (netlist, sweep) = fixture(statement);
        let abort = CountingAbort::new(128);
        assert!(
            matches!(
                evaluate_ac_measurements_with_abort(&netlist, &sweep, &abort),
                Err(SimulationError::Aborted)
            ),
            "{statement}: only {} cancellation polls",
            abort.count()
        );
        assert_eq!(abort.count(), 129);
        assert_eq!(abort.polls_after_abort(), 0);
    }
}

#[test]
fn continuous_measurements_cancel_inside_a_single_row() {
    let (netlist, sweep) = fixture(".MEAS AC_CONT expensive FIND {work(14)} AT=10");
    let abort = CountingAbort::new(128);
    assert!(
        matches!(
            evaluate_ac_continuous_measurements_with_limits_and_abort(
                &netlist,
                &sweep,
                &ResourceLimits::default(),
                &abort,
            ),
            Err(SimulationError::Aborted)
        ),
        "only {} cancellation polls",
        abort.count()
    );
    assert_eq!(abort.count(), 129);
    assert_eq!(abort.polls_after_abort(), 0);
}
