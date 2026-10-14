//! What an X-card may say about one Verilog-A instance, and what the instance
//! does with it.
//!
//! Two properties, both of them "this card is an instance card like any
//! other":
//!
//! * an instance parameter expression is evaluated in the context every other
//!   element's is — the deck's `.param` scope, `TEMPER`/`TNOM`/`VT`, and the
//!   draw a statistical `.param` made for this build — so the same expression
//!   on an X card and on a resistor resolves to the same number;
//! * an instance runs at the temperature its own card names. `TEMP=` is
//!   absolute Celsius, `DTEMP=`/`TRISE=` are one offset from the circuit
//!   temperature in kelvin, and a master that declares one of those names owns
//!   it — the engine applies no temperature of its own, exactly as it steps
//!   aside from `m` for a master declaring `m`.
//!
//! Both routes are pinned: adding a process to a module must not change what
//! its card may say. The refusals these rules imply live beside the rest of
//! the seam's typed diagnostics in `veriloga_elaboration_diagnostics`.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist, SimulationConfig};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` written to a unique path, deleted when the guard drops.
///
/// The uniqueness matters: the engine's Verilog-A cache is keyed by canonical
/// path, so two tests sharing a filename would share a cache entry.
struct ModelFile(PathBuf);

impl ModelFile {
    fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_instance_params_{name}_{}_{sequence}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create model file");
        file.write_all(source.as_bytes()).expect("write model");
        Self(path)
    }

    fn deck_path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn node_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("the operating point must solve: {error}\n{deck}"));
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing node {node} in {:?}", result.node_names))
}

fn refusal(deck: &str) -> String {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("the deck must be refused")
        .to_string()
}

/// A conductance the card sets, and nothing else.
const VRES: &str = r#"
`include "disciplines.vams"
module vres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

/// A resistance proportional to the device's own operating temperature:
/// `r` ohms at 300 K, so the operating point reads the temperature back.
const VTRES: &str = r#"
`include "disciplines.vams"
module vtres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) * 300.0 / (r * $temperature);
endmodule
"#;

/// The same law, with `dtemp` declared by the master. The engine must leave
/// the name to the module: the resistance moves with the parameter and not
/// with the device's temperature.
const OWN_DTEMP: &str = r#"
`include "disciplines.vams"
module owndt(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    parameter real dtemp = 0.0;
    analog I(p, n) <+ V(p, n) * (1.0 + dtemp) * 300.0 / (r * $temperature);
endmodule
"#;

/// [`VTRES`] with a process, so the same card takes the mixed route.
const MIXED_TRES: &str = r#"
`include "disciplines.vams"
module mtres(p, n, q);
    inout p, n;
    electrical p, n;
    output q;
    reg q;
    parameter real r = 1000.0 from (0:inf);
    initial q = 1'b0;
    always #5 q = ~q;
    analog I(p, n) <+ V(p, n) * 300.0 / (r * $temperature);
endmodule
"#;

/// The divider voltage a `1k` upper leg and a resistance `lower` produce.
fn divided(lower: f64) -> f64 {
    lower / (1000.0 + lower)
}

fn assert_close(what: &str, actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-7 * expected.abs().max(1.0),
        "{what}: got {actual}, expected {expected}"
    );
}

/// The same expression on an X card and on a resistor must resolve to the
/// same number, because both go through the same evaluation context.
///
/// The card used to be evaluated against the bare `.param` scope, so an
/// instance naming `TEMPER` or `VT` — the two scalars any other element's
/// expressions may name — failed to elaborate at all.
#[test]
fn an_instance_expression_resolves_where_a_resistors_does() {
    let model = ModelFile::new("expression_scope", VRES);
    // A divider whose two legs are the same expression: it reads 0.5 only if
    // the X card and the resistor agreed on the value.
    let divider = |expression: &str, options: &str| {
        format!(
            "* one expression, two element families\n\
             {options}\n\
             v1 in 0 dc 1.0\n\
             x1 in out vres r={expression}\n\
             r2 out 0 {expression}\n\
             .va \"{}\" vres\n\
             .op\n\
             .end\n",
            model.deck_path()
        )
    };

    for (expression, options) in [
        ("{2000*TEMPER/54}", ".options temp=54"),
        ("{2000*TEMP/54}", ".options temp=54"),
        ("{vt*1e5}", ".options temp=27"),
        ("{1000*27/TNOM}", ".options tnom=27"),
    ] {
        assert_close(
            expression,
            node_voltage(&divider(expression, options), "out"),
            0.5,
        );
    }
}

