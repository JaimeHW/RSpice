use super::*;
use crate::abort_signal::NoAbort;
use crate::device::MatrixStamper;

mod current_coupling;
mod flux;
mod native;

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
        value,
        slope,
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
