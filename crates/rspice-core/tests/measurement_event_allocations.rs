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
