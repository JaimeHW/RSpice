use super::*;
use crate::circuit::SourceTimeBasis;
use crate::engine::transient::source_events::PhysicalSourceEvents;
use rspice_veriloga_runtime::transport_delay::DelayEvent;

fn roots(circuit: &mut crate::CircuitData, stop: Value) -> PhysicalSourceEvents {
    circuit.set_independent_source_context(
        SourceTimeBasis {
            tstep: stop / 10.0,
            tstop: stop,
        },
        SpiceDialect::default(),
        options().limits,
    );
    Engine::collect_physical_source_events(circuit, stop, &options().limits, &NoAbort).unwrap()
}

#[test]
fn causal_event_orders_preserve_coordinates_and_accept_finite_source_currents() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rb in ["", "RB=100 RBM=20 IRB=1e-5"] {
            let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(&format!(
                "causal source corner\nVc c 0 {}\nVoffset bias 0 {}\nVb b bias PWL(0 0 1 0 2 {})\nI1 c 0 PWL(0 0 1 0 1 {})\nQ1 c b 0 qm\nC1 b 0 2p\n.model qm {kind}(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=.2p {rb})\n.end\n",
                p * 2.0,
                p * 0.6,
                p * 0.04,
                p * 1e-6
            ));
            let sources = roots(&mut circuit, 2.0);
            let before = history.clone();
            let point = engine
                .prepare_physical_event(
                    &circuit,
                    &history,
                    PhysicalEventStep {
                        incoming: &incoming,
                        time: 1.0,
                        dt: 1.0,
                        phase_events: PhysicalEventOrders::FromCauses {
                            sources: &sources,
                            accepted_time: 0.0,
                        },
                    },
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(history, before);
            assert_eq!(
                &point.state.solution[..circuit.num_nodes()],
                &incoming[..circuit.num_nodes()]
            );
            assert!(point.state.source_impulses.iter().all(|&q| q == 0.0));
            assert_eq!(
                point.bjt.values[0]
                    .phase_sample
                    .unwrap()
                    .event
                    .unwrap()
                    .order,
                DelayEventOrder::AtLeast(1)
            );
            let base = circuit.get_node_by_name("b").unwrap() - 1;
            close(point.state.coordinate_rates[base].unwrap(), p * 0.04, 1e-14);
            let cap_current = point.capacitors[0].current;
            close(cap_current, p * 8e-14, 1e-25);
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
                [(1.0, 1)]
            );
            assert_eq!(circuit.capacitors.i_prev[0], cap_current);
        }
    }
}

#[test]
fn causal_event_orders_keep_voltage_jumps_and_algebraic_current_feedback_at_zero() {
    for (source, connection) in [
        ("Vb b bias PWL(0 0 1 0 1 .04)", "Vc c 0 2"),
        (
            "Vb b bias 0\nI1 c 0 PWL(0 0 1 0 1 1n)",
            "Vc supply 0 2\nRc supply c 1k",
        ),
    ] {
        let (engine, mut circuit, _, incoming, history) = fixture(&format!(
            "causal algebraic jump\nVoffset bias 0 .6\n{source}\n{connection}\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=0)\n.end\n"
        ));
        let sources = roots(&mut circuit, 2.0);
        let point = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time: 1.0,
                    dt: 1.0,
                    phase_events: PhysicalEventOrders::FromCauses {
                        sources: &sources,
                        accepted_time: 0.0,
                    },
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .unwrap();
        let sample = point.bjt.values[0].phase_sample.unwrap();
        assert_eq!(sample.event.unwrap().order, DelayEventOrder::AtLeast(0));
        assert_ne!(sample.current, sample.event.unwrap().left_limit);
        assert_ne!(
            &point.state.solution[..circuit.num_nodes()],
            &incoming[..circuit.num_nodes()]
        );
    }
}

