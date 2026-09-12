//! Event-domain cases where a Verilog-A/AMS module, an XSPICE code model and
//! the analog solution meet on the same net.
//!
//! Three shapes live here:
//!
//! * bare HDL event nets — a Verilog module with no analog half at all,
//!   driving another HDL module, an XSPICE code model, or an RC load;
//! * the D/A boundary a discrete output crosses when the deck gives it an
//!   analog node, with and without a load on that node;
//! * the six mixed-signal expectations an earlier audit wrote, in their
//!   ORIGINAL unloaded form. The copies that live in
//!   `veriloga_mixed_signal_regressions.rs` each carry a load the original did
//!   not have (`Rload q 0 1e12` and friends), which is exactly the shape the
//!   unloaded-output defect hides behind, so the unloaded originals are kept
//!   here and marked with the lane that owns each remaining failure.
//!
//! Every `#[ignore]` reason starts with its repair lane, so counting the ignore
//! attributes whose reason opens with an `R` lane id across `tri_family_*.rs`
//! counts the open lanes; an ignored case is expected to FAIL under
//! `-- --ignored` until that lane lands.
//!
//! Pinned quantities print as `TRIFAMILY <key>=<value>`; set
//! `RSPICE_TRI_FAMILY_EMIT=1` and run with `--nocapture` to re-measure them all
//! in one pass without asserting any of them.
#![cfg(feature = "veriloga")]

#[path = "common/digital_trace_invariants.rs"]
mod digital_trace_invariants;

use rspice_core::engine::TransientResult;
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

const EMIT_ENV: &str = "RSPICE_TRI_FAMILY_EMIT";

/// `out` of deck E, four nanoseconds after the divider's first rise.
const DECK_E_OUT_EARLY: f64 = 1.214404;
/// `out` of deck E, eight nanoseconds after the divider's first rise.
const DECK_E_OUT_LATE: f64 = 1.888954;
/// First transition of the flop output in deck E3, and the delay every one of
/// its transitions sits behind the HDL clock rise that caused it: the model's
/// 0.1 ns `clk_delay` composed with its own default 1 ns output delay.
const DECK_E3_FIRST_EDGE: f64 = 1.61e-8;
const DECK_E3_FLOP_DELAY: f64 = 1.1e-9;

//=============================================================================
// Shared helpers
//=============================================================================

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ModelFile(std::path::PathBuf);

impl ModelFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_tri_family_events_{}_{}.va",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).expect("write model file");
        Self(path)
    }

    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn emitting() -> bool {
    std::env::var_os(EMIT_ENV).is_some()
}

fn pin_f64(key: &str, observed: f64, expected: f64, tolerance: f64) {
    println!("TRIFAMILY {key}={observed:.6e}");
    if emitting() {
        return;
    }
    assert!(
        (observed - expected).abs() <= tolerance,
        "{key} is {observed:e}, pinned at {expected:e} +/- {tolerance:e}; \
         re-measure with {EMIT_ENV}=1"
    );
}

fn waveform<'a>(result: &'a TransientResult, name: &str) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no analog node {name} in {:?}", result.node_names));
    result.voltages[index].as_slice()
}

/// The first recorded value at or after `time`.
fn value_at(result: &TransientResult, name: &str, time: f64) -> f64 {
    let values = waveform(result, name);
    result
        .time
        .iter()
        .zip(values)
        .find(|(sample, _)| **sample >= time)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| panic!("{name} has no sample at or after {time:e}"))
}

fn trace_points(result: &TransientResult, name: &str) -> Vec<(f64, DigitalState)> {
    result
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| {
            panic!(
                "no digital trace {name} in {:?}",
                result
                    .digital_traces
                    .iter()
                    .map(|trace| trace.node_name.as_str())
                    .collect::<Vec<_>>()
            )
        })
        .points
        .iter()
        .map(|point| (point.time, point.value.state))
        .collect()
}

/// Times at which a trace takes the value one, having not held it before.
fn rising_edges(result: &TransientResult, name: &str) -> Vec<f64> {
    let mut edges = Vec::new();
    let mut previous = None;
    for (time, state) in trace_points(result, name) {
        if state == DigitalState::One && previous != Some(DigitalState::One) && time > 0.0 {
            edges.push(time);
        }
        previous = Some(state);
    }
    edges
}

