use super::*;

#[test]
fn physical_startup_observation_preflight_refusal_preserves_all_model_targets() {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(
        "startup observation refusal\nVc c 0 DC 0 PWL(0 1 1 1)\nC1 c 0 1p\nR1 c 0 1k\nVb b 0 .6\nQ1 0 b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.end\n",
    );
    let before_solution = solution.clone();
    let before_history = history.clone();
    let before_caps = (
        circuit.capacitors.v_prev.clone(),
        circuit.capacitors.i_prev.clone(),
    );
    let before_models = circuit
        .bjts
        .devices
        .iter()
        .map(|bjt| bjt.accepted_nonlinear_checkpoint().unwrap())
        .collect::<Vec<_>>();
    let mut observed = false;
    let result = engine.transition_physical_startup_with_observation(
        PhysicalStartupTargets {
            circuit: &mut circuit,
            solution: &mut solution,
            history: &mut history,
        },
        &options(),
        1e-20,
        &NoAbort,
        |point| {
            observed = true;
            assert!(point.impulses().any(|(_, charge)| charge != 0.0));
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ResultValues,
                11,
                10,
            )
            .map_err(SimulationError::from)
        },
    );
    assert!(observed);
    assert!(matches!(result, Err(SimulationError::ResourceLimit(_))));
    assert_eq!(solution, before_solution);
    assert_eq!(history, before_history);
    assert_eq!(
        (
            circuit.capacitors.v_prev.clone(),
            circuit.capacitors.i_prev.clone()
        ),
        before_caps
    );
    assert_eq!(
        circuit
            .bjts
            .devices
            .iter()
            .map(|bjt| bjt.accepted_nonlinear_checkpoint().unwrap())
            .collect::<Vec<_>>(),
        before_models
    );
}

#[test]
fn physical_startup_solves_coupled_rates_in_the_selected_gp_charge_chart() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for direction in [-1.0, 1.0] {
            let injected = p * direction * 1e-3;
            let (engine, mut circuit, _, mut solution, _) = fixture(&format!(
                "coupled GP diffusion derivative\nVc c 0 {}\nRb b 0 1k\nIb 0 b DC 0 PWL(0 {injected} 1 {injected})\nQ1 c b 0 qm\n.model qm {kind}(IS=1u TF=1 PTF=30 VAF=10 XTF=2)\n.end\n",
                p * 2.0,
            ));
            let base = circuit.get_node_by_name("b").unwrap() - 1;
            // The selected storage is at the constitutive join. The base
            // voltage rate is a solved unknown, not prescribed by a source.
            solution[base] = 0.0;
            let mut history = Engine::initialize_bjt_history(
                &circuit,
                &solution,
                ReactiveHistorySeed::SolvedBias,
            );
            Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
            let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
            let static_base = sampler.models()[0].mna_terminal_currents_at_solution(&solution)[1];
            let capacitance =
                1e-6 / circuit.bjts.devices[0].vt * if direction > 0.0 { 3.6 } else { 1.0 };
            if direction > 0.0 {
                let mut limited = options();
                limited.iterations = 1;
                let (mut target, mut voltage, mut memory) =
                    (circuit.clone(), solution.clone(), history.clone());
                let error = engine
                    .transition_physical_startup(
                        &mut target,
                        &mut voltage,
                        &mut memory,
                        &limited,
                        1e-20,
                        &NoAbort,
                    )
                    .err()
                    .unwrap()
                    .to_string();
                assert!(
                    error.contains("charge directions did not settle"),
                    "{error}"
                );
                assert_eq!(voltage, solution);
                assert_eq!(memory, history);
            }
            let report = engine
                .transition_physical_startup(
                    &mut circuit,
                    &mut solution,
                    &mut history,
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(solution[base], 0.0);
            close(
                report.coordinate_rates[base].unwrap(),
                (injected - static_base) / capacitance,
                1e-12,
            );
            close(
                history.charge_cq_prev[0][BJT_QBE_BRANCH_INDEX],
                injected - static_base,
                1e-15,
            );
            close(
                history.accepted_terminal_currents[0].unwrap()[1],
                injected,
                1e-15,
            );
        }
    }
}

