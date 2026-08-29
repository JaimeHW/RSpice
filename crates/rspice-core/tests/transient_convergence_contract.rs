//! Public transient convergence contract: rejected Newton iterates must never
//! be committed as waveform points.

use rspice_core::engine::{Engine, SimulationConfig, SimulationError, SpiceDialect};
use rspice_core::netlist::Netlist;

const UNSATISFIABLE_TRANSIENT: &str = "\
* No real transient operating point: v^2 + v + 1 = 0
 B1 n 0 I={V(n)*V(n)+1}
 R1 n 0 1
 .tran 1u 2u uic
 .end
";

fn assert_rejected_newton_iterate_is_not_published(dialect: SpiceDialect) {
    let netlist = Netlist::parse(UNSATISFIABLE_TRANSIENT).expect("contract deck parses");
    let mut config = SimulationConfig::default().with_spice_dialect(dialect);
    config.transient_max_iterations = 2;
    config.transient_nonlinear_max_iterations = Some(2);
    config.transient_initial_timestep = Some(1.0e-6);
    config.min_timestep = 1.0e-6;
    config.max_timestep = 1.0e-6;
    let engine = Engine::new(config);

    let error = engine
        .run_tran(&netlist, 2.0e-6, 1.0e-6)
        .expect_err("an equation with no real root must fail transient convergence");
    assert!(
        matches!(error, SimulationError::ConvergenceFailed(_)),
        "unexpected error for {dialect:?}: {error}"
    );

    let quality = engine.convergence_quality();
    assert_eq!(quality.force_accepted_points, 0, "dialect={dialect:?}");
    assert!(
        quality.force_accepted_indices.is_empty(),
        "dialect={dialect:?}"
    );
}

#[test]
fn every_dialect_rejects_nonconverged_newton_points() {
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        assert_rejected_newton_iterate_is_not_published(dialect);
    }
}
