#![cfg(feature = "veriloga")]
//! `.MEASURE TRAN` on a deck whose HDL drives event nets.
//!
//! `D(n)` and `E(n)` read a net's event trace as a step function on the
//! instants the run accepted its events at. Every expected value is a closed
//! form of the fixture, never an RSpice output: `q` toggles every 10 ns from
//! 0, so it rises at 10, 30, 50, 70 and 90 ns and its period is 20 ns; `p`
//! toggles every 7 ns from 0. Event times agree to 4 ulp.

use rspice_core::analysis::MeasurementUnit;
use rspice_core::analysis::measure::MeasureResult;
use rspice_core::analysis::measure_signals::{
    evaluate_dc_measurements, evaluate_tran_measurements,
};
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{AnalysisCommand, Netlist};
use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A scratch directory removed on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new(prefix: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "rspice_measure_events_{prefix}_{}_{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create scratch directory");
        Self(dir)
    }

    /// Write `contents` as `name` and return the path a deck can include.
    fn write(&self, name: &str, contents: &str) -> String {
        let path = self.0.join(name);
        fs::write(&path, contents).expect("write scratch file");
        path.display().to_string().replace('\\', "/")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// `q` toggles every 10 ns and `p` every 7 ns, both from 0.
const CLOCKS: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module clocks(q, p);\n\
  output q; reg q;\n\
  output p; reg p;\n\
  initial begin q = 1'b0; p = 1'b0; end\n\
  always #10 q = ~q;\n\
  always #7 p = ~p;\n\
endmodule\n";

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn run(netlist: &Netlist) -> TransientResult {
    try_run(netlist).unwrap_or_else(|error| panic!("transient runs: {error}"))
}

fn try_run(netlist: &Netlist) -> Result<TransientResult, rspice_core::SimulationError> {
    let (stop, max_step) = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                ..
            } => Some((
                *stop,
                rspice_core::execution::resolve_transient_maximum_step(
                    *step, *stop, *start, *max_step,
                )
                .expect("valid .tran"),
            )),
            _ => None,
        })
        .expect("deck has a .tran card");
    Engine::new(SimulationConfig::default()).run_tran(netlist, stop, max_step)
}

fn measure<'a>(results: &'a [MeasureResult], name: &str) -> &'a MeasureResult {
    results
        .iter()
        .find(|result| result.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} is measured: {results:?}"))
}

fn value(results: &[MeasureResult], name: &str) -> f64 {
    let result = measure(results, name);
    assert!(result.error.is_none(), "{name}: {:?}", result.error);
    result.value.unwrap_or_else(|| panic!("{name} has a value"))
}

/// The clock pair into a 1 MΩ load on `q`, with `cards` after `.tran`.
fn clocks_deck(scratch: &Scratch, cards: &str) -> Netlist {
    let module = scratch.write("clocks.vams", CLOCKS);
    parse(&format!(
        "* clocks\n.hdl \"{module}\"\nx1 q p clocks\nr1 q 0 1meg\n.param vcc=1\n.tran 0.1n 100n\n{cards}\n.end\n"
    ))
}

/// A converted HDL output measures on its analog side without the deck
/// `.ESAVE`ing the converter's private event endpoint, whose column the run
/// leaves empty.
#[test]
fn an_analog_period_measures_without_saving_the_converter_endpoint() {
    let scratch = Scratch::new("f1");
    let netlist = clocks_deck(
        &scratch,
        ".measure tran tv TRIG V(q) VAL=0.5 RISE=1 TARG V(q) VAL=0.5 RISE=2",
    );
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    let period = value(&results, "tv");
    assert!(
        (period - 20.0e-9).abs() <= 1.0e-15,
        "V(q) rises 20 ns apart: {period:e}"
    );
}

/// The clock pair, plus a 4-bit `b` that counts on every rise of `q`: it
/// reads 1 from 10 ns, 2 from 30 ns, 3 from 50 ns, 4 from 70 ns and 5 from
/// 90 ns. Its bits drive no analog load.
const COUNTER: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module counter(q, p, b);\n\
  output q; reg q;\n\
  output p; reg p;\n\
  output [3:0] b; reg [3:0] b;\n\
  initial begin q = 1'b0; p = 1'b0; b = 4'd0; end\n\
  always #10 q = ~q;\n\
  always #7 p = ~p;\n\
  always @(posedge q) b <= b + 4'd1;\n\