#[test]
fn physical_startup_uses_outgoing_gp_diffusion_tangent_at_zero_bias() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for direction in [-1.0, 1.0] {
            for xtf in [0.0, 2.0] {
                let (engine, mut circuit, _, mut solution, mut history) = fixture(&format!(
                    "sided GP diffusion derivative\nVc c 0 {}\nVb b 0 DC 0 PWL(0 0 1 {})\nQ1 c b 0 qm\n.model qm {kind}(IS=1u TF=1 PTF=30 VAF=10 XTF={xtf})\n.end\n",
                    p * 2.0,
                    p * direction,
                ));
                let expected = 1e-6 / circuit.bjts.devices[0].vt
                    * p
                    * direction
                    * if direction > 0.0 {
                        1.2 * (1.0 + xtf)
                    } else {
                        1.0
                    };
                engine
                    .transition_physical_startup(
                        &mut circuit,
                        &mut solution,
                        &mut history,
                        &options(),
                        1e-20,
                        &NoAbort,
                    )
                    .unwrap();
                close(
                    history.charge_cq_prev[0][BJT_QBE_BRANCH_INDEX],
                    expected,
                    1e-15,
                );
            }
        }
    }
}

#[test]
fn physical_startup_uses_dc_storage_and_outgoing_waveform_rates() {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(
        "DC to transient RLC\nV1 s 0 DC 1 PWL(0 2 1 6)\nR1 s n 4\nL1 n 0 0.5\nC1 s 0 3u\nI1 0 s DC 0.01 PWL(0 0.03 1 0.05)\n.end\n",
    );
    let l = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
    circuit.inductors.i_prev[0] = solution[l];
    close(solution[l], 0.25, 1e-13);
    let report = engine
        .transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let s = circuit.get_node_by_name("s").unwrap() - 1;
    let n = circuit.get_node_by_name("n").unwrap() - 1;
    let v = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    close(solution[s], 2.0, 1e-13);
    close(solution[n], 1.0, 1e-13);
    close(solution[l], 0.25, 1e-13);
    close(solution[v], 0.03 - 0.25 - 12e-6, 1e-13);
    close(report.coordinate_rates[s].unwrap(), 4.0, 1e-13);
    close(report.coordinate_rates[l].unwrap(), 2.0, 1e-13);
    assert_eq!(report.impulses.len(), 1);
    assert_eq!(report.impulses[0].0, v);
    close(report.impulses[0].1, -3e-6, 1e-18);
    close(circuit.capacitors.v_prev[0], 2.0, 1e-13);
    close(circuit.capacitors.i_prev[0], 12e-6, 1e-18);
    assert_eq!(circuit.capacitors.v_prev, circuit.capacitors.v_prev_prev);
    assert_eq!(
        circuit.capacitors.v_prev,
        circuit.capacitors.v_prev_prev_prev
    );
    close(circuit.inductors.v_prev[0], 1.0, 1e-13);
    assert_eq!(circuit.inductors.i_prev, circuit.inductors.i_prev_prev_prev);
    assert_eq!(history.accepted_dt_prev, 0.0);
    assert_eq!(history.accepted_dt_prev_prev, 0.0);
}

#[test]
fn physical_startup_preserves_selected_independent_capacitor_charges() {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(
        "initial charge redistribution\nV1 s 0 DC 0 PWL(0 2 1 2)\nR1 s n 10\nC1 n 0 2u\nC2 n 0 3u\n.end\n",
    );
    // Selected storage need not equal the nodal Newton starting guess, or
    // each other before an instantaneous redistribution at a shared node.
    circuit.capacitors.v_prev[0] = 1.0;
    circuit.capacitors.v_prev[1] = 0.0;
    let report = engine
        .transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let n = circuit.get_node_by_name("n").unwrap() - 1;
    close(solution[n], 0.4, 1e-13);
    close(report.coordinate_rates[n].unwrap(), 32000.0, 1e-8);
    close(circuit.capacitors.i_prev[0], 0.064, 1e-13);
    close(circuit.capacitors.i_prev[1], 0.096, 1e-13);
    close(report.impulses[0].1, 0.0, 1e-20);
}