#[test]
fn causal_event_orders_use_each_tied_delay_owner_before_merging() {
    for jumping in [0, 1] {
        let (engine, mut circuit, _, incoming, mut history) = fixture(
            "causal tied GP arrivals\nVc c1 0 2\nVb b 0 .6\nVs supply 0 2\nRc supply c2 1k\nQ1 c1 b 0 qm\nQ2 c2 b 0 qm\n.model qm NPN(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=0)\n.end\n",
        );
        let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
        let accepted_time = delay * 0.75;
        let sources = roots(&mut circuit, delay * 2.0);
        for (index, phase) in history.phase.iter_mut().enumerate() {
            let old = phase
                .as_ref()
                .unwrap()
                .accepted_samples()
                .next_back()
                .unwrap()
                .1;
            let mut replacement = DelayBuffer::new(0);
            replacement
                .accept_event(
                    0.0,
                    DelayEvent {
                        left: old,
                        right: old + if jumping == index { 1e-10 } else { 0.0 },
                        order: if jumping == index {
                            DelayEventOrder::Unknown
                        } else {
                            DelayEventOrder::AtLeast(1)
                        },
                    },
                    delay,
                    None,
                )
                .unwrap();
            // Retain a settled present input independently of the old event
            // arriving now. The event operator does not qualify the earlier
            // trajectory; its interpolation contract still uses these knots.
            replacement
                .accept_sample(delay * 0.5, old, delay, None)
                .unwrap();
            replacement
                .accept_sample(accepted_time, old, delay, None)
                .unwrap();
            *phase = Some(replacement);
        }
        let before = history.clone();
        let next = bjt::arrival::next(&circuit, &history, accepted_time, delay, &NoAbort)
            .unwrap()
            .unwrap();
        assert_eq!(next.order, DelayEventOrder::Unknown);
        let missed_time = delay.next_up();
        let missed = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time: missed_time,
                    dt: missed_time - accepted_time,
                    phase_events: PhysicalEventOrders::FromCauses {
                        sources: &sources,
                        accepted_time,
                    },
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .err()
            .unwrap();
        assert!(
            missed.to_string().contains("unprocessed physical delay"),
            "{missed}"
        );
        let point = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time: delay,
                    dt: delay - accepted_time,
                    phase_events: PhysicalEventOrders::FromCauses {
                        sources: &sources,
                        accepted_time,
                    },
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .unwrap();
        let expected = if jumping == 0 {
            DelayEventOrder::AtLeast(1)
        } else {
            DelayEventOrder::Unknown
        };
        for value in &point.bjt.values {
            assert_eq!(value.phase_sample.unwrap().event.unwrap().order, expected);
        }
        if jumping == 0 {
            assert_eq!(
                &point.state.solution[..circuit.num_nodes()],
                &incoming[..circuit.num_nodes()]
            );
            let branch = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
            close(
                point.state.solution[branch] - incoming[branch],
                -1e-10,
                1e-16,
            );
        } else {
            let c2 = circuit.get_node_by_name("c2").unwrap() - 1;
            close(point.state.solution[c2] - incoming[c2], -1e-7, 1e-10);
        }
        assert_eq!(history, before);
        for phase in history.phase.iter_mut().flatten() {
            *phase = DelayBuffer::from_checkpoint(phase.checkpoint()).unwrap();
        }
        let repeated = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time: delay,
                    dt: delay - accepted_time,
                    phase_events: PhysicalEventOrders::FromCauses {
                        sources: &sources,
                        accepted_time,
                    },
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .unwrap();
        assert_eq!(repeated.state.solution, point.state.solution);
        assert_eq!(
            repeated.state.coordinate_rates,
            point.state.coordinate_rates
        );
    }
}