endmodule\n";

/// The counter fixture with `cards` after `.tran`, measured.
fn counter_measures(cards: &str) -> Vec<MeasureResult> {
    let scratch = Scratch::new("counter");
    let module = scratch.write("counter.vams", COUNTER);
    let netlist = parse(&format!(
        "* counter\n.hdl \"{module}\"\nx1 q p b3 b2 b1 b0 counter\nr1 q 0 1meg\n.param vcc=1\n.tran 0.1n 100n\n{cards}\n.end\n"
    ));
    let result = run(&netlist);
    evaluate_tran_measurements(&netlist, &result)
}

/// Two event times agree to 4 ulp.
fn assert_instant(actual: f64, expected: f64, what: &str) {
    let ulp = expected.abs() * f64::EPSILON;
    assert!(
        (actual - expected).abs() <= 4.0 * ulp,
        "{what}: {actual:e}, expected {expected:e}"
    );
}

/// Why `name` failed, folded to lower case: the parser folds operand names.
fn error(results: &[MeasureResult], name: &str) -> String {
    let result = measure(results, name);
    assert!(result.value.is_none(), "{name} fails: {result:?}");
    result
        .error
        .as_deref()
        .unwrap_or_else(|| panic!("{name} names why it fails"))
        .to_ascii_lowercase()
}

/// `q` rises at 10, 30, 50, 70 and 90 ns: one period is 20 ns, the first
/// rise to the last 80 ns.
#[test]
fn a_digital_period_is_exact() {
    let results = counter_measures(
        ".measure tran tq TRIG D(q) VAL=0.5 RISE=1 TARG D(q) VAL=0.5 RISE=2\n\
         .measure tran tlast TRIG D(q) VAL=0.5 RISE=1 TARG D(q) VAL=0.5 RISE=LAST",
    );
    assert_instant(value(&results, "tq"), 20.0e-9, "one period");
    assert_instant(value(&results, "tlast"), 80.0e-9, "first rise to last");
    let units = measure(&results, "tq").units.as_ref().expect("units");
    assert_eq!(units.value, MeasurementUnit::Known("s".to_string()));
}

/// `p` is 0 on [0, 7), 1 on [7, 14), 0 on [14, 21) and 1 from 21 ns: a value
/// at an event instant is the new one.
#[test]
fn a_digital_value_is_right_continuous() {
    let results = counter_measures(
        ".measure tran f20 FIND D(p) AT=20n\n\
         .measure tran f21 FIND D(p) AT=21n\n\
         .measure tran f14 FIND D(p) AT=14n",
    );
    assert_eq!(value(&results, "f20"), 0.0);
    assert_eq!(value(&results, "f21"), 1.0);
    assert_eq!(value(&results, "f14"), 0.0);
    let found = measure(&results, "f21");
    assert_eq!(found.event_axis, Some(21.0e-9));
    let units = found.units.as_ref().expect("units");
    assert_eq!(units.value, MeasurementUnit::Known("1".to_string()));
}

/// `p` is 1 on [7, 14) and [21, 28): half of [0, 28), and 3 ns of [0, 10).
#[test]
fn a_digital_average_is_its_duty() {
    let results = counter_measures(
        ".measure tran duty AVG D(p) FROM=0 TO=28n\n\
         .measure tran part AVG D(p) FROM=0 TO=10n\n\
         .measure tran high INTEG D(p) FROM=0 TO=28n\n\
         .measure tran rms RMS D(p) FROM=0 TO=10n\n\
         .measure tran top MAX D(p) FROM=0 TO=10n OUTPUT=TIME\n\
         .measure tran swing PP D(p) FROM=0 TO=10n",
    );
    assert!((value(&results, "duty") - 0.5).abs() <= 1e-15);
    assert!((value(&results, "part") - 0.3).abs() <= 1e-15);
    assert!((value(&results, "high") - 14.0e-9).abs() <= 1e-23);
    assert!((value(&results, "rms") - 0.3f64.sqrt()).abs() <= 1e-15);
    assert_instant(value(&results, "top"), 7.0e-9, "p first reaches 1");
    assert_eq!(value(&results, "swing"), 1.0);
}