#[test]
fn physical_startup_publishes_one_sided_gp_anchor_and_finite_currents() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rb in ["", "RB=100 RBM=100", "RB=100 RBM=20 IRB=1e-5"] {
            let (engine, mut circuit, _, mut solution, mut history) = fixture(&format!(
                "physical GP startup\nVc c 0 {}\nVb b 0 DC {} PWL(0 {} 1 {})\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p XCJC=.3 {rb})\n.end\n",
                p * 2.0,
                p * 0.6,
                p * 0.64,
                p * 0.66,
            ));
            let before = history.phase[0]
                .as_ref()
                .unwrap()
                .accepted_samples()
                .next()
                .unwrap()
                .1;
            engine
                .transition_physical_startup(
                    &mut circuit,
                    &mut solution,
                    &mut history,
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            let after = circuit.bjts.devices[0]
                .legacy_forward_transport_branch(&history.dynamic_internal_prev[0])
                .unwrap()
                .current;
            let phase = history.phase[0].as_ref().unwrap();
            let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
            assert_eq!(
                phase.accepted_samples().collect::<Vec<_>>(),
                vec![(0.0, after)]
            );
            assert_eq!(
                phase.accepted_left_limits().collect::<Vec<_>>(),
                vec![(0.0, before)]
            );
            assert_eq!(
                phase.accepted_event_orders().collect::<Vec<_>>(),
                vec![(0.0, 0)]
            );
            assert_eq!(phase.next_event_after(0.0).unwrap().unwrap().time, delay);
            let restored = DelayBuffer::from_checkpoint(phase.checkpoint()).unwrap();
            assert_eq!(&restored, phase);
            close(
                phase
                    .difference_before_arrival(delay, after, delay, None)
                    .unwrap()
                    .output,
                before - after,
                1e-20,
            );
            close(
                phase
                    .difference_with_coefficients(delay, after, delay, None)
                    .unwrap()
                    .output,
                0.0,
                1e-20,
            );
            let lead = history.accepted_terminal_currents[0].unwrap();
            close(lead.iter().sum(), 0.0, 1e-13);
            assert!(lead.iter().all(|i| i.is_finite()));
            assert_eq!(history.charge_q_prev, history.charge_q_prev_prev_prev);
            assert_eq!(
                history.dynamic_internal_prev,
                history.dynamic_internal_prev_prev
            );
            assert_eq!(history.accepted_dt_prev, 0.0);
            assert!(history.charge_cq_prev[0].iter().any(|i| *i != 0.0));
            let checkpoint = Engine::capture_accepted_junction_transient_history_checkpoint(
                &circuit,
                crate::engine::transient::AcceptedJunctionHistories {
                    bjt_history: &history,
                    diode_history: &Default::default(),
                    jfet_history: &Default::default(),
                    vbic_snapshot_cache: &[None],
                    bsim3_history: &Default::default(),
                    bsim4_history: &Default::default(),
                    mosfet_history: &Default::default(),
                },
            );
            Engine::validate_accepted_junction_transient_history_checkpoint(&circuit, &checkpoint)
                .unwrap();
        }
    }
}

#[test]
fn physical_startup_preserves_mutual_flux_and_commits_winding_currents() {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(
        "selected mutual flux\nV1 a 0 DC 0 PWL(0 1 1 1)\nV2 b 0 DC 0 PWL(0 -.5 1 -.5)\nR1 a n1 2\nR2 b n2 3\nL1 n1 0 .5\nL2 n2 0 .25\nK1 L1 L2 .4\n.end\n",
    );
    circuit.inductors.i_prev.copy_from_slice(&[0.2, -0.1]);
    let m = circuit.coupled_inductor_pairs[0].device.m;
    let report = engine
        .transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let a = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
    let b = circuit.num_nodes() + circuit.inductors.branch_indices[1] - 1;
    close(solution[a], 0.2, 1e-13);
    close(solution[b], -0.1, 1e-13);
    close(circuit.inductors.v_prev[0], 0.6, 1e-13);
    close(circuit.inductors.v_prev[1], -0.2, 1e-13);
    close(
        report.coordinate_rates[a].unwrap(),
        (0.25 * 0.6 + m * 0.2) / (0.125 - m * m),
        1e-13,
    );
    close(
        report.coordinate_rates[b].unwrap(),
        (-0.5 * 0.2 - m * 0.6) / (0.125 - m * m),
        1e-13,
    );
    assert!(report.impulses.iter().all(|(_, impulse)| *impulse == 0.0));
    assert_eq!(circuit.inductors.i_prev, circuit.inductors.i_prev_prev);
    assert_eq!(circuit.inductors.i_prev, circuit.inductors.i_prev_prev_prev);
}

