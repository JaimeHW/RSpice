#![cfg(feature = "veriloga")]
//! Verilog-AMS on top: a design that instantiates SPICE cells as leaves of its
//! own hierarchy, and a discipline boundary inside the design.
//!
//! Each case has a closed-form answer. A leaf is built as a card of the deck,
//! so the leaves' arithmetic is the deck's. A discrete port inside the design
//! that lands on a circuit node converts as a deck-level port does: a
//! converter where the node is analog, and no converter where only XSPICE
//! digital inputs read it.

use rspice_core::circuit::{ConverterKind, ConverterSource};
use rspice_core::engine::TransientResult;
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_verilog_on_top_{}_{}.vams",
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

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn operating_point(deck: &Netlist, node: &str) -> f64 {
    let result = Engine::default()
        .run_dc_op(deck)
        .unwrap_or_else(|error| panic!("the operating point solves: {error}"));
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names))
}

/// A `reg` held at 1, driving a net of the design that an electrical port of
/// the design also connects. The drive reaches the deck only through the
/// discipline boundary inside the design.
const HELD_ONE: &str = "`include \"disciplines.vams\"\n\
                        `timescale 1ns/1ps\n\
                        module drv(o);\n\
                        \x20 output o;\n\
                        \x20 reg o;\n\
                        \x20 initial o = 1'b1;\n\
                        endmodule\n\
                        module t(n);\n\
                        \x20 inout n;\n\
                        \x20 electrical n;\n\
                        \x20 drv u(n);\n\
                        endmodule\n";

/// The shipped `d2a` signature, selected by a rule set of the design's own.
const D2A_RULES: &str = "connectmodule d2a(d, a);\n\
                         \x20   input d;\n\
                         \x20   output a;\n\
                         \x20   logic d;\n\
                         \x20   electrical a;\n\
                         \x20   parameter real vsup = 3.3;\n\
                         \x20   parameter real vsupmin = 0;\n\
                         \x20   parameter real vlo = 0;\n\
                         \x20   parameter real vhi = vsup;\n\
                         \x20   parameter real vx = (vlo + vhi) / 2;\n\
                         \x20   parameter real vz = vx;\n\
                         \x20   parameter real rlo = 20;\n\
                         \x20   parameter real rhi = 20;\n\
                         \x20   parameter real rx = min(rlo, rhi);\n\
                         \x20   parameter real rz = inf;\n\
                         \x20   parameter real rsup = min(rlo, rhi) / 10;\n\
                         \x20   parameter real rpull = 10e3;\n\
                         \x20   parameter real rweak = 100e3;\n\
                         \x20   parameter real trise = 0;\n\
                         \x20   parameter real tfall = 0;\n\
                         \x20   parameter real tdelay = 0;\n\
                         endmodule\n\
                         connectrules r;\n\
                         \x20 connect d2a;\n\
                         endconnectrules\n";

/// A discrete output inside the design on a net an electrical port of the
/// design carries out to the deck is converted by the deck's D/A: a 1 at
/// `.param vcc` = 3.3 V behind the D/A's 20 ohm, into 1k, is
/// 3.3 * 1000 / 1020 = 3.235294 V. A rule set naming the shipped `d2a`
/// signature selects the same converter at circuit level, so it gives the
/// same answer rather than an internal error.
#[test]
fn a_discipline_boundary_inside_the_design_is_converted_at_the_deck() {
    let expected = 3.3 * 1000.0 / 1020.0;
    for (label, source, options) in [
        ("no rules", HELD_ONE.to_string(), ""),
        (
            "shipped d2a rule",
            format!("{HELD_ONE}{D2A_RULES}"),
            ".options connectrules=r\n",
        ),
    ] {
        let source = SourceFile::new(&source);
        let deck = parse(&format!(
            "* {label}\n\
             .hdl \"{}\" module=t\n\
             {options}\
             .param vcc=3.3\n\
             xt out t\n\
             rload out 0 1k\n\
             .op\n\
             .end\n",
            source.path()
        ));
        let out = operating_point(&deck, "out");
        println!("ATTRIBUTE {label}: v(out)={out:.9} expected={expected:.9}");
        assert!(
            (out - expected).abs() < 1.0e-6,
            "{label}: v(out) is 3.3 * 1000 / 1020 = {expected}, got {out}"
        );
    }
}

