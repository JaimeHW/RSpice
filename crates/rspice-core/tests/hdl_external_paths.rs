#![cfg(feature = "veriloga")]
//! HDL access to values inside the SPICE hierarchy.
//!
//! `$cds_get_analog_value` reads a node or a branch current anywhere in the
//! deck from discrete code, by its absolute path. The fixture is a divider
//! `.subckt div a b` of 1k over 3k at `xdiv in 0 div`, driven by 1 V, so its
//! midpoint `xdiv.m` sits at 0.75 V and carries 0.25 mA.
//!
//! `--features veriloga` runs the analog half on the bytecode interpreter and
//! `--features veriloga-native` on native code; the expectations are the same.

use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_hdl_external_paths_{}_{}.vams",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).expect("write source file");
        Self(path)
    }

    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for SourceFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The 1k-over-3k divider, with `vp m m2 0` in its lower leg when `probe`.
fn divider(probe: bool) -> &'static str {
    if probe {
        ".subckt div a b\nr1 a m 1k\nvp m m2 0\nr2 m2 b 3k\n.ends\n"
    } else {
        ".subckt div a b\nr1 a m 1k\nr2 m b 3k\n.ends\n"
    }
}

/// A testbench that reads `call` at 10 ns and holds it on its `wreal` port.
fn reader(call: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module tb(o);\n\
         \x20 output o; wreal o;\n\
         \x20 real r;\n\
         \x20 assign o = r;\n\
         \x20 initial begin r = 0.0; #10 r = {call}; end\n\
         endmodule\n"
    )
}

fn deck(source: &SourceFile, probe: bool) -> Netlist {
    let text = format!(
        "external path read\n\
         .hdl \"{}\"\n\
         vin in 0 1\n\
         {}\
         xdiv in 0 div\n\
         xtb o tb\n\
         .tran 1n 20n\n\
         .end\n",
        source.path(),
        divider(probe)
    );
    Netlist::parse(&text).unwrap_or_else(|error| panic!("deck parses: {error}\n{text}"))
}

/// The value the testbench holds on `o` at the end of the run.
fn read_at_10ns(call: &str, probe: bool) -> f64 {
    let source = SourceFile::new(&reader(call));
    let deck = deck(&source, probe);
    let result = Engine::default()
        .run_tran(&deck, 20.0e-9, 1.0e-9)
        .unwrap_or_else(|error| panic!("transient solves: {error}"));
    let trace = result
        .real_trace_named("o")
        .unwrap_or_else(|| panic!("no real trace `o`"));
    let last = trace.last().expect("the testbench assigns o");
    assert!(
        (last.time - 10.0e-9).abs() < 1.0e-15,
        "the read lands at 10 ns: {trace:?}"
    );
    println!("{call} at 10 ns: {:.12e}", last.value);
    last.value
}

fn build_error(call: &str, probe: bool) -> String {
    let source = SourceFile::new(&reader(call));
    let deck = deck(&source, probe);
    match Engine::default().build_circuit(&deck) {
        Ok(_) => panic!("`{call}` builds"),
        Err(error) => error.to_string(),
    }
}

/// Test 1: the midpoint of the divider, 1 V * 3k / 4k.
#[test]
fn a_node_inside_a_subcircuit_instance_reads_its_potential() {
    let value = read_at_10ns("$cds_get_analog_value(\"xdiv.m\")", false);
    assert!((value - 0.75).abs() < 1.0e-9, "V(xdiv.m) = {value}");
    // `$root.` names the same deck root, and `"pot"` is the default.
    let value = read_at_10ns("$cds_get_analog_value(\"$root.xdiv.m\", 0, \"pot\")", false);
    assert!((value - 0.75).abs() < 1.0e-9, "V($root.xdiv.m) = {value}");
}

/// A port of an instance is the net the instance connects it to.
#[test]
fn a_port_of_an_instance_reads_the_net_it_connects() {
    let value = read_at_10ns("$cds_get_analog_value(\"xdiv.a\")", false);
    assert!((value - 1.0).abs() < 1.0e-9, "V(xdiv.a) = {value}");
}

/// Test 2: the current through `vp`, positive into its first node, 1 V / 4k.
#[test]
fn a_voltage_source_inside_an_instance_reads_its_branch_current() {
    let value = read_at_10ns("$cds_get_analog_value(\"xdiv.vp\", 0, \"flow\")", true);
    assert!((value - 0.25e-3).abs() < 1.0e-12, "I(xdiv.vp) = {value}");
}

