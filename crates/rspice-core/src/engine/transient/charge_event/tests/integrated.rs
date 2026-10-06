use super::*;

#[test]
fn integrated_reference_reserves_the_floating_components_omitted_kcl_budget() {
    let options = options();
    let topology =
        ChargeEventTopology::new(3, 3, &[(1, 2), (1, 3)], vec![], vec![], &options, &NoAbort)
            .unwrap();
    let result = topology
        .solve_integrated(
            (&[1.0, 1.0, 1.0], &[0.0; 3]),
            &[1e-3 - 1e-9 - 3e-24, -1e-3 + 1e-9, 3e-24],
            &options,
            &NoAbort,
            |state, _| {
                let mut sample = EventSample::new(3, &options)?;
                branch(&mut sample.q, state, 1, 2, 1e-9);
                branch(&mut sample.q, state, 1, 3, 1e-30);
                branch(&mut sample.f, state, 1, 0, 1.0);
                sample.f.stamp_rhs(1, 1.001 + 2e-13);
                sample.f.stamp_rhs(2, 1e3);
                sample.f.stamp_rhs(2, -1e3 - 1e-3);
                sample.f.stamp_rhs(3, 1.0);
                sample.f.stamp_rhs(3, -1.0 - 2e-13);
                Ok(sample)
            },
        )
        .unwrap();
    close(result.coordinate_rates[0].unwrap(), 0.0, 1e-6);
    // The large row's reference error fits its own relative current tolerance,
    // but not the omitted node's. It must retain the physical current instead.
    close(result.coordinate_rates[1].unwrap(), -1e6, 1e-4);
    close(result.coordinate_rates[2].unwrap(), 3e6, 1e-4);
}

#[test]
fn integrated_reference_preserves_a_nonzero_weak_storage_rate() {
    let options = options();
    let topology =
        ChargeEventTopology::new(2, 2, &[(1, 0), (2, 0)], vec![], vec![], &options, &NoAbort)
            .unwrap();
    let sample = |state: &[Value], _: &dyn AbortSignal| {
        let mut sample = EventSample::new(2, &options)?;
        branch(&mut sample.q, state, 1, 0, 1e-9);
        branch(&mut sample.q, state, 2, 0, 1e-30);
        sample.f.stamp_rhs(1, 2e-3);
        // The original physical terms, and hence the relative KCL scale,
        // survive cancellation into a rounding-sized net residual.
        sample.f.stamp_rhs(2, 1e3);
        sample.f.stamp_rhs(2, -1e3 + 2e-13);
        Ok(sample)
    };
    for strong_reference in [2e-3, 0.0] {
        let result = topology
            .solve_integrated(
                (&[1.0, 1.0], &[1e-9, 1e-30]),
                &[strong_reference, 3e-24],
                &options,
                &NoAbort,
                sample,
            )
            .unwrap();
        close(result.coordinate_rates[0].unwrap(), 2e6, 1e-8);
        close(result.coordinate_rates[1].unwrap(), 3e6, 1e-8);
        assert_eq!(result.solution, [1.0, 1.0]);
    }
    let ordinary = topology
        .solve_continuous(&[1.0, 1.0], &[1e-9, 1e-30], &options, &NoAbort, sample)
        .unwrap();
    assert!(ordinary.coordinate_rates[1].unwrap() > 1e17);
}

#[test]
fn integrated_reference_retains_the_outgoing_prescribed_voltage_slope() {
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
            slope: 5.0,
        }],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let result = topology
        .solve_integrated(
            (&[1.0, -1.006e-3], &[2e-6, 0.0]),
            &[6e-6, 0.0],
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
    assert_eq!(result.coordinate_rates, [Some(5.0), None]);
    close(result.solution[1], -1.01e-3, 1e-16);
    assert_eq!(result.source_impulses, [0.0]);
}
