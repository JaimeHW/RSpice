//! Public transient convergence contract: rejected Newton iterates must never
//! be committed as waveform points.

use std::sync::Arc;

use rspice_core::ConvergenceConfig;
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

#[test]
fn successful_gmin_rescue_is_not_counted_as_a_rejected_timestep() {
    let netlist = Netlist::parse(
        "cubic continuation rescue\n\
         .options gmin=0\n\
         I1 0 n PULSE(0 -2 0 1f 1f 10n 20n)\n\
         B1 n 0 I={V(n)*V(n)*V(n)-2*V(n)}\n\
         .tran 0 1f uic\n\
         .end\n",
    )
    .expect("cubic continuation fixture parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        transient_nonlinear_max_iterations: Some(8),
        convergence_config: ConvergenceConfig {
            gmin_initial: 10.0,
            gmin_target: 1.0e-15,
            junction_gmin_target: 0.0,
            ..Default::default()
        },
        locked_time_grid: Some(Arc::new(vec![0.0, 1.0e-15])),
        ..Default::default()
    });

    let result = engine
        .run_tran(&netlist, 1.0e-15, 1.0e-15)
        .expect("GMIN continuation recovers the nonlinear endpoint");
    assert_eq!(
        result.time.last().expect("terminal sample").to_bits(),
        1.0e-15_f64.to_bits()
    );
    assert!(
        result
            .voltages
            .iter()
            .flatten()
            .all(|value| value.is_finite()),
        "the rescued physical solution is finite"
    );

    let quality = engine.convergence_quality();
    assert_eq!(
        quality.timestep_reductions, 1,
        "only the first terminal Newton rejection reduces a timestep; the second plain-Newton failure is rescued in-step"
    );
    assert_eq!(quality.force_accepted_points, 0);
}