/// Test 5: a path that names nothing is refused, naming the nearest scope that
/// exists and what it holds.
#[test]
fn an_unresolved_path_names_the_nearest_scope_and_its_nodes() {
    let message = build_error("$cds_get_analog_value(\"xdiv.q\")", false);
    assert!(
        message.contains("`xdiv` exists; it has no node `q`; its nodes are a, b, m"),
        "{message}"
    );
    assert!(
        message.contains("$cds_get_analog_value(\"xdiv.q\")"),
        "{message}"
    );
    let message = build_error("$cds_get_analog_value(\"xnone.m\")", false);
    assert!(
        message.contains("the deck's top level has no node `xnone.m`"),
        "{message}"
    );
}

/// Test 6: `flow` of an element that owns no branch, and of one that is not there.
#[test]
fn a_flow_read_needs_an_element_that_owns_a_branch() {
    let message = build_error("$cds_get_analog_value(\"xdiv.r1\", 0, \"flow\")", false);
    assert!(
        message.contains("`xdiv.r1` owns no branch current"),
        "{message}"
    );
    let message = build_error("$cds_get_analog_value(\"xdiv.vq\", 0, \"flow\")", true);
    assert!(
        message.contains("`xdiv` exists; it has no element `vq`; its elements are r1, vp, r2"),
        "{message}"
    );
}

/// Test 6: the forms outside the unambiguous subset are refused at compile
/// time, each naming the call.
#[test]
fn calls_outside_the_supported_subset_are_refused_naming_the_call() {
    for (call, expected) in [
        (
            "$cds_get_analog_value(path)",
            "the path must be a string literal",
        ),
        (
            "$cds_get_analog_value(\"xdiv.m\", 1)",
            "the index must be 0",
        ),
        (
            "$cds_get_analog_value(\"xdiv.m\", 0, \"charge\")",
            "unknown quantity \"charge\"",
        ),
    ] {
        let source = SourceFile::new(&reader(call).replace(
            "  real r;\n",
            "  real r;\n  parameter string path = \"xdiv.m\";\n",
        ));
        let deck = deck(&source, false);
        let message = match Engine::default().build_circuit(&deck) {
            Ok(_) => panic!("`{call}` builds"),
            Err(error) => error.to_string(),
        };
        assert!(
            message.contains("$cds_get_analog_value"),
            "{call}: {message}"
        );
        assert!(message.contains(expected), "{call}: {message}");
    }
}

/// Test 6: analog code reads a deck node through an alias, not this call.
#[test]
fn the_call_in_analog_code_points_to_the_node_alias() {
    let source = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         module tb(o);\n\
         \x20 inout o; electrical o;\n\
         \x20 analog V(o) <+ $cds_get_analog_value(\"xdiv.m\");\n\
         endmodule\n",
    );
    let text = format!(
        "analog read\n.hdl \"{}\"\nvin in 0 1\n{}xdiv in 0 div\nxtb o tb\nr o 0 1k\n.op\n.end\n",
        source.path(),
        divider(false)
    );
    let deck = Netlist::parse(&text).expect("deck parses");
    let message = match Engine::default().build_circuit(&deck) {
        Ok(_) => panic!("the analog read builds"),
        Err(error) => error.to_string(),
    };
    assert!(message.contains("$cds_get_analog_value"), "{message}");
    assert!(message.contains("$analog_node_alias"), "{message}");
}

/// A module `alias` with port `p`, whose local net `m_alias` is bound to the
/// deck path `path` in `analog initial`, and whose analog block is `body`.
fn alias_module(path: &str, body: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         module alias(p);\n\
         \x20 inout p; electrical p;\n\
         \x20 electrical m_alias;\n\
         \x20 integer ok;\n\
         \x20 analog initial ok = $analog_node_alias(m_alias, \"{path}\");\n\
         \x20 analog begin\n\
         \x20   {body}\n\
         \x20 end\n\
         endmodule\n"
    )
}

/// The divider deck with the alias module at `xa p alias`, `p` loaded by 1k.
fn alias_deck(source: &SourceFile, analysis: &str) -> Netlist {
    let text = format!(
        "node alias\n\
         .hdl \"{}\"\n\
         vin in 0 dc 1 ac 1\n\
         {}\
         xdiv in 0 div\n\
         xa p alias\n\
         rp p 0 1k\n\
         {analysis}\n\
         .end\n",
        source.path(),
        divider(false)
    );
    Netlist::parse(&text).unwrap_or_else(|error| panic!("deck parses: {error}\n{text}"))
}

fn operating_point(deck: &Netlist, node: &str) -> f64 {
    let result = Engine::default()
        .run_dc_op(deck)
        .unwrap_or_else(|error| panic!("the operating point solves: {error}"));
    let value = result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    println!("V({node}) at the operating point: {value:.12e}");
    value
}