/// Two 1k leaves in series across a 1 V port, the other end on the design's
/// ground: the midpoint, a net of the design only the leaves touch, sits at
/// 1 * 1k / (1k + 1k) = 0.5 V. The same holds when one leaf is a deck
/// `.subckt` instead of the Annex E `resistor`.
#[test]
fn spice_leaves_inside_the_design_divide_a_port() {
    for (label, upper) in [
        ("two resistors", "resistor #(.r(1e3)) r1(p, m);"),
        ("a subckt and a resistor", "half r1(p, m);"),
    ] {
        let source = SourceFile::new(&format!(
            "`include \"disciplines.vams\"\n\
             module top(p);\n\
             \x20 inout p;\n\
             \x20 electrical p, m;\n\
             \x20 electrical gnd;\n\
             \x20 ground gnd;\n\
             \x20 {upper}\n\
             \x20 resistor #(.r(1e3)) r2(m, gnd);\n\
             endmodule\n"
        ));
        let deck = parse(&format!(
            "* {label}\n\
             .subckt half a b\n\
             r1 a b 1k\n\
             .ends\n\
             .hdl \"{}\" module=top\n\
             vin in 0 1\n\
             xtop in top\n\
             .op\n\
             .end\n",
            source.path()
        ));
        let mid = operating_point(&deck, "xtop.m");
        println!("ATTRIBUTE {label}: v(xtop.m)={mid:.12}");
        assert!(
            (mid - 0.5).abs() < 1.0e-9,
            "{label}: v(xtop.m) is 1 * 1k / 2k = 0.5, got {mid}"
        );
    }
}

/// A portless design whose `reg` drives a deck subcircuit whose port reaches
/// only an XSPICE `d_buffer` input. The net is digital on both sides, so the
/// build inserts no converter, and the buffer's output repeats every value
/// of the `reg` one buffer delay later.
#[test]
fn a_reg_on_a_digital_only_leaf_port_shares_its_event_net() {
    let source = SourceFile::new(
        "`timescale 1ns/1ps\n\
         module top;\n\
         \x20 reg q;\n\
         \x20 initial begin\n\
         \x20   q = 1'b0;\n\
         \x20   #10 q = 1'b1;\n\
         \x20   #10 q = 1'b0;\n\
         \x20   #10 q = 1'b1;\n\
         \x20 end\n\
         \x20 abuf u1(q);\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* d_buffer leaf\n\
         .subckt abuf a\n\
         a1 a y buf\n\
         .ends\n\
         .model buf d_buffer(rise_delay=1n fall_delay=1n)\n\
         .hdl \"{}\" module=top\n\
         xtop top\n\
         .tran 0.1n 40n\n\
         .end\n",
        source.path()
    ));
    let circuit = Engine::default()
        .build_circuit(&deck)
        .unwrap_or_else(|error| panic!("the deck builds: {error}"));
    assert!(
        circuit.converter_insertions().is_empty(),
        "a digital-only net needs no converter: {:?}",
        circuit.converter_insertions()
    );

    let result = Engine::default()
        .run_tran(&deck, 40.0e-9, 0.1e-9)
        .unwrap_or_else(|error| panic!("transient solves: {error}"));
    let reg = edges(&result, "xtop.q");
    let buffered = edges(&result, "xtop.u1.y");
    println!("ATTRIBUTE reg={reg:?} buffered={buffered:?}");
    let expected = [
        (10.0e-9, DigitalState::One),
        (20.0e-9, DigitalState::Zero),
        (30.0e-9, DigitalState::One),
    ];
    for (trace, delay) in [(&reg, 0.0), (&buffered, 1.0e-9)] {
        assert_eq!(trace.len(), expected.len(), "{trace:?}");
        for ((time, state), (at, value)) in trace.iter().zip(expected) {
            assert_eq!(*state, value, "{trace:?}");
            assert!(
                (time - (at + delay)).abs() < 1.0e-15,
                "an edge at {at:e} lands at {:e}, got {time:e}: {trace:?}",
                at + delay
            );
        }
    }
}