/// Assert that consecutive rising edges of `name` are `period` apart.
fn assert_period(result: &TransientResult, name: &str, period: f64) {
    let edges = rising_edges(result, name);
    assert!(
        edges.len() >= 3,
        "{name} needs at least three rises to show a period, got {edges:?}"
    );
    for pair in edges.windows(2) {
        let gap = pair[1] - pair[0];
        assert!(
            (gap - period).abs() < 1e-12,
            "{name} rises {gap:e} apart, expected {period:e}; edges {edges:?}"
        );
    }
}

const CLOCK_MODULE: &str =
    "module dclk(q);\n output q; reg q;\n initial q=0;\n always #5 q=~q;\nendmodule\n";
const DIVIDER_MODULE: &str = "module ddiv(clk,q);\n input clk; wire clk; output q; reg q;\n initial q=0; always @(posedge clk) q<=~q;\nendmodule\n";

/// The clock and divider `.va` files both decks E..G share.
fn digital_pair() -> (ModelFile, ModelFile) {
    (ModelFile::new(CLOCK_MODULE), ModelFile::new(DIVIDER_MODULE))
}

fn run(deck: &str, tstop: f64, step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, tstop, step)
        .unwrap_or_else(|error| panic!("{error}\n--- deck ---\n{deck}"));
    digital_trace_invariants::assert_one_digital_value_per_instant(&result, deck);
    result
}

//=============================================================================
// Deck E: a bare HDL event net with an RC load on the output
//=============================================================================

#[test]
fn deck_e_bare_hdl_event_net_divides_the_clock_and_drives_an_rc() {
    let (clock, divider) = digital_pair();
    let result = run(
        &format!(
            "* bare HDL event net into an analog RC\n\
             Xclock clk dclk\n\
             Xdiv clk qdiv ddiv\n\
             R1 qdiv out 1k\n\
             C1 out 0 10p\n\
             .va \"{}\" dclk\n\
             .va \"{}\" ddiv\n\
             .end\n",
            clock.path(),
            divider.path()
        ),
        100e-9,
        0.5e-9,
    );

    // The clock toggles every 5 ns, so it rises every 10 ns; the divider
    // toggles on each of those rises, so it rises every 20 ns.
    assert_period(&result, "clk", 10e-9);
    assert_period(&result, "qdiv", 20e-9);
    let clock_edges = rising_edges(&result, "clk").len();
    let divider_edges = rising_edges(&result, "qdiv").len();
    assert!(
        divider_edges * 2 >= clock_edges - 1 && divider_edges * 2 <= clock_edges + 1,
        "the divider must rise half as often as the clock, got {divider_edges} of {clock_edges}"
    );

    // The analog node charges toward the driven level through R1 and C1, which
    // is a 10.2 ns time constant once the bridge's own 20 ohm source is in it.
    let start = rising_edges(&result, "qdiv")[0];
    let early = value_at(&result, "out", start + 4e-9);
    let late = value_at(&result, "out", start + 8e-9);
    let level = value_at(&result, "qdiv", start + 8e-9);
    assert!(
        early > 0.1 && late > early && late < level,
        "out must climb toward the driven level {level}, got {early} then {late}"
    );
    let tau = 4e-9 / ((level - early) / (level - late)).ln();
    assert!(
        (tau - 10.2e-9).abs() < 3e-9,
        "out must charge with the 10.2 ns RC time constant, fitted {tau:e} \
         from {early} and {late} toward {level}"
    );
    pin_f64("deck_e_out_early", early, DECK_E_OUT_EARLY, 1e-3);
    pin_f64("deck_e_out_late", late, DECK_E_OUT_LATE, 1e-3);
}

//=============================================================================
// Deck E2: HDL clock, XSPICE inverter, HDL divider on shared event nets
//=============================================================================

