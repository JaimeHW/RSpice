//! Bound allocation growth for long and many-instance startup cards, without
//! timing assumptions.

use std::fmt::Write;

use rspice_core::netlist::{StartupDirectiveRecord, StartupDirectiveScope};

use super::allocations;

fn source(count: usize, directive: &str, scoped: bool) -> String {
    let mut source = String::from("Long startup card\n");
    if scoped {
        source.push_str(".subckt cell p params: level=1\nRport p 0 1k\n");
    } else {
        source.push_str(".param level=1\n");
    }
    for index in 0..count {
        writeln!(source, "R{index:04} N{index:04} 0 1k").unwrap();
    }
    write!(source, "{directive}").unwrap();
    for index in 0..count {
        write!(source, " V(N{index:04})={{level}}").unwrap();
    }
    source.push('\n');
    if scoped {
        source.push_str(".ends\nX1 out cell\n");
    }
    source.push_str(".end\n");
    source
}

fn allocated(source: &str, count: usize) -> usize {
    let (parsed, bytes) = allocations::measured(|| rspice_core::Netlist::parse(source));
    let parsed = parsed.unwrap();
    assert_eq!(parsed.startup_directives()[0].entries().len(), count);
    for entry in parsed.startup_directives()[0].entries() {
        assert_eq!(entry.voltage(), 1.0);
    }
    bytes
}

#[test]
fn four_times_more_startup_entries_use_near_linear_allocation_volume() {
    let mut measurements = Vec::new();
    for directive in [".IC", ".NODESET"] {
        for scoped in [false, true] {
            let small = allocated(&source(64, directive, scoped), 64);
            let large = allocated(&source(256, directive, scoped), 256);
            println!("{directive} scoped={scoped}: bytes64={small} bytes256={large}");
            measurements.push((directive, scoped, small, large));
        }
    }
    for (directive, scoped, small, large) in measurements {
        // Four times the input may change vector/hash-table capacity rounding.
        // A 6x ceiling allows that overhead while rejecting per-entry copies
        // of the entire card (quadratic growth approaches 16x).
        assert!(
            large <= 6 * small,
            "{directive} scoped={scoped}: {small} -> {large}"
        );
    }
}

fn instanced_source(count: usize, directive: &str) -> String {
    let mut source = String::from("Instanced startup card\n.subckt cell p\nR1 p x 1k\nR2 x 0 1k\n");
    writeln!(source, "{directive} V(x)=1\n.ends").unwrap();
    for index in 0..count {
        writeln!(source, "X{index:04} N{index:04} cell").unwrap();
    }
    source.push_str(".end\n");
    source
}

fn instance_count(record: &StartupDirectiveRecord) -> usize {
    match record.scope() {
        StartupDirectiveScope::Subcircuit {
            qualified_instances,
            ..
        } => qualified_instances.len(),
        StartupDirectiveScope::TopLevel => panic!("the card is scoped to its subcircuit"),
    }
}

/// Bytes the elaboration of a subcircuit card over `count` instances
/// allocates once parsing has merged every instance's target into the card.
fn elaborated(count: usize, directive: &str) -> usize {
    let parsed = rspice_core::Netlist::parse(&instanced_source(count, directive)).unwrap();
    let card = &parsed.startup_directives()[0];
    assert_eq!(instance_count(card), count);
    assert_eq!(card.entries()[0].qualified_nodes().len(), count);
    let (flattened, bytes) =
        allocations::measured(|| rspice_core::netlist::flatten_netlist_with_models(&parsed));
    let elaborated = flattened.unwrap().scoped_startup_directives;
    assert_eq!(elaborated.len(), count);
    for record in &elaborated {
        assert_eq!(instance_count(record), 1);
        assert_eq!(record.entries()[0].qualified_nodes().len(), 1);
        assert_eq!(record.entries()[0].voltage(), 1.0);
    }
    bytes
}

#[test]
fn four_times_more_instances_of_a_startup_card_use_near_linear_allocation_volume() {
    for directive in [".IC", ".NODESET"] {
        let small = elaborated(64, directive);
        let large = elaborated(256, directive);
        println!("{directive}: bytes64={small} bytes256={large}");
        // Each instance's card names one instance and one node. Building it
        // by copying the merged card, which names all of them, makes the
        // elaboration quadratic (approaching 16x); the 6x ceiling is the one
        // above.
        assert!(large <= 6 * small, "{directive}: {small} -> {large}");
    }
}
