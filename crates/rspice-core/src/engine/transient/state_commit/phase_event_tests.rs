//! Sided GP records join the existing all-native preparation transaction.

use super::*;
use bjt::BjtPhaseContext;

fn fixture() -> (Engine, crate::CircuitData, BjtTransientHistory, Vec<Value>) {
    let deck = Netlist::parse(
        "phase event acceptance\n\
        Q1 c b1 0 phase\nQ2 c b2 0 phase\nQ3 c b3 0 ordinary\n\
        Rc c 0 1k\nRb1 b1 0 1k\nRb2 b2 0 1k\nRb3 b3 0 1k\n\
        C1 in 0 1p\nRin in 0 1k\nL1 coil 0 1n\nRcoil coil 0 1k\n\
        Bint state 0 V=sdt(v(in))\nRstate state 0 1k\n\
        .model phase NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p VAF=20)\n\
        .model ordinary NPN(IS=1e-16)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let mut circuit = engine.build_circuit(&deck).unwrap();
    let mut solution = vec![0.0; circuit.matrix_size()];
    solution[circuit.get_node_by_name("c").unwrap() - 1] = 2.0;
    for bjt in &circuit.bjts.devices {
        assert!(!bjt.mna_promoted());
        solution[bjt.node_base - 1] = 0.6;
    }
    circuit
        .behavioral_sources
        .accept_transient_step(&solution, 0.0)
        .unwrap();
    let mut history =
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    (engine, circuit, history, solution)
}

fn commit(
    engine: &Engine,
    circuit: &mut crate::CircuitData,
    history: &mut BjtTransientHistory,
    solution: &[Value],
    time: Value,
    phase_context: BjtPhaseContext<'_>,
) -> Result<(), SimulationError> {
    let previous_time = history.phase[0]
        .as_ref()
        .unwrap()
        .accepted_samples()
        .next_back()
        .unwrap()
        .0;
    let coeff = CompanionCoefficients::backward_euler();
    engine.update_reactive_history(
        circuit,
        AcceptedReactiveStep {
            accepted_solution: solution,
            accepted_time: time,
            dt: time - previous_time,
            coeff: &coeff,
            bsim4_trnqs_coeff: &coeff,
        },
        TransientDeviceHistories {
            bjt: history,
            jfet: &mut Default::default(),
            diode: &mut Default::default(),
            mosfet: &mut Default::default(),
            vdmos: &mut Default::default(),
            b3soi: &mut Default::default(),
            bsim3: &mut Default::default(),
            bsim4: &mut Default::default(),
            ekv26: &mut Default::default(),
        },
        AcceptedReactiveSnapshots {
            bjt_phase: phase_context,
            xyce_one_step_order2: false,
            vbic_snapshots: None,
            capacitor_accepted_states: None,
            mosfet_caps: None,
            mosfet_gate_companion_charges: None,
            suppress_gate_charge_history: false,
            tline_dc_refs: &[],
            coupled_tline_refs: &[],
        },
        ReactiveBreakpointScheduling {
            breakpoints: &mut BreakpointManager::new(),
            tstop: 4e-9,
            voltage_reltol: 1e-3,
            voltage_abstol: 1e-6,
            current_abstol: 1e-12,
        },
        DynamicBreakpointSink {
            dynamic_breakpoints_added: &mut 0,
            warned_dynamic_breakpoint_cap: &mut false,
            pending_dynamic_breakpoints: &mut Vec::new(),
        },
    )
}

fn limits(history: &BjtTransientHistory) -> [Option<Value>; 3] {
    std::array::from_fn(|index| {
        history.phase[index]
            .as_ref()
            .map(|phase| phase.accepted_samples().next_back().unwrap().1)
    })
}

#[test]
fn gp_phase_event_acceptance_preserves_jump_corner_and_incoming_interval_error() {
    let (engine, mut circuit, mut history, mut solution) = fixture();
    let left = limits(&history);
    let before = history.clone();
    solution[circuit.bjts.devices[0].node_base - 1] = 0.65;
    let coeff = CompanionCoefficients::backward_euler();
    let time = 1e-9;
    let step = AcceptedReactiveStep {
        accepted_solution: &solution,
        accepted_time: time,
        dt: time,
        coeff: &coeff,
        bsim4_trnqs_coeff: &coeff,
    };
    let ordinary = engine
        .bjt_phase_step_control(&circuit, &history, step, &[], Default::default())
        .unwrap()
        .unwrap();
    assert!(ordinary.normalized_error > 1.0);
    let context = BjtPhaseContext {
        incoming_arrival: false,
        input_left_limits: Some(&left),
    };
    let mut unphased = history.clone();
    unphased.phase.fill(None);
    let static_transport = Engine::prepare_bjt_history(
        &circuit,
        &unphased,
        &solution,
        &coeff,
        time,
        time,
        None,
        Default::default(),
    )
    .unwrap();
    let sided = engine
        .bjt_phase_step_control(&circuit, &history, step, &[], context)
        .unwrap()
        .unwrap();
    assert_eq!(
        sided.normalized_error, 0.0,
        "the held incoming value closes a constant interval"
    );
    assert_eq!(history, before, "preparation mutated accepted history");
    commit(
        &engine,
        &mut circuit,
        &mut history,
        &solution,
        time,
        context,
    )
    .unwrap();
    for (index, &left) in left.iter().enumerate().take(2) {
        let phase = history.phase[index].as_ref().unwrap();
        let current = circuit.bjts.devices[index]
            .legacy_forward_transport_branch(&history.dynamic_internal_prev[index])
            .unwrap()
            .current;
        assert_eq!(phase.accepted_samples().next_back(), Some((time, current)));
        assert_eq!(
            phase.accepted_left_limits().next_back(),
            Some((time, left.unwrap()))
        );
        if index == 0 {
            assert!(current > 2.0 * left.unwrap());
            let actual = history.accepted_terminal_currents[index].unwrap();
            let baseline = static_transport.values[index].lead_currents.unwrap();
            for terminal in 0..BJT_EXTERNAL_STATE_DIM {
                let sign = match terminal {
                    BJT_EXT_C_INDEX => 1.0,
                    BJT_EXT_E_INDEX => -1.0,
                    _ => 0.0,
                };
                let expected = sign * (left.unwrap() - current);
                assert!(
                    (actual[terminal] - baseline[terminal] - expected).abs() < 1e-13,
                    "accepted lead current used the wrong input side at terminal {terminal}"
                );
            }
        } else {
            assert_eq!(
                current,
                left.unwrap(),
                "the explicit equal-sided corner must survive"
            );
        }
        let arrival = phase.next_discontinuity_after(time).unwrap().unwrap();
        assert!(arrival >= time + circuit.bjts.devices[index].legacy_excess_phase_delay());
        assert_eq!(phase.next_discontinuity_after(arrival).unwrap(), None);
    }
    assert!(history.phase[2].is_none());
    // An ordinary subsequent knot introduces no new physical event.
    let arrivals: Vec<_> = history
        .phase
        .iter()
        .take(2)
        .map(|phase| {
            phase
                .as_ref()
                .unwrap()
                .next_discontinuity_after(time)
                .unwrap()
        })
        .collect();
    commit(
        &engine,
        &mut circuit,
        &mut history,
        &solution,
        1.1e-9,
        Default::default(),
    )
    .unwrap();
    for (index, expected) in arrivals.into_iter().enumerate() {
        let phase = history.phase[index].as_ref().unwrap();
        assert_eq!(phase.accepted_left_limits().count(), 1);
        assert_eq!(phase.next_discontinuity_after(1.1e-9).unwrap(), expected);
    }
}

#[test]
fn gp_phase_event_acceptance_rejects_invalid_context_without_partial_history() {
    let (engine, mut circuit, mut history, solution) = fixture();
    let left = limits(&history);
    let mut nonfinite = left;
    nonfinite[1] = Some(Value::NAN);
    let mut unsupported = left;
    unsupported[2] = Some(1.0);
    let contexts = [
        BjtPhaseContext {
            incoming_arrival: true,
            input_left_limits: None,
        },
        BjtPhaseContext {
            incoming_arrival: true,
            input_left_limits: Some(&left),
        },
        BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&left[..2]),
        },
        BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&nonfinite),
        },
        BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&unsupported),
        },
    ];
    let before = history.clone();
    let passive_before = format!("{:?}{:?}", circuit.capacitors, circuit.inductors);
    let behavior_before = format!("{:?}", circuit.behavioral_sources);
    for context in contexts {
        assert!(
            commit(
                &engine,
                &mut circuit,
                &mut history,
                &solution,
                1e-9,
                context
            )
            .is_err()
        );
        assert_eq!(history, before);
        assert_eq!(
            format!("{:?}{:?}", circuit.capacitors, circuit.inductors),
            passive_before
        );
        assert_eq!(format!("{:?}", circuit.behavioral_sources), behavior_before);
    }
    let mut inaccurate = left;
    inaccurate[0] = Some(1000.0 * left[0].unwrap());
    let error = commit(
        &engine,
        &mut circuit,
        &mut history,
        &solution,
        1e-9,
        BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&inaccurate),
        },
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("phase-history interpolation error"),
        "{error}"
    );
    assert_eq!(
        history, before,
        "declaring an event must not waive incoming interval accuracy"
    );
    // Fail the later BJT's sample validation after the first pair has already
    // been prepared. Neither it nor another model may advance its history.
    let delay = circuit.bjts.devices[1].legacy_excess_phase_delay();
    history.phase[1]
        .as_mut()
        .unwrap()
        .accept_sample(1e-9, left[1].unwrap(), delay, None)
        .unwrap();
    let before = history.clone();
    assert!(
        commit(
            &engine,
            &mut circuit,
            &mut history,
            &solution,
            1e-9,
            BjtPhaseContext {
                incoming_arrival: false,
                input_left_limits: Some(&left)
            }
        )
        .is_err()
    );
    assert_eq!(history, before);
    assert_eq!(
        format!("{:?}{:?}", circuit.capacitors, circuit.inductors),
        passive_before
    );
    assert_eq!(format!("{:?}", circuit.behavioral_sources), behavior_before);
}

