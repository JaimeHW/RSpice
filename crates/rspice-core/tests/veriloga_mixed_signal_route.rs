//! What a deck gets when it instantiates a mixed Verilog-AMS module.
//!
//! It gets a transient. This file used to pin the opposite — that a `.va` whose
//! module carried both an analog block and a process was refused at code
//! generation, before a node was allocated — because nothing elaborated one
//! into `CircuitData` and running the analog equations alone would have been a
//! plausible curve for a circuit the deck did not describe. That door is the
//! one this file now goes through.
//!
//! # What the route is
//!
//! `.VERILOGA` compiles the file once, with `enable_ams` on, and the compiled
//! artifact's *discrete plan* decides what the X-card builds: empty, and it is
//! the `VerilogADevice` it has always been; non-empty, and it is a
//! `MixedSignalHost` that executes both halves. Every module port named in that
//! plan is a discipline boundary and takes a bridge — an A/D bridge for a port
//! the module reads, a D/A bridge for one it drives — with the deck's supply
//! setting the thresholds and levels exactly as the XSPICE auto-bridge sets
//! its own.
//!
//! # What the tests here are
//!
//! Four end-to-end properties and the refusals that bound them:
//!
//! * an analog oscillator counted by a digital counter, where the count in
//!   `digital_traces` has to equal the number of times the recorded analog
//!   waveform crossed the bridge threshold going up;
//! * a digital clock divider driving an analog RC, where the analog node has to
//!   show the divided period and reach it through a ramp rather than a step;
//! * a run whose stepper rejects timepoints, whose digital trace has to equal
//!   the digital trace of a run that rejects none — a rejected trial commits
//!   nothing, at deck level;
//! * the analog-only module, which has to keep answering the way a plain
//!   resistor does, because nothing about this route may reach a deck that has
//!   no mixed module in it.
#![cfg(feature = "veriloga")]

use rspice_core::engine::TransientResult;
use rspice_core::{Engine, Netlist, SimulationConfig};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` written to a unique path, deleted when the guard drops.
///
/// The deletion matters more than it looks: the engine's Verilog-A cache is
/// keyed by canonical path, so two tests sharing a filename would share a cache
/// entry, and a leaked file would let a later run resolve a model this run
/// compiled.
struct ModelFile(PathBuf);

impl ModelFile {
    fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_mixed_route_{name}_{}_{sequence}.va",
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

fn run(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .expect("the deck runs")
}

fn error_for(deck: &str, tstop: f64, max_step: f64) -> String {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .err()
        .map(|error| error.to_string())
        .unwrap_or_else(|| panic!("the deck was expected to be refused, and ran"))
}

/// How many changes a boundary net's digital trace records.
///
/// The trace opens with the value the net held at time zero and then carries
/// one point per change, so the number of changes is one fewer than the number
/// of points. A net with no trace at all has not been recorded, which is a
/// different failure from a net that never moved, so this refuses rather than
/// answering zero.
fn digital_transitions(result: &TransientResult, net: &str) -> usize {
    let points = result
        .digital_trace_named(net)
        .unwrap_or_else(|| panic!("net '{net}' has no digital trace"));
    points.len().saturating_sub(1)
}

fn waveform(result: &TransientResult, node: &str) -> Vec<f64> {
    let index = result
        .node_index_named(node)
        .unwrap_or_else(|| panic!("node '{node}' is not in the result"));
    result.voltage_waveform(index).to_vec()
}

/// How many times a recorded waveform crosses a level going up.
///
/// Strictly above, because that is the A/D bridge's own tie-break: it asks
/// `voltage <= low` before it asks `voltage >= high`, so a sample sitting
/// exactly on a threshold reads as zero. With `in_low` and `in_high` equal —
/// which is what a supply-derived auto-bridge gives — that makes the bit one
/// exactly when the voltage is strictly above. Counting with `>=` here would
/// disagree with the bridge on the one sample that matters, the sine's own
/// starting value.
fn upward_crossings(samples: &[f64], threshold: f64) -> usize {
    let mut crossings = 0;
    let mut above = samples.first().is_some_and(|value| *value > threshold);
    for &value in samples.iter().skip(1) {
        let now_above = value > threshold;
        if now_above && !above {
            crossings += 1;
        }
        above = now_above;
    }
    crossings
}

// ---------------------------------------------------------------------------
// (a) An analog oscillator counted by a digital counter
// ---------------------------------------------------------------------------

/// Both halves, wired to each other through the boundary. The `analog` block is
/// a 1 kilohm resistor between `p` and `n`; `clk` is read across an A/D bridge
/// and the two flip-flops are driven by it; `q0` and `q1` are driven back out
/// across D/A bridges. Executing only the first would be a different circuit,
/// and executing only the second would not be a circuit at all.
const OSC_COUNTER: &str = r#"
`include "disciplines.vams"
module osc_counter(p, n, clk, q0, q1);
    inout p, n;
    electrical p, n;
    input clk;
    output q0, q1;
    wire clk;
    reg q0, q1;
    initial q0 = 1'b0;
    initial q1 = 1'b0;
    always @(posedge clk) q0 <= ~q0;
    always @(posedge q0) q1 <= ~q1;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn an_analog_oscillator_is_counted_by_the_digital_half_it_drives() {
    let model = ModelFile::new("osc_counter", OSC_COUNTER);
    let deck = format!(
        "* an analog oscillator clocking a counter across an a2d boundary\n\
         vosc clk 0 sin(1.65 1.65 100meg)\n\
         rload p 0 10k\n\
         x1 p 0 clk q0 q1 osc_counter\n\
         .va \"{}\" osc_counter\n\
         .tran 0.2n 95n\n\
         .end\n",
        model.deck_path()
    );

    let result = run(&deck, 95.0e-9, 0.2e-9);
    assert!(!result.time.is_empty(), "the run produced no timepoints");

    // The deck's supply is the default 3.3 V, so the A/D bridge switches at
    // half of it. That is the same threshold `add_planned_xspice_auto_bridge`
    // gives an auto-bridged node, which is the point: the boundary is not a
    // second set of numbers.
    let clock = waveform(&result, "clk");
    let cycles = upward_crossings(&clock, 1.65);
    assert!(
        cycles >= 8,
        "the oscillator should have completed several cycles in 95 ns, saw {cycles}"
    );

    // Every up-crossing of the analog waveform is one posedge of the module's
    // `clk`, and every posedge toggles `q0`. If a rejected trial had leaked, or
    // a timepoint had been delivered to the digital wheel twice, this would be
    // larger than the cycle count rather than equal to it.
    assert_eq!(
        digital_transitions(&result, "q0"),
        cycles,
        "the counter's first stage must toggle once per analog cycle"
    );
    // And the second stage divides the first, so it toggles on `q0`'s rising
    // edges alone.
    assert_eq!(
        digital_transitions(&result, "q1"),
        cycles.div_ceil(2),
        "the counter's second stage must toggle once per two first-stage toggles"
    );
}

