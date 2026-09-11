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
//! a mixed analog host plus the circuit's shared digital runtime. HDL-only
//! port bits join resolved event nets. Bits also used by continuous devices
//! keep A/D or D/A bridges, with supply-derived thresholds and output levels.
//!
//! # What the tests here are
//!
//! End-to-end properties, and the refusals that bound them:
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
//!   no mixed module in it;
//! * the accepted-sample hook a live consumer watches, whose view of a bridge
//!   net has to be the history the finished result keeps — the deck-level
//!   statement of D5 lockstep for anything reading the run as it goes;
//! * a vector discrete port, bridged one net per bit and declared as one bus:
//!   the bits are co-timed because the discrete half publishes the whole
//!   vector at once, the declaration reaches the live hook and both
//!   interchange routes, and saving one member retains them all;
//! * a vector discrete port whose range is not anchored at zero — `[7:4]` and
//!   `[4:7]` — where a declared index is a name for a bit rather than an offset
//!   into the value, so the deck's first net for the port is whichever bit the
//!   left declared bound calls out.
#![cfg(feature = "veriloga")]

use rspice_core::abort_signal::{AbortSignal, DigitalEventCode, TransientSample};
use rspice_core::engine::{DigitalBusSource, TransientResult};
use rspice_core::{Engine, Netlist, SimulationConfig};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
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

#[test]
fn mixed_internal_potentials_and_voltage_branch_unknowns_reach_both_domains() {
    let model = ModelFile::new(
        "internal_probe",
        r#"
module internal_probe(out, sampled);
    inout out; electrical out, inner;
    output sampled; reg sampled;
    initial begin sampled=0; #1 sampled=(V(inner)>1.5); end
    analog begin
        V(inner)<+2;
        V(out)<+3*V(inner);
    end
endmodule
"#,
    );
    let netlist = Netlist::parse(&format!(
        "* internal potential and voltage contributions\n.param vcc=3.3\nX1 out sampled internal_probe\nRsampled sampled 0 1k\n.va \"{}\" internal_probe\n.end\n",
        model.deck_path()
    )).unwrap();
    let result = Engine::default().run_tran(&netlist, 3e-9, 0.2e-9).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        result.voltages[out]
            .iter()
            .all(|value| (*value - 6.0).abs() < 1e-10)
    );
    let sampled = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("sampled"))
        .unwrap();
    assert!(result.voltages[sampled][0].abs() < 1e-10);
    assert!((result.voltages[sampled].last().unwrap() - 3.3 / 1.02).abs() < 1e-6);
    let trace = result
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case("sampled"))
        .unwrap();
    assert_eq!(trace.points.len(), 2);
    assert_eq!(trace.points[1].time, 1e-9);
}

#[test]
fn analog_initialization_finish_precedes_all_digital_process_execution() {
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    // This design is valid but never settles its first digital time slot.
    // Without a prior analog finish, the scheduler must diagnose the loop.
    let digital = "reg q; initial q=0; always #0 q=~q;";
    let engine = Engine::default();
    for placement in ["absent", "same", "before", "after"] {
        let initial = if placement == "same" {
            "analog initial $finish(0);"
        } else {
            ""
        };
        let mixed = ModelFile::new(
            "init_barrier",
            &format!(
                "module init_barrier(p,n); inout p,n; electrical p,n; {digital} {initial} analog I(p,n)<+V(p,n); endmodule"
            ),
        );
        let pure = ModelFile::new(
            "pure_finish",
            "module pure_finish(p,n); inout p,n; electrical p,n; analog initial $finish(0); analog I(p,n)<+V(p,n); endmodule",
        );
        let pure_card = format!(
            "Xfinish p 0 pure_finish\n.va \"{}\" pure_finish\n",
            pure.deck_path()
        );
        let mixed_card = format!(
            "Xmixed p 0 init_barrier\n.va \"{}\" init_barrier\n",
            mixed.deck_path()
        );
        let cards = match placement {
            "before" => format!("{pure_card}{mixed_card}"),
            "after" => format!("{mixed_card}{pure_card}"),
            _ => mixed_card,
        };
        let netlist = Netlist::parse(&format!(
            "* initialization barrier\nV1 p 0 1\n{cards}.end\n"
        ))
        .unwrap();
        let outcome = engine.run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, 1e-6, 1e-7, signal)
        });
        if placement == "absent" {
            let error = outcome.expect_err("the control design must execute its non-settling loop");
            assert!(
                error.to_string().contains("did not settle at tick 0"),
                "{error}"
            );
        } else {
            let SimulationOutcome::Finished {
                result: None,
                finish,
            } = outcome.unwrap()
            else {
                panic!("{placement}: initialization finish must precede the digital loop");
            };
            assert_eq!(finish.point, ModelFinishPoint::Initialization);
            assert_eq!(
                finish.instance,
                if placement == "same" {
                    "Xmixed"
                } else {
                    "Xfinish"
                }
            );
        }
    }
}

#[test]
fn mixed_analog_initialization_uses_the_resolved_deck_temperature() {
    use rspice_core::{NoAbort, SimulationOutcome};
    let model = ModelFile::new(
        "mixed_temperature",
        "module mixed_temperature(p,n); inout p,n; electrical p,n; reg q; initial q=0; analog initial if ($temperature>350) $finish(0); analog I(p,n)<+V(p,n); endmodule",
    );
    let engine = Engine::default();
    for temperature in [27, 100] {
        let netlist = Netlist::parse(&format!(
            "* temperature at initialization\n.temp {temperature}\nV1 p 0 1\nX1 p 0 mixed_temperature\n.va \"{}\" mixed_temperature\n.end\n", model.deck_path()
        )).unwrap();
        let outcome = engine
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_tran_with_abort(&netlist, 1e-9, 1e-10, signal)
            })
            .unwrap();
        if temperature == 100 {
            assert!(matches!(
                outcome,
                SimulationOutcome::Finished { result: None, .. }
            ));
        } else {
            assert!(matches!(outcome, SimulationOutcome::Completed(_)));
        }
    }
}