/// A leaf port that reaches an XSPICE digital input and a resistor both has
/// no one discipline: the build refuses it, naming the port and the node,
/// rather than choosing one.
#[test]
fn a_leaf_port_reaching_digital_and_analog_elements_is_refused() {
    let source = SourceFile::new(
        "`timescale 1ns/1ps\n\
         module top;\n\
         \x20 reg q;\n\
         \x20 initial q = 1'b1;\n\
         \x20 mixed u1(q);\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* mixed leaf port\n\
         .subckt mixed a\n\
         a1 a y buf\n\
         rl a 0 1k\n\
         .ends\n\
         .model buf d_buffer(rise_delay=1n fall_delay=1n)\n\
         .hdl \"{}\" module=top\n\
         xtop top\n\
         .tran 0.1n 10n\n\
         .end\n",
        source.path()
    ));
    let message = match Engine::default().build_circuit(&deck) {
        Ok(circuit) => panic!(
            "a port reaching both domains is refused, not converted: {:?}",
            circuit.converter_insertions()
        ),
        Err(error) => error.to_string(),
    };
    println!("ATTRIBUTE refusal={message}");
    for fragment in ["'xtop'", "'q'", "'xtop.q'", "continuous loading"] {
        assert!(
            message.to_ascii_lowercase().contains(fragment),
            "the refusal names {fragment}: {message}"
        );
    }
}

/// The corpus LDO testbench with Verilog-AMS on top: the trim register's four
/// `reg` outputs drive the `ldo` subcircuit's switch controls through
/// electrical nets of `top`. Each is a discipline boundary inside the design,
/// so the report lists one D/A per trim bit on `xtop.tr<k>`, supplied by the
/// deck's `.param vcc` = 1.8 V.
#[test]
fn the_corpus_trim_register_converts_at_the_deck_supply() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c12-ldo-tb/deck/top.op.cir");
    let deck = Netlist::parse_file(&path)
        .unwrap_or_else(|error| panic!("{} parses: {error}", path.display()));
    let circuit = Engine::default()
        .build_circuit(&deck)
        .unwrap_or_else(|error| panic!("the deck builds: {error}"));
    let mut trims: Vec<_> = circuit
        .converter_insertions()
        .iter()
        .filter(|record| record.boundary.starts_with("xtop.tr"))
        .collect();
    trims.sort_by(|a, b| a.boundary.cmp(&b.boundary));
    let boundaries: Vec<&str> = trims
        .iter()
        .map(|record| record.boundary.as_str())
        .collect();
    assert_eq!(
        boundaries,
        ["xtop.tr0", "xtop.tr1", "xtop.tr2", "xtop.tr3"],
        "{:?}",
        circuit.converter_insertions()
    );
    for record in trims {
        println!("ATTRIBUTE {record:?}");
        assert_eq!(record.kind, ConverterKind::D2a, "{record:?}");
        let supply = record.supply.as_ref().expect("a D/A has a supply");
        assert_eq!(supply.level, 1.8, "{record:?}");
        assert_eq!(
            supply.source,
            ConverterSource::Parameter("vcc".into()),
            "{record:?}"
        );
    }
}

/// The value changes of `node`'s event trace after time zero.
fn edges(result: &TransientResult, node: &str) -> Vec<(f64, DigitalState)> {
    let trace = result
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| {
            panic!(
                "no trace for {node}: {:?}",
                result
                    .digital_traces
                    .iter()
                    .map(|trace| &trace.node_name)
                    .collect::<Vec<_>>()
            )
        });
    let mut edges: Vec<(f64, DigitalState)> = Vec::new();
    let mut last = None;
    for point in &trace.points {
        let state = point.value.state;
        if last.is_some_and(|last| last != state) && point.time > 0.0 {
            edges.push((point.time, state));
        }
        last = Some(state);
    }
    edges
}
