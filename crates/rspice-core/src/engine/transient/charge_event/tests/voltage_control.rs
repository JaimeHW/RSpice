use super::*;

#[test]
fn voltage_control_transfers_jumps_and_rates_without_drawing_input_impulses() {
    let options = options();
    for floating in [false, true] {
        let nodes = if floating { 3 } else { 2 };
        let size = nodes + 2;
        let negative = if floating { 3 } else { 0 };
        let sources = vec![
            EventVoltageSource {
                positive: 1,
                negative: 0,
                branch: nodes,
                equation: EventVoltageEquation::Affine {
                    value: 1.0,
                    slope: 2e6,
                    control: None,
                },
            },
            EventVoltageSource {
                positive: 2,
                negative,
                branch: nodes + 1,
                equation: EventVoltageEquation::Affine {
                    value: 0.0,
                    slope: 0.0,
                    control: Some(EventVoltageControl {
                        positive: 1,
                        negative: 0,
                        gain: -3.0,
                    }),
                },
            },
        ];
        let topology = ChargeEventTopology::new(
            nodes,
            size,
            &[(1, 0), (2, negative)],
            sources,
            vec![EventBranchEquation::Algebraic(options.voltage_tolerance); 2],
            &options,
            &NoAbort,
        )
        .unwrap();
        let state = topology
            .solve(
                &vec![0.0; size],
                &vec![0.0; size],
                &options,
                &NoAbort,
                |state, _| {
                    let mut sample = EventSample::new(size, &options)?;
                    branch(&mut sample.q, state, 1, 0, 2e-12);
                    branch(&mut sample.q, state, 2, negative, 5e-12);
                    branch(&mut sample.f, state, 2, 0, 0.5e-3);
                    if floating {
                        branch(&mut sample.f, state, 3, 0, 1e-3);
                    }
                    Ok(sample)
                },
            )
            .unwrap();
        close(state.solution[0], 1.0, 1e-12);
        close(state.solution[1], if floating { -2.0 } else { -3.0 }, 1e-12);
        close(state.coordinate_rates[0].unwrap(), 2e6, 1e-6);
        close(
            state.coordinate_rates[1].unwrap(),
            if floating { -4e6 } else { -6e6 },
            1e-6,
        );
        if floating {
            close(state.solution[2], 1.0, 1e-12);
            close(state.coordinate_rates[2].unwrap(), 2e6, 1e-6);
        }
        close(state.source_impulses[0], -2e-12, 1e-25);
        close(state.source_impulses[1], 15e-12, 1e-25);
        close(state.solution[nodes], -4e-6, 1e-14);
        close(
            state.solution[nodes + 1],
            if floating { 1.03e-3 } else { 1.53e-3 },
            1e-14,
        );
        assert_eq!(&state.coordinate_rates[nodes..], &[None, None]);
    }
}

#[test]
fn voltage_control_feedback_is_solved_jointly_and_a_singular_loop_is_refused() {
    let options = options();
    for gain in [0.5, 1.0] {
        let sources = vec![
            EventVoltageSource {
                positive: 1,
                negative: 0,
                branch: 2,
                equation: EventVoltageEquation::Affine {
                    value: if gain == 1.0 { 0.0 } else { 1.0 },
                    slope: 3e6,
                    control: Some(EventVoltageControl {
                        positive: 2,
                        negative: 0,
                        gain,
                    }),
                },
            },
            EventVoltageSource {
                positive: 2,
                negative: 0,
                branch: 3,
                equation: EventVoltageEquation::Affine {
                    value: 0.0,
                    slope: 0.0,
                    control: Some(EventVoltageControl {
                        positive: 1,
                        negative: 0,
                        gain,
                    }),
                },
            },
        ];
        let topology = ChargeEventTopology::new(
            2,
            4,
            &[(1, 0), (2, 0)],
            sources,
            vec![EventBranchEquation::Algebraic(options.voltage_tolerance); 2],
            &options,
            &NoAbort,
        )
        .unwrap();
        let result = topology.solve(&[0.0; 4], &[0.0; 4], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(4, &options)?;
            branch(&mut sample.q, state, 1, 0, 1e-12);
            branch(&mut sample.q, state, 2, 0, 2e-12);
            Ok(sample)
        });
        if gain == 1.0 {
            assert!(
                result.is_err(),
                "undetermined feedback must not certify the zero guess"
            );
            continue;
        }
        let state = result.unwrap();
        close(state.solution[0], 4.0 / 3.0, 1e-12);
        close(state.solution[1], 2.0 / 3.0, 1e-12);
        close(state.coordinate_rates[0].unwrap(), 4e6, 1e-6);
        close(state.coordinate_rates[1].unwrap(), 2e6, 1e-6);
        for (&impulse, &current) in state.source_impulses.iter().zip(&state.solution[2..]) {
            close(impulse, -4e-12 / 3.0, 1e-25);
            close(current, -4e-6, 1e-14);
        }
    }
}

#[test]
fn voltage_control_descriptor_checks_control_terminals_and_coefficients() {
    let options = options();
    for (positive, negative, gain) in [
        (3, 0, 1.0),
        (0, 3, 1.0),
        (1, 0, Value::NAN),
        (1, 0, Value::INFINITY),
    ] {
        let mut source = source(2, 0, 0.0, 0.0);
        source.equation = EventVoltageEquation::Affine {
            value: 0.0,
            slope: 0.0,
            control: Some(EventVoltageControl {
                positive,
                negative,
                gain,
            }),
        };
        assert!(matches!(ChargeEventTopology::new(2,3,&[],vec![source],
            vec![EventBranchEquation::Algebraic(options.voltage_tolerance)], &options,&NoAbort), Err(SimulationError::Circuit(message)) if message.contains("voltage-source descriptor")));
    }
}
