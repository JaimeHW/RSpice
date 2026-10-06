use super::*;

#[test]
fn accepted_operating_point_does_not_turn_roundoff_into_a_small_capacitor_excitation() {
    let options = options();
    let topology =
        ChargeEventTopology::new(1, 1, &[(1, 0)], vec![], vec![], &options, &NoAbort).unwrap();
    let sample = |state: &[Value], _: &dyn AbortSignal| {
        let mut sample = EventSample::new(1, &options)?;
        branch(&mut sample.q, state, 1, 0, 1e-30);
        branch(&mut sample.f, state, 1, 0, 1e-3);
        sample.f.stamp_rhs(1, 1e-3 - 1e-17);
        Ok(sample)
    };
    let op = topology
        .solve_operating_point(&[1.0], &[1e-30], &options, &NoAbort, sample)
        .unwrap();
    assert_eq!(op.solution, [1.0]);
    assert_eq!(op.coordinate_rates, [Some(0.0)]);

    // Without an authenticated operating point this same small residual may
    // be a real authored excitation. Neither its size nor the capacitance
    // authorizes suppressing the physical rate.
    let ordinary = topology
        .solve(&[1.0], &[1e-30], &options, &NoAbort, sample)
        .unwrap();
    assert!(ordinary.coordinate_rates[0].unwrap() < -9e12);

    let invalid =
        topology.solve_operating_point(&[1.0], &[1e-30], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(1, &options)?;
            branch(&mut sample.q, state, 1, 0, 1e-30);
            sample.f.stamp_rhs(1, 1e-3);
            Ok(sample)
        });
    assert!(
        invalid
            .err()
            .expect("invalid operating point must fail")
            .to_string()
            .contains("finite-rate equation failed")
    );
}

#[test]
fn operating_point_rates_retain_prescribed_slope_and_finite_source_current() {
    let options = options();
    let topology = ChargeEventTopology::new(
        1,
        2,
        &[(1, 0)],
        vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 1,
            value: 1.0,
            slope: 3.0,
        }],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let result = topology
        .solve_operating_point(
            &[1.0, -1e-3],
            &[2e-6, 0.0],
            &options,
            &NoAbort,
            |state, _| {
                let mut sample = EventSample::new(2, &options)?;
                branch(&mut sample.q, state, 1, 0, 2e-6);
                branch(&mut sample.f, state, 1, 0, 1e-3);
                Ok(sample)
            },
        )
        .unwrap();
    assert_eq!(result.coordinate_rates, [Some(3.0), None]);
    close(result.solution[1], -1.006e-3, 1e-16);
    assert_eq!(result.source_impulses, [0.0]);
}

#[test]
fn operating_point_projection_still_audits_the_original_inductor_voltage() {
    let options = options();
    let topology = ChargeEventTopology::new(
        1,
        2,
        &[],
        vec![],
        vec![EventBranchEquation::Flux {
            flux_tolerance: 1e-20,
            voltage_tolerance: options.voltage_tolerance,
        }],
        &options,
        &NoAbort,
    )
    .unwrap();
    let result =
        topology.solve_operating_point(&[1.0, 0.0], &[0.0, 0.0], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(2, &options)?;
            branch(&mut sample.f, state, 1, 0, 1.0);
            sample.f.stamp_rhs(1, 1.0 - state[1]);
            sample.f.stamp(1, 2, 1.0);
            sample.q.stamp(2, 2, -1.0);
            sample.q.stamp_rhs(2, state[1]);
            sample.f.stamp(2, 1, 1.0);
            sample.f.stamp_rhs(2, -state[0]);
            Ok(sample)
        });
    assert!(
        result
            .err()
            .expect("nonzero inductor voltage is not an operating point")
            .to_string()
            .contains("finite-rate equation failed")
    );
}
