use super::*;

mod integrated;
use crate::abort_signal::NoAbort;
use crate::device::MatrixStamper;

mod current_control;
mod current_coupling;
mod flux;
mod native;
mod operating_point;
mod voltage_control;

fn options() -> EventOptions {
    EventOptions {
        limits: ResourceLimits::default(),
        solver: SolverOptions::default(),
        nodal_gmin: 0.0,
        iterations: 80,
        backtracks: 32,
        voltage_tolerance: 1e-11,
        current_tolerance: 1e-13,
        charge_tolerance: 1e-25,
        relative_tolerance: 1e-11,
    }
}

fn branch(stamp: &mut EventStamp, state: &[Value], pos: usize, neg: usize, coefficient: Value) {
    let value = coefficient * (voltage(state, pos) - voltage(state, neg));
    for (row, sign) in [(pos, 1.0), (neg, -1.0)] {
        stamp.stamp_rhs(row, -sign * value);
        stamp.stamp(row, pos, sign * coefficient);
        stamp.stamp(row, neg, -sign * coefficient);
    }
}

fn source(positive: usize, negative: usize, value: Value, slope: Value) -> EventVoltageSource {
    EventVoltageSource {
        positive,
        negative,
        branch: 2,
        equation: EventVoltageEquation::Affine {
            value,
            slope,
            control: None,
        },
    }
}

fn close(actual: Value, expected: Value, absolute: Value) {
    assert!(
        (actual - expected).abs() <= absolute + 1e-10 * expected.abs(),
        "{actual:e} != {expected:e}"
    );
}

#[test]
fn charge_event_recovers_divider_voltage_rate_finite_current_and_source_impulse() {
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        3,
        &[(1, 2), (2, 0)],
        vec![source(1, 0, 3.0, 0.0)],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let incoming = vec![0.0; 3];
    let state = topology
        .solve(&incoming, &[0.0; 3], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(3, &options)?;
            branch(&mut sample.q, state, 1, 2, 2e-12);
            branch(&mut sample.q, state, 2, 0, 1e-12);
            branch(&mut sample.f, state, 2, 0, 1e-3);
            Ok(sample)
        })
        .unwrap();
    close(state.solution[0], 3.0, 1e-12);
    close(state.solution[1], 2.0, 1e-12);
    close(state.source_impulses[0], -2e-12, 1e-25);
    close(state.coordinate_rates[0].unwrap(), 0.0, 1e-6);
    close(
        state.coordinate_rates[1].unwrap(),
        -2.0 / (1e3 * 3e-12),
        1e-5,
    );
    close(state.solution[2], -4.0 / 3.0 * 1e-3, 1e-13);
    assert_eq!(state.coordinate_rates[2], None);
    assert_eq!(incoming, vec![0.0; 3]);
    assert!(state.iterations <= 3);
}

#[test]
fn charge_event_differentiates_floating_common_mode_and_source_constraints() {
    let options = options();
    for (source_slope, current_slope) in [(0.0, 0.0), (2e6, 0.0), (0.0, 3e-6)] {
        let topology = ChargeEventTopology::new(
            2,
            3,
            &[(1, 2)],
            vec![source(1, 2, 3.0, source_slope)],
            vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
            &options,
            &NoAbort,
        )
        .unwrap();
        let state = topology
            .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |state, _| {
                let mut sample = EventSample::new(3, &options)?;
                branch(&mut sample.q, state, 1, 2, 2e-12);
                branch(&mut sample.f, state, 1, 0, 1e-3);
                branch(&mut sample.f, state, 2, 0, 0.5e-3);
                sample.f_time[0] = current_slope;
                Ok(sample)
            })
            .unwrap();
        close(state.solution[0], 1.0, 1e-12);
        close(state.solution[1], -2.0, 1e-12);
        close(state.source_impulses[0], -6e-12, 1e-25);
        let first_rate = (source_slope * 0.5e-3 - current_slope) / 1.5e-3;
        close(state.coordinate_rates[0].unwrap(), first_rate, 1e-12);
        close(
            state.coordinate_rates[1].unwrap(),
            first_rate - source_slope,
            1e-12,
        );
        close(state.solution[2], -1e-3 - 2e-12 * source_slope, 1e-13);
    }
}

