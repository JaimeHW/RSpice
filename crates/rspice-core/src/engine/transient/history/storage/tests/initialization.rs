use super::*;

fn lane_reservations() -> usize {
    fail_reservation_after(usize::MAX, || {
        BjtTransientHistory::try_unseeded(1).unwrap();
        usize::MAX - HISTORY_RESERVATIONS_BEFORE_FAILURE.get().unwrap()
    })
}

#[test]
fn bjt_initialization_refuses_every_lane_and_uic_copy_without_changing_the_input() {
    let engine = Engine::default();
    let netlist = Netlist::parse("BJT initial storage\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm IC=.4,1.1\nQ2 c b 0 qm IC=.5,1.2\n.model qm NPN IS=1e-16 CJE=1p CJC=.2p\n.end\n").unwrap();
    let circuit = engine.build_circuit(&netlist).unwrap();
    let solution = vec![0.0; circuit.matrix_size()];
    let before = solution.clone();
    let lanes = lane_reservations();
    assert!(lanes > 1);
    for seed in [
        ReactiveHistorySeed::SolvedBias,
        ReactiveHistorySeed::UicStartup,
    ] {
        let baseline = Engine::initialize_bjt_history(&circuit, &solution, seed).unwrap();
        let total = lanes + usize::from(seed == ReactiveHistorySeed::UicStartup) * 2;
        for count in 0..total {
            let error = fail_reservation_after(count, || {
                Engine::initialize_bjt_history(&circuit, &solution, seed)
            })
            .unwrap_err();
            assert_allocation(&error);
            let SimulationError::Allocation { object, .. } = error else {
                unreachable!()
            };
            assert_eq!(
                object,
                if count < lanes {
                    "BJT state initialization"
                } else {
                    "BJT UIC solution seed"
                }
            );
            assert_eq!(solution, before);
        }
        assert_eq!(
            Engine::initialize_bjt_history(&circuit, &solution, seed).unwrap(),
            baseline
        );
    }
    fail_reservation_after(0, || {
        let empty = BjtTransientHistory::try_unseeded(0).unwrap();
        assert_eq!(empty, BjtTransientHistory::default());
        assert_eq!(HISTORY_RESERVATIONS_BEFORE_FAILURE.get(), Some(0));
    });
}

#[test]
fn public_bjt_initialization_failures_leave_the_engine_reusable() {
    let lanes = lane_reservations();
    for parameters in ["TF=1n PTF=90", "LEVEL=4 RCI=1 RBI=1 TF=1n"] {
        let netlist = Netlist::parse(&format!("BJT initialization failure\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm IC=.4,1.1\n.model qm NPN IS=1e-16 CJE=1p CJC=.2p {parameters}\n.end\n")).unwrap();
        let engine = Engine::default();
        for mode in [
            TransientStartupMode::OperatingPoint,
            TransientStartupMode::Uic,
        ] {
            let baseline = engine
                .run_tran_with_startup_mode(&netlist, 1e-10, 1e-12, mode)
                .unwrap();
            for count in [0, 1, lanes - 1] {
                let error = fail_reservation_after(count, || {
                    engine.run_tran_with_startup_mode(&netlist, 1e-10, 1e-12, mode)
                })
                .unwrap_err();
                assert_allocation(&error);
                assert!(matches!(
                    error,
                    SimulationError::Allocation {
                        object: "BJT state initialization",
                        ..
                    }
                ));
            }
            if parameters.starts_with("TF=") && mode == TransientStartupMode::Uic {
                let error = fail_reservation_after(lanes, || {
                    engine.run_tran_with_startup_mode(&netlist, 1e-10, 1e-12, mode)
                })
                .unwrap_err();
                assert!(matches!(
                    error,
                    SimulationError::Allocation {
                        object: "BJT UIC solution seed",
                        ..
                    }
                ));
            }
            let rerun = engine
                .run_tran_with_startup_mode(&netlist, 1e-10, 1e-12, mode)
                .unwrap();
            assert_eq!(rerun.time, baseline.time);
            assert_eq!(rerun.voltages, baseline.voltages);
            assert_eq!(rerun.branch_currents, baseline.branch_currents);
        }
    }
}

#[test]
fn restart_bjt_initialization_failure_preserves_all_accepted_participants() {
    let engine = Engine::default();
    let netlist = Netlist::parse("restart storage failure\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm\nC1 b 0 1p\nL1 c x 1n\nR1 x 0 1k\n.model qm NPN IS=1e-16 TF=1n PTF=90\n.end\n").unwrap();
    let mut circuit = engine.build_circuit(&netlist).unwrap();
    let solution = vec![0.0; circuit.matrix_size()];
    let mut history =
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias)
            .unwrap();
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    circuit.capacitors.v_prev.fill(0.37);
    circuit.inductors.i_prev.fill(0.002);
    let before = history.clone();
    let circuit_before = format!(
        "{:?}{:?}{:?}",
        circuit.capacitors, circuit.inductors, circuit.bjts.devices
    );
    for count in [0, lane_reservations() - 1] {
        let error = fail_reservation_after(count, || {
            Engine::reseed_reactive_histories_for_restart(
                &mut circuit,
                &solution,
                1e-12,
                AcceptedJunctionHistoryRestart::Reinitialize,
                TransientDeviceHistories {
                    bjt: &mut history,
                    jfet: &mut JfetTransientHistory::default(),
                    diode: &mut DiodeTransientHistory::default(),
                    mosfet: &mut MosfetTransientHistory::default(),
                    vdmos: &mut VdmosTransientHistory::default(),
                    b3soi: &mut B3SoiTransientHistory::default(),
                    bsim3: &mut Bsim3TransientHistory::default(),
                    bsim4: &mut Bsim4TransientHistory::default(),
                    ekv26: &mut Ekv26TransientHistory::default(),
                },
            )
        })
        .unwrap_err();
        assert_allocation(&error);
        assert_eq!(history, before);
        assert_eq!(
            format!(
                "{:?}{:?}{:?}",
                circuit.capacitors, circuit.inductors, circuit.bjts.devices
            ),
            circuit_before
        );
    }
}

#[test]
fn periodic_bjt_initialization_reserves_all_generations_before_changing_bias() {
    let netlist = Netlist::parse("periodic BJT storage\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 CJE=1p\n.end\n").unwrap();
    let mut circuit = Engine::default().build_circuit(&netlist).unwrap();
    let solution = vec![0.0; circuit.matrix_size()];
    let rates = vec![0.0; circuit.num_nodes()];
    let before = format!("{:?}", circuit.bjts.devices);
    let lanes = lane_reservations();
    for count in [0, lanes, 2 * lanes, 3 * lanes - 1] {
        let error = fail_reservation_after(count, || {
            Engine::initialize_periodic_bjt_history(&mut circuit, [&solution; 3], &rates, 1e-12)
        })
        .unwrap_err();
        assert_allocation(&error);
        assert_eq!(format!("{:?}", circuit.bjts.devices), before);
    }
    Engine::initialize_periodic_bjt_history(&mut circuit, [&solution; 3], &rates, 1e-12).unwrap();
}
