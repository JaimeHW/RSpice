use super::*;
use crate::abort_signal::NoAbort;

fn options() -> charge_event::EventOptions {
    charge_event::EventOptions {
        limits: crate::resource::ResourceLimits::default(),
        solver: crate::solver::SolverOptions::default(),
        iterations: 80,
        backtracks: 32,
        voltage_tolerance: 1e-11,
        current_tolerance: 1e-13,
        charge_tolerance: 1e-25,
        relative_tolerance: 1e-11,
    }
}

fn close(actual: Value, expected: Value, absolute: Value) {
    assert!(
        (actual - expected).abs() <= absolute + 1e-10 * expected.abs(),
        "{actual:e} != {expected:e}"
    );
}

fn fixture(
    text: &str,
) -> (
    Engine,
    crate::CircuitData,
    crate::solver::StaticMatrix,
    Vec<Value>,
    BjtTransientHistory,
) {
    let deck = Netlist::parse(text).unwrap();
    let mut engine = Engine::default();
    let c = &mut engine.config.convergence_config;
    c.gmin_target = 0.0;
    c.junction_gmin_target = 0.0;
    c.voltage_reltol = 1e-11;
    c.voltage_abstol = 1e-11;
    c.current_abstol = 1e-13;
    c.residual_reltol = 1e-11;
    let mut circuit = engine.build_circuit(&deck).unwrap();
    let mut matrix = engine.build_matrix(&circuit).unwrap();
    circuit.link_indices(&matrix);
    let incoming = engine
        .solve_dc_operating_point(&deck, &mut circuit, &mut matrix)
        .unwrap();
    for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
        let v = Engine::differential_voltage(&incoming, stamp.pp.row, stamp.nn.row);
        circuit.capacitors.v_prev[index] = v;
        circuit.capacitors.v_prev_prev[index] = v;
        circuit.capacitors.v_prev_prev_prev[index] = v;
    }
    let mut history =
        Engine::initialize_bjt_history(&circuit, &incoming, ReactiveHistorySeed::SolvedBias);
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    (engine, circuit, matrix, incoming, history)
}

fn accept(
    engine: &Engine,
    circuit: &mut crate::CircuitData,
    matrix: &mut crate::solver::StaticMatrix,
    history: &mut BjtTransientHistory,
    point: &PreparedPhysicalEvent,
    solution: &mut [Value],
) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
    let coefficients = CompanionCoefficients::backward_euler();
    engine.accept_transient_models(
        circuit,
        matrix,
        solution,
        point.time,
        point.dt,
        &coefficients,
        false,
        true,
        0.0,
        false,
        false,
        Some(acceptance::NativeHistoryAcceptance {
            histories: TransientDeviceHistories {
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
            bsim4_trnqs_coeff: &coefficients,
            snapshots: AcceptedReactiveSnapshots {
                physical_event: Some(point),
                bjt_phase: point.phase_context(),
                xyce_one_step_order2: false,
                vbic_snapshots: None,
                capacitor_accepted_states: None,
                mosfet_caps: None,
                mosfet_gate_companion_charges: None,
                suppress_gate_charge_history: false,
                tline_dc_refs: &[],
                coupled_tline_refs: &[],
            },
            scheduling: ReactiveBreakpointScheduling {
                breakpoints: &mut BreakpointManager::new(),
                tstop: 2.0,
                voltage_reltol: 1e-11,
                voltage_abstol: 1e-11,
                current_abstol: 1e-13,
            },
            sink: DynamicBreakpointSink {
                dynamic_breakpoints_added: &mut 0,
                warned_dynamic_breakpoint_cap: &mut false,
                pending_dynamic_breakpoints: &mut Vec::new(),
            },
        }),
    )
}

#[test]
fn physical_event_acceptance_keeps_finite_rlc_currents_separate_from_impulse() {
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "RLC physical acceptance\nV1 s 0 PWL(0 0 1 0 1 2 2 4)\nR1 s n 4\nL1 n 0 .5\nC1 s 0 3u\nI1 0 s PWL(0 0 1 0 1 .01 2 .02)\n.end\n",
    );
    let original_cap = circuit.capacitors.i_prev.clone();
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1e-3,
                phase_events: PhysicalEventOrders::Declared(&[]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    close(point.state.source_impulses[0], -6e-6, 1e-18);
    assert_eq!(circuit.capacitors.i_prev, original_cap);
    let mut solution = point.state().solution.clone();
    let (discontinuity, static_history) = accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    assert!(discontinuity && static_history.is_some());
    close(circuit.capacitors.v_prev[0], 2.0, 1e-12);
    close(circuit.capacitors.i_prev[0], 6e-6, 1e-16);
    assert!((circuit.capacitors.i_prev[0] - 6e-6 / point.dt).abs() > 1e-3);
    close(circuit.inductors.i_prev[0], 0.0, 1e-14);
    close(circuit.inductors.v_prev[0], 2.0, 1e-12);
    let source = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    close(solution[source], 0.01 - 6e-6, 1e-14);
    assert_eq!(
        point.impulses().collect::<Vec<_>>(),
        vec![(source, point.state().source_impulses[0])]
    );
    close(static_history.unwrap()[source], 0.0, 1e-12);
}

