//! Hierarchy validation: connection-count checking, recursive-instantiation
//! detection, and `*` wildcards in `.save` selections.
//!
//! The first two are hard-error contracts (Spectre/HSPICE parity): a port
//! mismatch silently mis-wires a circuit if tolerated, and a recursive
//! definition should be reported as a named cycle, not as a depth blowout
//! a hundred expansions later.

use rspice_core::netlist::{ElementKind, Flattener, Netlist, SaveSet, SaveSignal};

fn flatten_err(deck: &str) -> String {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut flattener = Flattener::new(&netlist.subcircuits);
    flattener
        .flatten(&netlist)
        .expect_err("flattening should fail")
        .to_string()
}

fn flatten_names(deck: &str) -> Vec<String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut flattener = Flattener::new(&netlist.subcircuits);
    flattener
        .flatten(&netlist)
        .expect("flattening succeeds")
        .into_iter()
        .map(|element| element.name)
        .collect()
}

//=============================================================================
// Connection-count validation
//=============================================================================

#[test]
fn too_few_connections_is_an_error() {
    let err = flatten_err(
        "* pin mismatch: 1 node for 2 ports\n\
         v1 in 0 dc 1\n\
         x1 in cell\n\
         .subckt cell a b\n\
         r1 a b 1k\n\
         .ends\n\
         .end\n",
    );
    assert!(err.contains("connects 1 node(s)"), "got: {err}");
    assert!(err.contains("declares 2 port(s)"), "got: {err}");
    assert!(err.to_lowercase().contains("cell"), "got: {err}");
}

#[test]
fn too_many_connections_is_an_error() {
    let err = flatten_err(
        "* pin mismatch: 3 nodes for 2 ports\n\
         v1 in 0 dc 1\n\
         x1 in 0 extra cell\n\
         .subckt cell a b\n\
         r1 a b 1k\n\
         .ends\n\
         .end\n",
    );
    assert!(err.contains("connects 3 node(s)"), "got: {err}");
    assert!(err.contains("declares 2 port(s)"), "got: {err}");
}

#[test]
fn mismatch_error_names_the_instance_path() {
    // The offending instance is nested: the error should carry the full
    // hierarchical path, not just the leaf name.
    let err = flatten_err(
        "* nested mismatch\n\
         v1 in 0 dc 1\n\
         x1 in 0 outer\n\
         .subckt outer a b\n\
         x2 a inner\n\
         .ends\n\
         .subckt inner p q\n\
         r1 p q 1k\n\
         .ends\n\
         .end\n",
    );
    let lower = err.to_lowercase();
    assert!(lower.contains("x1.x2"), "got: {err}");
}

#[test]
fn matching_connections_flatten() {
    let names = flatten_names(
        "* ok\n\
         v1 in 0 dc 1\n\
         x1 in 0 cell\n\
         .subckt cell a b\n\
         r1 a b 1k\n\
         .ends\n\
         .end\n",
    );
    assert!(
        names.iter().any(|n| n.eq_ignore_ascii_case("x1.r1")),
        "have {names:?}"
    );
}

#[test]
fn subcircuit_local_coupling_references_flattened_inductors() {
    let netlist = Netlist::parse(
        "* local K references\n\
         x1 a 0 cell\n\
         .subckt cell p n\n\
         l1 p m 1u\n\
         l2 m n 2u\n\
         k1 l1 l2 0.5\n\
         .ends\n\
         .end\n",
    )
    .expect("deck parses");
    let mut flattener = Flattener::new(&netlist.subcircuits);
    let flattened = flattener.flatten(&netlist).expect("flattening succeeds");

    assert!(
        flattened
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case("x1.l1")),
        "flattened l1 not found in {flattened:?}"
    );
    assert!(
        flattened
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case("x1.l2")),
        "flattened l2 not found in {flattened:?}"
    );

    let coupling = flattened
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("x1.k1"))
        .expect("flattened coupling exists");
    let ElementKind::Coupling { inductors, .. } = &coupling.kind else {
        panic!("x1.k1 is not a coupling: {coupling:?}");
    };
    assert_eq!(inductors.len(), 2);
    assert!(inductors[0].eq_ignore_ascii_case("x1.l1"));
    assert!(inductors[1].eq_ignore_ascii_case("x1.l2"));
}

//=============================================================================
// Recursive-instantiation detection
//=============================================================================

#[test]
fn direct_recursion_reports_the_cycle() {
    let err = flatten_err(
        "* self-instantiating subcircuit\n\
         v1 a 0 dc 1\n\
         x1 a 0 loop\n\
         .subckt loop p q\n\
         r1 p q 1k\n\
         x2 p q loop\n\
         .ends\n\
         .end\n",
    );
    let lower = err.to_lowercase();
    assert!(lower.contains("recursive"), "got: {err}");
    assert!(lower.contains("loop -> loop"), "got: {err}");
    // Caught at the first re-entry, not after running into the depth limit.
    assert!(!lower.contains("depth"), "got: {err}");
}