#[test]
fn physical_startup_failure_and_last_poll_abort_preserve_all_targets() {
    use crate::abort_signal::CountingAbort;
    let (engine, circuit, _, solution, history) = fixture(
        "atomic startup\nVc c 0 2\nVb b 0 DC .6 PWL(0 .64 1 .66)\nQ1 c b 0 qm\nQ2 c b 0 qm\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\nC1 c 0 2u\nR1 c l 10\nL1 l 0 .1\n.end\n",
    );
    let census = CountingAbort::new(usize::MAX);
    engine
        .transition_physical_startup(
            &mut circuit.clone(),
            &mut solution.clone(),
            &mut history.clone(),
            &options(),
            1e-20,
            &census,
        )
        .unwrap();
    assert!(census.count() > 20);
    for threshold in [0, census.count() / 2, census.count() - 1] {
        let (mut circuit, mut solution, mut history) =
            (circuit.clone(), solution.clone(), history.clone());
        let original_solution = solution.clone();
        let original_history = history.clone();
        let caps = (
            circuit.capacitors.v_prev.clone(),
            circuit.capacitors.i_prev.clone(),
        );
        let inductors = (
            circuit.inductors.i_prev.clone(),
            circuit.inductors.v_prev.clone(),
        );
        let caches: Vec<_> = circuit
            .bjts
            .devices
            .iter()
            .map(|b| b.accepted_nonlinear_checkpoint().unwrap())
            .collect();
        let abort = CountingAbort::new(threshold);
        let result = engine.transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &abort,
        );
        assert!(
            matches!(result, Err(SimulationError::Aborted)),
            "poll {threshold}"
        );
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(solution, original_solution);
        assert_eq!(history, original_history);
        assert_eq!(
            caps,
            (
                circuit.capacitors.v_prev.clone(),
                circuit.capacitors.i_prev.clone()
            )
        );
        assert_eq!(
            inductors,
            (
                circuit.inductors.i_prev.clone(),
                circuit.inductors.v_prev.clone()
            )
        );
        assert_eq!(
            caches,
            circuit
                .bjts
                .devices
                .iter()
                .map(|b| b.accepted_nonlinear_checkpoint().unwrap())
                .collect::<Vec<_>>()
        );
    }
    for bad in 0..6 {
        let (mut circuit, mut solution, mut history) =
            (circuit.clone(), solution.clone(), history.clone());
        let mut opts = options();
        match bad {
            0 => history.phase[1] = None,
            1 => history.charge_q_prev[1][0] = Value::NAN,
            2 => {
                circuit.inductors.v_prev.clear();
            }
            3 => opts.limits.max_result_values = 1,
            4 => history.phase[1]
                .as_mut()
                .unwrap()
                .accept_sample(
                    0.1,
                    0.0,
                    circuit.bjts.devices[1].legacy_excess_phase_delay(),
                    None,
                )
                .unwrap(),
            5 => history.dynamic_internal_prev[1][0] = Value::INFINITY,
            _ => unreachable!(),
        }
        let before = format!(
            "{solution:?}{history:?}{:?}{:?}",
            circuit.capacitors, circuit.inductors
        );
        assert!(
            engine
                .transition_physical_startup(
                    &mut circuit,
                    &mut solution,
                    &mut history,
                    &opts,
                    1e-20,
                    &NoAbort
                )
                .is_err()
        );
        assert_eq!(
            before,
            format!(
                "{solution:?}{history:?}{:?}{:?}",
                circuit.capacitors, circuit.inductors
            )
        );
    }
}