#[test]
fn physical_event_acceptance_binds_zero_resistor_and_voltage_source_impulses() {
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "series impulse ownership\nV1 a 0 PWL(0 0 1 0 1 2 2 4)\nR0 a b 0\nC1 b 0 3u\n.end\n",
    );
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(&[]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let v = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    let r = circuit.num_nodes() + circuit.resistor_branches.branch_indices[0] - 1;
    let impulses = point
        .impulses()
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(impulses.len(), 2);
    close(impulses[&v], -6e-6, 1e-18);
    close(impulses[&r], 6e-6, 1e-18);
    let mut solution = point.state().solution.clone();
    accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    close(solution[r], 6e-6, 1e-16);
    close(solution[v], -6e-6, 1e-16);
    close(circuit.capacitors.i_prev[0], 6e-6, 1e-16);
}

#[test]
fn physical_event_acceptance_requires_a_record_for_a_changed_gp_input() {
    let (engine, circuit, _, incoming, history) = fixture(
        "missing GP input event\nVc c 0 2\nVb b 0 DC .6 PWL(0 .6 1 .6 1 .64)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let before = history.clone();
    let failure = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(&[None]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .err()
        .unwrap();
    assert!(
        failure
            .to_string()
            .contains("without a physical event record"),
        "{failure}"
    );
    assert_eq!(history, before);
}

#[test]
fn physical_event_acceptance_keeps_native_gp_charge_rates_and_total_lead_currents() {
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rb in ["", "RB=100 RBM=20 IRB=1e-5", "RB=100 RBM=100"] {
            let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(&format!(
                "GP physical acceptance\nVc c 0 {}\nVb b 0 DC {} PWL(0 {} 1 {} 1 {})\nVe e 0 0\nQ1 c b e qm\n.model qm {kind}(IS=1e-16 BF=100 BR=2 VAF=20 VAR=10 TF=1n PTF=30 CJE=1p CJC=.2p XCJC=.4 {rb})\n.end\n",
                2.0 * polarity,
                0.6 * polarity,
                0.6 * polarity,
                0.6 * polarity,
                0.64 * polarity
            ));
            let before = history.clone();
            let point = engine
                .prepare_physical_event(
                    &circuit,
                    &history,
                    PhysicalEventStep {
                        incoming: &incoming,
                        time: 1.0,
                        dt: 1.0,
                        phase_events: PhysicalEventOrders::Declared(&[Some(
                            DelayEventOrder::Unknown,
                        )]),
                    },
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(history, before);
            assert_eq!(
                point.phase_current_couplings(),
                &[Some(charge_event::CurrentJumpCoupling::Cancels)]
            );
            let expected_currents = point.bjt.values[0].currents;
            let expected_charges = point.bjt.values[0].charges;
            let mut solution = point.state.solution.clone();
            let (discontinuity, _) = accept(
                &engine,
                &mut circuit,
                &mut matrix,
                &mut history,
                &point,
                &mut solution,
            )
            .unwrap();
            assert!(discontinuity);
            assert_eq!(history.charge_q_prev[0], expected_charges);
            assert_eq!(history.charge_cq_prev[0], expected_currents);
            let terminal = history.accepted_terminal_currents[0].unwrap();
            for (index, name) in ["Vc", "Vb", "Ve"].iter().enumerate() {
                let source = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .position(|source| source.eq_ignore_ascii_case(name))
                    .unwrap();
                let branch =
                    circuit.num_nodes() + circuit.voltage_sources.branch_indices[source] - 1;
                close(terminal[index], -solution[branch], 1e-11);
            }
            let phase = history.phase[0].as_ref().unwrap();
            assert_eq!(phase.accepted_sample_count(), 2);
            assert_eq!(
                phase.checkpoint().left_limits,
                vec![(1.0, point.left_limits[0].unwrap())]
            );
        }
    }
}

#[test]
fn physical_event_acceptance_rejects_stale_state_and_phase_without_history_rotation() {
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "stale physical event\nVc c 0 2\nVb b 0 DC .6 PWL(0 .6 1 .6 1 .64)\nQ1 c b 0 qm\nC1 b 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(&[Some(DelayEventOrder::Unknown)]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let mut solution = point.state.solution.clone();
    solution[0] += 0.01;
    let before = history.clone();
    let cap_before = circuit.capacitors.i_prev.clone();
    assert!(
        accept(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut history,
            &point,
            &mut solution
        )
        .is_err()
    );
    assert_eq!(history, before);
    assert_eq!(circuit.capacitors.i_prev, cap_before);
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    history.phase[0]
        .as_mut()
        .unwrap()
        .accept_sample(0.5, point.left_limits[0].unwrap(), delay, None)
        .unwrap();
    let advanced = history.clone();
    solution.clone_from(&point.state.solution);
    assert!(
        accept(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut history,
            &point,
            &mut solution
        )
        .is_err()
    );
    assert_eq!(history, advanced);
    assert_eq!(circuit.capacitors.i_prev, cap_before);
}

#[test]
fn physical_event_acceptance_preserves_per_device_orders_and_ordinary_samples() {
    use DelayEventOrder::{AtLeast, Unknown};
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(&format!(
            "ordered GP source corners\nVc c 0 0\nV1 b1 0 PWL(0 0 1 0 2 {})\nV2 b2 0 PWL(0 0 1 0 2 {})\nV3 b3 0 PWL(0 0 1 0 2 {})\nV4 b4 0 0\nQ1 c b1 0 qm\nQ2 c b2 0 qm\nQ3 c b3 0 qm\nQ4 c b4 0 qm\nQ5 c b4 0 plain\nC1 b1 0 1p\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.model plain {kind}(IS=1e-16)\n.end\n",
            0.1 * polarity,
            0.2 * polarity,
            0.3 * polarity
        ));
        // The three ideal base voltages are continuous at the start of their
        // ramps, so the GP forward input is continuous. Bound zero and Unknown
        // deliberately retain less information than this fixture establishes.
        let orders = [
            Some(AtLeast(1)),
            Some(AtLeast(0)),
            Some(Unknown),
            None,
            None,
        ];
        let before = history.clone();
        let point = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time: 1.0,
                    dt: 1.0,
                    phase_events: PhysicalEventOrders::Declared(&orders),
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .unwrap();
        assert_eq!(history, before);
        for (index, slope) in [0.1, 0.2, 0.3].into_iter().enumerate() {
            let base = circuit.bjts.devices[index].node_base - 1;
            close(
                point.state.coordinate_rates[base].unwrap(),
                slope * polarity,
                1e-14,
            );
        }
        let mut solution = point.state.solution.clone();
        assert!(
            accept(
                &engine,
                &mut circuit,
                &mut matrix,
                &mut history,
                &point,
                &mut solution
            )
            .unwrap()
            .0
        );
        for (index, expected) in orders[..4].iter().copied().enumerate() {
            let phase = history.phase[index].as_ref().unwrap();
            let image = phase.checkpoint();
            assert_eq!(image.left_limits.len(), usize::from(expected.is_some()));
            assert_eq!(
                image.event_orders,
                match expected {
                    Some(AtLeast(order)) => vec![(1.0, order)],
                    _ => vec![],
                }
            );
            let restored = DelayBuffer::from_checkpoint(image.clone()).unwrap();
            assert_eq!(restored.checkpoint(), image);
            let arrival = phase.next_event_after(1.0).unwrap();
            assert_eq!(arrival, restored.next_event_after(1.0).unwrap());
            assert_eq!(arrival.map(|event| event.order), expected);
            if let Some(arrival) = arrival {
                assert!(arrival.time > 1.0);
            }
        }
        assert!(history.phase[4].is_none());
        // This known current is reconstructed from the outgoing ramp rate,
        // despite an exactly zero capacitor voltage at the corner.
        close(circuit.capacitors.i_prev[0], polarity * 1e-13, 1e-25);
    }
}

#[test]
fn physical_event_acceptance_rejects_positive_order_on_a_later_gp_value_jump() {
    use DelayEventOrder::AtLeast;
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "inconsistent GP event order\nVc c 0 0\nV1 b1 0 PWL(0 0 1 0 2 .1)\nV2 b2 0 PWL(0 0 1 0 1 .01)\nQ1 c b1 0 qm\nQ2 c b2 0 qm\nC1 b1 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let before = history.clone();
    let caps = format!("{:?}", circuit.capacitors);
    let prepare = |orders: &[Option<DelayEventOrder>]| {
        engine.prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(orders),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
    };
    let error = prepare(&[Some(AtLeast(1)), Some(AtLeast(1))])
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("positive delay event order requires equal"),
        "{error}"
    );
    assert_eq!(history, before);
    assert_eq!(format!("{:?}", circuit.capacitors), caps);
    let point = prepare(&[Some(AtLeast(1)), Some(AtLeast(0))]).unwrap();
    let mut solution = point.state.solution.clone();
    accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    for (index, order) in [1, 0].into_iter().enumerate() {
        let image = history.phase[index].as_ref().unwrap().checkpoint();
        assert_eq!(image.event_orders, vec![(1.0, order)]);
        assert_eq!(image.left_limits.len(), 1);
        if order == 0 {
            assert_ne!(image.left_limits[0].1, image.samples.last().unwrap().1);
        }
    }
}