#[test]
fn unsupported_analog_control_reaches_the_engine_as_a_refusal() {
    for discrete in ["", "reg started; initial started=1;"] {
        for (task, statement) in [
            ("$fatal", "analog begin if (0) $fatal(1, \"invalid\"); end"),
            ("$stop", "analog begin @(final_step) $stop; end"),
            ("$error", "analog initial $error(\"invalid\");"),
        ] {
            let model = ModelFile::new(
                "control_refusal",
                &format!(
                    "module control_refusal(p,n); inout p,n; electrical p,n; {discrete}\n{statement}\nanalog I(p,n)<+V(p,n); endmodule"
                ),
            );
            let deck = format!(
                "* model control cannot disappear at loading\nV1 p 0 1\nX1 p 0 control_refusal\n.va \"{}\" control_refusal\n.end\n",
                model.deck_path()
            );
            let netlist = Netlist::parse(&deck).unwrap();
            let engine = Engine::default();
            // Repeating the load must not replace a failed compile with a
            // cached numerical model that has lost its control statements.
            for _ in 0..2 {
                let error = engine
                    .run_tran(&netlist, 1e-6, 1e-7)
                    .unwrap_err()
                    .to_string();
                assert!(
                    error.contains(task) && error.contains("requires simulation control"),
                    "{error}"
                );
                if discrete.is_empty() {
                    let error = engine.run_dc_op(&netlist).unwrap_err().to_string();
                    assert!(
                        error.contains(task) && error.contains("requires simulation control"),
                        "{error}"
                    );
                }
            }
        }
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
    // A 100 MHz sine starting at its mean and rising crosses that mean upward
    // once per 10 ns period, so ten times in the 95 ns run: at the first
    // timepoint after zero and then every 10 ns to 90 ns.
    assert_eq!(
        cycles, 10,
        "the oscillator completes ten cycles above the bridge threshold in 95 ns"
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
    // Ideal D/A jumps restart integration directly. A 1 ns ceiling can now
    // accept every point; the larger ceiling makes RC curvature force retries.
    let (coarse, coarse_rejections) = run_divider_with_rejection_count(&deck, 4.0e-9);
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
// (e) A process sampling its module's own continuous net
// ---------------------------------------------------------------------------

/// Verilog-AMS LRM 2.4 section 7.3.3's sampler, in a deck.
///
/// The clause's own example is `always @(posedge clk) out = V(in);`, and this
/// is it with a threshold: the process wakes on a clock edge that arrives
/// across an A/D bridge and reads the module's own analog terminal. Nothing
/// leaves the module to carry that value — there is no second X-card, no
/// bridge on `p`, and no deck node between the two halves. The coupling is the
/// language's.
const SAMPLER: &str = r#"
`include "disciplines.vams"
module tracker(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= (V(p, n) > 1.0);
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

#[test]
fn a_process_samples_its_modules_own_analog_terminal_at_the_edge_that_woke_it() {
    let model = ModelFile::new("tracker", SAMPLER);
    // A ramp from 0 V to 2 V over 100 ns, so the 1 V threshold is at 50 ns,
    // and two clock edges that straddle it: one at 20 ns where the ramp is at
    // 0.4 V, one at 60 ns where it is at 1.2 V.
    let deck = format!(
        "* a process sampling its own module's analog terminal\n\
         vin p 0 pwl(0 0 100n 2.0)\n\
         vclk clk 0 pulse(0 3.3 20n 0.1n 0.1n 10n 40n)\n\
         x1 p 0 clk qs tracker\n\
         rq qs 0 10k\n\
         .va \"{}\" tracker\n\
         .tran 0.2n 100n\n\
         .end\n",
        model.deck_path()
    );

    let result = run(&deck, 100.0e-9, 0.2e-9);

    // One transition, and only one: the first edge sampled 0.4 V and wrote the
    // zero `q` already held, the second sampled 1.2 V and wrote one. A probe
    // that read a stale sample, or the same sample twice, would give a
    // different count — zero if it never crossed, two if it crossed back.
    assert_eq!(
        digital_transitions(&result, "qs"),
        1,
        "the sampler crosses its threshold exactly once between the two clock edges"
    );

    let points = result
        .digital_trace_named("qs")
        .expect("the sampled output has a digital trace");
    let transition = points.last().expect("the trace records the transition");

    // The value the process read is the converged solution at the timepoint
    // the edge was detected in, not the one before it — the host refreshes the
    // probe bank from the same candidate its A/D bridges sample, and does it
    // before publishing the transition that wakes the process. So the write
    // lands in the clock edge's own tick.
    //
    // The clock crosses the A/D threshold of 1.65 V half way up a 0.1 ns rise
    // that starts at 60 ns, so the edge is at 60.05 ns and its tick is 60 ns.
    // A sample taken from the *previous* accepted timepoint instead would put
    // this a whole clock period later, at 100 ns, which is what this bound
    // separates.
    assert!(
        (transition.time - 60.0e-9).abs() < 1.0e-9,
        "the sampled write must land at the clock edge that woke it, saw {:e} s",
        transition.time
    );
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

/// The deck a two-bit boundary port needs: one node per bit, declared MSB
/// first, which is the order `rspice-ui`'s netlister emits a vector pin's
/// formals in and the order the declaration lists its members in.
fn vector_deck(model: &ModelFile, saves: &str) -> String {
    format!(
        "* a two-bit discrete boundary, one deck node per bit\n\
         x1 p 0 count#1 count#0 vector_mixed\n\
         rp p 0 1meg\n\
         {saves}\
         .va \"{}\" vector_mixed\n\
         .tran 1n 40n\n\
         .end\n",
        model.deck_path()
    )
}

/// The times a boundary net's recorded history changes at, excluding the
/// opening point every trace carries.
fn change_times(result: &TransientResult, net: &str) -> Vec<f64> {
    result
        .digital_trace_named(net)
        .unwrap_or_else(|| panic!("net '{net}' has no digital trace"))
        .iter()
        .skip(1)
        .map(|point| point.time)
        .collect()
}

/// **A vector discrete port is one net per bit, and the declaration says the
/// bits are one word.**
///
/// The deck has no spelling for a vector: a node list is flat, so an N-bit
/// boundary is N nodes and each is its own recordable conductor. What would
/// otherwise be lost — that those N nodes are one port of one module, with the
/// range its author wrote — is exactly what the bus declaration carries, and
/// this pins both halves: the members are bridged and recorded, and the
/// declaration names them MSB first under the instance's own name.
#[test]
fn a_vector_discrete_port_bridges_one_net_per_bit() {
    let model = ModelFile::new("vector", VECTOR_PORT);
    let result = run(&vector_deck(&model, ""), 40.0e-9, 1.0e-9);

    assert_eq!(
        result.digital_buses.len(),
        1,
        "one vector boundary port declares one bus, saw {:?}",
        result.digital_buses
    );
    let bus = &result.digital_buses[0];
    assert_eq!(bus.name, "x1.count");
    assert_eq!((bus.msb, bus.lsb), (1, 0));
    assert_eq!(
        bus.members,
        vec!["COUNT#1".to_string(), "COUNT#0".to_string()],
        "members are the deck nodes, in the engine's own spelling, declared MSB first"
    );
    assert_eq!(bus.source, DigitalBusSource::Engine);

    // `always #5 count = count + 2'b01` steps the whole vector every five
    // nanoseconds, and a whole-vector step is one bus event: bit zero moves at
    // every step and bit one at every other one, so bit one's change times are
    // a subset of bit zero's and both land on the five-nanosecond grid.
    let low = change_times(&result, "count#0");
    let high = change_times(&result, "count#1");
    assert!(
        low.len() >= 6,
        "the counter must run for this to test anything, saw {low:?}"
    );
    for time in low.iter().chain(&high) {
        let ticks = time * 1.0e9 / 5.0;
        assert!(
            (ticks - ticks.round()).abs() < 1.0e-6,
            "a count transition landed at t={time:e}, off the #5 grid"
        );
    }
    for time in &high {
        assert!(
            low.iter()
                .any(|candidate| (candidate - time).abs() < 1.0e-15),
            "bit one changed at t={time:e} with no co-timed change of bit zero; a vector port \
             publishes as one transition, so every bit that moves moves at one instant"
        );
    }
    assert_eq!(
        high.len(),
        low.len() / 2,
        "bit one must carry every second step of the count: bit zero {low:?}, bit one {high:?}"
    );
}

#[test]
fn a_named_block_counter_keeps_its_state_through_the_engine_route() {
    let local_source = VECTOR_PORT.replace(
        "always #5 count = count + 2'b01;",
        "always #5 begin : accumulator\n\
         reg [1:0] saved = 2'b00;\n\
         saved = saved + 2'b01;\n\
         count = saved;\n\
         end",
    );
    let reference = ModelFile::new("module_counter", VECTOR_PORT);
    let local = ModelFile::new("block_counter", &local_source);
    let expected = run(&vector_deck(&reference, ""), 40.0e-9, 1.0e-9);
    for max_step in [1.0e-9, 0.2e-9] {
        let actual = run(&vector_deck(&local, ""), 40.0e-9, max_step);
        for net in ["count#0", "count#1"] {
            let trace = |result: &TransientResult| {
                result
                    .digital_trace_named(net)
                    .expect("counter trace")
                    .iter()
                    .map(|point| (point.time, point.value))
                    .collect::<Vec<_>>()
            };
            assert_eq!(trace(&actual), trace(&expected), "{net}, step {max_step}");
        }
    }
}

#[test]
fn a_vector_discrete_port_refuses_a_deck_that_names_one_node_for_it() {
    let model = ModelFile::new("vector_short", VECTOR_PORT);
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
        lowered.contains("needing 4 nodes") && lowered.contains("one net per bit"),
        "the refusal must say how many nodes the boundary needs and why: {error}"
    );
}

/// **Selecting one member retains the whole bus.**
///
/// A declaration whose member has no trace cannot say what the bus held, so
/// retention is per bus rather than per node — the rule `TransientCapturePlan`
/// applies, seen from a deck.
#[test]
fn saving_one_bus_member_retains_every_member() {
    let model = ModelFile::new("vector_save", VECTOR_PORT);
    let result = run(&vector_deck(&model, ".save count#0\n"), 40.0e-9, 1.0e-9);
    assert_eq!(
        result.digital_buses.len(),
        1,
        "the saved member's bus must still be declared, saw {:?}",
        result.digital_buses
    );
    assert!(
        result.digital_trace_named("count#1").is_some(),
        "'.save count#0' selected one member; the other must be retained with it, traces are {:?}",
        result
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_discrete_port_with_an_unimplemented_xspice_domain_boundary_is_refused() {
    let model = ModelFile::new("shared_event_net", CLOCK_DIVIDER);
    for (event_card, declaration, kind) in [
        (
            "a_adc [in] [qdiv] adc",
            ".model adc adc_bridge (in_low=0.4 in_high=0.6)",
            "four-state digital",
        ),
        (
            "a_real qdiv observed rg",
            ".model rg real_gain",
            "real-valued",
        ),
    ] {
        let mut previous_error = None;
        for mixed_first in [false, true] {
            let mixed = "x1 p 0 qdiv clock_divider";
            let cards = if mixed_first {
                format!("{mixed}\n{event_card}")
            } else {
                format!("{event_card}\n{mixed}")
            };
            let deck = format!(
                "* loaded or unlike mixed/event boundaries require a conversion contract\n\
                 vin in 0 dc 1\n\
                 r1 in 0 1k\n\
                 {cards}\n\
                 {declaration}\n\
                 rp p 0 1meg\n\
                 rqdiv qdiv 0 1k\n\
                 .va \"{}\" clock_divider\n\
                 .end\n",
                model.deck_path()
            );
            let netlist = Netlist::parse(&deck).unwrap();
            let error = Engine::default()
                .build_circuit(&netlist)
                .unwrap_err()
                .to_string();
            let lowered = error.to_lowercase();
            assert!(
                lowered.contains("event-driven")
                    && lowered.contains("qdiv")
                    && lowered.contains("x1")
                    && lowered.contains(kind),
                "the refusal must identify the actual instance, port, node and event type: {error}"
            );
            if let Some(previous) = previous_error {
                assert_eq!(
                    error, previous,
                    "card order must not change the connection contract"
                );
            }
            previous_error = Some(error);
        }
    }
}

#[test]
fn a_digital_only_module_drives_its_analog_boundary() {
    let model = ModelFile::new(
        "digital_only",
        "module digital_only(q); output q; reg q; initial q=1'b1; endmodule",
    );
    let netlist = Netlist::parse(&format!(
        "* digital-only Verilog device\n.param vcc=3.3\nX1 out digital_only\nR1 out 0 1k\n.va \"{}\" digital_only\n.end\n",
        model.deck_path(),
    )).unwrap();
    let result = Engine::default().run_tran(&netlist, 2e-9, 1e-10).unwrap();
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let expected = 3.3 * 1000.0 / (1000.0 + 20.0);
    assert!(!result.time.is_empty());
    for &voltage in &result.voltages[output] {
        assert!(
            (voltage - expected).abs() < 1e-9,
            "the 20-ohm D/A source must drive the 1-kohm load: {voltage}"
        );
    }
}

#[test]
fn a_portless_mixed_initializer_finishes_without_contribution_equations() {
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    let model = ModelFile::new(
        "portless_init",
        "module portless_init; reg q; initial q=0; always #0 q=~q; analog initial $finish(0); endmodule",
    );
    let netlist = Netlist::parse(&format!(
        "* procedural analog initialization precedes digital execution\nX1 portless_init\n.va \"{}\" portless_init\n.end\n",
        model.deck_path(),
    )).unwrap();
    for ac in [false, true] {
        let outcome = Engine::default()
            .run_with_outcome(&NoAbort, |engine, signal| {
                if ac {
                    engine
                        .run_ac_with_abort(&netlist, &[1e3], signal)
                        .map(|_| ())
                } else {
                    engine
                        .run_tran_with_abort(&netlist, 1e-9, 1e-10, signal)
                        .map(|_| ())
                }
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: None,
            finish,
        } = outcome
        else {
            panic!(
                "the initializer must finish before the non-settling digital process starts: {outcome:?}"
            );
        };
        assert_eq!(finish.point, ModelFinishPoint::Initialization);
        assert_eq!(finish.diagnostic_level, 0);
    }
}

#[test]
fn ac_equilibrium_refuses_portless_mixed_models() {
    let model = ModelFile::new(
        "portless_ac",
        "module portless_ac; reg q; initial q=0; always #1 q=~q; analog $finish(0); endmodule",
    );
    let netlist = Netlist::parse(&format!(
        "* portless mixed model still requires an implemented analysis host\nX1 portless_ac\n.va \"{}\" portless_ac\n.end\n",
        model.deck_path(),
    )).unwrap();
    let error = Engine::default()
        .run_ac(&netlist, &[1e3, 1e4])
        .expect_err("AC must not omit the portless mixed model");
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains("ac analysis") && message.contains("x1"),
        "{error}"
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

// ---------------------------------------------------------------------------
// (e) The boundary value a live consumer sees while the run is still going
// ---------------------------------------------------------------------------

/// The committed digital state at one accepted point: every digital node the
/// sample carries, named through the sample's own node table.
type DigitalState = Vec<(String, DigitalEventCode)>;

/// One point as a live consumer sees it: the accepted analog time paired with
/// the digital state committed at that time.
type BoundarySample = (f64, DigitalState);

/// An abort signal that keeps the committed digital state of every accepted
/// point, with node ids resolved through the sample's own node table.
#[derive(Default)]
struct BoundaryRecorder {
    samples: Mutex<Vec<BoundarySample>>,
}

impl AbortSignal for BoundaryRecorder {
    fn is_aborted(&self) -> bool {
        false
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        let digital = sample
            .digital_values
            .iter()
            .map(|&(node_id, value)| {
                let name = node_id
                    .checked_sub(1)
                    .and_then(|index| sample.node_names.get(index))
                    .cloned()
                    .unwrap_or_else(|| format!("<node {node_id}>"));
                (name, value)
            })
            .collect();
        self.samples
            .lock()
            .expect("boundary recorder")
            .push((sample.time.last().copied().unwrap_or(f64::NAN), digital));
    }
}

/// **D5 lockstep, at the live boundary.** The committed digital value at an
/// accepted analog time is final for that time — the digital wheel has already
/// turned when the point is accepted, and no later step revises it.
///
/// That is what makes one message per accepted point enough for a live view of
/// a mixed run, and it is only true if the value the hook publishes is the
/// value the run's own trace keeps. So the hook's stream is change-compressed
/// exactly the way `record_digital_snapshot` compresses a trace, and the two
/// must be the same history: same values, same times, same order. A hook that
/// ran before the boundary snapshot of its own step would reproduce the same
/// values one accepted step late.
#[test]
fn the_live_hook_publishes_the_boundary_value_the_trace_keeps() {
    let model = ModelFile::new("clock_divider_live_hook", CLOCK_DIVIDER);
    let netlist = Netlist::parse(&divider_deck(&model)).expect("the deck parses");
    let recorder = BoundaryRecorder::default();
    let result = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 200.0e-9, 1.0e-9, &recorder)
        .expect("the deck runs");
    let samples = recorder.samples.into_inner().expect("boundary recorder");

    assert_eq!(
        samples.len(),
        result.time.len(),
        "the hook must fire exactly once per accepted point"
    );
    for (index, (time, _)) in samples.iter().enumerate() {
        assert_eq!(
            time.to_bits(),
            result.time[index].to_bits(),
            "hook call {index} reported t={time:e} for the result's point at t={:e}",
            result.time[index]
        );
    }

    // The hook carries the committed value as its event code, so the trace is
    // encoded the same way before the two histories are compared.
    let recorded: Vec<(f64, DigitalEventCode)> = result
        .digital_trace_named("qdiv")
        .expect("the divided output has a digital trace")
        .iter()
        .map(|point| (point.time, DigitalEventCode(point.value.event_code())))
        .collect();
    assert!(
        recorded.len() > 10,
        "the divider must toggle for this to test anything, its trace is {recorded:?}"
    );

    let mut observed: Vec<(f64, DigitalEventCode)> = Vec::new();
    for (time, digital) in &samples {
        for (name, value) in digital {
            if !name.eq_ignore_ascii_case("qdiv") {
                continue;
            }
            if observed.last().is_some_and(|(_, held)| held == value) {
                continue;
            }
            observed.push((*time, *value));
        }
    }
    assert_eq!(
        observed, recorded,
        "the hook's view of the bridge net is not the history the result kept"
    );
}

// ---------------------------------------------------------------------------
// (f) The bus a live consumer sees, and the two interchange routes it reaches
// ---------------------------------------------------------------------------

/// One declared bus as a live consumer can read it: the name, the declared
/// range, and the member nodes resolved through the sample's own node table.
type NamedBus = (String, i64, i64, Vec<String>);

/// An abort signal that keeps the bus table of every accepted point, with
/// member node ids resolved through the sample's own node table.
#[derive(Default)]
struct BusRecorder {
    tables: Mutex<Vec<Vec<NamedBus>>>,
}

impl AbortSignal for BusRecorder {
    fn is_aborted(&self) -> bool {
        false
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        let table = sample
            .digital_buses
            .iter()
            .map(|bus| {
                let members = bus
                    .members
                    .iter()
                    .map(|node_id| {
                        node_id
                            .checked_sub(1)
                            .and_then(|index| sample.node_names.get(index))
                            .cloned()
                            .unwrap_or_else(|| format!("<node {node_id}>"))
                    })
                    .collect();
                (bus.name.clone(), bus.msb, bus.lsb, members)
            })
            .collect();
        self.tables.lock().expect("bus recorder").push(table);
    }
}

/// **The hook and the result declare one bus, over the same nodes.**
///
/// The sample's table is resolved once when the run's capture plan is compiled
/// and is run-constant thereafter, so a live view can key its columns by node
/// id and never re-resolve. That is only safe if it is the same declaration
/// the finished result publishes — a hook naming other nodes would give a live
/// viewer a bus the saved run does not have.
#[test]
fn the_live_hook_names_the_bus_the_result_declares() {
    let model = ModelFile::new("vector_live_hook", VECTOR_PORT);
    let netlist = Netlist::parse(&vector_deck(&model, "")).expect("the deck parses");
    let recorder = BusRecorder::default();
    let result = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 40.0e-9, 1.0e-9, &recorder)
        .expect("the deck runs");
    let tables = recorder.tables.into_inner().expect("bus recorder");

    assert_eq!(
        tables.len(),
        result.time.len(),
        "the hook must fire exactly once per accepted point"
    );
    let expected: Vec<NamedBus> = result
        .digital_buses
        .iter()
        .map(|bus| (bus.name.clone(), bus.msb, bus.lsb, bus.members.clone()))
        .collect();
    assert_eq!(expected.len(), 1, "the run must declare the boundary's bus");
    for (index, table) in tables.iter().enumerate() {
        assert_eq!(
            *table, expected,
            "the hook's bus table at accepted point {index} is not the run's declaration"
        );
    }
}

/// **The declaration reaches both interchange routes as one vector.**
///
/// Neither route is new here — L1 gave the VCD projection its `$var` and the
/// rawfile its `Digital Bus` plot family — and that is the point: an engine
/// producer had to feed the same declaration those routes already read, not a
/// second one shaped for each. The rawfile is round-tripped rather than
/// inspected, because what matters is that a reader gets the grouping back.
#[test]
fn a_boundary_bus_reaches_the_vcd_and_the_rawfile_as_one_vector() {
    use rspice_core::execution::{decode_event_plots, transient_bus_plots, transient_event_plots};
    use rspice_core::io::{RawFormat, parse_raw_plots_reader_with_limits, write_event_plots};
    use rspice_core::resource::ResourceLimits;

    let model = ModelFile::new("vector_routes", VECTOR_PORT);
    let result = run(&vector_deck(&model, ""), 40.0e-9, 1.0e-9);

    let document = rspice_core::execution::event_vcd_document(
        "tran",
        &result.digital_traces,
        &result.real_traces,
        &result.digital_buses,
    )
    .expect("the run's event histories project onto a VCD document");
    let mut vcd = Vec::new();
    rspice_core::io::write_vcd(&mut vcd, &document).expect("the VCD document writes");
    let vcd = String::from_utf8(vcd).expect("the VCD writer emits UTF-8");
    assert!(
        vcd.contains("$var wire 2 ") && vcd.contains("x1.count [1:0] $end"),
        "the VCD must declare the boundary bus as one two-bit vector:\n{vcd}"
    );
    assert!(
        !vcd.contains("COUNT#1") && !vcd.contains("COUNT#0"),
        "a member of a declared bus is carried by the vector, not beside it:\n{vcd}"
    );

    let mut raw = Vec::new();
    write_event_plots(
        &mut raw,
        &transient_event_plots(&result.digital_traces, &result.real_traces),
        &transient_bus_plots(&result.digital_traces, &result.digital_buses)
            .expect("a two-bit bus over nine events fits the reassembly ceiling"),
        RawFormat::Ascii,
    )
    .expect("the run's event histories write as rawfile plots");
    let file = parse_raw_plots_reader_with_limits(
        &mut std::io::Cursor::new(raw),
        ResourceLimits::default(),
    )
    .expect("the rawfile parses back");
    let decoded = decode_event_plots(&file).expect("the appended plots decode");
    assert_eq!(
        decoded.digital_buses.len(),
        1,
        "the rawfile must carry the one bus the run declared"
    );
    let read_back = &decoded.digital_buses[0];
    assert_eq!(read_back.name, "x1.count");
    assert_eq!((read_back.msb, read_back.lsb), (1, 0));
    assert_eq!(
        read_back.members,
        vec!["COUNT#1".to_string(), "COUNT#0".to_string()]
    );
    assert!(
        decoded
            .digital_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case("count#0")),
        "the member plots are written beside the bus plot, so a reader that never heard of the \
         family still sees every conductor"
    );
}

// ---------------------------------------------------------------------------
// (g) A boundary port whose range is not anchored at zero
// ---------------------------------------------------------------------------

/// `[7:4]` and `[1:0]` on one module: four conductors carrying bits called 7,
/// 6, 5 and 4, and two carrying bits called 1 and 0.
///
/// IEEE 1364-2005 section 3.3.1 lets a vector be declared over any two bounds,
/// and `count[7]` is then a name for the port's most significant bit rather
/// than an offset into it. The boundary refused this declaration for as long as
/// the discrete half read the name as an offset — a `[7:4]` port would have
/// bound four deck nets to four bits the module could not read.
const HIGH_VECTOR_PORT: &str = r#"
`include "disciplines.vams"
module high_vector_mixed(p, n, count, sel, hit);
    inout p, n;
    electrical p, n;
    output [7:4] count;
    reg [7:4] count;
    input [1:0] sel;
    wire [1:0] sel;
    output hit;
    wire hit;
    assign hit = sel[1] & ~sel[0];
    initial count = 4'b0000;
    always #5 count = count + 4'b0001;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

/// The deck a `[7:4]` output and a `[1:0]` input need: one node per bit of
/// each, declared MSB first, in port order.
fn high_vector_deck(model: &ModelFile) -> String {
    format!(
        "* a four-bit output declared [7:4] and a two-bit input declared [1:0]\n\
         x1 p 0 count#7 count#6 count#5 count#4 sel#1 sel#0 hit high_vector_mixed\n\
         rp p 0 1meg\n\
         vsel1 sel#1 0 3.3\n\
         vsel0 sel#0 0 0\n\
         rhit hit 0 1meg\n\
         .va \"{}\" high_vector_mixed\n\
         .tran 1n 80n\n\
         .end\n",
        model.deck_path()
    )
}

/// The value words one declared vector carries, in order.
///
/// Found through the `$var` line's identifier rather than by taking every `b…`
/// line, because a module with two vector ports declares two vectors and their
/// value changes are interleaved in one stream.
fn vcd_bus_words(vcd: &str, reference: &str) -> Vec<String> {
    let identifier = vcd
        .lines()
        .find_map(|line| {
            let rest = line.strip_prefix("$var wire ")?;
            let mut parts = rest.split_whitespace();
            let _width = parts.next()?;
            let identifier = parts.next()?;
            let name = parts.next()?;
            let range = parts.next()?;
            (format!("{name} {range}") == reference).then(|| identifier.to_string())
        })
        .unwrap_or_else(|| panic!("no `$var` declares `{reference}`:\n{vcd}"));
    vcd.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let value = parts.next()?;
            let owner = parts.next()?;
            (value.starts_with('b') && owner == identifier).then(|| value.to_string())
        })
        .collect()
}

/// The last value a boundary net's recorded history holds.
fn final_state(result: &TransientResult, net: &str) -> rspice_core::xspice::DigitalState {
    result
        .digital_trace_named(net)
        .unwrap_or_else(|| panic!("net '{net}' has no digital trace"))
        .last()
        .expect("a recorded trace has at least its opening point")
        .value
        .state
}

/// **A declared index is a name, all the way out to the deck's conductors.**
///
/// The counter runs `0000` to `1111` and wraps, and every step reaches the
/// result as one four-bit word under the range its author wrote. The bits are
/// the ones the module calls 7, 6, 5 and 4 — the deck's first net for the port
/// is its most significant bit, which is what the left declared bound means.
#[test]
fn a_boundary_port_declared_above_bit_zero_bridges_every_bit_it_names() {
    let model = ModelFile::new("high_vector", HIGH_VECTOR_PORT);
    let result = run(&high_vector_deck(&model), 80.0e-9, 1.0e-9);

    let bus = result
        .digital_buses
        .iter()
        .find(|bus| bus.name == "x1.count")
        .unwrap_or_else(|| panic!("no bus for the [7:4] port: {:?}", result.digital_buses));
    assert_eq!((bus.msb, bus.lsb), (7, 4));
    assert_eq!(
        bus.members,
        vec![
            "COUNT#7".to_string(),
            "COUNT#6".to_string(),
            "COUNT#5".to_string(),
            "COUNT#4".to_string(),
        ],
        "members are the deck nodes, declared MSB first"
    );
    assert_eq!(bus.source, DigitalBusSource::Engine);

    // Every bit is a conductor of its own, and the one called 4 is the one
    // that toggles at every step while the one called 7 divides it by eight.
    let low = change_times(&result, "count#4");
    let high = change_times(&result, "count#7");
    assert!(
        low.len() >= 16,
        "the counter must run through a whole cycle, saw {low:?}"
    );
    assert_eq!(
        high.len(),
        low.len() / 8,
        "the bit named 7 is the most significant of four: bit 4 {low:?}, bit 7 {high:?}"
    );

    // The vector input's first deck net is its bit 1, which is the bit `hit`
    // reads: `sel` is `2'b10`, and the module says so.
    assert_eq!(
        final_state(&result, "hit"),
        rspice_core::xspice::DigitalState::One,
        "`hit` is `sel[1] & ~sel[0]`, and the deck drives sel#1 high and sel#0 low"
    );

    // The VCD is where the word order is visible: one four-bit value per
    // change, counting up and wrapping.
    let document = rspice_core::execution::event_vcd_document(
        "tran",
        &result.digital_traces,
        &result.real_traces,
        &result.digital_buses,
    )
    .expect("the run's event histories project onto a VCD document");
    let mut vcd = Vec::new();
    rspice_core::io::write_vcd(&mut vcd, &document).expect("the VCD document writes");
    let vcd = String::from_utf8(vcd).expect("the VCD writer emits UTF-8");
    assert!(
        vcd.contains("$var wire 4 ") && vcd.contains("x1.count [7:4] $end"),
        "the VCD must declare the port under the range its author wrote:\n{vcd}"
    );

    let words = vcd_bus_words(&vcd, "x1.count [7:4]");
    let expected: Vec<String> = (0..16).map(|value| format!("b{value:04b}")).collect();
    assert!(
        words.len() >= expected.len(),
        "the counter must reach 1111, saw {words:?}"
    );
    assert_eq!(
        &words[..expected.len()],
        expected.as_slice(),
        "the port counts 0000 to 1111 as one word"
    );

    // And the rawfile carries the same grouping back.
    let mut raw = Vec::new();
    rspice_core::io::write_event_plots(
        &mut raw,
        &rspice_core::execution::transient_event_plots(&result.digital_traces, &result.real_traces),
        &rspice_core::execution::transient_bus_plots(&result.digital_traces, &result.digital_buses)
            .expect("a four-bit bus over one counter cycle fits the reassembly ceiling"),
        rspice_core::io::RawFormat::Ascii,
    )
    .expect("the run's event histories write as rawfile plots");
    let file = rspice_core::io::parse_raw_plots_reader_with_limits(
        &mut std::io::Cursor::new(raw),
        rspice_core::resource::ResourceLimits::default(),
    )
    .expect("the rawfile parses back");
    let decoded =
        rspice_core::execution::decode_event_plots(&file).expect("the appended plots decode");
    let read_back = decoded
        .digital_buses
        .iter()
        .find(|bus| bus.name == "x1.count")
        .expect("the rawfile carries the [7:4] bus");
    assert_eq!((read_back.msb, read_back.lsb), (7, 4));
    assert_eq!(
        read_back.members,
        vec![
            "COUNT#7".to_string(),
            "COUNT#6".to_string(),
            "COUNT#5".to_string(),
            "COUNT#4".to_string(),
        ]
    );
}

/// The same range written the other way round, where the leading declared
/// index is the *smaller* number and still the most significant bit.
///
/// The two writes are what tell the readings apart: `q[4]` is the port's top
/// bit and `q[7]` its bottom one, so the deck's first net rises first and its
/// last net rises second. A boundary that read a declared index as a position
/// would have bridged neither.
const ASCENDING_VECTOR_PORT: &str = r#"
`include "disciplines.vams"
module ascending_vector_mixed(p, n, q);
    inout p, n;
    electrical p, n;
    output [4:7] q;
    reg [4:7] q;
    initial begin
        q = 4'b0000;
        #5 q[4] = 1'b1;
        #5 q[7] = 1'b1;
    end
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn an_ascending_boundary_port_puts_its_leading_index_on_the_decks_first_net() {
    let model = ModelFile::new("ascending_vector", ASCENDING_VECTOR_PORT);
    let deck = format!(
        "* a four-bit output declared [4:7], leading index first\n\
         x1 p 0 q#4 q#5 q#6 q#7 ascending_vector_mixed\n\
         rp p 0 1meg\n\
         .va \"{}\" ascending_vector_mixed\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    );
    let result = run(&deck, 20.0e-9, 1.0e-9);

    let bus = result
        .digital_buses
        .iter()
        .find(|bus| bus.name == "x1.q")
        .unwrap_or_else(|| panic!("no bus for the [4:7] port: {:?}", result.digital_buses));
    assert_eq!((bus.msb, bus.lsb), (4, 7));
    assert_eq!(
        bus.members,
        vec![
            "Q#4".to_string(),
            "Q#5".to_string(),
            "Q#6".to_string(),
            "Q#7".to_string(),
        ],
        "declaration order is the deck's node order, and it starts at the left bound"
    );

    use rspice_core::xspice::DigitalState;
    assert_eq!(
        final_state(&result, "q#4"),
        DigitalState::One,
        "`q[4]` is the left declared bound, so it is the port's most significant bit"
    );
    assert_eq!(final_state(&result, "q#5"), DigitalState::Zero);
    assert_eq!(final_state(&result, "q#6"), DigitalState::Zero);
    assert_eq!(
        final_state(&result, "q#7"),
        DigitalState::One,
        "`q[7]` is the right declared bound, so it is the port's least significant bit"
    );

    // The two writes land one after the other, five nanoseconds apart, so the
    // word goes 0000, 1000, 1001 — the top bit first.
    let document = rspice_core::execution::event_vcd_document(
        "tran",
        &result.digital_traces,
        &result.real_traces,
        &result.digital_buses,
    )
    .expect("the run's event histories project onto a VCD document");
    let mut vcd = Vec::new();
    rspice_core::io::write_vcd(&mut vcd, &document).expect("the VCD document writes");
    let vcd = String::from_utf8(vcd).expect("the VCD writer emits UTF-8");
    assert!(
        vcd.contains("$var wire 4 ") && vcd.contains("x1.q [4:7] $end"),
        "the VCD must declare the port as `[4:7]`, the range its author wrote:\n{vcd}"
    );
    assert_eq!(
        vcd_bus_words(&vcd, "x1.q [4:7]"),
        vec![
            "b0000".to_string(),
            "b1000".to_string(),
            "b1001".to_string()
        ],
        "the write to `q[4]` moves the leading bit and the write to `q[7]` the trailing one"
    );
}

#[test]
fn linked_circuit_instances_share_precision_and_sample_their_own_analog_inputs() {
    // The coarse process retains its 1 ns delay unit, but every boundary uses
    // the circuit's finest 1 ps precision. Delays start at the resolved analog
    // threshold crossing, independent of the maximum analog step and deck order.
    let source = r#"
`timescale UNIT/PRECISION
module NAME(p, clk, q);
    inout p; electrical p;
    input clk; wire clk;
    output q; reg q;
    initial q = (V(p) > 0.5);
    always @(posedge clk) q <= #DELAY (V(p) > 0.5);
    analog I(p) <+ V(p) / 1000000.0;
endmodule
"#;
    let coarse = ModelFile::new(
        "coarse_sampler",
        &source
            .replace("UNIT", "1ns")
            .replace("PRECISION", "100ps")
            .replace("NAME", "coarse_sampler")
            .replace("DELAY", "0.5"),
    );
    let fine = ModelFile::new(
        "fine_sampler",
        &source
            .replace("UNIT", "1ps")
            .replace("PRECISION", "1ps")
            .replace("NAME", "fine_sampler")
            .replace("DELAY", "25"),
    );
    for reverse in [false, true] {
        let instances = if reverse {
            "Xfine pf clk qf fine_sampler\nXcoarse pc clk qc coarse_sampler"
        } else {
            "Xcoarse pc clk qc coarse_sampler\nXfine pf clk qf fine_sampler"
        };
        let deck = format!(
            "* shared digital precision and analog probe bank\n.param vcc=1\n\
             Vcoarse pc 0 pwl(0 0.25 0.2n 0.25 0.3n 0.75)\n\
             Vfine pf 0 pwl(0 0.75 0.2n 0.75 0.3n 0.25)\n\
             Vclock clk 0 pwl(0 0 0.9n 0 1n 1)\n\
             {instances}\nRc qc 0 1k\nRf qf 0 1k\n\
             .va \"{}\" coarse_sampler\n.va \"{}\" fine_sampler\n.end\n",
            coarse.deck_path(),
            fine.deck_path()
        );
        for max_step in [20e-12, 75e-12] {
            let result = run(&deck, 2e-9, max_step);
            let clock = result.digital_trace_named("clk").unwrap();
            assert_eq!(clock.len(), 2, "one shared A/D edge");
            let delivered = clock[1].time;
            assert!(
                (delivered - 0.95e-9).abs() < 2e-20,
                "the analog threshold crossing must be resolved: {delivered:e}, max_step={max_step:e}"
            );
            assert!(
                (delivered - (delivered / 1e-10).round() * 1e-10).abs() > 1e-11,
                "the fixture must exercise promotion away from the coarse model's tick grid"
            );
            for (node, initial, final_value, delay) in [
                ("qc", 0.0, 1.0 / 1.02, 500e-12),
                ("qf", 1.0 / 1.02, 0.0, 25e-12),
            ] {
                let event_time = ((delivered + delay) / 1e-12).round() * 1e-12;
                let voltage = waveform(&result, node);
                assert!(
                    (voltage[0] - initial).abs() < 1e-8,
                    "{node}: initial analog probe/load"
                );
                assert!(
                    (voltage.last().unwrap() - final_value).abs() < 1e-8,
                    "{node}: final analog probe/load"
                );
                let points = result.digital_trace_named(node).unwrap();
                assert_eq!(
                    points.len(),
                    2,
                    "{node}: one delayed transition, reverse={reverse}"
                );
                assert!(
                    (points[1].time - event_time).abs() < 1e-22,
                    "{node}: expected {event_time:e}, got {:e}, reverse={reverse}",
                    points[1].time
                );
            }
        }
    }
}

