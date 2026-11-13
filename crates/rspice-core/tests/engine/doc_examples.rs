//! Executable copies of the examples in the crate documentation.
//!
//! `Cargo.toml` sets `doctest = false`, and that is the right call: the
//! generated Verilog-A catalog makes a doctest pass expensive, and doctests
//! link the whole crate per example. The cost is that every `rust` block in
//! the docs is checked by review alone, and review does not catch an example
//! that stopped compiling three refactors ago.
//!
//! This file is the compromise. Examples that make a promise about the API —
//! the crate-root example is the first code a new reader runs — are mirrored
//! here as real tests. When one of these changes, change the doc comment it
//! came from, and vice versa; each test names its source.
//!
//! This is not meant to mirror every example in the crate. It covers the
//! front door, where being wrong is most expensive.

use rspice_core::{Engine, Netlist};

/// Mirrors the example in `src/lib.rs` (crate-level docs).
///
/// The deliberate detail is the first line: `divider` is the SPICE title
/// line, not an element. A deck whose first line is `V1 1 0 10` silently
/// loses that source, which is exactly the mistake the doc example exists to
/// pre-empt — so the test keeps the title and asserts the source survived.
#[test]
fn crate_root_example_runs() {
    let netlist = Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end")
        .expect("crate-root example deck parses");
    let result = Engine::default()
        .run_dc_op(&netlist)
        .expect("crate-root example solves");
    assert_eq!(result.voltage(1), 10.0);
}

/// The title-line rule the example above depends on, asserted directly.
///
/// If the parser ever started treating the first line as an element, the
/// example would still solve — `V1` would just be gone, and node 1 would
/// read 0 V instead of 10 V. Pinning both halves means the failure names
/// itself.
#[test]
fn first_deck_line_is_the_title_not_an_element() {
    let with_title =
        Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end").expect("titled deck parses");
    let titled_voltage = Engine::default()
        .run_dc_op(&with_title)
        .expect("titled deck solves")
        .voltage(1);

    // Same circuit, but the title line is missing, so `V1 1 0 10` is
    // consumed as the title and the source disappears.
    let without_title = Netlist::parse("V1 1 0 10\nR1 1 0 1k\n.end").expect("untitled deck parses");
    let untitled_voltage = Engine::default()
        .run_dc_op(&without_title)
        .expect("untitled deck solves")
        .voltage(1);

    assert_eq!(titled_voltage, 10.0);
    assert_ne!(
        untitled_voltage, titled_voltage,
        "dropping the title line must change the circuit: the first line is \
         the title, so `V1 1 0 10` is consumed as one and the source is lost"
    );
}