/// The statistical draw a `.param` made for this build is one draw, and the
/// instance card sees the same one its neighbours do.
#[test]
fn an_instance_expression_sees_the_draw_the_rest_of_the_deck_sees() {
    let model = ModelFile::new("statistical_scope", VRES);
    let deck = format!(
        "* one agauss draw, read by an X card and a resistor\n\
         .options seed=7\n\
         .param rv={{agauss(1000,100,1)}}\n\
         v1 in 0 dc 1.0\n\
         x1 in out vres r={{rv}}\n\
         r2 out 0 {{rv}}\n\
         .va \"{}\" vres\n\
         .op\n\
         .end\n",
        model.deck_path()
    );
    assert_close("agauss draw", node_voltage(&deck, "out"), 0.5);
}

/// `TEMP`, `DTEMP` and `TRISE` on the card reach the device, and mean what
/// they mean everywhere else.
#[test]
fn an_instance_runs_at_the_temperature_its_card_names() {
    let model = ModelFile::new("instance_temperature", VTRES);
    let deck = |tail: &str, circuit_celsius: f64| {
        format!(
            "* a temperature-dependent master under an instance temperature\n\
             .options temp={circuit_celsius}\n\
             v1 in 0 dc 1.0\n\
             r1 in out 1k\n\
             x1 out 0 vtres r=1k{tail}\n\
             .va \"{}\" vtres\n\
             .op\n\
             .end\n",
            model.deck_path()
        )
    };

    let nominal = node_voltage(&deck("", 27.0), "out");
    let heated = node_voltage(&deck("", 37.0), "out");
    assert_close("27 C", nominal, divided(1000.0 * 300.15 / 300.0));
    assert!(
        (heated - nominal).abs() > 1e-6,
        "the fixture must be able to tell 27 C from 37 C: {heated} vs {nominal}"
    );

    for tail in [
        " dtemp=10",
        " trise=10",
        " temp=37",
        // One offset spelled twice with one value is one offset.
        " dtemp=10 trise=10",
        // An absolute temperature wins over an offset, as it does for a
        // passive instance.
        " temp=37 dtemp=100",
        // The card's own expressions resolve here too.
        " dtemp={5+5}",
    ] {
        assert_close(tail, node_voltage(&deck(tail, 27.0), "out"), heated);
    }
}

/// A master that declares the name keeps it, and the engine heats nothing.
#[test]
fn a_master_declaring_a_temperature_offset_owns_the_name() {
    let model = ModelFile::new("own_dtemp", OWN_DTEMP);
    let deck = format!(
        "* dtemp is this master's own parameter\n\
         .options temp=27\n\
         v1 in 0 dc 1.0\n\
         r1 in out 1k\n\
         x1 out 0 owndt r=1k dtemp=1\n\
         .va \"{}\" owndt\n\
         .op\n\
         .end\n",
        model.deck_path()
    );
    // The parameter doubles the conductance; the device stays at 27 C. Had
    // the engine also read the name as an offset, the leg would have been
    // 1k*301.15/(300*2) instead of 1k*300.15/(300*2).
    assert_close(
        "model-owned dtemp",
        node_voltage(&deck, "out"),
        divided(1000.0 * 300.15 / (300.0 * 2.0)),
    );
}