#[test]
fn mutual_recursion_reports_the_chain() {
    let err = flatten_err(
        "* a -> b -> a\n\
         v1 a 0 dc 1\n\
         x1 a 0 ping\n\
         .subckt ping p q\n\
         x2 p q pong\n\
         .ends\n\
         .subckt pong p q\n\
         x3 p q ping\n\
         .ends\n\
         .end\n",
    );
    let lower = err.to_lowercase();
    assert!(lower.contains("ping -> pong -> ping"), "got: {err}");
}

#[test]
fn sibling_instances_are_not_a_cycle() {
    // Two instances of the same definition at the same level: the stack
    // must pop between expansions.
    let names = flatten_names(
        "* sibling reuse\n\
         v1 in 0 dc 1\n\
         x1 in mid cell\n\
         x2 mid 0 cell\n\
         .subckt cell a b\n\
         r1 a b 1k\n\
         .ends\n\
         .end\n",
    );
    let count = names
        .iter()
        .filter(|n| n.to_lowercase().ends_with(".r1"))
        .count();
    assert_eq!(count, 2, "have {names:?}");
}

#[test]
fn diamond_reuse_is_not_a_cycle() {
    // The same leaf definition reached through two different branches is
    // ordinary reuse, not recursion.
    let names = flatten_names(
        "* diamond\n\
         v1 in 0 dc 1\n\
         x1 in 0 top\n\
         .subckt top a b\n\
         xb a m left\n\
         xc m b right\n\
         .ends\n\
         .subckt left a b\n\
         xd a b leaf\n\
         .ends\n\
         .subckt right a b\n\
         xd a b leaf\n\
         .ends\n\
         .subckt leaf a b\n\
         r1 a b 1k\n\
         .ends\n\
         .end\n",
    );
    assert!(
        names.iter().any(|n| n.eq_ignore_ascii_case("x1.xb.xd.r1")),
        "have {names:?}"
    );
    assert!(
        names.iter().any(|n| n.eq_ignore_ascii_case("x1.xc.xd.r1")),
        "have {names:?}"
    );
}

//=============================================================================
// `.save` hierarchy wildcards
//=============================================================================

fn saves(signals: Vec<SaveSignal>) -> SaveSet {
    SaveSet { signals }
}

#[test]
fn voltage_wildcard_selects_one_level() {
    let set = saves(vec![SaveSignal::Voltage("x1.*".into())]);
    assert!(set.selects("v(x1.ntail)"));
    assert!(set.selects("V(X1.MIR)"));
    assert!(set.selects("x1.ntail")); // bare vector name form
    assert!(!set.selects("v(x1.xb.nref)")); // deeper level
    assert!(!set.selects("v(x2.ntail)"));
    assert!(!set.selects("v(out)"));
}

#[test]
fn wildcards_stack_per_level() {
    let set = saves(vec![SaveSignal::Voltage("x1.*.*".into())]);
    assert!(set.selects("v(x1.xb.nref)"));
    assert!(!set.selects("v(x1.ntail)"));
    assert!(!set.selects("v(x1.xb.xq.deep)"));
}

#[test]
fn top_level_wildcard_does_not_cross_hierarchy() {
    let set = saves(vec![SaveSignal::Voltage("*".into())]);
    assert!(set.selects("v(out)"));
    assert!(!set.selects("v(x1.ntail)"));
}

#[test]
fn partial_segment_wildcards_match() {
    let set = saves(vec![SaveSignal::Voltage("x1.n*".into())]);
    assert!(set.selects("v(x1.ntail)"));
    assert!(set.selects("v(x1.nbias)"));
    assert!(!set.selects("v(x1.mir)"));
}

#[test]
fn current_wildcards_cover_branch_forms() {
    let set = saves(vec![SaveSignal::Current("x1.*".into())]);
    assert!(set.selects("i(x1.vmeas)"));
    assert!(set.selects("x1.r1#branch"));
    assert!(!set.selects("i(x1.xb.vref)"));
    assert!(!set.selects("i(v1)"));
}

#[test]
fn exact_selections_unchanged() {
    let set = saves(vec![SaveSignal::Voltage("out".into())]);
    assert!(set.selects("v(out)"));
    assert!(!set.selects("v(outx)"));
    assert!(set.selects("time")); // scale vectors always pass
}

#[test]
fn wildcard_survives_netlist_parse() {
    let deck = "* wildcard save\n\
                v1 in 0 dc 1\n\
                x1 in 0 cell\n\
                .subckt cell a b\n\
                r1 a m 1k\n\
                r2 m b 1k\n\
                .ends\n\
                .save v(x1.*)\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    assert!(netlist.saves.selects("v(x1.m)"));
    assert!(!netlist.saves.selects("v(in)"));
}