#[test]
fn charge_event_audits_discarded_charge_rows_and_refuses_singular_constraints() {
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        3,
        &[],
        vec![source(1, 0, 3.0, 0.0)],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let failure = topology
        .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(3, &options)?;
            branch(&mut sample.q, state, 1, 2, 2e-12);
            branch(&mut sample.f, state, 2, 0, 1e-3);
            Ok(sample)
        })
        .err()
        .unwrap();
    assert!(
        failure.to_string().contains("charge conservation"),
        "{failure}"
    );
    let topology = ChargeEventTopology::new(1, 1, &[], vec![], vec![], &options, &NoAbort).unwrap();
    assert!(
        topology
            .solve(&[0.0], &[0.0], &options, &NoAbort, |_, _| EventSample::new(
                1, &options
            ))
            .is_err()
    );
}

#[test]
fn charge_event_bounds_assembly_and_rejects_unowned_branch_dependencies() {
    let mut options = options();
    options.limits.max_matrix_unknowns = 2;
    assert!(
        ChargeEventTopology::new(
            2,
            3,
            &[],
            vec![],
            vec![EventBranchEquation::Algebraic(1e-12)],
            &options,
            &NoAbort
        )
        .is_err()
    );
    options.limits.max_matrix_unknowns = 3;
    let topology = ChargeEventTopology::new(
        2,
        3,
        &[(1, 2)],
        vec![source(1, 0, 1.0, 0.0)],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let failure = topology
        .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |_, _| {
            let mut sample = EventSample::new(3, &options)?;
            sample.f.stamp(1, 3, 1.0);
            Ok(sample)
        })
        .err()
        .unwrap();
    assert!(failure.to_string().contains("branch-current dependencies"));
    options.limits.max_result_values = 3 * 64 + 256;
    let failure = topology
        .solve(&[0.0; 3], &[0.0; 3], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(3, &options)?;
            branch(&mut sample.q, state, 1, 2, 1e-12);
            Ok(sample)
        })
        .err()
        .unwrap();
    assert!(failure.to_string().contains("result_values"), "{failure}");
}

#[test]
fn charge_event_voltage_seed_preserves_nonlinear_domain_fallback() {
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        3,
        &[(2, 1)],
        vec![source(1, 0, 2.0, 0.0)],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    // Projecting V1 alone either leaves sqrt(V2-V1)'s domain or places
    // (V2-V1)^3 at its zero slope. Coupled Newton instead translates both
    // nodes and conserves the incoming charge in a nonsingular chart.
    for (initial, exponent) in [(1.0_f64, 0.5), (2.0, 3.0)] {
        let incoming_charge = initial.powf(exponent);
        let mut invalid_seeds = 0;
        let state = topology
            .solve(
                &[0.0, initial, 0.0],
                &[-incoming_charge, incoming_charge, 0.0],
                &options,
                &NoAbort,
                |state, _| {
                    let mut sample = EventSample::new(3, &options)?;
                    let difference = state[1] - state[0];
                    let charge = difference.powf(exponent);
                    let derivative = exponent * difference.powf(exponent - 1.0);
                    if !charge.is_finite() || derivative == 0.0 {
                        invalid_seeds += 1;
                    }
                    for (row, sign) in [(2, 1.0), (1, -1.0)] {
                        sample.q.stamp_rhs(row, -sign * charge);
                        sample.q.stamp(row, 2, sign * derivative);
                        sample.q.stamp(row, 1, -sign * derivative);
                    }
                    branch(&mut sample.f, state, 2, 0, 1.0);
                    Ok(sample)
                },
            )
            .unwrap();
        assert!(invalid_seeds > 0, "must exercise an invalid projected seed");
        let outgoing = initial + 2.0;
        close(state.solution[0], 2.0, 1e-12);
        close(state.solution[1], outgoing, 1e-12);
        close(state.solution[2], -outgoing, 1e-12);
        close(state.source_impulses[0], 0.0, 1e-25);
        close(state.coordinate_rates[0].unwrap(), 0.0, 1e-12);
        let derivative = exponent * initial.powf(exponent - 1.0);
        close(
            state.coordinate_rates[1].unwrap(),
            -outgoing / derivative,
            1e-12,
        );
    }
}