/// The mixed route reads the same card the analog route does.
#[test]
fn a_mixed_instance_runs_at_the_temperature_its_card_names() {
    let model = ModelFile::new("mixed_temperature", MIXED_TRES);
    let deck = |tail: &str, circuit_celsius: f64| {
        format!(
            "* a mixed master under an instance temperature\n\
             .options temp={circuit_celsius}\n\
             v1 in 0 dc 1.0\n\
             r1 in out 1k\n\
             x1 out 0 q mtres{tail}\n\
             rq q 0 1k\n\
             .va \"{}\" mtres\n\
             .op\n\
             .end\n",
            model.deck_path()
        )
    };

    let nominal = node_voltage(&deck(" r=1k", 27.0), "out");
    let heated = node_voltage(&deck(" r=1k", 37.0), "out");
    assert!(
        (heated - nominal).abs() > 1e-6,
        "the mixed fixture must be able to tell 27 C from 37 C: {heated} vs {nominal}"
    );
    for tail in [" r=1k dtemp=10", " r=1k trise=10", " r=1k temp=37"] {
        assert_close(tail, node_voltage(&deck(tail, 27.0), "out"), heated);
    }
    // And the expression context is the analog route's: a mixed card naming
    // TEMPER resolves rather than failing to elaborate.
    assert_close(
        "mixed TEMPER",
        node_voltage(&deck(" r={1000*TEMPER/27}", 27.0), "out"),
        nominal,
    );
}

/// `m` keeps its meaning: the carve-out this file adds to is the same one.
#[test]
fn the_multiplicity_carve_out_is_unchanged() {
    let model = ModelFile::new("multiplicity", VRES);
    let deck = format!(
        "* m=4 on a master that declares no m\n\
         v1 in 0 dc 1.0\n\
         x1 in out vres r=1k\n\
         x2 out 0 vres r=4k m=4 dtemp=10\n\
         .va \"{}\" vres\n\
         .op\n\
         .end\n",
        model.deck_path()
    );
    assert_close("m=4 divider", node_voltage(&deck, "out"), 0.5);
}

/// A temperature offset given twice with two values is two answers, and the
/// refusal says which two spellings disagreed.
#[test]
fn one_offset_spelled_twice_with_two_values_is_refused() {
    let model = ModelFile::new("offset_conflict", VTRES);
    let message = refusal(&format!(
        "* dtemp and trise are one parameter\n\
         v1 in 0 dc 1.0\n\
         r1 in out 1k\n\
         x1 out 0 vtres r=1k dtemp=10 trise=20\n\
         .va \"{}\" vtres\n\
         .op\n\
         .end\n",
        model.deck_path()
    ));
    assert!(
        message.contains("DTEMP") && message.contains("TRISE"),
        "the refusal must name both spellings: {message}"
    );
}