#[test]
fn mixed_adc_refines_curved_rising_and_falling_crossings() {
    let model = ModelFile::new(
        "curved_sampler",
        r#"
`timescale 1ps/1ps
module curved_sampler(clk, q);
    input clk; wire clk;
    output q; reg q;
    initial q=0;
    always @(posedge clk) q<=#25 1;
endmodule
"#,
    );
    // The default solver cannot advance by less than 1e-20 s. Root delivery
    // is bounded by that floor plus endpoint roundoff, independently of the
    // 75 ps maximum step and the 1 ps digital tick grid.
    for (offset, rising, falling) in [(0, 1e-9 / 3.0, 2e-9 / 3.0), (1, 1e-9 / 6.0, 5e-9 / 6.0)] {
        let deck = format!(
            "* curved A/D event roots\n.param vcc=1\nVclk clk 0 sin({offset} 1 1g 0 0 -90)\nX1 clk q curved_sampler\nRq q 0 1k\n.va \"{}\" curved_sampler\n.end\n",
            model.deck_path()
        );
        let result = run(&deck, 1e-9, 75e-12);
        let clock = result.digital_trace_named("clk").unwrap();
        assert_eq!(
            clock.len(),
            3,
            "one rising and one falling edge, offset={offset}"
        );
        for (event, expected) in clock.iter().skip(1).zip([rising, falling]) {
            assert!(
                (event.time - expected).abs() < 2e-20,
                "offset={offset}: expected {expected:e}, got {:e}",
                event.time
            );
        }
        let output = result.digital_trace_named("q").unwrap();
        assert_eq!(output.len(), 2);
        let expected = ((rising + 25e-12) / 1e-12).round() * 1e-12;
        assert!((output[1].time - expected).abs() < 1e-22);
    }
}

#[test]
fn direct_hdl_event_bits_resolve_partial_vectors_without_electrical_bridges() {
    use rspice_core::xspice::DigitalState::{HighZ, One, Unknown, Zero};
    let source = ModelFile::new(
        "event_source",
        r#"
`timescale 1ns/1ps
module event_source(q);
 output [7:4] q; reg [7:4] q;
 initial begin q=4'b0011; #1 q=4'b1100; #1 q=4'bz0z1; #1 q=4'bzzzz; end
endmodule
"#,
    );
    let sink = ModelFile::new(
        "event_sink",
        r#"
`timescale 1ns/1ps
module event_sink(d,q);
 input [0:3] d; wire [0:3] d;
 output q; reg q;
 initial q=0;
 always @(d) q <= (d === 4'b1100);
endmodule
"#,
    );
    let other = ModelFile::new(
        "event_other",
        r#"
`timescale 1ns/1ps
module event_other(q);
 output q; wire q; reg enabled;
 initial begin enabled=1; #1.5 enabled=0; #1 enabled=1; #1 enabled=0; end
 assign q=enabled ? 1'b0 : 1'bz;
endmodule
"#,
    );
    for reverse in [false, true] {
        let cards = if reverse {
            "Xs b3 b2 b1 b0 loaded event_sink\nXo b0 event_other\nXd b3 b2 b1 b0 event_source"
        } else {
            "Xd b3 b2 b1 b0 event_source\nXo b0 event_other\nXs b3 b2 b1 b0 loaded event_sink"
        };
        let deck = format!(
            "* event bit graph\n{cards}\nRload loaded 0 1k\n.va \"{}\" event_source\n.va \"{}\" event_sink\n.va \"{}\" event_other\n.end\n",
            source.deck_path(),
            sink.deck_path(),
            other.deck_path()
        );
        let result = run(&deck, 4e-9, 0.1e-9);
        let b0 = result.digital_trace_named("b0").unwrap();
        assert_eq!(
            b0.iter().map(|point| point.value.state).collect::<Vec<_>>(),
            vec![Unknown, Zero, One, Unknown, Zero, HighZ],
            "co-driver contention and release, reverse={reverse}"
        );
        for node in ["b3", "b2", "b1", "b0"] {
            assert!(
                waveform(&result, node)
                    .iter()
                    .all(|voltage| voltage.abs() < 1e-14),
                "{node} must have an empty event placeholder, with no electrical D/A bridge"
            );
            assert_eq!(
                result
                    .digital_trace_named(node)
                    .unwrap()
                    .last()
                    .unwrap()
                    .value
                    .state,
                HighZ
            );
        }
        let output = result.digital_trace_named("loaded").unwrap();
        assert_eq!(
            output
                .iter()
                .map(|point| point.value.state)
                .collect::<Vec<_>>(),
            vec![Zero, One, Zero]
        );
        assert!((output[1].time - 1e-9).abs() < 1e-22);
        assert!((output[2].time - 2e-9).abs() < 1e-22);
        assert!(
            waveform(&result, "loaded")
                .iter()
                .any(|voltage| (voltage - 3.3 / 1.02).abs() < 1e-8),
            "the physical output still drives its 20-ohm/1-kohm load"
        );
    }
}