#[test]
fn charge_event_backtracks_nonlinear_charge_domain_failures() {
    let options = options();
    let topology =
        ChargeEventTopology::new(1, 1, &[(1, 0)], vec![], vec![], &options, &NoAbort).unwrap();
    let mut rejected = 0;
    // A switched charge law changes from Q=40 V to Q=V^2. The conserved
    // incoming charge is 4 C; the positive outgoing root is exactly 2 V.
    let result = topology
        .solve(&[0.1], &[4.0], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(1, &options)?;
            if state[0].abs() > 10.0 {
                sample.q.stamp_rhs(1, Value::NAN);
                rejected += 1;
            } else {
                sample.q.stamp_rhs(1, -state[0] * state[0]);
                sample.q.stamp(1, 1, 2.0 * state[0]);
            }
            branch(&mut sample.f, state, 1, 0, 1e-3);
            Ok(sample)
        })
        .unwrap();
    assert!(
        rejected > 0,
        "full Newton probe must exercise the domain failure"
    );
    close(result.solution[0], 2.0, 1e-12);
    close(result.coordinate_rates[0].unwrap(), -0.0005, 1e-14);
}

#[test]
fn charge_event_cancels_at_every_observed_boundary_without_modifying_input() {
    use crate::abort_signal::CountingAbort;
    let options = options();
    let topology =
        ChargeEventTopology::new(1, 1, &[(1, 0)], vec![], vec![], &options, &NoAbort).unwrap();
    let incoming = [0.0];
    let solve = |abort: &dyn AbortSignal| {
        topology.solve(&incoming, &[1e-12], &options, abort, |state, _| {
            let mut sample = EventSample::new(1, &options)?;
            branch(&mut sample.q, state, 1, 0, 1e-12);
            branch(&mut sample.f, state, 1, 0, 1e-3);
            sample.q_time[0] = 2e-3;
            Ok(sample)
        })
    };
    let baseline = CountingAbort::new(usize::MAX);
    let result = solve(&baseline).unwrap();
    close(result.solution[0], 1.0, 1e-12);
    close(result.coordinate_rates[0].unwrap(), -3e9, 1e-5);
    for threshold in 0..baseline.count() {
        let abort = CountingAbort::new(threshold);
        assert!(
            matches!(solve(&abort), Err(SimulationError::Aborted)),
            "threshold {threshold}"
        );
        assert_eq!(abort.observed_at(), Some(threshold + 1));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(incoming, [0.0]);
    }
}

#[test]
fn charge_event_nonlinear_voltage_seed_preserves_cancellation_and_singular_refusal() {
    use crate::abort_signal::CountingAbort;
    let options = options();
    let topology = ChargeEventTopology::new(
        1,
        2,
        &[(1, 0)],
        vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 1,
            equation: EventVoltageEquation::Sampled,
        }],
        vec![EventBranchEquation::Algebraic(options.voltage_tolerance)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let incoming = [0.0; 2];
    let solve = |abort: &dyn AbortSignal, regular: bool| {
        topology.solve(&incoming, &incoming, &options, abort, |state, _| {
            let mut sample = EventSample::new(2, &options)?;
            branch(&mut sample.q, state, 1, 0, 1e-12);
            branch(&mut sample.f, state, 1, 0, 1e-3);
            let v = state[0];
            let linear = if regular { 1.0 } else { 0.0 };
            sample.f.stamp_rhs(2, 2.0 * linear - linear * v - v.powi(3));
            sample.f.stamp(2, 1, linear + 3.0 * v * v);
            Ok(sample)
        })
    };
    let baseline = CountingAbort::new(usize::MAX);
    let result = solve(&baseline, true).unwrap();
    close(result.solution[0], 1.0, 1e-12);
    close(result.solution[1], -1e-3, 1e-14);
    close(result.source_impulses[0], -1e-12, 1e-25);
    for threshold in 0..baseline.count() {
        let abort = CountingAbort::new(threshold);
        assert!(
            matches!(solve(&abort, true), Err(SimulationError::Aborted)),
            "threshold {threshold}"
        );
        assert_eq!(abort.observed_at(), Some(threshold + 1));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(incoming, [0.0; 2]);
    }
    // A zero residual with a zero voltage Jacobian is not an accepted state.
    // Local seeding must defer to the coupled solver's rank check.
    assert!(matches!(
        solve(&NoAbort, false),
        Err(SimulationError::Solver(
            crate::solver::SolverError::SingularMatrix
        ))
    ));
}

