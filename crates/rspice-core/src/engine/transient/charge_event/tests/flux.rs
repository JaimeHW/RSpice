use super::*;

fn flux_row(options: &EventOptions) -> EventBranchEquation {
    EventBranchEquation::Flux {
        flux_tolerance: 1e-20,
        voltage_tolerance: options.voltage_tolerance,
    }
}

pub(super) fn inductor(
    sample: &mut EventSample,
    state: &[Value],
    pos: usize,
    neg: usize,
    column: usize,
    inductance: Value,
) {
    // F + dQ/dt = 0: Q_branch=-L I, F_branch=Vpos-Vneg.
    for (node, sign) in [(pos, 1.0), (neg, -1.0)] {
        sample.f.stamp_rhs(node, -sign * state[column]);
        sample.f.stamp(node, column + 1, sign);
        sample.f.stamp(column + 1, node, sign);
    }
    sample
        .f
        .stamp_rhs(column + 1, -(voltage(state, pos) - voltage(state, neg)));
    sample.q.stamp_rhs(column + 1, inductance * state[column]);
    sample.q.stamp(column + 1, column + 1, -inductance);
}

fn rl_topology(options: &EventOptions) -> ChargeEventTopology {
    ChargeEventTopology::new(
        2,
        4,
        &[],
        vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 3,
            value: 4.0,
            slope: 0.0,
        }],
        vec![
            flux_row(options),
            EventBranchEquation::Algebraic(options.voltage_tolerance),
        ],
        options,
        &NoAbort,
    )
    .unwrap()
}

fn rl_sample(state: &[Value], options: &EventOptions) -> Result<EventSample> {
    let mut sample = EventSample::new(4, options)?;
    branch(&mut sample.f, state, 1, 2, 1.0 / 200.0);
    inductor(&mut sample, state, 2, 0, 2, 3e-3);
    Ok(sample)
}

#[test]
fn flux_event_preserves_rl_current_and_recovers_voltage_and_current_rates() {
    let options = options();
    let incoming = [1.0, 0.6, 0.002, -0.002];
    let result = rl_topology(&options)
        .solve(
            &incoming,
            &[0.0, 0.0, -6e-6, 0.0],
            &options,
            &NoAbort,
            |state, _| rl_sample(state, &options),
        )
        .unwrap();
    close(result.solution[0], 4.0, 1e-12);
    close(result.solution[1], 3.6, 1e-12);
    close(result.solution[2], incoming[2], 1e-14);
    close(result.solution[3], -0.002, 1e-14);
    close(result.source_impulses[0], 0.0, 1e-25);
    close(result.coordinate_rates[0].unwrap(), 0.0, 1e-12);
    close(result.coordinate_rates[1].unwrap(), -240000.0, 1e-7);
    close(result.coordinate_rates[2].unwrap(), 1200.0, 1e-9);
    assert_eq!(result.coordinate_rates[3], None);
}

#[test]
fn flux_event_solves_capacitive_impulse_and_inductor_flux_in_one_system() {
    let options = options();
    let topology = ChargeEventTopology::new(
        2,
        4,
        &[(1, 2), (2, 0)],
        vec![EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 3,
            value: 3.0,
            slope: 0.0,
        }],
        vec![
            flux_row(&options),
            EventBranchEquation::Algebraic(options.voltage_tolerance),
        ],
        &options,
        &NoAbort,
    )
    .unwrap();
    let result = topology
        .solve(
            &[0.0, 0.0, 0.002, -0.002],
            &[0.0, 0.0, -6e-6, 0.0],
            &options,
            &NoAbort,
            |state, _| {
                let mut sample = EventSample::new(4, &options)?;
                branch(&mut sample.q, state, 1, 2, 2e-12);
                branch(&mut sample.q, state, 2, 0, 1e-12);
                inductor(&mut sample, state, 2, 0, 2, 3e-3);
                Ok(sample)
            },
        )
        .unwrap();
    close(result.solution[0], 3.0, 1e-12);
    close(result.solution[1], 2.0, 1e-12);
    close(result.solution[2], 0.002, 1e-14);
    close(result.solution[3], -0.004 / 3.0, 1e-14);
    close(result.source_impulses[0], -2e-12, 1e-25);
    close(result.coordinate_rates[1].unwrap(), -0.002 / 3e-12, 1e-4);
    close(result.coordinate_rates[2].unwrap(), 2.0 / 3e-3, 1e-9);
}