/// The bus word reads 3 on [50, 70) ns and crosses 2.5 when it steps from 2
/// to 3, at 50 ns.
#[test]
fn a_bus_reads_its_word() {
    let results = counter_measures(
        ".measure tran word FIND D(x1.b) AT=55n\n\
         .measure tran three WHEN D(x1.b)=2.5 CROSS=1",
    );
    assert_eq!(value(&results, "word"), 3.0);
    assert_instant(value(&results, "three"), 50.0e-9, "the word reaches 3");
}

/// An HDL net that is X until 3 ns, 0 to 5 ns, X to 8 ns and 1 after; and
/// one that returns to 0 at 8 ns instead.
const UNDEFINED: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module undefined(r, s);\n\
  output r; reg r;\n\
  output s; reg s;\n\
  initial begin r = 1'bx; s = 1'bx; #3 r = 1'b0; s = 1'b0; #2 r = 1'bx; s = 1'bx; #3 r = 1'b1; s = 1'b0; end\n\
endmodule\n";

/// A crossing looks through X to the next known value; an aggregate over X
/// fails and names the stretch.
#[test]
fn an_undefined_stretch_is_transparent_to_a_crossing_and_fails_an_average() {
    let scratch = Scratch::new("undefined");
    let module = scratch.write("undefined.vams", UNDEFINED);
    let netlist = parse(&format!(
        "* undefined\n.hdl \"{module}\"\nx1 r s undefined\n.tran 0.1n 10n\n\
         .measure tran rise WHEN D(r)=0.5 RISE=1\n\
         .measure tran none WHEN D(s)=0.5 CROSS=1\n\
         .measure tran mean AVG D(r) FROM=3n TO=10n\n\
         .measure tran inside FIND D(r) AT=6n\n.end\n"
    ));
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    assert_instant(
        value(&results, "rise"),
        8.0e-9,
        "0 -> x -> 1 rises at the x -> 1 step",
    );
    assert!(
        error(&results, "none").contains("not found"),
        "0 -> x -> 0 is no crossing: {results:?}"
    );
    let mean = error(&results, "mean");
    assert!(
        mean.contains("d(r)") && mean.contains("[5e-9 s, 8e-9 s)"),
        "the first undefined stretch is named: {mean}"
    );
    let inside = error(&results, "inside");
    assert!(
        inside.contains("d(r) is x at 6e-9 s"),
        "the state and the instant are named: {inside}"
    );
}

/// A real output steps to 0.25 at 5 ns and to 0.75 at 9 ns.
const REAL_STEPS: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module steps(w);\n\
  output w; wreal w; real v;\n\
  assign w = v;\n\
  initial begin v = 0.0; #5 v = 0.25; #4 v = 0.75; end\n\
endmodule\n";

/// An HDL reader of the real net, which keeps it an event net.
const REAL_SINK: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module sink(w, d);\n\
  input w; wreal w;\n\
  output d; reg d;\n\
  initial d = 1'b0;\n\
  always @(w) d = (w > 0.5);\n\
endmodule\n";

/// The real net crosses 0.5 at its 9 ns step and averages 0.5 over [5, 13).
#[test]
fn a_real_net_is_a_step_function() {
    let scratch = Scratch::new("real");
    let source = scratch.write("steps.vams", REAL_STEPS);
    let sink = scratch.write("sink.vams", REAL_SINK);
    let netlist = parse(&format!(
        "* real\n.hdl \"{source}\"\n.hdl \"{sink}\"\nx1 w steps\nx2 w d sink\n.tran 0.1n 20n\n\
         .measure tran half WHEN E(w)=0.5 CROSS=1\n\
         .measure tran mean AVG E(w) FROM=5n TO=13n\n.end\n"
    ));
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    assert_instant(value(&results, "half"), 9.0e-9, "E(w) crosses 0.5");
    assert!((value(&results, "mean") - 0.5).abs() <= 1e-15);
    let units = measure(&results, "mean").units.as_ref().expect("units");
    assert_eq!(
        units.value,
        MeasurementUnit::Unknown,
        "a real net declares no unit"
    );
}

