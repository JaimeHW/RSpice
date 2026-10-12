use super::*;
use crate::abort_signal::NoAbort;

fn fixture(
    source: &str,
) -> (
    Engine,
    crate::CircuitData,
    Option<AcceptedTransientOperatingPointContract>,
    charge_event::EventOptions,
) {
    let mut engine = Engine::default();
    engine.config.spice_dialect = SpiceDialect::Ngspice;
    engine.config.convergence_config.gmin_target = 0.0;
    engine.config.convergence_config.junction_gmin_target = 0.0;
    let deck = Netlist::parse(&format!(
        "OP contract\n{source}\nR1 n 0 1k\nC1 n 0 1p\n.end\n"
    ))
    .unwrap();
    let mut circuit = engine.build_circuit(&deck).unwrap();
    circuit.set_independent_source_context(
        crate::circuit::SourceTimeBasis {
            tstep: 1e-10,
            tstop: 1e-8,
        },
        SpiceDialect::Ngspice,
        engine.config.resource_limits,
    );
    let mut matrix = engine.build_matrix(&circuit).unwrap();
    circuit.link_indices(&matrix);
    let (_, _, contract) = engine
        .solve_transient_initial_solution(&deck, &mut circuit, &mut matrix, &NoAbort)
        .unwrap();
    let options = charge_event::EventOptions {
        limits: engine.config.resource_limits,
        solver: matrix.solver_options(),
        nodal_gmin: 0.0,
        iterations: 80,
        backtracks: 32,
        voltage_tolerance: 1e-11,
        current_tolerance: 1e-13,
        charge_tolerance: 1e-25,
        relative_tolerance: 1e-11,
    };
    (engine, circuit, contract, options)
}

#[test]
fn startup_op_contract_requires_unchanged_conditioning_and_no_released_ic() {
    let (engine, circuit, contract, mut options) = fixture("V1 n 0 1");
    assert!(
        engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .unwrap()
            .stationary
    );
    options.nodal_gmin = 1e-12;
    assert!(
        engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .is_none()
    );
    options.nodal_gmin = 0.0;
    let mut changed = contract.unwrap();
    changed.junction_gmin = Some(1e-12);
    assert!(
        engine
            .physical_startup_operating_point(&circuit, Some(changed), &options, &NoAbort)
            .unwrap()
            .is_none()
    );
    changed = contract.unwrap();
    changed.linear_system = TransientOperatingPointLinearSystem::CurrentSeededInductors;
    assert!(
        engine
            .physical_startup_operating_point(&circuit, Some(changed), &options, &NoAbort)
            .unwrap()
            .is_none()
    );
    assert!(
        engine
            .physical_startup_operating_point(&circuit, None, &options, &NoAbort)
            .unwrap()
            .is_none()
    );
    let (engine, circuit, contract, options) = fixture(".ic V(n)=1");
    assert!(contract.is_none());
    assert!(
        engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .is_none()
    );
}

#[test]
fn startup_op_contract_does_not_suppress_a_subtolerance_authored_source_jump() {
    let (engine, circuit, contract, options) = fixture("I1 0 n PWL(0 0 0 1e-20 1 1e-20)");
    assert!(contract.is_some());
    let source = &circuit.current_sources;
    let published = source.value_at_time_on_side(0, 0.0, SourceTimeSide::Published);
    let outgoing = source.value_at_time_on_side(0, 0.0, SourceTimeSide::RightLimit);
    assert_ne!(published, outgoing);
    assert!((published - outgoing).abs() < options.current_tolerance);
    assert!(
        engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .is_none()
    );
}

#[test]
fn startup_op_contract_retains_time_varying_forcing_provenance() {
    let (engine, circuit, contract, options) = fixture("V1 n 0 PWL(0 0 1n 1)");
    let point = engine
        .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
        .unwrap()
        .unwrap();
    assert!(!point.stationary);
}

#[test]
fn startup_behavioral_stationarity_requires_constant_forcing_not_a_zero_initial_slope() {
    for (source, stationary) in [
        ("B1 n 0 V={1}", true),
        ("B1 n 0 V={1+time^2}", false),
        ("B1 0 n I={.001}", true),
        ("B1 0 n I={.001+time^2}", false),
    ] {
        let (engine, circuit, contract, options) = fixture(source);
        let point = engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .unwrap();
        assert_eq!(point.stationary, stationary, "{source}");
    }
    let (engine, circuit, contract, options) = fixture("B1 n 0 V={if(time>1,2,1)}");
    assert!(
        engine
            .physical_startup_operating_point(&circuit, contract, &options, &NoAbort)
            .unwrap()
            .is_none()
    );
}
