//! Many measurement statements over a wide signal table share one view of
//! it, without timing assumptions.

use std::collections::HashMap;
use std::fmt::Write;

use rspice_core::analysis::evaluate_tran_remeasurements;

use super::allocations;

/// Bytes `statements` averages allocate over a table of `signals` constant
/// waveforms, after checking every average is its waveform's level.
fn averaged(statements: usize, signals: usize) -> usize {
    let mut deck = String::from("* averages\n");
    for index in 0..statements {
        writeln!(deck, ".MEASURE TRAN avg{index} AVG V(n{index})").unwrap();
    }
    deck.push_str(".END\n");
    let netlist = rspice_core::Netlist::parse(&deck).unwrap();
    // Binary fractions, so each average is exactly its level.
    let time = [0.0, 0.5, 1.0];
    let levels: Vec<[f64; 3]> = (0..signals).map(|index| [index as f64; 3]).collect();
    let table: HashMap<String, &[f64]> = levels
        .iter()
        .enumerate()
        .map(|(index, level)| (format!("V(n{index})"), level.as_slice()))
        .collect();
    let (results, bytes) =
        allocations::measured(|| evaluate_tran_remeasurements(&netlist, &time, &table));
    assert_eq!(results.len(), statements);
    for (index, result) in results.iter().enumerate() {
        assert_eq!(result.value, Some(index as f64), "{}", result.name);
    }
    bytes
}

#[test]
fn eight_times_more_statements_over_one_table_allocate_less_than_twice_as_much() {
    let few = averaged(8, 2048);
    let many = averaged(64, 2048);
    println!("8 statements: {few} bytes; 64 statements: {many} bytes");
    // The table is copied and indexed once for the evaluation. A copy and
    // an index per statement grow with the statement count (approaching
    // 8x); what each statement adds of its own stays far below the table.
    assert!(many <= 2 * few, "{few} -> {many}");
}