#[test]
fn deck_e2_hdl_clock_through_an_xspice_inverter_still_divides() {
    let (clock, divider) = digital_pair();
    let result = run(
        &format!(
            "* HDL clock -> XSPICE inverter -> HDL divider\n\
             Xclock clk dclk\n\
             Ainv clk clkb inverter\n\
             .model inverter d_inverter(rise_delay=0.1n fall_delay=0.1n)\n\
             Xdiv clkb qdiv ddiv\n\
             R1 qdiv out 1k\n\
             C1 out 0 10p\n\
             .va \"{}\" dclk\n\
             .va \"{}\" ddiv\n\
             .end\n",
            clock.path(),
            divider.path()
        ),
        100e-9,
        0.5e-9,
    );

    assert_period(&result, "clk", 10e-9);
    assert_period(&result, "clkb", 10e-9);
    assert_period(&result, "qdiv", 20e-9);

    // The inverter's own delay is the only thing between the two event nets.
    let clock_edges = rising_edges(&result, "clk");
    let inverted = rising_edges(&result, "clkb");
    assert!(
        !inverted.is_empty(),
        "the XSPICE inverter published no rises"
    );
    for edge in &inverted {
        let nearest = clock_edges
            .iter()
            .map(|clock_edge| (edge - clock_edge - 5.1e-9).abs())
            .fold(f64::INFINITY, f64::min);
        assert!(
            nearest < 1e-12,
            "clkb must rise 0.1 ns after a clock fall, got {edge:e} against {clock_edges:?}"
        );
    }
}

//=============================================================================
// Deck E3: an HDL clock into an XSPICE flop with HDL data
//=============================================================================

#[test]
fn deck_e3_xspice_flop_follows_the_hdl_divider_through_its_clock_delay() {
    let (clock, divider) = digital_pair();
    let result = run(
        &format!(
            "* HDL clock and HDL data into an XSPICE d_dff\n\
             Xclock clk dclk\n\
             Xdiv clk qdiv ddiv\n\
             Aff qdiv clk d_low d_low q qn dff\n\
             .model dff d_dff(clk_delay=0.1n)\n\
             vlow lowa 0 0\n\
             Aad [lowa] [d_low] adc\n\
             .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
             Ada [q] [qa] dac\n\
             .model dac dac_bridge(out_low=0 out_high=3.3)\n\
             rq qa 0 10k\n\
             .va \"{}\" dclk\n\
             .va \"{}\" ddiv\n\
             .end\n",
            clock.path(),
            divider.path()
        ),
        100e-9,
        0.5e-9,
    );

    assert_period(&result, "qdiv", 20e-9);
    // The flop republishes the divider, so its own output divides too.
    assert_period(&result, "q", 20e-9);

    // Every flop transition is one of the HDL clock's own rises plus the
    // model's 0.1 ns clk_delay, and nothing else.
    let transitions: Vec<f64> = trace_points(&result, "q")
        .into_iter()
        .map(|(time, _)| time)
        .filter(|time| *time > 0.0)
        .collect();
    assert!(
        !transitions.is_empty(),
        "the flop published nothing after its initial value"
    );
    let clock_edges = rising_edges(&result, "clk");
    let offsets: Vec<f64> = transitions
        .iter()
        .map(|time| {
            clock_edges
                .iter()
                .map(|edge| time - edge)
                .filter(|delay| *delay >= 0.0)
                .fold(f64::INFINITY, f64::min)
        })
        .collect();
    for (time, offset) in transitions.iter().zip(&offsets) {
        assert!(
            (offset - DECK_E3_FLOP_DELAY).abs() < 1e-12,
            "flop transition {time:e} sits {offset:e} behind its HDL clock rise, not the \
             0.1 ns clk_delay composed with the model's output delay ({DECK_E3_FLOP_DELAY:e}); \
             clock rises at {clock_edges:?}"
        );
    }
    // The flop captures the value the divider held before the rise, so its
    // first transition is the second clock rise, not the first.
    assert!(
        transitions[0] > clock_edges[1],
        "the flop must publish the pre-edge value of the divider, first transition \
         {:e} against clock rises {clock_edges:?}",
        transitions[0]
    );
    pin_f64(
        "deck_e3_first_edge",
        transitions[0],
        DECK_E3_FIRST_EDGE,
        1e-12,
    );

    // The D/A bridge behind the flop follows it onto the analog node.
    let high = rising_edges(&result, "q")[0];
    assert!(
        value_at(&result, "qa", high + 1e-9) > 3.0,
        "the flop's analog image must follow it high"
    );
}

//=============================================================================
// Deck G: pure digital, with no analog node in the deck at all
//=============================================================================