#[test]
fn a_partly_electrical_input_variable_preserves_its_direct_event_bits() {
    let source = ModelFile::new(
        "partial_source",
        r#"
module partial_source(q); output q; reg q; initial begin q=0; #1 q=1; end endmodule
"#,
    );
    let sink = ModelFile::new(
        "partial_sink",
        r#"
module partial_sink(d,q);
 input [1:0] d; reg [1:0] d;
 output q; reg q; initial q=0;
 always @(d) q <= (d === 2'b11);
endmodule
"#,
    );
    let deck = format!(
        "* input variable across physical and event nodes\nVupper high ref pwl(0 0 0.5n 0 1n 3.3)\nXd low partial_source\nXs high low out partial_sink\nRout out ref 1k\n.va \"{}\" partial_source\n.va \"{}\" partial_sink\n.end\n",
        source.deck_path(),
        sink.deck_path()
    );
    let result = run(&deck, 2e-9, 0.1e-9);
    let output = result.digital_trace_named("out").unwrap();
    assert_eq!(
        output.len(),
        2,
        "one event bit edge after the analog bit has risen"
    );
    assert!((output[1].time - 1e-9).abs() < 1e-22);
    assert!((waveform(&result, "out").last().unwrap() - 3.3 / 1.02).abs() < 1e-8);
    assert!(
        waveform(&result, "low")
            .iter()
            .all(|value| value.abs() < 1e-14)
    );
}

#[test]
fn implicit_ground_preserves_mixed_analog_unknowns_and_event_bus_identities() {
    let source = ModelFile::new(
        "ground_source",
        r#"
module ground_source(p,n,bus);
 inout p,n; electrical p,n,inner;
 output [1:0] bus; reg [1:0] bus;
 initial begin bus=0; #1 bus=(V(inner,n)>1.5) ? 2'b11 : 2'b00; end
 analog begin V(inner,n)<+2; V(p,n)<+3*V(inner,n); end
endmodule
"#,
    );
    let sink = ModelFile::new(
        "ground_sink",
        r#"
module ground_sink(d,q);
 input [1:0] d; wire [1:0] d;
 output q; reg q; initial q=0;
 always @(d) q <= (d === 2'b11);
endmodule
"#,
    );
    for reference in ["0", "ref"] {
        let deck = format!(
            "* ground remap across both domains\nVanchor anchor {reference} 1\nRanchor anchor {reference} 1k\nXsource analog_out {reference} b1 b0 ground_source\nXsink b1 b0 loaded ground_sink\nRout analog_out {reference} 1k\nRload loaded {reference} 1k\n.va \"{}\" ground_source\n.va \"{}\" ground_sink\n.end\n",
            source.deck_path(),
            sink.deck_path()
        );
        let result = run(&deck, 2e-9, 0.1e-9);
        assert!(
            waveform(&result, "analog_out")
                .iter()
                .all(|value| (value - 6.0).abs() < 1e-10)
        );
        assert!((waveform(&result, "loaded").last().unwrap() - 3.3 / 1.02).abs() < 1e-8);
        for node in ["b1", "b0", "loaded"] {
            let points = result.digital_trace_named(node).unwrap();
            assert_eq!(points.len(), 2, "{node}, ground={reference}");
            assert_eq!(
                points[0].value.state,
                rspice_core::xspice::DigitalState::Zero
            );
            assert_eq!(
                points[1].value.state,
                rspice_core::xspice::DigitalState::One
            );
            assert!((points[1].time - 1e-9).abs() < 1e-22);
        }
        let bus = result
            .digital_buses
            .iter()
            .find(|bus| bus.name.eq_ignore_ascii_case("Xsource.bus"))
            .unwrap();
        assert_eq!(bus.members.len(), 2);
        assert!(bus.members[0].eq_ignore_ascii_case("b1"));
        assert!(bus.members[1].eq_ignore_ascii_case("b0"));
    }
}

#[test]
fn direct_hdl_connections_replay_after_rejected_analog_steps() {
    let source = ModelFile::new(
        "event_clock",
        r#"
module event_clock(clk); output clk; reg clk; initial clk=0; always #5 clk=~clk; endmodule
"#,
    );
    let sink = ModelFile::new(
        "event_divider",
        r#"
module event_divider(clk,q); input clk; wire clk; output q; reg q;
 initial q=0; always @(posedge clk) q<=~q;
endmodule
"#,
    );
    let deck = format!(
        "* replay direct HDL connections with an analog RC load\nXclock clk event_clock\nXdivider clk qdiv event_divider\nR1 qdiv out 1k\nC1 out 0 10p\n.va \"{}\" event_clock\n.va \"{}\" event_divider\n.end\n",
        source.deck_path(),
        sink.deck_path()
    );
    // End between clock events: replay is checked over interior activations,
    // independently of a final time rounded on opposite sides of a tick.
    let run_replay = |max_step| {
        let netlist = Netlist::parse(&deck).unwrap();
        let engine = Engine::default();
        let result = engine.run_tran(&netlist, 201e-9, max_step).unwrap();
        (result, engine.convergence_quality().timestep_reductions)
    };
    // Ideal D/A jumps now restart integration directly. Use an interval large
    // enough for the native RC curvature to exercise actual truncation retries.
    let (coarse, rejected) = run_replay(4e-9);
    let (fine, _) = run_replay(2e-11);
    assert!(
        rejected > 0,
        "the candidate rollback must actually be exercised"
    );
    for (node, expected) in [("clk", 41), ("qdiv", 21)] {
        let left = coarse.digital_trace_named(node).unwrap();
        let right = fine.digital_trace_named(node).unwrap();
        assert_eq!(left.len(), expected);
        assert_eq!(left.len(), right.len());
        for (left, right) in left.iter().zip(right) {
            assert_eq!(left.value, right.value);
            assert!((left.time - right.time).abs() < 1e-22);
        }
    }
}

#[test]
fn shared_hdl_xspice_events_replay_off_grid_after_rejected_analog_steps() {
    let source = ModelFile::new(
        "shared_clock",
        r#"
`timescale 1ns/1ps
module shared_clock(clk,future);
 output clk,future; reg clk,future;
 initial clk=0;
 always #5 clk=~clk;
 initial begin future=0; #10.101 future=1; end
endmodule
"#,
    );
    let sink = ModelFile::new(
        "shared_divider",
        r#"
`timescale 1ns/1ps
module shared_divider(fromx,future,q,captured);
 input fromx,future; wire fromx,future;
 output q,captured; reg q,captured; reg seen;
 initial begin q=0; captured=1; seen=0; end
 always @(posedge fromx) begin
   q<=~q;
   if ($realtime>0 && !seen) begin captured<=future; seen<=1; end
 end
endmodule
"#,
    );
    let deck = format!(
        "* shared HDL/XSPICE events with a native RC load\n\
         Xclock clk future shared_clock\n\
         Ainv clk fromx inverter\n\
         .model inverter d_inverter (rise_delay=100.6p fall_delay=100.6p)\n\
         Xdivider fromx future qdiv captured shared_divider\n\
         R1 qdiv out 1k\nC1 out 0 10p\n\
         .va \"{}\" shared_clock\n.va \"{}\" shared_divider\n.end\n",
        source.deck_path(),
        sink.deck_path()
    );
    let run_replay = |max_step| {
        let netlist = Netlist::parse(&deck).unwrap();
        let engine = Engine::default();
        let result = engine.run_tran(&netlist, 201e-9, max_step).unwrap();
        (result, engine.convergence_quality().timestep_reductions)
    };
    let (coarse, rejected) = run_replay(1e-9);
    let (fine, _) = run_replay(2e-11);
    assert!(
        rejected > 0,
        "the native stepper must reject an actual trial"
    );
    for (node, expected) in [
        ("clk", 41),
        ("fromx", 41),
        ("qdiv", 21),
        ("captured", 2),
        ("future", 2),
    ] {
        let left = coarse.digital_trace_named(node).unwrap();
        let right = fine.digital_trace_named(node).unwrap();
        assert_eq!(left.len(), expected, "{node}: {left:?}");
        assert_eq!(left.len(), right.len(), "{node}");
        for (left, right) in left.iter().zip(right) {
            assert_eq!(left.value, right.value, "{node}");
            assert!(
                (left.time - right.time).abs() < 2e-20,
                "{node}: {left:?} vs {right:?}"
            );
        }
    }
    for (index, point) in coarse
        .digital_trace_named("fromx")
        .unwrap()
        .iter()
        .enumerate()
        .skip(1)
    {
        let expected = index as f64 * 5e-9 + 100.6e-12;
        assert!(
            (point.time - expected).abs() < 2e-20,
            "{point:?}, expected {expected:.16e}"
        );
    }
    let captured = coarse.digital_trace_named("captured").unwrap();
    let future = coarse.digital_trace_named("future").unwrap();
    assert_eq!(
        final_state(&coarse, "captured"),
        rspice_core::xspice::DigitalState::Zero
    );
    assert!((captured[1].time - 10.1006e-9).abs() < 2e-20);
    assert!((future[1].time - 10.101e-9).abs() < 2e-20);
    assert!(
        captured[1].time < future[1].time,
        "rounded reporting time must not consume a future timer"
    );
    let out = waveform(&coarse, "out");
    assert!(out.iter().any(|v| *v > 2.0) && out.iter().any(|v| *v < 1.0));
}

#[test]
fn behavioral_voltage_reads_retain_physical_hdl_boundaries() {
    let source = ModelFile::new(
        "observed_bits",
        r#"
module observed_bits(q); output [2:0] q; reg [2:0] q;
 initial begin q=0; #1 q=3'b111; #1 q=0; end
endmodule
"#,
    );
    let deck = format!(
        "* physical observation is a domain boundary\nXbits qv qi free observed_bits\nBvoltage vmirror 0 V=V(qv)\nBcurrent imirror 0 I=V(qi)/1000\nRload imirror 0 1k\n.va \"{}\" observed_bits\n.end\n",
        source.deck_path()
    );
    let result = run(&deck, 3e-9, 0.1e-9);
    for node in ["qv", "qi"] {
        assert!(
            waveform(&result, node)
                .iter()
                .any(|value| (value - 3.3).abs() < 1e-8)
        );
    }
    for (mirror, input, sign) in [("vmirror", "qv", 1.0), ("imirror", "qi", -1.0)] {
        for (actual, input) in waveform(&result, mirror)
            .iter()
            .zip(waveform(&result, input))
        {
            assert!(
                (actual - sign * input).abs() < 1e-8,
                "{mirror} must observe the electrical voltage"
            );
        }
    }
    assert!(
        waveform(&result, "free")
            .iter()
            .all(|value| value.abs() < 1e-14)
    );
    let free = result.digital_trace_named("free").unwrap();
    assert_eq!(free.len(), 3);
    assert_eq!(free[1].value.state, rspice_core::xspice::DigitalState::One);
}

#[test]
fn linked_real_conditional_events_skip_unknown_inputs_and_drive_spice_loads() {
    conditional_event_circuit("select ? level : data");
}

#[test]
fn linked_four_state_conditional_events_skip_unknown_inputs_and_drive_spice_loads() {
    conditional_event_circuit("select ? $realtobits(level) : $realtobits(data + 0.0)");
}

fn conditional_event_circuit(event: &str) {
    let source = ModelFile::new(
        "conditional_source",
        r#"
`timescale 1ns/1ps
module conditional_source(data);
 output [3:0] data; reg [3:0] data;
 initial begin data=4'bx; #1 data=7; end
endmodule
"#,
    );
    let receiver = ModelFile::new(
        "conditional_receiver",
        &r#"
`timescale 1ns/1ps
module conditional_receiver(data,q);
 input [3:0] data; wire [3:0] data;
 output q; reg q;
 parameter real SWITCH=2.0;
 reg select; real level;
 initial begin select=1; level=2.5; q=0;
   @(EVENT_EXPRESSION) q=(select ? level : data)>6.5;
 end
 initial #SWITCH select=0;
endmodule
"#
        .replace("EVENT_EXPRESSION", event),
    );
    let deck = format!(
        "* conditional event programs across linked instances and native loads\n.param vcc=1\nXs d3 d2 d1 d0 conditional_source\nXa d3 d2 d1 d0 qa conditional_receiver SWITCH=2\nXb d3 d2 d1 d0 qb conditional_receiver SWITCH=3\nRa qa 0 1k\nRb qb 0 1k\nCa qa 0 1p\nCb qb 0 1p\n.va \"{}\" conditional_source\n.va \"{}\" conditional_receiver\n.end\n",
        source.deck_path(),
        receiver.deck_path()
    );
    let result = run(&deck, 5e-9, 0.1e-9);
    for (node, time) in [("qa", 2e-9), ("qb", 3e-9)] {
        let events = result.digital_trace_named(node).unwrap();
        assert_eq!(
            events.len(),
            2,
            "{node}: inactive input must not wake the receiver"
        );
        assert!(
            (events[1].time - time).abs() < 1e-22,
            "{node}: {:?}",
            events
        );
        assert!(
            (waveform(&result, node).last().unwrap() - 1.0 / 1.02).abs() < 1e-8,
            "{node}: native RC load must settle to the driven level"
        );
    }
}

#[test]
fn digital_flow_probes_observe_named_sources_and_parallel_branch_identity() {
    sampled_flow_circuit(true);
}

#[test]
fn analog_variable_reads_share_the_candidate_with_spice_loads_and_digital_inputs() {
    let model = ModelFile::new(
        "variable_sampler",
        r#"
`timescale 1ns/1ps
module variable_sampler(p,q);
 inout p; electrical p; output q; reg q;
 parameter real LOAD=1000;
 integer gain; reg signed [7:0] adjustment; reg startup_ok;
 real measured,period; integer count,enabled;
 analog begin
   measured=gain*V(p);
   @(timer(0,period,0,enabled)) count=-3;
   I(p)<+(V(p)-gain+adjustment-255)/1000;
 end
 initial begin
   gain=-2; adjustment=-1; period=1.537e-9; enabled=1; q=0;
   startup_ok=(measured-4*LOAD/(1000+LOAD)<1e-8)
     && (measured-4*LOAD/(1000+LOAD)>-1e-8);
   #1; gain=-4;
   q=startup_ok && (count==-3) && (adjustment==-1)
     && (measured-16*LOAD/(1000+LOAD)<1e-8)
     && (measured-16*LOAD/(1000+LOAD)>-1e-8);
 end
endmodule
"#,
    );
    let deck = format!(
        "* retained variables and coupled discrete inputs\n.param vcc=1\nXa pa qa variable_sampler LOAD=1000\nXb pb qb variable_sampler LOAD=2000\nRa pa 0 1k\nRb pb 0 2k\nRqa qa 0 1k\nRqb qb 0 1k\nCqa qa 0 1p\nCqb qb 0 1p\n.va \"{}\" variable_sampler\n.end\n",
        model.deck_path()
    );
    let result = run(&deck, 2e-9, 0.1e-9);
    let timer_error = result
        .time
        .iter()
        .map(|time| (time - 1.537e-9).abs())
        .fold(f64::INFINITY, f64::min);
    assert!(
        timer_error < 1e-18,
        "event-only period input missed its analog breakpoint by {timer_error:e} s"
    );
    for (node, expected) in [("pa", -2.0), ("pb", -8.0 / 3.0)] {
        assert!(
            (waveform(&result, node).last().unwrap() - expected).abs() < 1e-8,
            "{node}"
        );
    }
    for node in ["qa", "qb"] {
        let events = result.digital_trace_named(node).unwrap();
        assert_eq!(events.len(), 2, "{node}: {events:?}");
        assert_eq!(
            events[1].value.state,
            rspice_core::xspice::DigitalState::One
        );
        assert!((events[1].time - 1e-9).abs() < 1e-22, "{node}: {events:?}");
        assert!(
            (waveform(&result, node).last().unwrap() - 1.0 / 1.02).abs() < 1e-8,
            "{node}"
        );
    }
}

#[test]
fn digital_flow_probes_observe_anonymous_sources_and_reversed_direction() {
    sampled_flow_circuit(false);
}

fn sampled_flow_circuit(named_source: bool) {
    let (source_branch, sense, reverse) = if named_source {
        ("supply", "I(supply)", "-I(<supply>)")
    } else {
        ("p,n", "I(p,n)", "I(n,p)")
    };
    let model = ModelFile::new(
        "flow_sampler",
        &format!(
            r#"
`timescale 1ns/1ps
module flow_sampler(p,n,q);
 inout p,n; electrical p,n;
 output q; reg q;
 branch(p,n) supply,a; branch(n,p) b;
 parameter real LOAD=1000.0;
 real sample_i, reverse_i, a_i, b_i, p_i, n_i, sample_v;
 analog begin
   V({source_branch})<+2.0;
   I(a)<+0.003+ddt(1e-12*V(p,n));
   I(b)<+0.001;
 end
 initial begin
   q=0;
   #1;
   sample_i={sense}; reverse_i={reverse};
   a_i=I(a); b_i=I(<b>); p_i=I(<p>); n_i=I(<n>); sample_v=V(b);
   q=(sample_i+2.0/LOAD+0.002<1e-9) && (sample_i+2.0/LOAD+0.002>-1e-9)
     && (reverse_i+sample_i<1e-9) && (reverse_i+sample_i>-1e-9)
     && (a_i>0.002999999) && (a_i<0.003000001)
     && (b_i>0.000999999) && (b_i<0.001000001)
     && (p_i+2.0/LOAD<1e-9) && (p_i+2.0/LOAD>-1e-9)
     && (n_i-2.0/LOAD<1e-9) && (n_i-2.0/LOAD>-1e-9)
     && (sample_v<-1.999999) && (sample_v>-2.000001);
 end
endmodule
"#
        ),
    );
    let deck = format!(
        "* physical branch samples in linked mixed modules\n.param vcc=1\nXa pa 0 qa flow_sampler LOAD=1000\nXb pb 0 qb flow_sampler LOAD=2000\nRa pa 0 1k\nRb pb 0 2k\nCa pa 0 1p\nCb pb 0 2p\nRqa qa 0 1k\nRqb qb 0 1k\n.va \"{}\" flow_sampler\n.end\n",
        model.deck_path()
    );
    let result = run(&deck, 2e-9, 0.1e-9);
    for node in ["qa", "qb"] {
        let events = result.digital_trace_named(node).unwrap();
        assert_eq!(events.len(), 2, "{node}: {events:?}");
        assert_eq!(
            events[1].value.state,
            rspice_core::xspice::DigitalState::One,
            "{node}"
        );
        assert!((events[1].time - 1e-9).abs() < 1e-22, "{node}: {events:?}");
        assert!(
            (waveform(&result, node).last().unwrap() - 1.0 / 1.02).abs() < 1e-8,
            "{node}"
        );
    }
    for node in ["pa", "pb"] {
        assert!(
            waveform(&result, node)
                .iter()
                .all(|v| (*v - 2.0).abs() < 1e-9),
            "{node}"
        );
    }
}
