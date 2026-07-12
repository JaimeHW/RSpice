//! Cooperative-cancellation contracts for long periodic/RF analyses.

use rspice_core::AtomicAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::analysis::advanced::pac::PacConfig;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

fn fixture() -> (Engine, Netlist, AtomicAbort) {
    let netlist = Netlist::parse("v1 in 0 sin(0 1 1meg)\nr1 in out 1k\nc1 out 0 1n\n.end\n")
        .expect("fixture parses");
    let abort = AtomicAbort::new();
    abort.set();
    (Engine::new(SimulationConfig::default()), netlist, abort)
}

fn assert_aborted<T: std::fmt::Debug>(result: Result<T, SimulationError>) {
    assert!(
        matches!(result, Err(SimulationError::Aborted)),
        "expected SimulationError::Aborted, got {result:?}"
    );
}

#[test]
fn pss_honors_an_already_set_abort_signal() {
    let (engine, netlist, abort) = fixture();
    assert_aborted(engine.run_pss_with_abort(&netlist, PssConfig::new(1.0e6), &abort));
}

#[test]
fn harmonic_balance_honors_an_already_set_abort_signal() {
    let (engine, netlist, abort) = fixture();
    assert_aborted(engine.run_hb_with_abort(&netlist, HbConfig::new(1.0e6), &abort));
}

#[test]
fn pac_honors_an_already_set_abort_signal() {
    let (engine, netlist, abort) = fixture();
    assert_aborted(engine.run_pac_with_abort(&netlist, PacConfig::new(), &abort));
}

#[test]
fn driven_pnoise_honors_an_already_set_abort_signal() {
    let (engine, netlist, abort) = fixture();
    assert_aborted(engine.run_pnoise_with_abort(
        &netlist,
        1.0e6,
        &[1.0e3],
        "out",
        None,
        None,
        3,
        &abort,
    ));
}

#[test]
fn oscillator_pnoise_honors_an_already_set_abort_signal() {
    let (engine, netlist, abort) = fixture();
    assert_aborted(engine.run_pnoise_oscillator_with_abort(
        &netlist,
        PssConfig::autonomous(),
        &[1.0e3],
        &abort,
    ));
}
