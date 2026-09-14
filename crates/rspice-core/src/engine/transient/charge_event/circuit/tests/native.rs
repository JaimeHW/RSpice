use super::*;

#[test]
fn prepared_event_circuit_solves_biased_gp_jump_with_canonical_rbi_ports() {
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let (circuit, incoming) = dc(&format!(
            "biased GP jump\nVc c 0 {}\nVb b 0 DC {} PWL(0 {} 1 {} 1 {})\nVe e 0 0\nQ1 c b e qm\n.model qm {kind}(IS=1e-16 BF=100 BR=2 VAF=20 VAR=10 IKF=.01 IKR=.02 TF=1n PTF=30 CJE=1p CJC=.2p RB=100 RBM=20 IRB=1e-5 RC=2 RE=1)\n.end\n",
            2.0 * polarity,
            0.6 * polarity,
            0.6 * polarity,
            0.6 * polarity,
            0.64 * polarity
        ));
        let options = options();
        let original = &circuit.bjts.devices[0];
        assert!(original.mna_promoted() && original.needs_mna_rbi_branch());
        let before = original.accepted_nonlinear_checkpoint().unwrap();
        let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
        let forward = sampler.forward_inputs(&incoming, &NoAbort).unwrap()[0].unwrap();
        assert!(forward.abs() > 1e-8);
        let mut history = DelayBuffer::new(0);
        history
            .restore_checkpoint(&DelayCheckpoint {
                configuration: Some(DelayConfiguration::Fixed {
                    delay: original.legacy_excess_phase_delay(),
                }),
                samples: vec![(0.0, forward)],
                left_limits: vec![],
            })
            .unwrap();
        let checkpoint = history.checkpoint();
        let phase = [Some(EventPhase {
            history: &history,
            endpoint: forward,
        })];
        let left = sampler
            .sample(
                1.0,
                SourceTimeSide::LeftLimit,
                &incoming,
                &phase,
                &options,
                &NoAbort,
            )
            .unwrap();
        // Independent DC residual includes finite ideal-source currents.
        let sources = sampler
            .topology(1.0, SourceTimeSide::LeftLimit, &options, &NoAbort)
            .unwrap();
        for row in 0..circuit.num_nodes() {
            let mut residual = left.f.values[row];
            for source in &sources.sources {
                if source.positive == row + 1 {
                    residual += incoming[source.branch];
                }
                if source.negative == row + 1 {
                    residual -= incoming[source.branch];
                }
            }
            assert!(
                residual.abs() < 1e-11,
                "biased DC KCL row {row}: {residual:e}"
            );
        }
        let topology = sampler
            .topology(1.0, SourceTimeSide::RightLimit, &options, &NoAbort)
            .unwrap();
        let outgoing = topology
            .solve(&incoming, &left.q.values, &options, &NoAbort, |state, _| {
                sampler.sample(
                    1.0,
                    SourceTimeSide::RightLimit,
                    state,
                    &phase,
                    &options,
                    &NoAbort,
                )
            })
            .unwrap();
        let b = circuit.get_node_by_name("b").unwrap() - 1;
        close(outgoing.solution[b], 0.64 * polarity, 1e-12);
        assert_eq!(history.checkpoint(), checkpoint);
        assert_eq!(original.accepted_nonlinear_checkpoint().unwrap(), before);
    }
}

fn dc(text: &str) -> (crate::CircuitData, Vec<Value>) {
    let deck = crate::Netlist::parse(text).unwrap();
    let mut engine = crate::Engine::default();
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
    let solution = engine
        .solve_dc_operating_point(&deck, &mut circuit, &mut matrix)
        .unwrap();
    (circuit, solution)
}

fn bug805(text: &str) {
    // Execute each original source deck through the production builder and
    // DC owner. This proves the start event, not its 80-us delayed trajectory.
    let (circuit, incoming) = dc(text);
    assert_eq!(circuit.bjts.len(), 1);
    let model = &circuit.bjts.devices[0];
    assert!(model.mna_promoted() && model.needs_mna_rbi_branch());
    let before = model.accepted_nonlinear_checkpoint().unwrap();
    let delay = model.legacy_excess_phase_delay();
    close(delay, 1.487_816_392_838_078e-10, 1e-25);
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let forward = sampler.forward_inputs(&incoming, &NoAbort).unwrap()[0].unwrap();
    let mut history = DelayBuffer::new(0);
    history
        .restore_checkpoint(&DelayCheckpoint {
            configuration: Some(DelayConfiguration::Fixed { delay }),
            samples: vec![(0.0, forward)],
            left_limits: vec![],
        })
        .unwrap();
    let checkpoint = history.checkpoint();
    let phase = [Some(EventPhase {
        history: &history,
        endpoint: forward,
    })];
    let left = sampler
        .sample(
            0.0,
            SourceTimeSide::LeftLimit,
            &incoming,
            &phase,
            &options,
            &NoAbort,
        )
        .unwrap();
    for row in 0..circuit.num_nodes() {
        let source_current: Value = circuit
            .voltage_sources
            .branch_indices
            .iter()
            .enumerate()
            .map(|(index, ordinal)| {
                let sign = Value::from(circuit.voltage_sources.node_pos[index] == row + 1)
                    - Value::from(circuit.voltage_sources.node_neg[index] == row + 1);
                sign * incoming[circuit.num_nodes() + ordinal - 1]
            })
            .sum();
        assert!(
            (left.f.values[row] + source_current).abs() < 1e-11,
            "DC KCL row {row}"
        );
    }
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let outgoing = topology
        .solve(&incoming, &left.q.values, &options, &NoAbort, |state, _| {
            sampler.sample(
                0.0,
                SourceTimeSide::RightLimit,
                state,
                &phase,
                &options,
                &NoAbort,
            )
        })
        .unwrap();
    for (left, right) in incoming[..circuit.num_nodes()]
        .iter()
        .zip(&outgoing.solution)
    {
        close(*right, *left, 1e-10);
    }
    let collector = circuit.get_node_by_name("q2c").unwrap() - 1;
    close(
        outgoing.coordinate_rates[collector].unwrap(),
        7.4 / 5e-9,
        1e-5,
    );
    assert!(outgoing.source_impulses.iter().all(|q| q.abs() < 1e-23));
    assert_eq!(history.checkpoint(), checkpoint);
    assert_eq!(model.accepted_nonlinear_checkpoint().unwrap(), before);
}