// ---------------------------------------------------------------------------
// (b) A digital clock divider driving an analog RC
// ---------------------------------------------------------------------------

/// A clock the module generates for itself, so the deck route's breakpoint seam
/// is what makes this run correctly: `always #5` schedules an activation five
/// nanoseconds ahead, `MixedSignalHost::next_event_time` reports it, and the
/// transient's breakpoint manager is what makes the stepper land on it.
const CLOCK_DIVIDER: &str = r#"
`include "disciplines.vams"
module clock_divider(p, n, qdiv);
    inout p, n;
    electrical p, n;
    output qdiv;
    reg clk, qdiv;
    initial clk = 1'b0;
    initial qdiv = 1'b0;
    always #5 clk = ~clk;
    always @(posedge clk) qdiv <= ~qdiv;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn divider_deck(model: &ModelFile) -> String {
    format!(
        "* a digital clock divider driving an analog RC across a d2a boundary\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         r1 qdiv out 1k\n\
         c1 out 0 10p\n\
         .va \"{}\" clock_divider\n\
         .tran 1n 200n\n\
         .end\n",
        model.deck_path()
    )
}

#[test]
fn a_digital_divider_drives_the_analog_side_at_the_divided_frequency() {
    let model = ModelFile::new("clock_divider", CLOCK_DIVIDER);
    let result = run(&divider_deck(&model), 200.0e-9, 1.0e-9);

    // `always #5` toggles the internal clock every 5 ns, so its period is
    // 10 ns; `qdiv` toggles once per rising edge of it, so `qdiv`'s period is
    // 20 ns. Over 200 ns that is ten full divided cycles, which is twenty
    // toggles.
    assert_eq!(
        digital_transitions(&result, "qdiv"),
        20,
        "the divider must toggle once per 10 ns clock period over 200 ns"
    );

    // The digital trace's own timing is the proof of the divided frequency:
    // consecutive toggles are exactly one clock period apart, and the stepper
    // landed on each because the module's next event time became a breakpoint.
    let points = result
        .digital_trace_named("qdiv")
        .expect("the divider output has a digital trace");
    for pair in points.windows(2).skip(1) {
        let interval = pair[1].time - pair[0].time;
        assert!(
            (interval - 10.0e-9).abs() < 1.0e-12,
            "divided-clock toggles must be one 10 ns clock period apart, saw {interval:e} s \
             between {:e} and {:e}",
            pair[0].time,
            pair[1].time
        );
    }

    // The D/A bridge drives through a source resistance into an RC whose time
    // constant is comparable with the half period, so the analog node ramps.
    // A bridge that stepped instead would put every sample at one rail or the
    // other.
    let out = waveform(&result, "out");
    let low = out.iter().copied().fold(f64::INFINITY, f64::min);
    let high = out.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(
        high - low > 1.0,
        "the divided output must actually swing, saw {low} to {high}"
    );
    let midband = out
        .iter()
        .filter(|value| **value > low + 0.15 * (high - low) && **value < high - 0.15 * (high - low))
        .count();
    assert!(
        midband >= 10,
        "the RC must be caught mid-transition on many samples if the edge is a ramp, saw \
         {midband} of {}",
        out.len()
    );

    // And the analog node follows the divided period, not the clock's: it
    // crosses the mid level once per divided cycle.
    let analog_cycles = upward_crossings(&out, low + 0.5 * (high - low));
    assert_eq!(
        analog_cycles, 10,
        "the analog waveform must show one cycle per 20 ns divided period over 200 ns"
    );
}