#[test]
fn gp_phase_event_acceptance_keeps_unselected_devices_smooth_and_survives_restarts() {
    let (engine, mut circuit, mut history, mut solution) = fixture();
    let mut left = limits(&history);
    left[1] = None;
    solution[circuit.bjts.devices[0].node_base - 1] = 0.65;
    commit(
        &engine,
        &mut circuit,
        &mut history,
        &solution,
        1e-9,
        BjtPhaseContext {
            incoming_arrival: false,
            input_left_limits: Some(&left),
        },
    )
    .unwrap();
    assert_eq!(
        history.phase[0]
            .as_ref()
            .unwrap()
            .accepted_left_limits()
            .count(),
        1
    );
    assert_eq!(
        history.phase[1]
            .as_ref()
            .unwrap()
            .accepted_left_limits()
            .count(),
        0
    );
    assert_eq!(
        history.phase[1]
            .as_ref()
            .unwrap()
            .next_discontinuity_after(1e-9)
            .unwrap(),
        None
    );
    let accepted = history.phase.clone();
    for restart in [
        AcceptedJunctionHistoryRestart::Preserve,
        AcceptedJunctionHistoryRestart::Reinitialize,
    ] {
        Engine::reseed_reactive_histories_for_restart(
            &mut circuit,
            &solution,
            0.1e-9,
            restart,
            TransientDeviceHistories {
                bjt: &mut history,
                jfet: &mut Default::default(),
                diode: &mut Default::default(),
                mosfet: &mut Default::default(),
                vdmos: &mut Default::default(),
                b3soi: &mut Default::default(),
                bsim3: &mut Default::default(),
                bsim4: &mut Default::default(),
                ekv26: &mut Default::default(),
            },
        );
        assert_eq!(history.phase, accepted);
    }
}