#[test]
fn deck_g_pure_hdl_deck_runs_with_no_analog_node() {
    let (clock, divider) = digital_pair();
    let result = run(
        &format!(
            "* two HDL modules and nothing else\n\
             Xclock clk dclk\n\
             Xdiv clk qdiv ddiv\n\
             .va \"{}\" dclk\n\
             .va \"{}\" ddiv\n\
             .end\n",
            clock.path(),
            divider.path()
        ),
        100e-9,
        0.5e-9,
    );
    assert!(
        !result.time.is_empty(),
        "a deck with no analog device still has a time axis"
    );
    assert_period(&result, "clk", 10e-9);
    assert_period(&result, "qdiv", 20e-9);
}

//=============================================================================
// The D/A boundary, with and without a load
//=============================================================================

const TIMER_MODULE: &str =
    "module t4(q);\n output q; reg q;\n initial begin q=0; #4 q=1; end\nendmodule\n";

fn timer_deck(load: &str) -> (ModelFile, String) {
    let model = ModelFile::new(TIMER_MODULE);
    let deck = format!(
        "* a discrete output on a deck node\nX1 q t4\n.va \"{}\" t4\n{load}.end\n",
        model.path()
    );
    (model, deck)
}

#[test]
fn a_loaded_discrete_output_reaches_the_supply_level() {
    for (label, load) in [
        ("10k", "rq q 0 10k\n"),
        ("1G", "rq q 0 1G\n"),
        ("1p", "cq q 0 1p\n"),
    ] {
        let (_model, deck) = timer_deck(load);
        let result = run(&deck, 6e-9, 0.05e-9);
        let before = value_at(&result, "q", 1e-9);
        let after = value_at(&result, "q", 5e-9);
        assert!(
            before.abs() < 0.05,
            "{label}: the output is low before 4 ns, got {before}"
        );
        assert!(
            (after - 3.3).abs() < 0.02,
            "{label}: a driven output must read the 3.3 V supply level, got {after}"
        );
    }
}

#[test]
fn an_unloaded_discrete_output_reaches_the_supply_level() {
    let (_model, deck) = timer_deck("");
    let result = run(&deck, 6e-9, 0.05e-9);
    let after = value_at(&result, "q", 5e-9);
    assert!(
        (after - 3.3).abs() < 0.02,
        "a driven output must read 3.3 V whether or not the deck loads it, got {after}"
    );
}

//=============================================================================
// The earlier audit's six expectations, in their original unloaded form
//=============================================================================

#[test]
fn unloaded_scheduled_digital_read_uses_analog_value_at_its_own_time() {
    let model = ModelFile::new(
        "module sample_at_ten(p,q);\n input p; electrical p;\n output q; reg q;\n \
         initial begin q=0; #10 q=(V(p)>0.999); end\n analog I(p)<+0;\nendmodule\n",
    );
    let result = run(
        &format!(
            "* sample a known ramp at 10 ns\nV1 p 0 PWL(0 0 20n 2)\nX1 p q sample_at_ten\n\
             .va \"{}\" sample_at_ten\n.end\n",
            model.path()
        ),
        12e-9,
        0.2e-9,
    );
    let q = *waveform(&result, "q").last().unwrap();
    assert!(
        q > 3.0,
        "V(p) is 1 V at 10 ns, so the sampled comparison is true; final q={q}"
    );
}

#[test]
fn unloaded_digital_variable_drives_analog_equation() {
    let model = ModelFile::new(
        "module digital_to_analog(out);\n output out; electrical out;\n reg state;\n \
         initial begin state=0; #1 state=1; end\n analog V(out)<+state;\nendmodule\n",
    );
    let result = run(
        &format!(
            "* a discrete variable inside an analog contribution\nX1 out digital_to_analog\n\
             .va \"{}\" digital_to_analog\n.end\n",
            model.path()
        ),
        3e-9,
        0.2e-9,
    );
    let out = *waveform(&result, "out").last().unwrap();
    assert!(
        (out - 1.0).abs() < 1e-9,
        "state=1 must contribute 1 V, got {out}"
    );
}

