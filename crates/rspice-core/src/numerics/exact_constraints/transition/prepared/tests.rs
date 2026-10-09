use super::*;
use crate::NoAbort;
use std::sync::atomic::{AtomicUsize, Ordering};

type Triplets = Vec<(usize, usize, Value)>;

fn close(actual: Value, expected: Value) {
    assert!(
        (actual - expected).abs() <= 2e-13 * expected.abs().max(1.0),
        "{actual:e} != {expected:e}"
    );
}

fn audit(prepared: &PreparedTransition, storage: &[Value], result: &Transition, forcing: &[Value]) {
    for row in 0..prepared.size {
        let multiply = |terms: &[(usize, Value)], values: &[Value]| -> Value {
            terms
                .iter()
                .map(|&(column, coefficient)| coefficient * values[column])
                .sum()
        };
        let finite = multiply(&prepared.a[row], result.finite())
            + multiply(&prepared.e[row], result.rates());
        close(finite, forcing[row]);
        let jump = multiply(&prepared.e[row], result.finite()) - storage[row]
            + result
                .impulse(0)
                .map_or(0.0, |impulse| multiply(&prepared.a[row], impulse));
        close(jump, 0.0);
        for order in 0..prepared.impulse_orders {
            let residual = multiply(&prepared.e[row], result.impulse(order).unwrap())
                + result
                    .impulse(order + 1)
                    .map_or(0.0, |impulse| multiply(&prepared.a[row], impulse));
            close(residual, 0.0);
        }
    }
}

fn inductive() -> (Triplets, Triplets) {
    // v1, v2, iV, iH, iL. C=2, Rm=3, L=5.
    (
        vec![
            (0, 2, 1.0),
            (1, 3, 1.0),
            (1, 4, 1.0),
            (2, 0, 1.0),
            (3, 1, 1.0),
            (3, 2, -3.0),
            (4, 1, 1.0),
        ],
        vec![(0, 0, 2.0), (4, 4, -5.0)],
    )
}

fn capacitive() -> (Triplets, Triplets) {
    // v1, v2, iV, iH. C1=2, C2=5, Rm=3.
    (
        vec![
            (0, 2, 1.0),
            (1, 3, 1.0),
            (2, 0, 1.0),
            (3, 1, 1.0),
            (3, 2, -3.0),
        ],
        vec![(0, 0, 2.0), (1, 1, 5.0)],
    )
}

fn jet(row: usize, order: usize) -> Result<Value> {
    Ok(if row == 2 {
        [1.0, 2.0, 3.0, 4.0][order]
    } else {
        0.0
    })
}

