use super::*;
use crate::abort_signal::ImmediateAbort;
use crate::engine::PeriodicDcOperatingPointSeed;

#[test]
fn hb_bound_op_seed_reaches_dc_spectrum_and_transient_assisted_startup() {
    let source = "Bound HB OP\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1u\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let engine = Engine::default();
    let circuit = engine.build_circuit_with_abort(&netlist, &NoAbort).unwrap();
    let seed = PeriodicDcOperatingPointSeed::try_new(
        circuit.node_names_sorted(),
        circuit.branch_names_sorted(),
        vec![1.0, 0.25, -0.00075],
    )
    .unwrap();
    let mut state = HbSolverState::new(2, 3);
    state.try_prepare_mna_branches(1, 3).unwrap();
    Engine::hb_apply_bound_dc_seed(
        &seed,
        &mut state,
        seed.node_names(),
        seed.branch_names(),
        1,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(state.x[0][0], Complex64::new(1.0, 0.0));
    assert_eq!(state.x[1][0], Complex64::new(0.25, 0.0));
    assert_eq!(
        state.mna_branch_currents[0][0],
        Complex64::new(-0.00075, 0.0)
    );
    assert!(
        state
            .x
            .iter()
            .chain(&state.mna_branch_currents)
            .all(|row| row[1..].iter().all(|value| *value == Complex64::ZERO))
    );

    // Deliberately charge the capacitor below equilibrium: recomputing an OP
    // would replace 0.25 V with 1 V and erase this initial-condition response.
    let transient = engine
        .run_tran_for_periodic_seed(&netlist, 2e-4, 1e-6, Some(&seed), &NoAbort)
        .map(|(result, _)| result)
        .unwrap();
    let out = transient
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert_eq!(transient.voltages[out][0], 0.25);
    let expected = 1.0 - 0.75 * (-0.2_f64).exp();
    assert!((transient.voltages[out].last().unwrap() - expected).abs() < 2e-3);

    let config = HbConfig::new(1000.0).with_harmonics(3);
    for mode in [None, Some(1), Some(2)] {
        let deck = mode.map_or_else(
            || source.to_owned(),
            |mode| source.replace(".end", &format!(".options hbint tahb={mode}\n.end")),
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let result = engine
            .run_hb_with_dc_seed_and_abort(&netlist, config.clone(), &seed, &NoAbort)
            .unwrap();
        assert!((result.operating_point.spectral_state()[1][0].re - 1.0).abs() < 1e-9);
    }
    let direct = Netlist::parse(&source.replace(".end", ".options hbint tahb=0\n.end")).unwrap();
    assert!(
        engine
            .run_hb_with_dc_seed_and_abort(&direct, config.clone(), &seed, &NoAbort)
            .is_err()
    );
    assert!(
        engine
            .run_hb_with_abort(&direct, config.clone(), &NoAbort)
            .is_ok()
    );
    let wrong = PeriodicDcOperatingPointSeed::try_new(
        vec!["WRONG".into(), "OUT".into()],
        seed.branch_names().to_vec(),
        seed.solution().to_vec(),
    )
    .unwrap();
    assert!(
        engine
            .run_hb_with_dc_seed_and_abort(&netlist, config.clone(), &wrong, &NoAbort)
            .is_err()
    );
    assert!(matches!(
        engine.run_hb_with_dc_seed_and_abort(&netlist, config, &seed, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
}