// ---------------------------------------------------------------------------
// (c) A rejected step commits nothing
// ---------------------------------------------------------------------------

/// One run of the divider deck, with the number of timepoints its step
/// controller rejected on local truncation error.
///
/// `ConvergenceQuality::timestep_reductions` is the count of LTE rejections:
/// the transient's `if !accept` arm, which is entered when the estimated
/// truncation error exceeds the budget, is its only writer. (`lte_rejections`
/// beside it is never incremented by anything, so it is not the counter to read
/// — see the report accompanying this lane.)
fn run_divider_with_rejection_count(deck: &str, max_step: f64) -> (TransientResult, usize) {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 200.0e-9, max_step)
        .expect("the run completes");
    let rejections = engine.convergence_quality().timestep_reductions;
    (result, rejections)
}

#[test]
fn rejected_timepoints_leave_the_digital_half_exactly_where_they_found_it() {
    let model = ModelFile::new("clock_divider_rollback", CLOCK_DIVIDER);
    let deck = divider_deck(&model);

    // Two runs of one deck whose step controllers do very different amounts of
    // work. The coarse ceiling makes the controller propose steps it then has
    // to throw away; the fine one leaves it little to reject. Neither changes
    // the circuit or the module.
    let (coarse, coarse_rejections) = run_divider_with_rejection_count(&deck, 1.0e-9);
    let (fine, fine_rejections) = run_divider_with_rejection_count(&deck, 2.0e-11);

    assert!(
        coarse_rejections > 0,
        "this test is vacuous unless the coarse run actually rejected timepoints"
    );
    assert!(
        coarse_rejections >= fine_rejections * 4,
        "the two runs must differ substantially in how much they rejected, saw {coarse_rejections} \
         against {fine_rejections}"
    );
    assert!(
        fine.time.len() >= coarse.time.len() * 4,
        "the two runs must also differ substantially in which timepoints they kept, saw {} \
         against {}",
        coarse.time.len(),
        fine.time.len()
    );

    // The digital half is driven by its own time wheel, whose activations are
    // breakpoints the stepper lands on exactly. Which analog candidates were
    // tried and thrown away is therefore invisible to it — unless a rejected
    // trial left something behind. It cannot: every trial the solver opens for
    // a Newton evaluation is a probe that is rolled back before the stamp
    // returns, so a rejected timepoint never reached the module at all. If one
    // had, the difference of tens of rejections between these two runs would
    // put the divider tens of toggles apart.
    let coarse_points = coarse
        .digital_trace_named("qdiv")
        .expect("the coarse run traced the divider output");
    let fine_points = fine
        .digital_trace_named("qdiv")
        .expect("the fine run traced the divider output");
    assert_eq!(
        coarse_points.len(),
        fine_points.len(),
        "a rejected timepoint must not add or drop a digital transition"
    );
    for (coarse_point, fine_point) in coarse_points.iter().zip(fine_points) {
        assert_eq!(
            coarse_point.value, fine_point.value,
            "a rejected timepoint must not change what the boundary settled to"
        );
        assert!(
            (coarse_point.time - fine_point.time).abs() < 1.0e-15,
            "a rejected timepoint must not move a digital transition: {:e} against {:e}",
            coarse_point.time,
            fine_point.time
        );
    }
}

// ---------------------------------------------------------------------------
// (d) The analog-only module is untouched by any of this
// ---------------------------------------------------------------------------

