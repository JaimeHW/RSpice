use super::*;
use crate::NoAbort;
use crate::resource::ResourceLimits;

type Symbol = (usize, usize);

fn row(terms: &[(usize, Value)], source: Option<Symbol>) -> ExactRow<Symbol> {
    let mut row = ExactRow::default();
    for &(coordinate, value) in terms {
        ExactElimination::<Symbol>::add_integer(
            &mut row.nodes,
            coordinate,
            integer_coefficient(value).unwrap(),
        );
    }
    if let Some(source) = source {
        row.values.insert(source, integer_coefficient(1.0).unwrap());
    }
    row
}

fn close(size: usize, rows: Vec<ExactRow<Symbol>>) -> ExactElimination<Symbol> {
    close_descriptor(
        size,
        rows,
        ResourceLimits::default(),
        size * 32,
        &NoAbort,
        |(source, order)| Ok((source, order + 1)),
        |_| panic!("independent source constraint"),
    )
    .unwrap()
    .unwrap()
}

fn form(reducer: &ExactElimination<Symbol>, coordinate: usize) -> BTreeMap<Symbol, Value> {
    let mut row = ExactRow {
        query: BigInt::from(1),
        ..ExactRow::default()
    };
    row.nodes.insert(coordinate, BigInt::from(1));
    reducer.reduce(&mut row, &NoAbort).unwrap();
    assert!(row.nodes.is_empty(), "unresolved physical coordinate");
    row.values
        .into_iter()
        .map(|(symbol, coefficient)| {
            (
                symbol,
                -coefficient_ratio(&coefficient, &row.query).unwrap(),
            )
        })
        .collect()
}

#[test]
fn ccvs_descriptor_preserves_independent_flux_and_differentiated_control_current() {
    // C*v1' + iV = 0, iH + iL = 0, v1 = u,
    // v2 = R*iV, L*iL' = v2. Only winding current is independent.
    let c = 2e-12;
    let r = 3e3;
    let l = 4e-6;
    let rows = vec![
        row(&[(6, c), (3, 1.0)], None),
        row(&[(4, 1.0), (5, 1.0)], None),
        row(&[(1, 1.0)], Some((0, 0))),
        row(&[(2, 1.0), (3, -r)], None),
        row(&[(2, 1.0), (10, -l)], None),
    ];
    let mut closure = close(5, rows);
    assert_eq!(closure.rows.len(), 4);
    assert!(
        closure
            .admit(row(&[(5, 1.0)], Some((1, 0))), 1, &NoAbort)
            .unwrap()
            .is_none()
    );
    assert_eq!(form(&closure, 1), BTreeMap::from([((0, 0), 1.0)]));
    assert_eq!(form(&closure, 3), BTreeMap::from([((0, 1), -c)]));
    assert_eq!(form(&closure, 2), BTreeMap::from([((0, 1), -r * c)]));
    assert_eq!(form(&closure, 4), BTreeMap::from([((1, 0), -1.0)]));
}

#[test]
fn ccvs_capacitive_output_exposes_second_derivative_instead_of_losing_it() {
    // The second capacitor differentiates the CCVS's differentiated control:
    // iH = R*C1*C2*u''. This circuit needs more than delta-current output.
    let mut rows = vec![
        row(&[(5, 2.0), (3, 1.0)], None),
        row(&[(6, 5.0), (4, 1.0)], None),
        row(&[(1, 1.0)], Some((0, 0))),
        row(&[(2, 1.0), (3, -3.0)], None),
    ];
    for _ in 0..4 {
        let closure = close(4, rows.clone());
        assert_eq!(closure.rows.len(), 4);
        assert_eq!(form(&closure, 4), BTreeMap::from([((0, 2), 30.0)]));
        rows.rotate_left(1);
    }
}

#[test]
fn descriptor_closure_retains_exact_feedback_rank_below_product_precision() {
    let a = 1.0 + 2_f64.powi(-30);
    let b = 1.0 - 2_f64.powi(-30);
    assert_eq!(a * b, 1.0);
    let closure = close(
        2,
        vec![
            row(&[(1, 1.0), (2, -a)], None),
            row(&[(2, 1.0), (1, -b)], Some((0, 0))),
        ],
    );
    assert_eq!(closure.rows.len(), 2);
    assert_eq!(
        form(&closure, 2),
        BTreeMap::from([((0, 0), 2_f64.powi(60))])
    );
    assert_eq!(
        form(&closure, 1),
        BTreeMap::from([((0, 0), a * 2_f64.powi(60))])
    );
}