/// Test 3: the alias reads the deck node, 0.75 V, and the call returns 1.
#[test]
fn an_aliased_net_reads_the_deck_node() {
    let source = SourceFile::new(&alias_module(
        "xdiv.m",
        "V(p) <+ V(m_alias); I(m_alias) <+ 0;",
    ));
    let value = operating_point(&alias_deck(&source, ".op"), "p");
    assert!((value - 0.75).abs() < 1.0e-9, "V(p) = {value}");
    let source = SourceFile::new(&alias_module("$root.xdiv.m", "V(p) <+ ok;"));
    let value = operating_point(&alias_deck(&source, ".op"), "p");
    assert!((value - 1.0).abs() < 1.0e-12, "the call returns {value}");
}

/// Test 4: a contribution to the aliased net loads the deck node through an
/// ordinary terminal: 1k across the divider's 0.75 V behind 750 ohm gives
/// 0.75 * 1000 / 1750 at the operating point, and the same transfer in AC,
/// where the Jacobian alone carries it.
#[test]
fn an_aliased_net_loads_the_deck_node_exactly() {
    let expected = 0.75 * 1000.0 / 1750.0;
    let body = "I(m_alias) <+ V(m_alias) / 1k; I(p) <+ 0;";
    let source = SourceFile::new(&alias_module("xdiv.m", body));
    let value = operating_point(&alias_deck(&source, ".op"), "xdiv.m");
    assert!((value - expected).abs() < 1.0e-9, "V(xdiv.m) = {value}");

    let deck = alias_deck(&source, ".ac lin 1 1k 1k");
    let points = Engine::default()
        .run_ac(&deck, &[1.0e3])
        .unwrap_or_else(|error| panic!("AC solves: {error}"));
    let point = &points[0];
    let at = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("xdiv.m"))
        .expect("xdiv.m is a node");
    let transfer = point.voltages[at];
    println!("V(xdiv.m)/V(in) at 1 kHz: {transfer:.12e}");
    assert!(
        (transfer.re - expected).abs() < 1.0e-9 && transfer.im.abs() < 1.0e-9,
        "V(xdiv.m)/V(in) = {transfer}"
    );

    // 1 mA into the node: 0.75 V + 750 ohm * 1 mA.
    let source = SourceFile::new(&alias_module("xdiv.m", "I(m_alias) <+ -1m; I(p) <+ 0;"));
    let value = operating_point(&alias_deck(&source, ".op"), "xdiv.m");
    assert!((value - 1.5).abs() < 1.0e-9, "V(xdiv.m) = {value}");
}

/// Test 5: an alias whose path names nothing returns 0, keeps a net of its
/// own, and warns once, naming the path.
#[test]
fn an_unresolved_alias_returns_zero_and_warns_once() {
    let source = SourceFile::new(&alias_module(
        "xdiv.q",
        "V(p) <+ ok; I(m_alias) <+ V(m_alias) / 1k;",
    ));
    let deck = alias_deck(&source, ".op");
    let engine = Engine::default();
    for _ in 0..2 {
        let result = engine
            .run_dc_op(&deck)
            .unwrap_or_else(|error| panic!("the operating point solves: {error}"));
        let value = result.try_voltage_named("p").expect("p is a node");
        assert!(value.abs() < 1.0e-12, "the call returns {value}");
    }
    let warnings: Vec<_> = engine
        .compiler_diagnostics()
        .into_iter()
        .filter(|diagnostic| diagnostic.message.contains("xdiv.q"))
        .collect();
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(
        warnings[0]
            .message
            .contains("$analog_node_alias(m_alias, \"xdiv.q\")"),
        "{}",
        warnings[0].message
    );
}

/// Test 6: an alias outside `analog initial` is refused, naming the call.
#[test]
fn an_alias_outside_analog_initial_is_refused() {
    let source = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         module alias(p);\n\
         \x20 inout p; electrical p;\n\
         \x20 electrical m_alias;\n\
         \x20 analog begin\n\
         \x20   $analog_node_alias(m_alias, \"xdiv.m\");\n\
         \x20   V(p) <+ V(m_alias);\n\
         \x20 end\n\
         endmodule\n",
    );
    let message = match Engine::default().build_circuit(&alias_deck(&source, ".op")) {
        Ok(_) => panic!("the alias in the analog block builds"),
        Err(error) => error.to_string(),
    };
    assert!(message.contains("$analog_node_alias"), "{message}");
    assert!(message.contains("only in `analog initial`"), "{message}");
}
