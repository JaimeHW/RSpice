//! Selecting a single event must not materialize every crossing of a waveform.
#![cfg(not(target_arch = "wasm32"))]

#[path = "common/allocations.rs"]
mod allocations;

use rspice_core::analysis::MeasureEngine;
use std::collections::HashMap;

fn engine(source: &str) -> MeasureEngine {
    let netlist = rspice_core::Netlist::parse(source).unwrap();
    let mut engine = MeasureEngine::new();
    for statement in netlist.measurements {
        engine.add(statement);
    }
    engine
}

#[test]
fn first_and_reverse_indexed_crossings_do_not_allocate_per_input_event() {
    let scalar = engine(
        "* scalar events\n\
        .MEASURE TRAN first WHEN V(out)=0 CROSS=1\n\
        .MEASURE TRAN last WHEN V(out)=0 CROSS=-1\n\
        .MEASURE TRAN previous WHEN V(out)=0 CROSS=-2\n.END\n",
    );
    let continuous = engine(
        "* reverse continuous events\n\
        .MEASURE TRAN_CONT last WHEN V(out)=0 CROSS=-1\n\
        .MEASURE TRAN_CONT previous WHEN V(out)=0 CROSS=-2\n.END\n",
    );
    for points in [129, 100_001] {
        let axis: Vec<_> = (0..points).map(|index| index as f64).collect();
        let signal: Vec<_> = (0..points)
            .map(|index| if index % 2 == 0 { -1.0 } else { 1.0 })
            .collect();
        let signals = HashMap::from([("V(out)".to_string(), signal.as_slice())]);
        let (results, bytes) = allocations::measured(|| scalar.evaluate(&axis, &signals));
        assert_eq!(
            results
                .iter()
                .map(|result| result.value)
                .collect::<Vec<_>>(),
            vec![
                Some(0.5),
                Some(points as f64 - 1.5),
                Some(points as f64 - 2.5)
            ]
        );
        assert!(
            bytes < 128 * 1024,
            "scalar event selection allocated {bytes} bytes for {points} points"
        );
        let (results, bytes) =
            allocations::measured(|| continuous.evaluate_continuous(&axis, &signals, &[]));
        assert_eq!(results.len(), 2);
        for (index, result) in results.iter().enumerate() {
            assert!(result.passed());
            assert_eq!(result.records.len(), 1);
            assert_eq!(result.records[0].value, points as f64 - 1.5 - index as f64);
        }
        assert!(
            bytes < 128 * 1024,
            "reverse continuous event selection allocated {bytes} bytes for {points} points"
        );
    }
}

#[test]
fn rejected_streams_and_unpaired_delays_do_not_buffer_all_candidates() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::{ResourceLimits, SimulationError};

    let axis: Vec<_> = (0..100_001).map(|index| index as f64).collect();
    let signal: Vec<_> = (0..axis.len())
        .map(|index| if index % 2 == 0 { -1.0 } else { 1.0 })
        .collect();
    let signals = HashMap::from([("V(out)".to_string(), signal.as_slice())]);
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 6;
    let events = engine("* bounded events\n.MEAS TRAN_CONT events WHEN V(out)=0 CROSS=1\n.END\n");
    let (result, bytes) = allocations::measured(|| {
        events.evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits, &NoAbort)
    });
    assert!(matches!(result, Err(SimulationError::ResourceLimit(limit)) if limit.requested == 9));
    assert!(
        bytes < 128 * 1024,
        "rejected stream allocated {bytes} bytes"
    );

    let delay = engine(
        "* unpaired delay\n.MEAS TRAN_CONT delay TRIG V(out) VAL=0 CROSS=1 TARG V(out) VAL=2 CROSS=1\n.END\n",
    );
    let (result, bytes) = allocations::measured(|| {
        delay.evaluate_continuous_with_limits_and_abort(&axis, &signals, &[], &limits, &NoAbort)
    });
    let results = result.unwrap();
    assert!(results[0].failure.is_some());
    assert_eq!(results[0].failure_metadata.unwrap().trigger_axis, Some(0.5));
    assert!(bytes < 128 * 1024, "unpaired delay allocated {bytes} bytes");
}