#[test]
fn ordinary_dynamic_and_algebraic_coordinates_have_no_spurious_impulses() {
    let limits = ResourceLimits::default();
    let ode = PreparedTransition::new(1, &[(0, 0, 3.0)], &[(0, 0, 2.0)], limits, &NoAbort).unwrap();
    assert_eq!(ode.impulse_orders, 0);
    assert_eq!(ode.forcing_orders, [0]);
    let result = ode
        .evaluate(
            &[8.0],
            |row, order| {
                assert_eq!((row, order), (0, 0));
                Ok(20.0)
            },
            limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(result.finite(), &[4.0]);
    assert_eq!(result.rates(), &[4.0]);
    assert!(result.impulse(0).is_none());
    audit(&ode, &[8.0], &result, &[20.0]);
    let algebraic = PreparedTransition::new(1, &[(0, 0, 2.0)], &[], limits, &NoAbort).unwrap();
    assert_eq!(algebraic.impulse_orders, 0);
    let result = algebraic
        .evaluate(&[0.0], |_, order| Ok([6.0, 8.0][order]), limits, &NoAbort)
        .unwrap();
    assert_eq!(result.finite(), &[3.0]);
    assert_eq!(result.rates(), &[4.0]);
    audit(&algebraic, &[0.0], &result, &[6.0]);
}

#[test]
fn clamped_capacitor_separates_impulsive_and_finite_source_current() {
    let limits = ResourceLimits::default();
    let prepared = PreparedTransition::new(
        2,
        &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 1.0)],
        &[(0, 0, 2.0)],
        limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(prepared.impulse_orders, 1);
    let result = prepared
        .evaluate(
            &[1.0, 0.0],
            |row, order| {
                Ok(if row == 1 {
                    [1.0, 3.0, 5.0][order]
                } else {
                    0.0
                })
            },
            limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(result.finite(), &[1.0, -10.0]);
    assert_eq!(result.rates(), &[3.0, -22.0]);
    assert_eq!(result.impulse(0).unwrap(), &[0.0, -1.0]);
    audit(&prepared, &[1.0, 0.0], &result, &[0.0, 1.0]);
}

#[test]
fn ccvs_voltage_impulse_changes_winding_current_and_preserves_every_equation() {
    let limits = ResourceLimits::default();
    let (a, e) = inductive();
    for permutation in [[0, 1, 2, 3, 4], [4, 3, 2, 1, 0], [2, 4, 1, 0, 3]] {
        let permute = |entries: &Triplets| {
            entries
                .iter()
                .map(|&(row, column, value)| (permutation[row], permutation[column], value))
                .collect::<Vec<_>>()
        };
        let prepared =
            PreparedTransition::new(5, &permute(&a), &permute(&e), limits, &NoAbort).unwrap();
        assert_eq!(prepared.impulse_orders, 1);
        let mut storage = [0.0; 5];
        storage[permutation[4]] = -35.0;
        let mut forcing = [0.0; 5];
        forcing[permutation[2]] = 1.0;
        let result = prepared
            .evaluate(
                &storage,
                |row, order| {
                    let original = permutation
                        .iter()
                        .position(|&mapped| mapped == row)
                        .unwrap();
                    jet(original, order)
                },
                limits,
                &NoAbort,
            )
            .unwrap();
        for (original, (&value, &rate)) in [1.0, -12.0, -4.0, -5.8, 5.8]
            .iter()
            .zip(&[2.0, -18.0, -6.0, 2.4, -2.4])
            .enumerate()
        {
            close(result.finite()[permutation[original]], value);
            close(result.rates()[permutation[original]], rate);
        }
        for (original, &value) in [0.0, -6.0, -2.0, 0.0, 0.0].iter().enumerate() {
            close(result.impulse(0).unwrap()[permutation[original]], value);
        }
        audit(&prepared, &storage, &result, &forcing);
    }
}

#[test]
fn ccvs_capacitive_load_retains_delta_prime_current_and_the_regular_jet() {
    let limits = ResourceLimits::default();
    let (a, e) = capacitive();
    let prepared = PreparedTransition::new(4, &a, &e, limits, &NoAbort).unwrap();
    assert_eq!(prepared.impulse_orders, 2);
    let result = prepared.evaluate(&[0.0; 4], jet, limits, &NoAbort).unwrap();
    assert_eq!(result.finite(), &[1.0, -12.0, -4.0, 90.0]);
    assert_eq!(result.rates(), &[2.0, -18.0, -6.0, 120.0]);
    assert_eq!(result.impulse(0).unwrap(), &[0.0, -6.0, -2.0, 60.0]);
    assert_eq!(result.impulse(1).unwrap(), &[0.0, 0.0, 0.0, 30.0]);
    assert!(result.impulse(2).is_none());
    audit(&prepared, &[0.0; 4], &result, &[0.0, 0.0, 1.0, 0.0]);
}

#[test]
fn transition_keeps_feedback_rank_below_binary64_product_precision() {
    let limits = ResourceLimits::default();
    let a = 1.0 + 2_f64.powi(-30);
    let b = 1.0 - 2_f64.powi(-30);
    assert_eq!(a * b, 1.0);
    let prepared = PreparedTransition::new(
        2,
        &[(0, 0, 1.0), (0, 1, -a), (1, 0, -b), (1, 1, 1.0)],
        &[],
        limits,
        &NoAbort,
    )
    .unwrap();
    let result = prepared
        .evaluate(
            &[0.0; 2],
            |row, order| Ok(if row == 1 { [1.0, 2.0][order] } else { 0.0 }),
            limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(result.finite(), &[a * 2_f64.powi(60), 2_f64.powi(60)]);
    assert_eq!(result.rates(), &[a * 2_f64.powi(61), 2_f64.powi(61)]);
}

#[test]
fn nilpotent_chain_preserves_every_required_impulse_derivative() {
    // N*x' + x=b, with N a length-five nilpotent Jordan block. The exact
    // regular solution is sum_k (-N)^k b^(k); jump coefficients follow
    // xi_0=q-N*x+, xi_(k+1)=-N*xi_k. No chosen timestep or order cap enters.
    let limits = ResourceLimits::default();
    let a = (0..5).map(|i| (i, i, 1.0)).collect::<Vec<_>>();
    let e = (0..4).map(|i| (i, i + 1, 1.0)).collect::<Vec<_>>();
    let prepared = PreparedTransition::new(5, &a, &e, limits, &NoAbort).unwrap();
    assert_eq!(prepared.impulse_orders, 4);
    let storage = [2.0, 3.0, 4.0, 5.0, 0.0];
    let result = prepared
        .evaluate(
            &storage,
            |row, order| {
                Ok(if row == 4 {
                    [1.0, 2.0, 6.0, 24.0, 120.0, 720.0][order]
                } else {
                    0.0
                })
            },
            limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(result.finite(), &[120.0, -24.0, 6.0, -2.0, 1.0]);
    assert_eq!(result.rates(), &[720.0, -120.0, 24.0, -6.0, 2.0]);
    for (order, expected) in [
        [26.0, -3.0, 6.0, 4.0, 0.0],
        [3.0, -6.0, -4.0, 0.0, 0.0],
        [6.0, 4.0, 0.0, 0.0, 0.0],
        [-4.0, 0.0, 0.0, 0.0, 0.0],
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(result.impulse(order).unwrap(), expected);
    }
    assert!(result.impulse(4).is_none());
    audit(&prepared, &storage, &result, &[0.0, 0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn every_distributional_order_is_audited_before_publication() {
    let limits = ResourceLimits::default();
    let (a, e) = capacitive();
    let prepared = PreparedTransition::new(4, &a, &e, limits, &NoAbort).unwrap();
    let jets = prepared
        .forcing_orders
        .iter()
        .enumerate()
        .map(|(row, &order)| {
            (0..=order)
                .map(|order| jet(row, order).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for coordinate in [0, 3, 5, 7, 11, 12, 15] {
        let mut result = prepared.evaluate(&[0.0; 4], jet, limits, &NoAbort).unwrap();
        result.values[coordinate] += 0.1;
        let storage = prepared
            .storage_from_products(std::iter::empty(), limits, &NoAbort)
            .unwrap();
        assert!(
            prepared.audit(&storage, &jets, &result, &NoAbort).is_err(),
            "unaudited coordinate {coordinate}"
        );
    }
}

struct StopAfter {
    polls: AtomicUsize,
    limit: usize,
}

#[test]
fn authored_capacitor_and_winding_storage_sets_the_startup_jump() {
    let limits = ResourceLimits::default();
    let (a, e) = inductive();
    let prepared = PreparedTransition::new(5, &a, &e, limits, &NoAbort).unwrap();
    // C1 starts at 0.5 V, L starts at 7 A; neither is inferred from a seed x.
    let storage = [1.0, 0.0, 0.0, 0.0, -35.0];
    let result = prepared.evaluate(&storage, jet, limits, &NoAbort).unwrap();
    for (&actual, expected) in result.finite().iter().zip([1.0, -12.0, -4.0, -6.4, 6.4]) {
        close(actual, expected);
    }
    assert_eq!(result.impulse(0).unwrap(), &[0.0, -3.0, -1.0, 0.0, 0.0]);
    audit(&prepared, &storage, &result, &[0.0, 0.0, 1.0, 0.0, 0.0]);

    let (a, e) = capacitive();
    let prepared = PreparedTransition::new(4, &a, &e, limits, &NoAbort).unwrap();
    // C1 starts at 1.5 V, C2 at -4 V. C2's incoming charge affects delta;
    // C1's incoming charge also affects the higher-order current at C2.
    let storage = [3.0, -20.0, 0.0, 0.0];
    let result = prepared.evaluate(&storage, jet, limits, &NoAbort).unwrap();
    assert_eq!(result.finite(), &[1.0, -12.0, -4.0, 90.0]);
    assert_eq!(result.impulse(0).unwrap(), &[0.0, 3.0, 1.0, 40.0]);
    assert_eq!(result.impulse(1).unwrap(), &[0.0, 0.0, 0.0, -15.0]);
    audit(&prepared, &storage, &result, &[0.0, 0.0, 1.0, 0.0]);
}

#[test]
fn incompatible_storage_cannot_create_spurious_algebraic_impulses() {
    let limits = ResourceLimits::default();
    // Direct sum of a clamped capacitor and a purely algebraic coordinate.
    // Global impulse depth is one, but the third row still cannot hold charge.
    let prepared = PreparedTransition::new(
        3,
        &[(0, 1, 1.0), (1, 0, 1.0), (2, 2, 2.0)],
        &[(0, 0, 2.0)],
        limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(prepared.impulse_orders, 1);
    for storage in [
        [0.0, 1.0, 0.0],
        [0.0, 0.0, Value::from_bits(1)],
        [Value::NAN, 0.0, 0.0],
    ] {
        assert!(
            prepared
                .evaluate(
                    &storage,
                    |_, _| panic!("invalid storage reached forcing"),
                    limits,
                    &NoAbort
                )
                .is_err()
        );
    }
    // A floating capacitor gives a nontrivial constraint q1 + q2 = 0.
    let floating = PreparedTransition::new(
        2,
        &[(0, 0, 1.0), (1, 1, 1.0)],
        &[(0, 0, 3.0), (0, 1, -3.0), (1, 0, -3.0), (1, 1, 3.0)],
        limits,
        &NoAbort,
    )
    .unwrap();
    let result = floating
        .evaluate(&[6.0, -6.0], |_, _| Ok(0.0), limits, &NoAbort)
        .unwrap();
    assert_eq!(result.finite(), &[1.0, -1.0]);
    for (&actual, expected) in result.rates().iter().zip([-1.0 / 6.0, 1.0 / 6.0]) {
        close(actual, expected);
    }
    assert!(
        floating
            .evaluate(
                &[6.0, -5.9],
                |_, _| panic!("invalid storage reached forcing"),
                limits,
                &NoAbort
            )
            .is_err()
    );
    // Rounded assembly of a valid charge remains admissible at the numerical
    // equation gate; no fixed floor admits tiny but entirely unbalanced charge.
    floating
        .evaluate(
            &[6.0, -6.0 + Value::EPSILON * 4.0],
            |_, _| Ok(0.0),
            limits,
            &NoAbort,
        )
        .unwrap();
}
impl AbortSignal for StopAfter {
    fn is_aborted(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.limit
    }
}

#[test]
fn malformed_singular_unrepresentable_and_bounded_transitions_are_explicit() {
    let limits = ResourceLimits::default();
    for (size, a) in [
        (0, vec![]),
        (usize::MAX, vec![]),
        (1, vec![]),
        (1, vec![(1, 0, 1.0)]),
        (1, vec![(0, 0, Value::NAN)]),
    ] {
        assert!(PreparedTransition::new(size, &a, &[], limits, &NoAbort).is_err());
    }
    let (a, e) = capacitive();
    for restricted in [
        ResourceLimits {
            max_result_values: 1,
            ..limits
        },
        ResourceLimits {
            max_matrix_unknowns: 4,
            ..limits
        },
    ] {
        assert!(matches!(
            PreparedTransition::new(4, &a, &e, restricted, &NoAbort),
            Err(ConstraintError::ResourceLimit(_))
        ));
    }
    for limit in [1, 50] {
        let abort = StopAfter {
            polls: AtomicUsize::new(0),
            limit,
        };
        assert!(matches!(
            PreparedTransition::new(4, &a, &e, limits, &abort),
            Err(ConstraintError::Aborted)
        ));
        assert_eq!(abort.polls.load(Ordering::Relaxed), limit);
    }
    let prepared = PreparedTransition::new(4, &a, &e, limits, &NoAbort).unwrap();
    assert!(matches!(
        prepared.evaluate(
            &[0.0; 4],
            jet,
            ResourceLimits {
                max_result_values: 1,
                ..limits
            },
            &NoAbort
        ),
        Err(ConstraintError::ResourceLimit(_))
    ));
    assert!(prepared.evaluate(&[0.0; 3], jet, limits, &NoAbort).is_err());
    assert!(
        prepared
            .evaluate(&[0.0; 4], |_, _| Ok(Value::NAN), limits, &NoAbort)
            .is_err()
    );
    for limit in [1, 8] {
        let abort = StopAfter {
            polls: AtomicUsize::new(0),
            limit,
        };
        assert!(matches!(
            prepared.evaluate(&[0.0; 4], jet, limits, &abort),
            Err(ConstraintError::Aborted)
        ));
        assert_eq!(abort.polls.load(Ordering::Relaxed), limit);
    }
    let result = prepared.evaluate(&[0.0; 4], jet, limits, &NoAbort).unwrap();
    assert_eq!(result.finite()[0], 1.0);
    let huge = PreparedTransition::new(1, &[(0, 0, 1e-300)], &[], limits, &NoAbort).unwrap();
    assert!(
        huge.evaluate(&[0.0], |_, _| Ok(1e300), limits, &NoAbort)
            .is_err()
    );
}

#[test]
fn unchanged_picofarad_storage_does_not_create_roundoff_actions() {
    let limits = ResourceLimits::default();
    let (mut a, mut e) = capacitive();
    a.push((1, 1, 0.1));
    e[0].2 = 2e-12;
    e[1].2 = 5e-12;
    let prepared = PreparedTransition::new(4, &a, &e, limits, &NoAbort).unwrap();
    let storage = prepared
        .charge_from_coordinates(&[1.0, 0.0, 0.0, 0.0], limits, &NoAbort)
        .unwrap();
    let result = prepared
        .evaluate_storage(
            &storage,
            |row, order| Ok(if row == 2 && order == 0 { 1.0 } else { 0.0 }),
            limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(result.finite(), [1.0, 0.0, 0.0, 0.0]);
    assert!(result.rates().iter().all(|&v| v == 0.0));
    for order in 0..result.impulse_count() {
        assert!(result.impulse(order).unwrap().iter().all(|&v| v == 0.0));
    }
    let abort = crate::abort_signal::CountingAbort::new(2);
    assert!(matches!(
        prepared.charge_from_coordinates(&[1.0, 0.0, 0.0, 0.0], limits, &abort),
        Err(ConstraintError::Aborted)
    ));
    assert_eq!(abort.polls_after_abort(), 0);
}

#[test]
fn unchanged_nonbinary_flux_has_no_rounding_impulse() {
    let limits = ResourceLimits::default();
    let kernel = PreparedTransition::new(
        2,
        &[(0, 1, 1.0), (1, 0, 1.0)],
        &[(1, 1, -5e-9)],
        limits,
        &NoAbort,
    )
    .unwrap();
    for current in [0.001, 0.0013, -0.00017] {
        let storage = kernel
            .charge_from_coordinates(&[0.0, current], limits, &NoAbort)
            .unwrap();
        let forcing = |row, order| Ok(if row == 0 && order == 0 { current } else { 0.0 });
        let result = kernel
            .evaluate_storage(&storage, forcing, limits, &NoAbort)
            .unwrap();
        assert_eq!(result.finite(), [0.0, current]);
        for order in 0..result.impulse_count() {
            assert!(result.impulse(order).unwrap().iter().all(|&v| v == 0.0));
        }
        // A representable stored flux rounded before projection is different.
        let rounded = kernel
            .evaluate(&[0.0, -5e-9 * current], forcing, limits, &NoAbort)
            .unwrap();
        assert_ne!(rounded.impulse(0).unwrap()[0], 0.0);
    }
}