#[test]
fn causal_event_orders_refuse_skipped_clocks_and_an_inaccurate_continuous_limit() {
    let (engine, mut circuit, _, incoming, history) = fixture(
        "causal limit audit\nVc c 0 2\nVoffset bias 0 .6\nVb b bias PWL(0 0 1 0 2 .04)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let sources = roots(&mut circuit, 2.0);
    let before = history.clone();
    for (time, expected) in [
        (0.5, "no physical"),
        (1.0_f64.next_up(), "unprocessed physical source"),
    ] {
        let error = engine
            .prepare_physical_event(
                &circuit,
                &history,
                PhysicalEventStep {
                    incoming: &incoming,
                    time,
                    dt: time,
                    phase_events: PhysicalEventOrders::FromCauses {
                        sources: &sources,
                        accepted_time: 0.0,
                    },
                },
                &options(),
                1e-20,
                &NoAbort,
            )
            .err()
            .unwrap();
        assert!(error.to_string().contains(expected), "{error}");
    }
    let mut bad = incoming.clone();
    bad[circuit.get_node_by_name("b").unwrap() - 1] += 1e-4;
    let error = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &bad,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .err()
        .unwrap();
    assert!(
        error.to_string().contains("continuous event limit"),
        "{error}"
    );
    assert_eq!(history, before);
}

#[test]
fn causal_event_orders_do_not_infer_regularity_from_equal_values_at_a_gp_join() {
    let (engine, mut circuit, _, incoming, history) = fixture(
        "uncertified diffusion join\nVc c 0 0\nVb b 0 PWL(0 0 1 0 2 .04)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let sources = roots(&mut circuit, 2.0);
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let sample = point.bjt.values[0].phase_sample.unwrap();
    assert_eq!(sample.current, sample.event.unwrap().left_limit);
    assert_eq!(sample.event.unwrap().order, DelayEventOrder::AtLeast(0));
}

#[test]
fn causal_event_orders_preserve_history_on_resource_abort_and_clock_refusals() {
    use crate::abort_signal::CountingAbort;
    let (engine, mut circuit, _, incoming, history) = fixture(
        "causal refusal ownership\nVc c 0 2\nVo bias 0 .6\nVb b bias PWL(0 0 1 0 2 .04)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let sources = roots(&mut circuit, 2.0);
    let before = history.clone();
    let step = |dt| PhysicalEventStep {
        incoming: &incoming,
        time: 1.0,
        dt,
        phase_events: PhysicalEventOrders::FromCauses {
            sources: &sources,
            accepted_time: 0.0,
        },
    };
    let count = CountingAbort::new(usize::MAX);
    let _point = engine
        .prepare_physical_event(&circuit, &history, step(1.0), &options(), 1e-20, &count)
        .unwrap();
    for threshold in [1, count.count() / 2, count.count() - 1] {
        assert!(matches!(
            engine.prepare_physical_event(
                &circuit,
                &history,
                step(1.0),
                &options(),
                1e-20,
                &CountingAbort::new(threshold)
            ),
            Err(SimulationError::Aborted)
        ));
    }
    let mut small = options();
    small.limits.max_result_values = 1;
    assert!(
        engine
            .prepare_physical_event(&circuit, &history, step(1.0), &small, 1e-20, &NoAbort)
            .is_err()
    );
    let error = engine
        .prepare_physical_event(&circuit, &history, step(0.1), &options(), 1e-20, &NoAbort)
        .err()
        .unwrap();
    assert!(error.to_string().contains("accepted clock"), "{error}");
    assert_eq!(history, before);
}

#[test]
fn causal_event_orders_do_not_reuse_waveform_continuity_for_the_dc_startup_transition() {
    let (engine, mut circuit, _, incoming, history) = fixture(
        "causal startup owner\nVc c 0 2\nVo bias 0 .6\nVb b bias PWL(0 0 1 .04)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let sources = roots(&mut circuit, 2.0);
    assert_eq!(
        sources.at(0.0).unwrap()[0].order,
        DelayEventOrder::AtLeast(1)
    );
    let error = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 0.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("DC-to-transient transition owner"),
        "{error}"
    );
}

#[test]
fn gp_startup_phase_seed_matches_the_physical_solution() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rb in ["", "RC=2 RE=1 RB=100 RBM=100", "RB=100 RBM=20 IRB=1e-5"] {
            let (_, mut circuit, _, incoming, history) = fixture(&format!(
                "GP startup input ownership\nVc c 0 {}\nVo bias 0 {}\nVb b bias PWL(0 0 1 {})\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p {rb})\n.end\n",
                p * 2.0,
                p * 0.6,
                p * 0.04
            ));
            let _sources = roots(&mut circuit, 2.0);
            let mut sampler =
                PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
            let actual = sampler.forward_inputs(&incoming, &NoAbort).unwrap()[0].unwrap();
            let phase = history.phase[0].as_ref().unwrap();
            let seeded = phase.accepted_samples().next_back().unwrap().1;
            assert_eq!(
                seeded,
                actual,
                "{kind} {rb}: seed internal {:?}, physical internal {:?}",
                history.dynamic_internal_prev[0].map(Value::to_bits),
                sampler.models()[0]
                    .mna_internal_state_at_solution(&incoming)
                    .map(Value::to_bits)
            );
            for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
                sampler
                    .sample(
                        0.0,
                        side,
                        &incoming,
                        &[Some(EventPhase {
                            history: phase,
                            endpoint: actual,
                        })],
                        &options(),
                        &NoAbort,
                    )
                    .unwrap();
            }
        }
    }
}