#[test]
fn physical_startup_seed_charge_matches_all_physical_gp_storage_ports() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rb in ["", "RC=2 RE=1 RB=100 RBM=100", "RB=100 RBM=20 IRB=1e-5"] {
            let (_, circuit, _, solution, history) = fixture(&format!(
                "startup charge incidence\nVc c 0 {}\nVb b 0 {}\nVe e 0 {}\nVs s 0 {}\nQ1 c b e s qm\n.model qm {kind}(LEVEL=1 IS=1e-16 TF=1n TR=2n PTF=30 CJE=1p CJC=.2p CJS=.3p XCJC=.3 CBEO=.1p CBCO=.15p {rb})\n.end\n",
                p * 2.0,
                p * 0.6,
                p * 0.1,
                p * -0.2,
            ));
            assert!(circuit.bjts.devices[0].uses_legacy_gummel_poon());
            let mut sampler =
                PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
            let seed = super::super::startup::seed(&circuit, &history, &sampler, &NoAbort).unwrap();
            let phases = [Some(EventPhase {
                history: history.phase[0].as_ref().unwrap(),
                endpoint: seed.inputs[0].unwrap(),
            })];
            let physical = sampler
                .sample(
                    0.0,
                    SourceTimeSide::LeftLimit,
                    &solution,
                    &phases,
                    &options(),
                    &NoAbort,
                )
                .unwrap();
            for (&actual, &expected) in seed.charges.iter().zip(physical.charge_values()) {
                close(actual, expected, 1e-26);
            }
        }
    }
}

fn original_bug805_startup(text: &str) {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(text);
    for (index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
        circuit.inductors.i_prev[index] = solution[circuit.num_nodes() + ordinal - 1];
    }
    let incoming = solution.clone();
    let report = engine
        .transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let collector = circuit.get_node_by_name("q2c").unwrap() - 1;
    close(
        report.coordinate_rates[collector].unwrap(),
        7.4 / 5e-9,
        1e-5,
    );
    for (a, b) in solution[..circuit.num_nodes()].iter().zip(&incoming) {
        close(*a, *b, 1e-10);
    }
    assert!(report.impulses.iter().all(|(_, q)| q.abs() < 1e-23));
    for phase in history.phase.iter().flatten() {
        assert_eq!(phase.accepted_sample_count(), 1);
        assert_eq!(phase.accepted_left_limits().count(), 1);
        assert_eq!(
            phase.accepted_event_orders().collect::<Vec<_>>(),
            vec![(0.0, 0)]
        );
    }
    assert_eq!(history.accepted_dt_prev, 0.0);
}

#[test]
fn physical_startup_bug805_original_alias_1() {
    original_bug805_startup(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir"
    )));
}

#[test]
fn physical_startup_bug805_original_alias_2() {
    original_bug805_startup(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir"
    )));
}

#[test]
fn physical_startup_bug805_original_alias_3() {
    original_bug805_startup(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir"
    )));
}

#[test]
fn physical_startup_uses_authored_bjt_ic_charge_and_transport_prehistory() {
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        let (engine, mut circuit, _, mut solution, _) = fixture(&format!(
            "UIC physical GP startup\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0 qm IC={},{}\n.model qm {kind}(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p XCJC=.3)\n.end\n",
            p * 2.0,
            p * 0.6,
            p * 0.4,
            p * 1.1,
        ));
        let mut history =
            Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::UicStartup);
        Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
        let before = history.charge_q_prev[0];
        let prehistory = history.phase[0]
            .as_ref()
            .unwrap()
            .accepted_samples()
            .next()
            .unwrap()
            .1;
        let report = engine
            .transition_physical_startup(
                &mut circuit,
                &mut solution,
                &mut history,
                &options(),
                1e-20,
                &NoAbort,
            )
            .unwrap();
        let after = history.charge_q_prev[0];
        let base = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("Vb"))
            .unwrap();
        let base = circuit.num_nodes() + circuit.voltage_sources.branch_indices[base] - 1;
        let impulse = report.impulses.iter().find(|v| v.0 == base).unwrap().1;
        close(
            impulse,
            -([
                BJT_QBE_BRANCH_INDEX,
                BJT_QBC_BRANCH_INDEX,
                BJT_QBCX_BRANCH_INDEX,
            ]
            .into_iter()
            .map(|i| after[i] - before[i])
            .sum::<Value>()),
            1e-24,
        );
        let phase = history.phase[0].as_ref().unwrap();
        assert_eq!(phase.accepted_left_limits().next(), Some((0.0, prehistory)));
        assert_ne!(phase.accepted_samples().next().unwrap().1, prehistory);
    }
}