#[test]
fn flux_event_conserves_coupled_linkage_with_both_mutual_polarities() {
    let options = options();
    let sources = vec![
        EventVoltageSource {
            positive: 1,
            negative: 0,
            branch: 4,
            value: 3.0,
            slope: 0.0,
        },
        EventVoltageSource {
            positive: 2,
            negative: 0,
            branch: 5,
            value: -1.0,
            slope: 0.0,
        },
    ];
    let topology = ChargeEventTopology::new(
        2,
        6,
        &[],
        sources,
        vec![
            flux_row(&options),
            flux_row(&options),
            EventBranchEquation::Algebraic(options.voltage_tolerance),
            EventBranchEquation::Algebraic(options.voltage_tolerance),
        ],
        &options,
        &NoAbort,
    )
    .unwrap();
    let old_currents = [0.2, -0.3];
    for mutual in [-0.5e-3, 0.5e-3] {
        let old_flux = [
            0.0,
            0.0,
            -(2e-3 * old_currents[0] + mutual * old_currents[1]),
            -(mutual * old_currents[0] + 1e-3 * old_currents[1]),
            0.0,
            0.0,
        ];
        let result = topology
            .solve(
                &[
                    0.0,
                    0.0,
                    old_currents[0],
                    old_currents[1],
                    -old_currents[0],
                    -old_currents[1],
                ],
                &old_flux,
                &options,
                &NoAbort,
                |state, _| {
                    let mut sample = EventSample::new(6, &options)?;
                    inductor(&mut sample, state, 1, 0, 2, 2e-3);
                    inductor(&mut sample, state, 2, 0, 3, 1e-3);
                    sample.q.stamp(3, 4, -mutual);
                    sample.q.stamp(4, 3, -mutual);
                    sample.q.stamp_rhs(3, mutual * state[3]);
                    sample.q.stamp_rhs(4, mutual * state[2]);
                    Ok(sample)
                },
            )
            .unwrap();
        let determinant = 2e-3 * 1e-3 - mutual * mutual;
        close(result.solution[2], old_currents[0], 1e-14);
        close(result.solution[3], old_currents[1], 1e-14);
        close(
            result.coordinate_rates[2].unwrap(),
            (1e-3 * 3.0 + mutual) / determinant,
            1e-8,
        );
        close(
            result.coordinate_rates[3].unwrap(),
            (-2e-3 - mutual * 3.0) / determinant,
            1e-8,
        );
        for (index, old_current) in old_currents.iter().enumerate() {
            close(result.solution[4 + index], -*old_current, 1e-14);
            close(result.source_impulses[index], 0.0, 1e-25);
        }
    }
}

#[test]
fn flux_event_uses_nonlinear_linkage_and_its_explicit_time_partial() {
    let options = options();
    let topology = ChargeEventTopology::new(
        1,
        2,
        &[],
        vec![],
        vec![flux_row(&options)],
        &options,
        &NoAbort,
    )
    .unwrap();
    for coefficient_rate in [0.0, 0.4] {
        // Switch from Phi=4 I to Phi=I+I^3 at Iin=0.5 A: Phi stays 2 Wb,
        // so Iout=1 A. The parallel 2-ohm resistor sets Vout=-2 V.
        let result = topology
            .solve(
                &[-1.0, 0.5],
                &[0.0, -2.0],
                &options,
                &NoAbort,
                |state, _| {
                    let mut sample = EventSample::new(2, &options)?;
                    branch(&mut sample.f, state, 1, 0, 0.5);
                    inductor(&mut sample, state, 1, 0, 1, 1.0);
                    sample.q.stamp_rhs(2, state[1].powi(3));
                    sample.q.stamp(2, 2, -3.0 * state[1].powi(2));
                    sample.q_time[1] = -coefficient_rate * state[1];
                    Ok(sample)
                },
            )
            .unwrap();
        close(result.solution[0], -2.0, 1e-12);
        close(result.solution[1], 1.0, 1e-12);
        close(
            result.coordinate_rates[1].unwrap(),
            (-2.0 - coefficient_rate) / 4.0,
            1e-12,
        );
        close(
            result.coordinate_rates[0].unwrap(),
            (2.0 + coefficient_rate) / 2.0,
            1e-12,
        );
    }
}