/// A real output with no reader reaches its deck net through a converter the
/// builder inserts, whose branch the run does not retain: the net's voltage
/// measures without it.
#[test]
fn a_converted_real_output_measures_on_its_voltage() {
    let scratch = Scratch::new("lone_real");
    let module = scratch.write("steps.vams", REAL_STEPS);
    let netlist = parse(&format!(
        "* lone real\n.hdl \"{module}\"\nx1 w steps\n.tran 0.1n 20n\n\
         .measure tran settled FIND V(w) AT=20n\n.end\n"
    ));
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    let settled = value(&results, "settled");
    assert!(
        (settled - 0.75).abs() <= 1e-9,
        "V(w) holds the last real value: {settled}"
    );
}

/// A solved point has no event values, and a converter's private endpoint
/// sends the card to the deck net.
#[test]
fn an_event_operand_off_a_transient_and_a_private_endpoint_are_refused() {
    let netlist =
        parse("* dc\nv1 a 0 1\nr1 a 0 1k\n.dc v1 0 1 0.5\n.measure dc m FIND D(a) AT=0.5\n.end\n");
    let sweep = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "v1", 0.0, 1.0, 0.5)
        .expect("the sweep runs");
    let results = evaluate_dc_measurements(&netlist, &sweep);
    let refusal = error(&results, "m");
    assert!(
        refusal.contains("d(a)") && refusal.contains(".measure dc"),
        "{refusal}"
    );

    // The deck's output namespace refuses the endpoint's voltage before the
    // run, naming it.
    let scratch = Scratch::new("endpoint");
    let module = scratch.write("counter.vams", COUNTER);
    let netlist = parse(&format!(
        "* endpoint\n.hdl \"{module}\"\nx1 q p b3 b2 b1 b0 counter\nr1 q 0 1meg\n.param vcc=1\n\
         .tran 0.1n 100n\n.measure tran private FIND V(x1.q__event_0) AT=20n\n.end\n"
    ));
    let refusal = try_run(&netlist)
        .expect_err("V() of a private endpoint is refused")
        .to_string()
        .to_ascii_lowercase();
    assert!(refusal.contains("x1.q__event_0"), "{refusal}");

    // A bare name and D() reach the measurement, which sends them to the
    // deck net.
    let results = counter_measures(
        ".measure tran bare FIND x1.q__event_0 AT=20n\n\
         .measure tran digital FIND D(x1.q__event_0) AT=20n",
    );
    for name in ["bare", "digital"] {
        let refusal = error(&results, name);
        assert!(
            refusal.contains("x1.q__event_0") && refusal.contains("port q of x1"),
            "{name}: {refusal}"
        );
    }
}

//=============================================================================
// Across domains
//=============================================================================

/// Connect rules for a 1 V supply: a D/A that ramps linearly over 1 ns, and an
/// A/D with one threshold at half the supply and no band.
const RULES: &str = "connectrules deck;\n\
    connect a2d #(.vsup(1), .vthi(0.5), .vtlo(0.5));\n\
    connect d2a #(.vsup(1), .trise(1n), .tfall(1n));\n\
endconnectrules\n";

/// `module` and the built-in connect modules under [`RULES`], as one source.
fn with_rules(module: &str) -> String {
    let mut source = module.to_string();
    for (_, connect) in BUILTIN_CONNECT_MODULES {
        source.push_str(connect);
    }
    source.push_str(RULES);
    source
}

/// `q` rises at 10 ns and its D/A ramps from 0 to 1 V over 1 ns through
/// `rhi` = 20 Ω into 1 MΩ, so V(q) = 1e6/(1e6 + 20) of the ramp and crosses
/// 0.5 V 0.5·(1 + 20/10⁶) ns after the digital edge. The ramp is linear, so
/// the interpolated crossing is exact.
#[test]
fn a_digital_edge_to_an_analog_crossing_is_one_delay() {
    let scratch = Scratch::new("d_to_a");
    let module = scratch.write("counter.vams", &with_rules(COUNTER));
    let netlist = parse(&format!(
        "* digital edge to analog crossing\n.hdl \"{module}\" module=counter\n\
         .options connectrules=deck\nx1 q p b3 b2 b1 b0 counter\nr1 q 0 1meg\n.tran 0.01n 30n\n\
         .measure tran delay TRIG D(q) VAL=0.5 RISE=1 TARG V(q) VAL=0.5 RISE=1\n\
         .measure tran back TRIG V(q) VAL=0.5 RISE=1 TARG D(q) VAL=0.5 FALL=1\n.end\n"
    ));
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    let expected = 0.5e-9 * (1.0 + 20.0 / 1.0e6);
    let delay = value(&results, "delay");
    assert!(
        (delay - expected).abs() <= 1e-15,
        "D(q) rises at 10 ns, V(q) crosses 0.5 V {expected:e} s later: {delay:e}"
    );
    // Back from the analog crossing to the next digital fall, at 20 ns.
    let back = value(&results, "back");
    assert!((back - (10.0e-9 - expected)).abs() <= 1e-15, "{back:e}");
}