const ANALOG_ONLY: &str = r#"
`include "disciplines.vams"
module analog_deck_route(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn the_same_deck_shape_still_runs_when_the_module_is_analog_only() {
    // The control, and the inertness pin. `enable_ams` is now on for every
    // `.VERILOGA` include, so this module compiles through a compiler option it
    // did not use to see; its whole effect on a module with no digital content
    // is to skip a check that module passes, and this is where that is
    // observable rather than argued. The module's analog block *is* a 1 kilohm
    // resistor, so the trajectory it produces has to be the trajectory the
    // resistor produces, sample for sample.
    let model = ModelFile::new("analog", ANALOG_ONLY);
    let module_deck = format!(
        "* analog module deck route\n\
         v1 in 0 pulse(0 1 1n 1n 1n 10n 20n)\n\
         x1 in mid analog_deck_route\n\
         c1 mid 0 1n\n\
         rmid mid 0 10k\n\
         .va \"{}\" analog_deck_route\n\
         .tran 1n 60n\n\
         .end\n",
        model.deck_path()
    );
    let resistor_deck = "* the same circuit with the module written out\n\
         v1 in 0 pulse(0 1 1n 1n 1n 10n 20n)\n\
         x1 in mid 1k\n\
         c1 mid 0 1n\n\
         rmid mid 0 10k\n\
         .tran 1n 60n\n\
         .end\n"
        .replace("x1 in mid 1k", "r1 in mid 1k");

    let module_result = run(&module_deck, 60.0e-9, 1.0e-9);
    let resistor_result = run(&resistor_deck, 60.0e-9, 1.0e-9);

    assert!(
        module_result.digital_traces.is_empty(),
        "an analog-only module must not open a digital trace channel"
    );
    assert_eq!(
        module_result.time, resistor_result.time,
        "the analog-only module must accept the same timepoints its equivalent resistor does"
    );
    let module_mid = waveform(&module_result, "mid");
    let resistor_mid = waveform(&resistor_result, "mid");
    assert_eq!(module_mid.len(), resistor_mid.len());
    for (index, (module_value, resistor_value)) in module_mid.iter().zip(&resistor_mid).enumerate()
    {
        assert!(
            (module_value - resistor_value).abs() < 1.0e-12,
            "sample {index}: the analog-only module gave {module_value} where its equivalent \
             resistor gave {resistor_value}"
        );
    }
}

// ---------------------------------------------------------------------------
// What the route still refuses, and by name
// ---------------------------------------------------------------------------

const BIDIRECTIONAL: &str = r#"
`include "disciplines.vams"
module bidi_mixed(p, n, io);
    inout p, n;
    electrical p, n;
    inout io;
    reg io;
    initial io = 1'b0;
    always #5 io = ~io;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn a_bidirectional_discrete_port_is_refused_rather_than_guessed() {
    let model = ModelFile::new("bidi", BIDIRECTIONAL);
    let deck = format!(
        "* a bidirectional discrete boundary\n\
         x1 p 0 io bidi_mixed\n\
         rp p 0 1meg\n\
         rio io 0 1k\n\
         .va \"{}\" bidi_mixed\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 20.0e-9, 1.0e-9);
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("bidirectional") && lowered.contains("io"),
        "the refusal must name the shape and the port: {error}"
    );
}

const VECTOR_PORT: &str = r#"
`include "disciplines.vams"
module vector_mixed(p, n, count);
    inout p, n;
    electrical p, n;
    output [1:0] count;
    reg [1:0] count;
    initial count = 2'b00;
    always #5 count = count + 2'b01;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn a_vector_discrete_port_is_refused_because_the_deck_names_one_node() {
    let model = ModelFile::new("vector", VECTOR_PORT);
    let deck = format!(
        "* a two-bit discrete boundary onto a single deck node\n\
         x1 p 0 count vector_mixed\n\
         rp p 0 1meg\n\
         .va \"{}\" vector_mixed\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 20.0e-9, 1.0e-9);
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("bits wide") && lowered.contains("count"),
        "the refusal must say the port is a vector and name it: {error}"
    );
}

#[test]
fn a_discrete_port_joined_to_an_xspice_event_net_is_refused() {
    let model = ModelFile::new("shared_event_net", CLOCK_DIVIDER);
    let deck = format!(
        "* the module's discrete output tied to an XSPICE event net\n\
         vin in 0 pulse(0 1 0 1p 1p 1n 2n)\n\
         r1 in 0 1k\n\
         a_adc [in] [qdiv] adc\n\
         .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         .va \"{}\" clock_divider\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 20.0e-9, 1.0e-9);
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("event-driven") && lowered.contains("qdiv"),
        "the refusal must say the net is already event-driven and name it: {error}"
    );
}

#[test]
fn a_mixed_module_is_refused_by_the_analyses_that_cannot_represent_it() {
    let model = ModelFile::new("ac_refusal", CLOCK_DIVIDER);
    let deck = format!(
        "* a mixed module asked for a small-signal answer\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         vac p 0 ac 1\n\
         .va \"{}\" clock_divider\n\
         .ac dec 10 1k 1meg\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1.0e3, 1.0e4])
        .err()
        .map(|error| error.to_string())
        .expect("AC analysis must refuse a mixed module rather than omit it");
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("ac analysis") && lowered.contains("x1"),
        "the refusal must name the analysis and the instance: {error}"
    );
}