#[test]
fn prepared_event_circuit_bug805_original_alias_1_startup() {
    bug805(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir"
    )));
}
#[test]
fn prepared_event_circuit_bug805_original_alias_2_startup() {
    bug805(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir"
    )));
}
#[test]
fn prepared_event_circuit_bug805_original_alias_3_startup() {
    bug805(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir"
    )));
}

#[test]
fn prepared_event_circuit_gp_uses_held_physical_jacobian_and_both_history_sides() {
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let (circuit, mut state) = dc(&format!(
            "held GP event\nVc c 0 {}\nVb b 0 {}\nVe e 0 0\nQ1 c b e qm\n.model qm {kind}(IS=1e-16 BF=100 VAF=20 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
            2.0 * polarity,
            0.6 * polarity
        ));
        let options = options();
        let original = &circuit.bjts.devices[0];
        let before = original.accepted_nonlinear_checkpoint().unwrap();
        let delay = original.legacy_excess_phase_delay();
        let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
        let b = circuit.get_node_by_name("b").unwrap() - 1;
        let c = circuit.get_node_by_name("c").unwrap() - 1;
        let mut input_at = |base| {
            state[b] = polarity * base;
            sampler.forward_inputs(&state, &NoAbort).unwrap()[0].unwrap()
        };
        let start = input_at(0.55);
        let left = input_at(0.57);
        let right = input_at(0.60);
        let endpoint = input_at(0.62);
        let present = input_at(0.64);
        let mut history = DelayBuffer::new(0);
        history
            .restore_checkpoint(&DelayCheckpoint {
                configuration: Some(DelayConfiguration::Fixed { delay }),
                samples: vec![(0.0, start)],
                left_limits: vec![],
            })
            .unwrap();
        history
            .accept_discontinuity(delay, left, right, delay, None)
            .unwrap();
        let checkpoint = history.checkpoint();
        let phase = [Some(EventPhase {
            history: &history,
            endpoint,
        })];
        let time = 2.0 * delay;
        let mut quasistatic = EventSample::new(state.len(), &options).unwrap();
        sampler.models[0].stamp_periodic_fq(&state, &mut quasistatic.f, &mut quasistatic.q);
        for (side, delayed, rate) in [
            (SourceTimeSide::LeftLimit, left, (left - start) / delay),
            (
                SourceTimeSide::RightLimit,
                right,
                (endpoint - right) / delay,
            ),
        ] {
            let sample = sampler
                .sample(time, side, &state, &phase, &options, &NoAbort)
                .unwrap();
            close(
                sample.f.values[c] - quasistatic.f.values[c],
                delayed - present,
                1e-16,
            );
            close(sample.f_time[c], rate, 1e-6);
            // Perturb the present input while independently holding the
            // incoming endpoint fixed. This distinguishes -JF from (a-1)JF.
            let step = 1e-7;
            let mut plus = state.clone();
            let mut minus = state.clone();
            plus[b] += step;
            minus[b] -= step;
            let fp = sampler
                .sample(time, side, &plus, &phase, &options, &NoAbort)
                .unwrap()
                .f
                .values[c];
            let fm = sampler
                .sample(time, side, &minus, &phase, &options, &NoAbort)
                .unwrap()
                .f
                .values[c];
            close(entry(&sample.f, c, b), (fp - fm) / (2.0 * step), 1e-9);
        }
        assert_eq!(history.checkpoint(), checkpoint);
        assert_eq!(original.accepted_nonlinear_checkpoint().unwrap(), before);
        assert!(
            sampler
                .sample(
                    time,
                    SourceTimeSide::RightLimit,
                    &state,
                    &[None],
                    &options,
                    &NoAbort
                )
                .is_err()
        );
        let mut wrong = DelayBuffer::new(0);
        wrong
            .restore_checkpoint(&DelayCheckpoint {
                configuration: Some(DelayConfiguration::Fixed { delay: 2.0 * delay }),
                samples: vec![(0.0, start)],
                left_limits: vec![],
            })
            .unwrap();
        let failure = sampler
            .sample(
                time,
                SourceTimeSide::RightLimit,
                &state,
                &[Some(EventPhase {
                    history: &wrong,
                    endpoint,
                })],
                &options,
                &NoAbort,
            )
            .err()
            .unwrap();
        assert!(
            failure.to_string().contains("another nominal delay"),
            "{failure}"
        );
    }
}
