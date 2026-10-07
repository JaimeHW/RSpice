//! Bound allocation growth for long startup cards, without timing assumptions.
#![cfg(not(target_arch = "wasm32"))]

use std::fmt::Write;

#[path = "common/allocations.rs"]
mod allocations;

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
