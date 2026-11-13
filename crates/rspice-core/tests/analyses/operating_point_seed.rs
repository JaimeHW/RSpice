//! The linear solve that seeds an operating point's Newton iteration.
//!
//! It stamps only the linear devices, so a behavioral voltage source, whose
//! branch equation is its expression, leaves its branch row empty there. The
//! seed then starts from zero without asking the sparse solver to factor a
//! system that has no solution, which would otherwise report that the solver
//! could not factor it.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

use super::log_capture;

/// Run `body` with the warnings it logs on this thread collected.
fn warnings_of<T>(body: impl FnOnce() -> T) -> (T, Vec<String>) {
    log_capture::lines_logged(log::LevelFilter::Warn, body)
}

// LTspice 26.1.1 and QSPICE 2026 batch: V(y) = 2 V exactly.
#[test]
fn a_behavioral_voltage_source_seeds_newton_without_a_failed_factorization() {
    let netlist = Netlist::parse(
        "behavioral source seed\n\
         V1 a 0 1\n\
         R0 a 0 1k\n\
         B1 y 0 V=2*V(a)\n\
         R1 y 0 1k\n\
         .op\n\
         .end\n",
    )
    .expect("deck parses");
    let (result, warnings) = warnings_of(|| Engine::default().run_dc_op(&netlist));
    let result = result.expect("operating point solves");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("y"))
        .expect("V(y) is solved");
    let value = result.node_voltages[index];
    assert!((value - 2.0).abs() <= 1e-12, "V(y) = {value}");
    assert!(
        !warnings
            .iter()
            .any(|line| line.contains("could not factor")),
        "{warnings:?}"
    );
}