/// An HDL input that only reads its net.
const SENSE: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module sense(a, y);\n\
  input a; wire a;\n\
  output y; reg y;\n\
  initial y = 1'b0;\n\
  always @(a) y = a;\n\
endmodule\n";

/// A ramp from 0 to 1 V over 1 µs crosses the A/D's 0.5 V threshold at
/// 500 ns, and the HDL input sees the edge there: the delay is 0 within the
/// declared time precision (1 ps) and the solver's root tolerance.
#[test]
fn an_analog_crossing_to_a_digital_edge_is_one_delay() {
    let scratch = Scratch::new("a_to_d");
    let module = scratch.write("sense.vams", &with_rules(SENSE));
    let netlist = parse(&format!(
        "* analog crossing to digital edge\n.hdl \"{module}\" module=sense\n\
         .options connectrules=deck\nvin a 0 pwl(0 0 1u 1)\nx1 a y sense\n.tran 1n 1u\n\
         .measure tran delay TRIG V(a) VAL=0.5 RISE=1 TARG D(a) VAL=0.5 RISE=1\n.end\n"
    ));
    let result = run(&netlist);
    let results = evaluate_tran_measurements(&netlist, &result);
    let delay = value(&results, "delay");
    assert!(
        delay.abs() <= 1e-12,
        "the edge follows the crossing: {delay:e}"
    );
}

/// `p` first rises at 7 ns, while `q` is low: the analog value read at the
/// exact digital edge is 0 V. And `q`, high half of every 20 ns, has a duty
/// of one half on its converted net too.
#[test]
fn an_analog_value_reads_at_a_digital_edge() {
    let results = counter_measures(
        ".measure tran low FIND V(q) WHEN D(p)=0.5 RISE=1\n\
         .measure tran level FIND D(q) WHEN V(q)=0.5 RISE=1\n\
         .measure tran duty AVG D(q) FROM=10n TO=90n",
    );
    let low = measure(&results, "low");
    assert!(low.error.is_none(), "{low:?}");
    assert!(low.value.expect("value").abs() <= 1e-9, "{low:?}");
    assert_instant(low.event_axis.expect("instant"), 7.0e-9, "p rises");
    // V(q) crosses 0.5 V on its way up, after the step, where D(q) reads 1.
    assert_eq!(value(&results, "level"), 1.0);
    assert!((value(&results, "duty") - 0.5).abs() <= 1e-15);
}

/// A step has no derivative; a sampled waveform and a step cannot be compared
/// in one condition; an expression reads sampled points.
#[test]
fn shapeless_and_cross_domain_readings_are_refused() {
    let results = counter_measures(
        ".measure tran slope DERIV D(q) AT=15n\n\
         .measure tran mixed WHEN V(q)=D(p)\n\
         .measure tran clause TRIG V(q)=D(p) TARG D(q) VAL=0.5 RISE=2\n\
         .measure tran twice PARAM='D(q)*2'",
    );
    let slope = error(&results, "slope");
    assert!(slope.contains("deriv") && slope.contains("d(q)"), "{slope}");
    for name in ["mixed", "clause"] {
        let mixed = error(&results, name);
        assert!(
            mixed.contains("v(q)") && mixed.contains("d(p)"),
            "{name}: {mixed}"
        );
    }
    let twice = error(&results, "twice");
    assert!(twice.contains("param") && twice.contains("d(q)"), "{twice}");
}