#[test]
fn physical_event_acceptance_keeps_order_metadata_private_until_final_validation() {
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "ordered GP rejected acceptance\nVc c 0 0\nVb b 0 PWL(0 0 1 0 2 .1)\nQ1 c b 0 qm\nC1 b 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(&[Some(DelayEventOrder::AtLeast(1))]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let before = history.clone();
    let caps = format!("{:?}", circuit.capacitors);
    let mut solution = point.state.solution.clone();
    let base = circuit.bjts.devices[0].node_base - 1;
    solution[base] = 0.01;
    let error = accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("prepared event does not match"),
        "{error}"
    );
    assert_eq!(history, before);
    assert_eq!(format!("{:?}", circuit.capacitors), caps);
    assert!(
        history.phase[0]
            .as_ref()
            .unwrap()
            .next_event_after(0.0)
            .unwrap()
            .is_none()
    );
    solution.clone_from(&point.state.solution);
    accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    let image = history.phase[0].as_ref().unwrap().checkpoint();
    assert_eq!(image.event_orders, vec![(1.0, 1)]);
    assert_eq!(image.left_limits, vec![(1.0, 0.0)]);
    assert_eq!(image.samples.len(), 2);
}

#[test]
fn physical_event_acceptance_preflights_order_storage_before_history_rotation() {
    use rspice_veriloga_runtime::transport_delay::MAX_DELAY_HISTORY_SAMPLES;
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "ordered GP storage budget\nVc c 0 0\nV1 b1 0 PWL(0 0 1 0 2 .1)\nV2 b2 0 PWL(0 0 1 0 2 .1)\nQ1 c b1 0 qm\nQ2 c b2 0 qm\nC1 b1 0 1p\n.model qm NPN(IS=1e-16 TF=4 PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let mut image = history.phase[1].as_ref().unwrap().checkpoint();
    // Both the existing samples and the new event are inside the configured
    // horizon. Two slots fit a right sample and left limit; the known-order
    // record needs a third slot and must be rejected during preparation.
    let count = MAX_DELAY_HISTORY_SAMPLES - 2;
    image.samples = (0..count)
        .map(|i| (i as Value / (2 * count) as Value, 0.0))
        .collect();
    history.phase[1] = Some(DelayBuffer::from_checkpoint(image).unwrap());
    let before = history.clone();
    let caps = format!("{:?}", circuit.capacitors);
    let prepare = |order| {
        engine.prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::Declared(&[
                    Some(DelayEventOrder::AtLeast(1)),
                    Some(order),
                ]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
    };
    let error = prepare(DelayEventOrder::AtLeast(1)).err().unwrap();
    assert!(
        error
            .to_string()
            .contains("delay history requires more than"),
        "{error}"
    );
    assert_eq!(history, before);
    assert_eq!(format!("{:?}", circuit.capacitors), caps);
    let point = prepare(DelayEventOrder::Unknown).unwrap();
    let mut solution = point.state.solution.clone();
    accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    assert_eq!(
        history.phase[0].as_ref().unwrap().checkpoint().event_orders,
        vec![(1.0, 1)]
    );
    let phase = history.phase[1].as_ref().unwrap();
    assert_eq!(phase.accepted_sample_count(), count + 1);
    assert_eq!(phase.accepted_left_limits().count(), 1);
    assert_eq!(phase.accepted_event_orders().count(), 0);
}

mod causal;
mod startup;
