//! Limits apply while events are produced, not after constructing the streams.
use rspice_core::abort_signal::{CountingAbort, NoAbort};
use rspice_core::analysis::MeasureEngine;
use rspice_core::{Netlist, ResourceKind, ResourceLimits, SimulationError};
use std::collections::HashMap;

fn engine(cards: &str) -> MeasureEngine {
    let netlist = Netlist::parse_with_options(
        &format!("* continuous limits\n{cards}\n.END\n"),
        rspice_core::netlist::NetlistParseOptions {
            expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let mut engine = MeasureEngine::new();
    for statement in netlist.measurements {
        engine.add(statement);
    }
    engine
}

fn limits(values: usize) -> ResourceLimits {
    {
        let mut policy = ResourceLimits::default();
        policy.max_result_values = values;
        policy
    }
}

#[test]
fn numeric_fields_and_all_statements_share_the_budget() {
    let engine = engine(
        ".MEAS TRAN_CONT events WHEN V(out)=0 CROSS=1 FAILVALUE=2\n.MEAS TRAN_CONT sample FIND V(out) AT=0.5",
    );
    let axis = [0.0, 1.0, 2.0];
    let signal = [-1.0, 1.0, -1.0];
    let signals = HashMap::from([("V(out)".into(), signal.as_slice())]);
    let results = engine
        .evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits(11), &NoAbort)
        .unwrap();
    assert_eq!(results[0].records.len(), 2);
    assert_eq!(results[1].records[0].value, 0.0);
    for result in results {
        result.validate_invariants().unwrap();
    }
    let error = engine
        .evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits(10), &NoAbort)
        .unwrap_err();
    assert!(
        matches!(error, SimulationError::ResourceLimit(limit) if limit.resource == ResourceKind::ResultValues && limit.requested == 11 && limit.limit == 10)
    );
}

#[test]
fn last_occurrence_and_unmatched_delay_do_not_charge_discarded_events() {
    let engine = engine(
        ".MEAS TRAN_CONT last WHEN V(out)=0 CROSS=-1\n.MEAS TRAN_CONT missing TRIG V(out) VAL=0 CROSS=1 TARG V(out) VAL=2 CROSS=1",
    );
    let axis: Vec<_> = (0..100_001).map(|i| i as f64).collect();
    let signal: Vec<_> = (0..100_001)
        .map(|i| if i % 2 == 0 { -1.0 } else { 1.0 })
        .collect();
    let signals = HashMap::from([("V(out)".into(), signal.as_slice())]);
    let results = engine
        .evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits(4), &NoAbort)
        .unwrap();
    assert_eq!(results[0].records[0].value, 99_999.5);
    assert!(results[1].failure.is_some());
    assert_eq!(results[1].failure_metadata.unwrap().trigger_axis, Some(0.5));
    for result in results {
        result.validate_invariants().unwrap();
    }
}

#[test]
fn delay_pairs_count_both_endpoints_and_failure_limit() {
    let engine = engine(
        ".MEAS TRAN_CONT delay TRIG V(out) VAL=0 CROSS=1 TARG V(out) VAL=0.5 CROSS=1 FAILVALUE=2",
    );
    let axis = [0.0, 1.0, 2.0];
    let signal = [-1.0, 1.0, -1.0];
    let signals = HashMap::from([("V(out)".into(), signal.as_slice())]);
    let results = engine
        .evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits(10), &NoAbort)
        .unwrap();
    assert_eq!(results[0].records.len(), 2);
    assert_eq!(results[0].records[0].value, 0.25);
    assert_eq!(results[0].records[1].value, -0.25);
    let error = engine
        .evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits(9), &NoAbort)
        .unwrap_err();
    assert!(matches!(error, SimulationError::ResourceLimit(limit) if limit.requested == 10));
}

#[test]
fn empty_and_failed_streams_cannot_bypass_the_budget() {
    let engine =
        engine(".MEAS TRAN_CONT one WHEN V(missing)=0\n.MEAS TRAN_CONT two WHEN V(missing)=0");
    for axis in [&[][..], &[0.0, 1.0][..]] {
        let error = engine
            .evaluate_continuous_with_limits_and_abort(
                axis,
                &HashMap::new(),
                &[],
                &limits(1),
                &NoAbort,
            )
            .unwrap_err();
        assert!(matches!(error, SimulationError::ResourceLimit(limit) if limit.requested == 2));
        let results = engine
            .evaluate_continuous_with_limits_and_abort(
                axis,
                &HashMap::new(),
                &[],
                &limits(2),
                &NoAbort,
            )
            .unwrap();
        assert!(results.iter().all(|result| result.failure.is_some()));
    }
}

#[test]
fn input_axis_limit_and_preexisting_cancellation_are_typed() {
    let engine = engine(".MEAS TRAN_CONT sample FIND V(out) AT=0.5");
    let axis = [0.0, 1.0];
    let signals = HashMap::from([("V(out)".into(), axis.as_slice())]);
    let mut policy = limits(3);
    policy.max_analysis_points = 1;
    assert!(
        matches!(engine.evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &policy, &NoAbort), Err(SimulationError::ResourceLimit(limit)) if limit.resource == ResourceKind::AnalysisPoints)
    );
    assert!(matches!(
        engine.evaluate_continuous_with_limits_and_abort(
            &axis,
            &signals,
            &[],
            &limits(3),
            &CountingAbort::new(1)
        ),
        Err(SimulationError::Aborted)
    ));
}
