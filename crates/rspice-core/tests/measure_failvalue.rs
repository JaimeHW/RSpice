//! Public contracts for Xyce `.MEAS ... FAILVALUE` parsing and evaluation.

use rspice_core::analysis::MeasureEngine;
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use std::collections::HashMap;

fn parse_xyce(deck: &str) -> Netlist {
    Netlist::parse_with_options(
        deck,
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..NetlistParseOptions::default()
        },
    )
    .expect("Xyce measurement deck parses")
}

fn evaluate(deck: &str, axis: &[f64], values: &[f64]) -> Vec<rspice_core::MeasureResult> {
    let netlist = parse_xyce(deck);
    let mut engine = MeasureEngine::new();
    for statement in netlist.measurements {
        engine.add(statement);
    }
    let signals = HashMap::from([("V(OUT)".to_string(), values)]);
    engine.evaluate(axis, &signals)
}

#[test]
fn failvalue_is_typed_order_independent_and_last_duplicate_wins() {
    let netlist = parse_xyce(
        "FAILVALUE parser contract\n\
         .param LIMIT=3\n\
         V1 FAILVALUE 0 1\n\
         .dc V1 0 1 1\n\
         .measure dc sample AVG V(FAILVALUE) FROM=0 FAILVALUE +LIMIT TO=1 FAILVALUE=-{LIMIT/2}\n\
         .measure dc zero AVG V(FAILVALUE) FAILVALUE = 0\n\
         .end\n",
    );

    assert_eq!(netlist.measurements[0].fail_value, Some(-1.5));
    assert_eq!(netlist.measurements[1].fail_value, Some(0.0));
    assert!(Netlist::parse(
        "FAILVALUE is Xyce-specific\nV1 out 0 1\n.tran 1n 2n\n.measure tran sample AVG V(out) FAILVALUE=1\n.end\n"
    )
    .is_err());

    for qualifier in [
        "FAILVALUE",
        "FAILVALUE=",
        "FAILVALUE=UNKNOWN",
        "FAILVALUE FROM=0",
    ] {
        let deck = format!(
            "malformed FAILVALUE\nV1 out 0 1\n.tran 1n 2n\n.measure tran sample AVG V(out) {qualifier}\n.end\n"
        );
        assert!(
            Netlist::parse_with_options(
                &deck,
                NetlistParseOptions {
                    expression_dialect: ExpressionDialect::Xyce,
                    ..NetlistParseOptions::default()
                },
            )
            .is_err()
        );
    }
}

#[test]
fn failvalue_uses_the_exact_inclusive_absolute_value_comparison() {
    let results = evaluate(
        "FAILVALUE comparator contract\n\
         V1 out 0 0\n\
         .tran 1 2\n\
         .measure tran positive MAX V(out) FAILVALUE=2\n\
         .measure tran negative MIN V(out) FAILVALUE=2\n\
         .measure tran below MAX V(out) FAILVALUE=2.000000000000001\n\
         .end\n",
        &[0.0, 1.0],
        &[-2.0, 2.0],
    );

    for result in &results[..2] {
        assert_eq!(result.failure_limit, Some(2.0));
        assert!(result.failure_limit_exceeded);
        assert!(!result.passed);
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("FAILVALUE"))
        );
    }
    assert_eq!(results[2].raw_value, Some(2.0));
    assert!(!results[2].failure_limit_exceeded);
    assert!(results[2].passed);
}

#[test]
fn projected_extrema_verify_and_feed_param_from_the_raw_dependent_value() {
    let netlist = parse_xyce(
        "FAILVALUE projected extrema contract\n\
         V1 out 0 0\n\
         .tran 1 30\n\
         .measure tran peak MAX V(out) OUTPUT=TIME FAILVALUE=4\n\
         .end\n",
    );
    let mut engine = MeasureEngine::new();
    engine.add(netlist.measurements[0].clone());
    let dependent = Netlist::parse(
        "Dependent measurement contract\n\
         V1 out 0 0\n\
         .tran 1 30\n\
         .measure tran dependent PARAM='peak'\n\
         .end\n",
    )
    .expect("dependent PARAM measurement parses")
    .measurements
    .into_iter()
    .next()
    .expect("dependent PARAM measurement exists");
    engine.add(dependent);
    let axis = [10.0, 20.0, 30.0];
    let values = [2.0, 5.0, 3.0];
    let signals = HashMap::from([("V(OUT)".to_string(), values.as_slice())]);
    let results = engine.evaluate(&axis, &signals);

    assert_eq!(results[0].value, Some(20.0));
    assert_eq!(results[0].raw_value, Some(5.0));
    assert_eq!(results[0].event_axis, Some(20.0));
    assert_eq!(results[0].failure_limit, Some(4.0));
    assert!(results[0].failure_limit_exceeded);
    assert!(!results[0].passed);

    assert_eq!(results[1].value, Some(5.0));
    assert_eq!(results[1].raw_value, Some(5.0));
    assert!(results[1].passed);
}
