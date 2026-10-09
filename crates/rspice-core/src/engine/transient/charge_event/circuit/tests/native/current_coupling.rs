use super::*;

#[test]
fn constant_bias_charge_preserves_resistive_current_fanout() {
    for (storage, bias) in [
        ("", "VB b 0 .7"),
        ("C1 b 0 1p", "VB b bias .5\nVX bias 0 .2"),
        (
            "D1 b 0 dm\n.model dm D IS=1e-16 CJO=2p",
            "VB b 0 DC .7 AC 1",
        ),
    ] {
        let (circuit, incoming) = dc(&format!(
            "constant bias current fanout\nVC c 0 2\n{bias}\n{storage}\nI1 b 0 PWL(0 0 1 0 1 1u 2 3u)\nF1 copy 0 VB -2\nR1 copy 0 1k\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 BF=100 TF=1n PTF=30\n.end\n",
        ));
        let options = options();
        let mut sampler =
            PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
        let input = sampler.forward_inputs(&incoming, &NoAbort).unwrap()[0].unwrap();
        let mut history = DelayBuffer::new(0);
        history
            .restore_checkpoint(&DelayCheckpoint {
                event_orders: Vec::new(),
                configuration: Some(DelayConfiguration::Fixed {
                    delay: sampler.models()[0].legacy_excess_phase_delay(),
                }),
                samples: vec![(0.0, input)],
                left_limits: vec![],
            })
            .unwrap();
        let phase = [Some(EventPhase {
            history: &history,
            endpoint: input,
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
        let topology = sampler
            .topology(1.0, SourceTimeSide::RightLimit, &options, &NoAbort)
            .unwrap();
        let trial = topology.physical_probe(&incoming);
        let physical = sampler
            .sample(
                1.0,
                SourceTimeSide::RightLimit,
                &trial,
                &phase,
                &options,
                &NoAbort,
            )
            .unwrap();
        topology
            .jump_equations(&trial, &left.q.values, &physical, &options, &NoAbort)
            .unwrap()
            .solve(&options, &NoAbort)
            .expect("nonsingular jump constraints for constant-bias fanout");
        let outgoing = topology
            .solve(
                &incoming,
                &left.q.values,
                &options,
                &NoAbort,
                |state, abort| {
                    sampler.sample(
                        1.0,
                        SourceTimeSide::RightLimit,
                        state,
                        &phase,
                        &options,
                        abort,
                    )
                },
            )
            .unwrap();
        let copy = circuit.get_node_by_name("copy").unwrap() - 1;
        close(outgoing.solution[copy] - incoming[copy], -2e-3, 1e-12);
        close(outgoing.coordinate_rates[copy].unwrap(), -4e-3, 1e-12);
        assert!(outgoing.source_impulses.iter().all(|q| q.abs() < 1e-25));
        // Inconsistent incoming charge would send an impulse through the CCCS
        // into a resistor. The finite-voltage reduction must still reject it.
        let mut changed_charge = left.q.values;
        changed_charge[circuit.get_node_by_name("b").unwrap() - 1] += 1e-12;
        let failure = topology
            .solve(
                &incoming,
                &changed_charge,
                &options,
                &NoAbort,
                |state, abort| {
                    sampler.sample(
                        1.0,
                        SourceTimeSide::RightLimit,
                        state,
                        &phase,
                        &options,
                        abort,
                    )
                },
            )
            .err()
            .unwrap();
        assert!(
            failure
                .to_string()
                .contains("nodal charge conservation failed"),
            "{failure}"
        );
    }
}

#[test]
fn current_event_coupling_gp_delay_has_storage_and_algebraic_feedback_cases() {
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        for cjc in ["0", ".2p"] {
            for resistance in ["", "RC=2 RE=1 RB=100 RBM=20 IRB=1e-5"] {
                let (circuit, incoming) = dc(&format!(
                    "GP delayed forcing\nVcc supply 0 {}\nRc supply c 1k\nVb b 0 {}\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 BF=100 BR=2 VAF=20 VAR=10 TF=1n PTF=30 CJE=1p CJC={cjc} {resistance})\n.end\n",
                    2.0 * polarity,
                    0.6 * polarity
                ));
                let options = options();
                let original = &circuit.bjts.devices[0];
                let original_state = original.accepted_nonlinear_checkpoint().unwrap();
                let delay = original.legacy_excess_phase_delay();
                let mut sampler =
                    PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort)
                        .unwrap();
                let input = sampler.forward_inputs(&incoming, &NoAbort).unwrap()[0].unwrap();
                let delta = polarity * 1e-5;
                let mut history = DelayBuffer::new(0);
                // Manufactured accepted delay history: its incoming arrival
                // value reproduces the independently solved production DC.
                // This isolates an arriving current jump, not a source jump
                // or a finite-duration transient companion.
                history
                    .restore_checkpoint(&DelayCheckpoint {
                        event_orders: Vec::new(),
                        configuration: Some(DelayConfiguration::Fixed { delay }),
                        samples: vec![(0.0, input), (delay, input + delta)],
                        left_limits: vec![(delay, input)],
                    })
                    .unwrap();
                let before = history.checkpoint();
                let phase = [Some(EventPhase {
                    history: &history,
                    endpoint: input,
                })];
                let time = 2.0 * delay;
                let left = sampler
                    .sample(
                        time,
                        SourceTimeSide::LeftLimit,
                        &incoming,
                        &phase,
                        &options,
                        &NoAbort,
                    )
                    .unwrap();
                let topology = sampler
                    .topology(time, SourceTimeSide::RightLimit, &options, &NoAbort)
                    .unwrap();
                let (p, n) = sampler.models()[0]
                    .legacy_forward_transport_nodes()
                    .unwrap();
                assert_eq!(
                    topology.current_jump_coupling(p, n).unwrap(),
                    if cjc == "0" {
                        CurrentJumpCoupling::Present
                    } else {
                        CurrentJumpCoupling::Cancels
                    }
                );
                // The incidence must agree with the current branch even
                // when RC/RE and nonlinear RBI own promoted private nodes.
                let model = &sampler.models()[0];
                let branch = model
                    .legacy_forward_transport_branch(
                        &model.mna_internal_state_at_solution(&incoming),
                    )
                    .unwrap();
                let external = [
                    model.node_collector,
                    model.node_base,
                    model.node_emitter,
                    model.node_substrate,
                ];
                let node = |i: Option<usize>, e: Option<usize>| {
                    i.map(|i| model.mna_internal_node(i))
                        .or_else(|| e.map(|e| external[e]))
                        .unwrap()
                };
                assert_eq!(
                    (p, n),
                    (
                        node(branch.pos_internal, branch.pos_external),
                        node(branch.neg_internal, branch.neg_external)
                    )
                );
                let outgoing = topology
                    .solve(&incoming, &left.q.values, &options, &NoAbort, |state, _| {
                        sampler.sample(
                            time,
                            SourceTimeSide::RightLimit,
                            state,
                            &phase,
                            &options,
                            &NoAbort,
                        )
                    })
                    .unwrap();
                let output_input = sampler
                    .forward_inputs(&outgoing.solution, &NoAbort)
                    .unwrap()[0]
                    .unwrap();
                let collector = circuit.get_node_by_name("c").unwrap() - 1;
                if cjc == "0" {
                    // A stateless collector responds algebraically. VAF
                    // couples that changed voltage back into forward input,
                    // so declaring every GP arrival smoother would be wrong.
                    assert!(
                        polarity * (outgoing.solution[collector] - incoming[collector]) < -1e-3
                    );
                    assert!((output_input - input).abs() > 1e-10 * input.abs());
                    if resistance.is_empty() {
                        close(
                            outgoing.solution[collector] - incoming[collector],
                            -1000.0 * delta,
                            1e-9,
                        );
                        // With fixed Vbe and XTF=0, only forward diffusion
                        // charge changes: Delta Qbe = TF * Delta If. Its
                        // impulse is supplied by the base voltage source.
                        let base = circuit
                            .voltage_sources
                            .names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case("Vb"))
                            .unwrap();
                        let branch =
                            circuit.num_nodes() + circuit.voltage_sources.branch_indices[base] - 1;
                        let impulse = topology
                            .source_branches()
                            .zip(&outgoing.source_impulses)
                            .find(|(coordinate, _)| *coordinate == branch)
                            .unwrap()
                            .1;
                        assert!(impulse.abs() > 1e-23);
                        close(*impulse, -1e-9 * (output_input - input), 1e-25);
                    }
                } else {
                    for (left, right) in incoming[..circuit.num_nodes()]
                        .iter()
                        .zip(&outgoing.solution)
                    {
                        close(*right, *left, 1e-10);
                    }
                    close(output_input, input, 1e-15);
                    assert!(outgoing.coordinate_rates[p - 1].unwrap().abs() > 1e4);
                    assert!(outgoing.source_impulses.iter().all(|q| q.abs() < 1e-23));
                }
                assert_eq!(history.checkpoint(), before);
                assert_eq!(
                    original.accepted_nonlinear_checkpoint().unwrap(),
                    original_state
                );
            }
        }
    }
}