#[test]
fn gp_startup_phase_seed_respects_adjacent_biases_without_changing_newton_cache() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for common in [0.0, 1024.0] {
            let (_, circuit, _, incoming, _) = fixture(&format!(
                "GP exact startup bias\nVe e 0 {common}\nVc c e {}\nVb b e {}\nQ1 c b e qm\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
                p * 2.0,
                p * 0.6
            ));
            let cache = circuit.bjts.devices[0]
                .accepted_nonlinear_checkpoint()
                .unwrap();
            let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
            let base = circuit.get_node_by_name("b").unwrap() - 1;
            let mut currents = Vec::new();
            for voltage in [
                incoming[base].next_down(),
                incoming[base],
                incoming[base].next_up(),
            ] {
                let mut point = incoming.clone();
                point[base] = voltage;
                let mut history = Engine::initialize_bjt_history(
                    &circuit,
                    &point,
                    ReactiveHistorySeed::SolvedBias,
                );
                Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
                let (charges, internal, _) =
                    sampler.models()[0].mna_charge_state_at_solution(&point);
                assert_eq!(
                    history.dynamic_internal_prev[0], internal,
                    "{kind} at {common}"
                );
                assert_eq!(history.charge_q_prev[0], charges.map(|q| q.charge));
                let expected = sampler.forward_inputs(&point, &NoAbort).unwrap()[0].unwrap();
                assert_eq!(
                    history.phase[0]
                        .as_ref()
                        .unwrap()
                        .accepted_samples()
                        .next_back()
                        .unwrap()
                        .1,
                    expected
                );
                currents.push(expected);
                assert_eq!(
                    circuit.bjts.devices[0]
                        .accepted_nonlinear_checkpoint()
                        .unwrap(),
                    cache
                );
            }
            assert!(currents.windows(2).all(|pair| pair[0] != pair[1]));
        }
    }
}

#[test]
fn gp_startup_phase_seed_keeps_authored_uic_separate_from_solved_bias() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        let (_, circuit, _, incoming, _) = fixture(&format!(
            "GP startup IC ownership\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0 qm IC={},{}\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
            p * 2.0,
            p * 0.6,
            p * 0.4,
            p * 1.1
        ));
        let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
        for seed in [
            ReactiveHistorySeed::SolvedBias,
            ReactiveHistorySeed::UicStartup,
        ] {
            let mut expected_point = incoming.clone();
            if seed == ReactiveHistorySeed::UicStartup {
                expected_point[circuit.get_node_by_name("b").unwrap() - 1] = p * 0.4;
                expected_point[circuit.get_node_by_name("c").unwrap() - 1] = p * 1.1;
            }
            let mut history = Engine::initialize_bjt_history(&circuit, &incoming, seed);
            Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
            let expected = sampler.forward_inputs(&expected_point, &NoAbort).unwrap()[0].unwrap();
            assert_eq!(
                history.phase[0]
                    .as_ref()
                    .unwrap()
                    .accepted_samples()
                    .next_back()
                    .unwrap()
                    .1,
                expected
            );
            let (charges, internal, _) =
                sampler.models()[0].mna_charge_state_at_solution(&expected_point);
            assert_eq!(history.dynamic_internal_prev[0], internal);
            assert_eq!(history.charge_q_prev[0], charges.map(|q| q.charge));
        }
    }
}
