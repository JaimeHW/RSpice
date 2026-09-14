use super::*;

#[test]
fn current_event_coupling_distinguishes_floating_differential_and_common_mode() {
    let options = options();
    let topology =
        ChargeEventTopology::new(2, 2, &[(1, 2)], vec![], vec![], &options, &NoAbort).unwrap();
    let current = 1e-3;
    let capacitance = 2e-6;
    let g1 = 1e-3;
    let g2 = 2e-3;
    for negative in [2, 0] {
        let same_group = negative == 2;
        assert_eq!(
            topology.current_jump_coupling(1, negative).unwrap(),
            if same_group {
                CurrentJumpCoupling::Cancels
            } else {
                CurrentJumpCoupling::Present
            }
        );
        let outgoing = topology
            .solve(&[0.0; 2], &[0.0; 2], &options, &NoAbort, |state, _| {
                let mut sample = EventSample::new(2, &options)?;
                branch(&mut sample.q, state, 1, 2, capacitance);
                branch(&mut sample.f, state, 1, 0, g1);
                branch(&mut sample.f, state, 2, 0, g2);
                sample.f.stamp_rhs(1, -current);
                sample.f.stamp_rhs(negative, current);
                Ok(sample)
            })
            .unwrap();
        let common = if same_group {
            0.0
        } else {
            -current / (g1 + g2)
        };
        close(outgoing.solution[0], common, 1e-12);
        close(outgoing.solution[1], common, 1e-12);
        // KCL: C(v1'-v2') + g1*v1 + I = 0; the differentiated
        // group constraint is g1*v1' + g2*v2' = 0.
        let differential_rate = -(g1 * common + current) / capacitance;
        close(
            outgoing.coordinate_rates[0].unwrap(),
            differential_rate * g2 / (g1 + g2),
            1e-9,
        );
        close(
            outgoing.coordinate_rates[1].unwrap(),
            -differential_rate * g1 / (g1 + g2),
            1e-9,
        );
        assert!(outgoing.source_impulses.is_empty());
    }
    assert_eq!(
        topology.current_jump_coupling(0, 0).unwrap(),
        CurrentJumpCoupling::Cancels
    );
    assert!(topology.current_jump_coupling(3, 0).is_err());
    assert!(topology.current_jump_coupling(0, 3).is_err());
}

#[test]
fn current_event_coupling_in_grounded_rlc_preserves_values_and_changes_rates() {
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        3,
        &[(1, 0), (2, 0)],
        vec![],
        vec![EventBranchEquation::Flux {
            flux_tolerance: 1e-20,
            voltage_tolerance: 1e-11,
        }],
        &options,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(
        topology.current_jump_coupling(1, 2).unwrap(),
        CurrentJumpCoupling::Cancels
    );
    let outgoing = topology
        .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(3, &options)?;
            branch(&mut sample.q, state, 1, 0, 2e-6);
            branch(&mut sample.q, state, 2, 0, 4e-6);
            branch(&mut sample.f, state, 1, 0, 1e-3);
            branch(&mut sample.f, state, 2, 0, 2e-3);
            sample.f.stamp(1, 3, 1.0);
            sample.f.stamp(2, 3, -1.0);
            sample.f.stamp_rhs(1, -state[2] - 1e-3);
            sample.f.stamp_rhs(2, state[2] + 1e-3);
            sample.f.stamp(3, 1, 1.0);
            sample.f.stamp(3, 2, -1.0);
            sample.f.stamp_rhs(3, -(state[0] - state[1]));
            sample.q.stamp(3, 3, -0.5);
            sample.q.stamp_rhs(3, 0.5 * state[2]);
            Ok(sample)
        })
        .unwrap();
    for value in outgoing.solution {
        close(value, 0.0, 1e-12);
    }
    close(outgoing.coordinate_rates[0].unwrap(), -500.0, 1e-9);
    close(outgoing.coordinate_rates[1].unwrap(), 250.0, 1e-9);
    close(outgoing.coordinate_rates[2].unwrap(), 0.0, 1e-12);
}

#[test]
fn current_event_coupling_does_not_certify_continuous_source_current_or_regular_system() {
    let options = options();
    let topology = ChargeEventTopology::new(
        1,
        2,
        &[],
        vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 1,
            value: 0.0,
            slope: 0.0,
        }],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(
        topology.current_jump_coupling(1, 0).unwrap(),
        CurrentJumpCoupling::Cancels
    );
    let outgoing = topology
        .solve(&[0.0; 2], &[0.0; 2], &options, &NoAbort, |_, _| {
            let mut sample = EventSample::new(2, &options)?;
            sample.f.stamp_rhs(1, -1e-3);
            Ok(sample)
        })
        .unwrap();
    close(outgoing.solution[0], 0.0, 1e-12);
    close(outgoing.solution[1], -1e-3, 1e-13);
    close(outgoing.source_impulses[0], 0.0, 1e-25);
    assert_eq!(outgoing.coordinate_rates[1], None);

    // A structural charge port cannot prove rank. Even with a zero residual
    // and cancelled forcing, an absent differential capacitance must fail.
    let singular =
        ChargeEventTopology::new(1, 1, &[(1, 0)], vec![], vec![], &options, &NoAbort).unwrap();
    assert_eq!(
        singular.current_jump_coupling(1, 0).unwrap(),
        CurrentJumpCoupling::Cancels
    );
    assert!(
        singular
            .solve(&[0.0], &[0.0], &options, &NoAbort, |_, _| EventSample::new(
                1, &options
            ))
            .is_err()
    );
}
