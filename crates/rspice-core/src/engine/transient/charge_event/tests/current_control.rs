use super::*;

fn controlled(
    nodes: usize,
    size: usize,
    ports: &[(usize, usize)],
    sources: Vec<EventVoltageSource>,
    controls: &[EventCurrentControl],
    options: &EventOptions,
) -> ChargeEventTopology {
    let basis = CurrentConservation::new(nodes, size, ports, &sources, controls, options, &NoAbort)
        .unwrap();
    let mut topology = ChargeEventTopology::new(
        nodes,
        size,
        ports,
        sources,
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance); size - nodes],
        options,
        &NoAbort,
    )
    .unwrap();
    topology
        .install_current_conservation(Arc::new(basis))
        .unwrap();
    topology
}

#[test]
fn current_controlled_startup_seed_satisfies_authored_voltage_constraints() {
    let options = options();
    for value in [-2.0, 2.0] {
        for gain in [-3.0, 3.0] {
            let sources = vec![
                EventVoltageSource {
                    positive: 1,
                    negative: 0,
                    branch: 3,
                    value,
                    slope: 0.0,
                    control: None,
                },
                EventVoltageSource {
                    positive: 2,
                    negative: 3,
                    branch: 4,
                    value: 0.5,
                    slope: 0.0,
                    control: Some(EventVoltageControl {
                        positive: 1,
                        negative: 0,
                        gain,
                    }),
                },
            ];
            let topology = controlled(3, 5, &[(1, 0), (2, 0), (3, 0)], sources, &[], &options);
            let incoming = [17.0, 0.25, -2.0, 0.0, 0.0];
            let mut trial = incoming;
            topology
                .project_current_controlled_voltage_seed(&incoming, &mut trial, &NoAbort)
                .unwrap();
            close(trial[0], value, 1e-14);
            close(trial[1] - trial[2], 0.5 + gain * trial[0], 1e-14);
            // The free coordinate is retained; branch slots remain private
            // impulse unknowns and are not populated with finite currents.
            assert_eq!(trial[1], incoming[1]);
            assert_eq!(trial[3..], incoming[3..]);
        }
    }
}

#[test]
fn controlled_current_transfers_source_charge_and_finite_current_to_another_capacitor() {
    let options = options();
    for gain in [-3.0, 2.0] {
        let sources = vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 2,
            value: 1.0,
            slope: 2e6,
            control: None,
        }];
        let topology = controlled(
            2,
            3,
            &[(1, 0), (2, 0)],
            sources,
            &[EventCurrentControl {
                positive: 2,
                negative: 0,
                branch: 2,
                gain,
            }],
            &options,
        );
        let state = topology
            .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |state, _| {
                let mut sample = EventSample::new(3, &options)?;
                branch(&mut sample.q, state, 1, 0, 2e-12);
                branch(&mut sample.q, state, 2, 0, 5e-12);
                branch(&mut sample.f, state, 2, 0, 1e-3);
                Ok(sample)
            })
            .unwrap();
        close(state.solution[0], 1.0, 1e-12);
        close(state.solution[1], gain * 0.4, 1e-12);
        close(state.source_impulses[0], -2e-12, 1e-25);
        close(state.solution[2], -4e-6, 1e-16);
        close(
            state.coordinate_rates[1].unwrap(),
            (-1e-3 * gain * 0.4 + gain * 4e-6) / 5e-12,
            1e-5,
        );
    }
}

#[test]
fn controlled_current_preserves_weighted_kcl_between_floating_charge_components() {
    let options = options();
    for [a, b, c, d] in [[1, 2, 3, 4], [4, 3, 2, 1], [3, 1, 4, 2], [2, 4, 1, 3]] {
        let sources = vec![EventVoltageSource {
            positive: a,
            negative: 0,
            branch: 4,
            value: 1.0,
            slope: 2e6,
            control: None,
        }];
        let topology = controlled(
            4,
            5,
            &[(a, b), (c, d)],
            sources,
            &[EventCurrentControl {
                positive: c,
                negative: 0,
                branch: 4,
                gain: 2.0,
            }],
            &options,
        );
        let state = topology
            .solve(&[0.0; 5], &[0.0; 5], &options, &NoAbort, |state, _| {
                let mut sample = EventSample::new(5, &options)?;
                branch(&mut sample.q, state, a, b, 2e-12);
                branch(&mut sample.q, state, c, d, 5e-12);
                branch(&mut sample.f, state, b, 0, 1e-3);
                branch(&mut sample.f, state, c, 0, 2e-3);
                branch(&mut sample.f, state, d, 0, 3e-3);
                Ok(sample)
            })
            .unwrap();
        for (node, value, rate) in [
            (a, 1.0, 2e6),
            (b, 1.0, -4.98e8),
            (c, 0.4, -5.52e7),
            (d, 0.4, -2.952e8),
        ] {
            close(state.solution[node - 1], value, 1e-12);
            close(state.coordinate_rates[node - 1].unwrap(), rate, 1e-5);
        }
        close(state.solution[4], -1e-3, 1e-15);
        close(state.source_impulses[0], 0.0, 1e-25);
        assert_eq!(
            topology.current_jump_coupling(c, d).unwrap(),
            CurrentJumpCoupling::Cancels
        );
        assert_eq!(
            topology.current_jump_coupling(a, c).unwrap(),
            CurrentJumpCoupling::Present
        );
    }
}