/// A string parameter selecting the law: `half` halves the resistance.
const VMODE: &str = r#"
`include "disciplines.vams"
module vmode(p, n);
    inout p, n;
    electrical p, n;
    parameter string mode = "full";
    parameter real r = 1000.0 from (0:inf);
    analog begin
        if (mode == "half")
            I(p, n) <+ V(p, n) / (r / 2.0);
        else
            I(p, n) <+ V(p, n) / r;
    end
endmodule
module vmode_top(p, n);
    inout p, n;
    electrical p, n;
    parameter string inner = "full";
    vmode #(.mode(inner)) u1(p, n);
endmodule
module vmode_fixed(p, n);
    inout p, n;
    electrical p, n;
    vmode #(.mode("half")) u1(p, n);
endmodule
"#;

/// A string override on the card reaches a string parameter, and a source
/// instance's string override reaches its child, each specializing the
/// source (the hierarchy keys a closed override by its value, `str:`).
#[test]
fn a_string_parameter_takes_its_override_from_the_card_and_from_the_source() {
    let model = ModelFile::new("string_mode", VMODE);
    let out = |card: &str| {
        node_voltage(
            &format!(
                "* a string parameter\nv1 in 0 dc 1.0\nr1 in out 1k\n{card}\n\
                 .va \"{}\" module=vmode\n.va \"{}\" module=vmode_top\n\
                 .va \"{}\" module=vmode_fixed\n.op\n.end\n",
                model.deck_path(),
                model.deck_path(),
                model.deck_path()
            ),
            "out",
        )
    };
    let full = 0.5;
    let half = 500.0 / 1500.0;
    assert_close("default", out("x1 out 0 vmode"), full);
    assert_close("card half", out("x1 out 0 vmode mode=\"half\""), half);
    assert_close("card full", out("x1 out 0 vmode mode=\"full\""), full);
    assert_close("source half", out("x1 out 0 vmode_fixed"), half);
    assert_close(
        "through the hierarchy",
        out("x1 out 0 vmode_top inner=\"half\""),
        half,
    );
    assert_close("hierarchy default", out("x1 out 0 vmode_top"), full);
    // Two cards, two strings, two specializations of one master.
    let netlist = Netlist::parse(&format!(
        "* two strings\nv1 in 0 dc 1.0\nr1 in a 1k\nr2 in b 1k\n\
         x1 a 0 vmode mode=\"half\"\nx2 b 0 vmode mode=\"full\"\n\
         .va \"{}\" module=vmode\n.op\n.end\n",
        model.deck_path()
    ))
    .unwrap();
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .unwrap();
    assert_close("x1", result.try_voltage_named("a").unwrap(), half);
    assert_close("x2", result.try_voltage_named("b").unwrap(), full);
    // A string is not a number, and a number is not a string.
    let message = refusal(&format!(
        "* a number for a string\nv1 in 0 dc 1.0\nr1 in out 1k\nx1 out 0 vmode mode=2\n\
         .va \"{}\" module=vmode\n.op\n.end\n",
        model.deck_path()
    ));
    assert!(
        message.contains("mode") && message.contains("string"),
        "{message}"
    );
    let message = refusal(&format!(
        "* a string for a number\nv1 in 0 dc 1.0\nr1 in out 1k\nx1 out 0 vmode r=\"big\"\n\
         .va \"{}\" module=vmode\n.op\n.end\n",
        model.deck_path()
    ));
    assert!(
        message.contains("'r'") && message.contains("string"),
        "{message}"
    );
}

/// One level of a recursive ladder conducts `count` mA/V into its own `mid`;
/// the recursion ends in one plain unit where the generate stops.
const LADDER: &str = r#"
`include "disciplines.vams"
module unit(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
module ladder(p, n);
    inout p, n;
    electrical p, n, mid;
    parameter integer count = 3 from [1:inf);
    analog I(p, mid) <+ V(p, mid) * count / 1000.0;
    generate if (count > 1) begin : more
        ladder #(.count(count - 1)) rest (mid, n);
    end else begin : last
        unit tail (mid, n);
    end endgenerate
endmodule
"#;

/// [`LADDER`] reading its level through an initial block, so the same card
/// takes the mixed route.
const MIXED_LADDER: &str = r#"
`include "disciplines.vams"
module unit(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
module ladder(p, n);
    inout p, n;
    electrical p, n, mid;
    parameter integer count = 3 from [1:inf);
    integer level;
    initial level = count;
    analog I(p, mid) <+ V(p, mid) * level / 1000.0;
    generate if (count > 1) begin : more
        ladder #(.count(count - 1)) rest (mid, n);
    end else begin : last
        unit tail (mid, n);
    end endgenerate
endmodule
"#;

/// A recursive hierarchy elaborates as deep as its card says, and each level
/// keeps its own closed `count`: the lower leg is 1k·(1/count + … + 1/1) for
/// the levels and 1k for the tail. A level that read another's value, or a
/// depth the card did not name, reads a different divider.
#[test]
fn a_recursive_hierarchy_elaborates_the_depth_its_card_names() {
    for (name, source) in [("ladder", LADDER), ("mixed_ladder", MIXED_LADDER)] {
        let model = ModelFile::new(name, source);
        for (card, levels) in [("", 3_u32), ("count=4", 4), ("count=1", 1)] {
            let deck = format!(
                "* a recursive ladder\nv1 in 0 dc 1.0\nr1 in out 1k\nx1 out 0 ladder {card}\n\
                 .va \"{}\" module=ladder\n.op\n.end\n",
                model.deck_path()
            );
            let lower = 1000.0 * (1..=levels).map(|k| 1.0 / f64::from(k)).sum::<f64>() + 1000.0;
            assert_close(
                &format!("{name} {card}"),
                node_voltage(&deck, "out"),
                divided(lower),
            );
        }
    }
}