#[test]
fn flux_event_requires_explicit_units_and_refuses_unowned_or_singular_storage() {
    let options = options();
    for invalid in [0.0, -1.0, Value::INFINITY, Value::NAN] {
        let row = EventBranchEquation::Flux {
            flux_tolerance: invalid,
            voltage_tolerance: 1e-12,
        };
        assert!(
            ChargeEventTopology::new(1, 2, &[], vec![], vec![row], &options, &NoAbort).is_err()
        );
        let row = EventBranchEquation::Flux {
            flux_tolerance: 1e-20,
            voltage_tolerance: invalid,
        };
        assert!(
            ChargeEventTopology::new(1, 2, &[], vec![], vec![row], &options, &NoAbort).is_err()
        );
    }
    assert!(
        ChargeEventTopology::new(
            1,
            2,
            &[],
            vec![EventVoltageSource {
                positive: 1,
                negative: 0,
                branch: 1,
                value: 1.0,
                slope: 0.0,
            }],
            vec![flux_row(&options)],
            &options,
            &NoAbort
        )
        .is_err()
    );
    let unowned = ChargeEventTopology::new(
        1,
        2,
        &[],
        vec![],
        vec![EventBranchEquation::Algebraic(1e-12)],
        &options,
        &NoAbort,
    )
    .unwrap();
    let failure = unowned
        .solve(&[0.0, 1.0], &[0.0, -1.0], &options, &NoAbort, |_, _| {
            panic!("unowned incoming flux")
        })
        .err()
        .unwrap();
    assert!(failure.to_string().contains("no prepared flux"));
    let failure = unowned
        .solve(&[0.0, 0.0], &[0.0, 0.0], &options, &NoAbort, |state, _| {
            let mut sample = EventSample::new(2, &options)?;
            inductor(&mut sample, state, 1, 0, 1, 1.0);
            Ok(sample)
        })
        .err()
        .unwrap();
    assert!(failure.to_string().contains("no prepared flux"));
    // A forced current jump in an ideal inductor cutset needs voltage
    // impulse/descriptor equations. Finite-voltage constraints cannot certify it.
    let cutset = ChargeEventTopology::new(
        1,
        2,
        &[],
        vec![],
        vec![flux_row(&options)],
        &options,
        &NoAbort,
    )
    .unwrap();
    assert!(
        cutset
            .solve(&[0.0, 0.0], &[0.0, 0.0], &options, &NoAbort, |state, _| {
                let mut sample = EventSample::new(2, &options)?;
                inductor(&mut sample, state, 1, 0, 1, 1.0);
                sample.f.stamp_rhs(1, 1.0);
                Ok(sample)
            })
            .is_err()
    );
}

#[test]
fn flux_event_cancellation_preserves_incoming_current_and_linkage() {
    use crate::abort_signal::CountingAbort;
    let options = options();
    let topology = rl_topology(&options);
    let incoming = [1.0, 0.6, 0.002, -0.002];
    let flux = [0.0, 0.0, -6e-6, 0.0];
    let solve = |abort: &dyn AbortSignal| {
        topology.solve(&incoming, &flux, &options, abort, |state, _| {
            rl_sample(state, &options)
        })
    };
    let baseline = CountingAbort::new(usize::MAX);
    solve(&baseline).unwrap();
    for threshold in 0..baseline.count() {
        let abort = CountingAbort::new(threshold);
        assert!(matches!(solve(&abort), Err(SimulationError::Aborted)));
        assert_eq!(abort.observed_at(), Some(threshold + 1));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(incoming[2], 0.002);
        assert_eq!(flux[2], -6e-6);
    }
}