#[test]
fn charge_event_revalidates_options_and_does_not_backtrack_structural_faults() {
    let mut options = options();
    let topology =
        ChargeEventTopology::new(1, 1, &[(1, 0)], vec![], vec![], &options, &NoAbort).unwrap();
    options.relative_tolerance = Value::NAN;
    assert!(
        topology
            .solve(&[0.0], &[0.0], &options, &NoAbort, |_, _| {
                panic!("invalid solve options must fail before model evaluation")
            })
            .is_err()
    );
    options.relative_tolerance = 1e-11;
    let mut calls = 0;
    let failure = topology
        .solve(&[0.0], &[1.0], &options, &NoAbort, |state, _| {
            calls += 1;
            let mut sample = EventSample::new(1, &options)?;
            branch(&mut sample.q, state, 1, 0, 1.0);
            if calls == 2 {
                sample.f.stamp(2, 1, 1.0);
                sample.q_time[0] = Value::NAN;
            }
            Ok(sample)
        })
        .err()
        .unwrap();
    assert!(failure.to_string().contains("index outside"), "{failure}");
    assert_eq!(calls, 2);
}

#[test]
fn capacitor_branch_charge_jump_and_finite_current_have_distinct_units() {
    use crate::abort_signal::CountingAbort;
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        4,
        &[(1, 2)],
        vec![source(1, 2, 2.0, 3.0)],
        vec![
            EventBranchEquation::Algebraic(options.voltage_tolerance),
            EventBranchEquation::ChargeCurrent {
                positive: 1,
                negative: 2,
                charge_tolerance: options.charge_tolerance,
                current_tolerance: options.current_tolerance,
            },
        ],
        &options,
        &NoAbort,
    )
    .unwrap();
    let incoming = [1.0, 0.0, 0.0, 0.0];
    let charge = [0.0, 0.0, 0.0, -5.0];
    let sample = |state: &[Value], _: &dyn AbortSignal| {
        let mut sample = EventSample::new(4, &options)?;
        branch(&mut sample.f, state, 2, 0, 1.0);
        sample.q.stamp(4, 1, -5.0);
        sample.q.stamp(4, 2, 5.0);
        sample.q.stamp_rhs(4, 5.0 * (state[0] - state[1]));
        Ok(sample)
    };
    let census = CountingAbort::new(usize::MAX);
    let state = topology
        .solve(&incoming, &charge, &options, &census, sample)
        .unwrap();
    // Five farads gain one volt: 5 C flows through C and -5 C through V.
    // The independent outgoing slope is 3 V/s, hence finite I_C=15 A.
    for (&actual, expected) in state.solution.iter().zip([2.0, 0.0, -15.0, 15.0]) {
        close(actual, expected, 1e-12);
    }
    assert_eq!(topology.source_branches().collect::<Vec<_>>(), [2, 3]);
    for (&actual, expected) in state.source_impulses.iter().zip([-5.0, 5.0]) {
        close(actual, expected, 1e-12);
    }
    close(state.coordinate_rates[0].unwrap(), 3.0, 1e-12);
    close(state.coordinate_rates[1].unwrap(), 0.0, 1e-12);
    assert_eq!(&state.coordinate_rates[2..], [None, None]);
    for poll in [0, census.count() / 2, census.count() - 1] {
        let abort = CountingAbort::new(poll);
        assert!(matches!(
            topology.solve(&incoming, &charge, &options, &abort, sample),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }
}

#[test]
fn capacitor_current_descriptors_validate_terminals_tolerances_and_ownership() {
    let options = options();
    let equation =
        |positive, charge_tolerance, current_tolerance| EventBranchEquation::ChargeCurrent {
            positive,
            negative: 0,
            charge_tolerance,
            current_tolerance,
        };
    for invalid in [
        equation(2, 1e-15, 1e-12),
        equation(1, 0.0, 1e-12),
        equation(1, 1e-15, Value::NAN),
        equation(1, Value::INFINITY, 1e-12),
    ] {
        assert!(
            ChargeEventTopology::new(1, 2, &[], vec![], vec![invalid], &options, &NoAbort,)
                .is_err()
        );
    }
    // A voltage constraint cannot also own the capacitor's current row.
    assert!(
        ChargeEventTopology::new(
            1,
            2,
            &[(1, 0)],
            vec![EventVoltageSource {
                positive: 1,
                negative: 0,
                branch: 1,
                equation: EventVoltageEquation::Affine {
                    value: 1.0,
                    slope: 0.0,
                    control: None
                },
            }],
            vec![equation(1, 1e-15, 1e-12)],
            &options,
            &NoAbort,
        )
        .is_err()
    );
}