#[test]
fn controlled_current_feedback_solves_both_source_impulses_and_finite_currents() {
    let options = options();
    let sources = vec![
        EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 2,
            value: 1.0,
            slope: 1e6,
            control: None,
        },
        EventVoltageSource {
            positive: 2,
            negative: 0,
            branch: 3,
            value: 2.0,
            slope: -2e6,
            control: None,
        },
    ];
    let topology = controlled(
        2,
        4,
        &[(1, 0), (2, 0)],
        sources,
        &[
            EventCurrentControl {
                positive: 1,
                negative: 0,
                branch: 3,
                gain: 0.5,
            },
            EventCurrentControl {
                positive: 2,
                negative: 0,
                branch: 2,
                gain: -0.25,
            },
        ],
        &options,
    );
    let state = topology
        .solve(&[0.0; 4], &[0.0; 4], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(4, &options)?;
            branch(&mut sample.q, state, 1, 0, 1e-12);
            branch(&mut sample.q, state, 2, 0, 2e-12);
            Ok(sample)
        })
        .unwrap();
    close(state.source_impulses[0], 1e-12 / 1.125, 1e-25);
    close(state.source_impulses[1], -4e-12 + 0.25e-12 / 1.125, 1e-25);
    close(state.solution[2], -3e-6 / 1.125, 1e-16);
    close(state.solution[3], 4e-6 - 0.75e-6 / 1.125, 1e-16);
}

#[test]
fn controlled_current_rank_retains_a_loop_below_binary64_product_precision() {
    let options = options();
    let sources = [
        EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 2,
            value: 0.0,
            slope: 0.0,
            control: None,
        },
        EventVoltageSource {
            positive: 2,
            negative: 0,
            branch: 3,
            value: 0.0,
            slope: 0.0,
            control: None,
        },
    ];
    let epsilon = 2.0_f64.powi(-30);
    assert_eq!((1.0 + epsilon) * (1.0 - epsilon), 1.0);
    for offset in [0.0, epsilon] {
        let controls = [
            EventCurrentControl {
                positive: 2,
                negative: 0,
                branch: 2,
                gain: -(1.0 + offset),
            },
            EventCurrentControl {
                positive: 1,
                negative: 0,
                branch: 3,
                gain: -(1.0 - offset),
            },
        ];
        let basis =
            CurrentConservation::new(2, 4, &[], &sources, &controls, &options, &NoAbort).unwrap();
        assert_eq!(
            basis.rows.iter().filter(|row| !row.is_empty()).count(),
            usize::from(offset == 0.0)
        );
    }
}

#[test]
fn controlled_current_preparation_rejects_invalid_controls_and_honors_resource_limits() {
    let sources = [EventVoltageSource {
        positive: 1,
        negative: 0,
        branch: 2,
        value: 0.0,
        slope: 0.0,
        control: None,
    }];
    for (node, branch, gain) in [(3, 2, 1.0), (2, 1, 1.0), (2, 2, Value::NAN)] {
        let controls = [EventCurrentControl {
            positive: node,
            negative: 0,
            branch,
            gain,
        }];
        assert!(matches!(
            CurrentConservation::new(2, 3, &[], &sources, &controls, &options(), &NoAbort),
            Err(SimulationError::Circuit(_))
        ));
    }
    let mut options = options();
    for checks in [1, 10] {
        let abort = crate::abort_signal::CountingAbort::new(checks);
        assert!(matches!(
            CurrentConservation::new(2, 3, &[], &sources, &[], &options, &abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }
    options.limits.max_result_values = 1;
    assert!(matches!(
        CurrentConservation::new(2, 3, &[], &sources, &[], &options, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
}