#[test]
fn differentiation_combines_symbols_with_a_shared_derivative() {
    // x1 = u + 2*v, x1' + x2 = 0, and u' = v' = w.
    let mut clamp = row(&[(1, 1.0)], Some((0, 0)));
    clamp
        .values
        .insert((1, 0), integer_coefficient(2.0).unwrap());
    let closure = close_descriptor(
        2,
        vec![row(&[(3, 1.0), (2, 1.0)], None), clamp],
        ResourceLimits::default(),
        64,
        &NoAbort,
        |(_, order)| Ok((2, order + 1)),
        |_| panic!("unexpected forcing constraint"),
    )
    .unwrap()
    .unwrap();
    assert_eq!(form(&closure, 2), BTreeMap::from([((2, 1), -3.0)]));
}

#[test]
fn forcing_only_constraints_remain_the_analysis_owners_explicit_decision() {
    let rows = || {
        vec![
            row(&[(1, 1.0)], Some((0, 0))),
            row(&[(1, 1.0)], Some((1, 0))),
        ]
    };
    let mut observed = Vec::new();
    let closure = close_descriptor(
        1,
        rows(),
        ResourceLimits::default(),
        32,
        &NoAbort,
        |(source, order)| Ok((source, order + 1)),
        |row| {
            assert!(row.nodes.is_empty());
            observed.push(row.values);
            Ok(ConstraintDisposition::RetainAtOwner)
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!(closure.rows.len(), 1);
    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0].len(), 2);
    assert!(
        close_descriptor(
            1,
            rows(),
            ResourceLimits::default(),
            32,
            &NoAbort,
            |(source, order)| Ok((source, order + 1)),
            |_| Ok(ConstraintDisposition::Defer),
        )
        .unwrap()
        .is_none()
    );
}

#[test]
fn descriptor_closure_rejects_invalid_coordinates_and_preserves_typed_failures() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct StopAfter {
        polls: AtomicUsize,
        limit: usize,
    }
    impl AbortSignal for StopAfter {
        fn is_aborted(&self) -> bool {
            let polls = self.polls.fetch_add(1, Ordering::Relaxed) + 1;
            polls >= self.limit
        }
    }
    let run = |size, rows, limits, abort: &dyn AbortSignal| {
        close_descriptor(
            size,
            rows,
            limits,
            32,
            abort,
            |(source, order): Symbol| Ok((source, order + 1)),
            |_| Ok(ConstraintDisposition::Defer),
        )
    };
    for (size, rows) in [
        (0, Vec::new()),
        (usize::MAX, Vec::new()),
        (1, vec![row(&[(0, 1.0)], None)]),
        (1, vec![row(&[(3, 1.0)], None)]),
    ] {
        assert!(matches!(
            run(size, rows, ResourceLimits::default(), &NoAbort),
            Err(SimulationError::Circuit(_))
        ));
    }
    let limits = ResourceLimits {
        max_result_values: 1,
        ..ResourceLimits::default()
    };
    assert!(matches!(
        run(1, vec![row(&[(1, 1.0)], None)], limits, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    for limit in [1, 10] {
        let abort = StopAfter {
            polls: AtomicUsize::new(0),
            limit,
        };
        assert!(matches!(
            run(
                1,
                vec![row(&[(1, 1.0)], Some((0, 0)))],
                ResourceLimits::default(),
                &abort
            ),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.polls.load(Ordering::Relaxed), limit);
    }
    let failure = close_descriptor(
        1,
        vec![row(&[(1, 1.0)], Some((0, 0)))],
        ResourceLimits::default(),
        32,
        &NoAbort,
        |_| {
            Err(SimulationError::Circuit(
                "unavailable forcing derivative".to_owned(),
            ))
        },
        |_| Ok(ConstraintDisposition::Defer),
    )
    .unwrap_err();
    assert!(
        failure
            .to_string()
            .contains("unavailable forcing derivative")
    );
}