#[test]
fn unloaded_mixed_instance_parameters_are_executable() {
    let model = ModelFile::new(
        "module param_device(p);\n inout p; electrical p;\n \
         parameter real resistance=1000;\n reg state; initial state=0;\n \
         analog I(p)<+V(p)/resistance;\nendmodule\n",
    );
    let result = run(
        &format!(
            "* an instance parameter on a module with a discrete half\nV1 p 0 1\n\
             X1 p param_device resistance=2000\n.va \"{}\" param_device\n.end\n",
            model.path()
        ),
        2e-9,
        0.2e-9,
    );
    // 1 V across a 2 kohm authored conductance is 0.5 mA out of the source.
    let branch = result
        .branch_names
        .iter()
        .position(|name| name.to_ascii_uppercase().contains("V1"))
        .unwrap_or_else(|| panic!("no V1 branch in {:?}", result.branch_names));
    let current = *result.branch_currents[branch]
        .last()
        .expect("the source branch has a waveform");
    assert!(
        (current.abs() - 0.5e-3).abs() < 1e-9,
        "the overridden 2 kohm must reach the analog half, source current {current:e}"
    );
}

#[test]
fn unloaded_high_impedance_output_releases_the_analog_net() {
    let model = ModelFile::new(
        "module released_output(p,q);\n input p; electrical p;\n output q; reg q;\n \
         initial q=1'bz;\n analog I(p)<+0;\nendmodule\n",
    );
    let result = run(
        &format!(
            "* pull up a released output\nV1 p 0 3.3\nRpull p q 1000\nX1 p q released_output\n\
             .va \"{}\" released_output\n.end\n",
            model.path()
        ),
        2e-9,
        0.2e-9,
    );
    let q = *waveform(&result, "q").last().unwrap();
    assert!(
        (q - 3.3).abs() < 1e-6,
        "a released output with a 1 kohm pull-up must reach 3.3 V, got {q}"
    );
}

#[test]
fn unloaded_crossing_does_not_execute_an_unrelated_future_timer_early() {
    let model = ModelFile::new(
        "module future_timer(p,clk,q);\n input p; electrical p;\n input clk; wire clk;\n \
         output q; reg q;\n reg seen;\n initial begin q=0; #4 q=1; end\n initial seen=0;\n \
         always @(posedge clk) seen=1;\n analog I(p)<+0;\nendmodule\n",
    );
    let result = run(
        &format!(
            "* a clock crossing must not pull an unrelated timer forward\nV1 p 0 1\n\
             Vclk clk 0 PWL(0 0 3.5n 0 3.6n 3.3 6n 3.3)\nX1 p clk q future_timer\n\
             .va \"{}\" future_timer\n.end\n",
            model.path()
        ),
        5e-9,
        0.05e-9,
    );
    let early: Vec<(f64, f64)> = result
        .time
        .iter()
        .zip(waveform(&result, "q"))
        .filter(|(time, value)| **time < 4e-9 - 1e-18 && **value > 3.0)
        .map(|(time, value)| (*time, *value))
        .collect();
    assert!(
        early.is_empty(),
        "an unrelated #4 timer must not drive the circuit before 4 ns, got {:?}",
        early.iter().take(4).collect::<Vec<_>>()
    );
}

#[test]
fn unloaded_connect_rules_survive_a_verilog_include_wrapper() {
    let mut source = String::from(
        "module rules_device(p,clk,q);\n input p; electrical p;\n input clk; wire clk;\n \
         output q; wire q;\n assign q=clk;\n analog I(p)<+0;\nendmodule\n",
    );
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(
        "\nconnectrules supply_rules;\nconnect a2d #(.vsup(1.0));\nconnect d2a #(.vsup(1.0));\nendconnectrules\n",
    );
    let direct = ModelFile::new(&source);
    let wrapper = ModelFile::new(&format!("`include \"{}\"\n", direct.path()));
    let final_q = |model: &ModelFile| {
        let result = run(
            &format!(
                "* the selected connect rules must survive an include\nV1 p 0 1\nVclk clk 0 1\n\
                 X1 p clk q rules_device\n.va \"{}\" rules_device\n.end\n",
                model.path()
            ),
            2e-9,
            0.2e-9,
        );
        *waveform(&result, "q").last().unwrap()
    };
    let direct_q = final_q(&direct);
    let wrapped_q = final_q(&wrapper);
    assert!(
        (direct_q - 1.0).abs() < 1e-9,
        "the selected rules convert at 1 V, got {direct_q}"
    );
    assert!(
        (wrapped_q - direct_q).abs() < 1e-9,
        "an include wrapper must not discard the selected connect rules; \
         direct {direct_q}, wrapped {wrapped_q}"
    );
}
