//! One transient deck per case, each carrying all three device families at
//! once: native SPICE primitives, XSPICE code models on event nets, and
//! Verilog-A/AMS modules on the runtime — the generated `diode_cmc` card too
//! where the build has it.
//!
//! Nothing else in the tree runs the three families together, so the cases
//! here are the only place a change that is correct for one family and wrong
//! for another shows up. Two kinds of case live in this file:
//!
//! * live assertions, which pin behaviour that is correct today, and
//! * `#[ignore]`d assertions, which state the CORRECT expectation for a defect
//!   that is open. Every ignore reason starts with the repair lane that owns
//!   it, so counting the ignore attributes whose reason opens with an `R` lane
//!   id across `tri_family_*.rs` counts the open lanes. An ignored case here is
//!   expected to FAIL under `-- --ignored` until its lane lands; when it
//!   passes, delete the attribute.
//!
//! # The sequence golden, and what it is worth
//!
//! Deck A pins a point count and an FNV-1a hash over the accepted time grid
//! and over every node waveform. A golden of this kind proves self-agreement
//! and nothing else: it says the route reproduces its own answer, not that the
//! answer is physically right. Read it as a tripwire for an unintended change
//! of the accepted grid, never as an oracle.
//!
//! ## Regenerating the goldens
//!
//! Set `RSPICE_TRI_FAMILY_EMIT=1` and run the file with `--nocapture`. Every
//! pinned quantity prints as `TRIFAMILY <key>=<value>` and no pinned equality
//! is asserted, so one run prints the complete set for the route it was built
//! with. Copy the printed values into the constants below — per route, because
//! the interpreter and the x64 JIT are separate routes and may land on
//! different grids — and re-run without the variable.
#![cfg(feature = "veriloga")]

#[path = "common/digital_trace_invariants.rs"]
mod digital_trace_invariants;

use rspice_core::engine::TransientResult;
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};
use std::collections::BTreeMap;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

//=============================================================================
// Pinned values, measured on the lane base
//=============================================================================

/// Environment variable that turns every pinned comparison into a print.
const EMIT_ENV: &str = "RSPICE_TRI_FAMILY_EMIT";

/// Whether the generated `diode_cmc` card is in this build. The deck carries
/// the card only when it is, so the goldens below are the with-card numbers
/// and are asserted only in that configuration.
const DIODE_CMC: bool = cfg!(feature = "veriloga-model-diode-cmc");

/// Deck A's accepted grid, re-measured after R2.24 ended the last of the three
/// retry ladders the deck carried. 1458 points before R2.14, 1270 after it and
/// 1132 now, and every point either change stopped taking is accounted for
/// instant by instant in the trace.
///
/// R2.14 ended two of the ladders:
///
/// * the mixed module's D/A output left the voltage-LTE norm, which ended the
///   six ladders at its own edges (0.05, 10.05 … 50.05 ns; 54/73/56/73/55/73
///   rejections, and the 50 accepted points below ten femtoseconds they left
///   behind while doubling back up), and
/// * the native estimator now restarts predictor history at a breakpoint,
///   which ended the twelve ladders at the far end of each `y` ramp (x.x65 ns;
///   51-68 rejections and 54 more sub-femtosecond points).
///
/// R2.24 ended the third, which was that restart's own first step: one point of
/// history predicts a constant at any width, so the estimate there was the
/// step's own increment and the width that satisfied it was
/// `(abstol + reltol·|v|)/slope` rather than anything the physics asked for.
/// All 1035 rejections the deck had left were that one step, all of them on
/// `p`, and they split by the level the `y` ramp left:
///
/// * 458 on a ramp leaving 3.3 V, where `|v|` is 1.65 V and the demand is
///   1.0006e-13 s. One timepoint spent 46 retries walking 2.42e-13 s down to
///   it, because the resize law aimed at the tolerance instead of inside it,
///   and left 3 accepted points per ramp — 18 over the run.
/// * 577 on a ramp leaving 0 V, where `|curr − pred|` and the reference
///   `max(|curr|, |pred|)` are the same number, so the reference floors at
///   `abstol/reltol` and the demand is `abstol/slope` = 6.07e-17 s. These were
///   every one of the 78 accepted steps below ten femtoseconds, and left 20
///   accepted points per ramp — 120 over the run.
///
/// Both corners are now taken at the controller's own proposal, 2.42e-13 s and
/// 1.70e-13 s, and 1.697e-13 s is the smallest step the whole run accepts.
///
/// Nothing else in this file moved: deck C2 keeps its pinned point count, deck
/// E its sampled voltages, and deck A its pinned analog solution at every
/// digital instant.
const DECK_A_POINTS: usize = 1132;
const DECK_A_GRID_HASH: u64 = 0x9485_9e98_f8c7_6df9;
const DECK_A_VOLT_HASH: u64 = 0x5cda_4227_0d10_7925;
const DECK_C2_POINTS: usize = 265;

/// Deck A's `d_clk` transitions, as the route dated them BEFORE the R2.2
/// refinement fix moved the accepted grid.
///
/// The grid is not the physics. Every quantity below is measured on the
/// pre-fix run and re-asserted after it, so a change that moves the accepted
/// timepoints has to leave the analog solution where it found it.
const DECK_A_DIGITAL_INSTANTS: &[f64] = &[
    1.05e-9,
    1.052501199058232e-9,
    6.050000000000001e-9,
    6.052545454545457e-9,
    1.1050000000000003e-8,
    1.10528973134341e-8,
    1.6050000000000005e-8,
    1.605254545454546e-8,
    2.1050000000000003e-8,
    2.105186424774527e-8,
    2.6050000000000005e-8,
    2.605254545454546e-8,
    3.105000000000001e-8,
    3.105289731343411e-8,
    3.6050000000000015e-8,
    3.605254545454548e-8,
    4.105000000000001e-8,
    4.105186424774528e-8,
    4.605000000000001e-8,
    4.6052545454545474e-8,
    5.1050000000000004e-8,
    5.1052897313434104e-8,
    5.605000000000001e-8,
    5.605254545454547e-8,
];

/// The nodes sampled at those instants, and how long after each instant.
///
/// Two and a half nanoseconds, not zero, and not half a nanosecond either.
/// `y` and `q` are D/A outputs: at the instant of an edge their value is a
/// step, and half a nanosecond later they are on the ramp the inverter's
/// `rise_delay` puts there, so a comparison at either instant would be
/// measuring which side of a moving corner the sample fell on rather than
/// whether the solution moved. Half the clock's half-period is past every
/// delay in the deck — the 0.5 ns inverter, the 0.1 ns `t_rise`, the mixed
/// module's own tick — and before the next edge, so all four nodes are flat
/// there and interpolating between two accepted points is exact whatever grid
/// they came from.
const DECK_A_INSTANT_NODES: [&str; 4] = ["A", "Y", "P", "Q"];
const DECK_A_INSTANT_OFFSET: f64 = 2.5e-9;

/// `DECK_A_INSTANT_NODES` sampled `DECK_A_INSTANT_OFFSET` after each of
/// `DECK_A_DIGITAL_INSTANTS`, four values per instant, measured pre-fix.
#[rustfmt::skip]
const DECK_A_INSTANT_VOLTAGES: &[f64] = &[
    0.6764946967126333, 0.0, 0.0, 3.2934131736526284,
    0.6764946967126333, 0.0, 0.0, 3.2934131736526293,
    0.6764946967126335, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126335, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126335, 0.0, 0.0, 0.0,
    0.6764946967126334, 0.0, 0.0, 0.0,
    0.6764946967126335, 3.3, 1.6499999999991748, 0.0,
    0.6764946967126335, 3.3, 1.6499999999991748, 0.0,
    0.6764946967126333, 0.0, 0.0, 3.293413173652629,
    0.6764946967126333, 0.0, 0.0, 3.293413173652629,
    0.6764946967126335, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126335, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126333, 0.0, 0.0, 0.0,
    0.6764946967126333, 0.0, 0.0, 0.0,
    0.6764946967126334, 3.3, 1.6499999999991748, 0.0,
    0.6764946967126334, 3.3, 1.6499999999991748, 0.0,
    0.6764946967126333, 0.0, 0.0, 3.293413173652629,
    0.6764946967126333, 0.0, 0.0, 3.293413173652629,
    0.6764946967126334, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126334, 3.3, 1.6499999999991748, 3.293413173652629,
    0.6764946967126333, 0.0, 0.0, 0.0,
    0.6764946967126333, 0.0, 0.0, 0.0,
    0.6764946967126334, 3.3, 1.6499999999991748, 0.0,
    0.6764946967126334, 3.3, 1.6499999999991748, 0.0,
];

//=============================================================================
// Shared helpers
//=============================================================================

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` at a unique path, removed when the guard drops. The engine's
/// Verilog-A cache is keyed by canonical path, so a shared filename would be a
/// shared cache entry between cases.
struct ModelFile(std::path::PathBuf);

impl ModelFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_tri_family_transient_{}_{}.va",
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

fn pin_usize(key: &str, observed: usize, expected: usize) {
    println!("TRIFAMILY {key}={observed}");
    if emitting() {
        return;
    }
    assert_eq!(
        observed, expected,
        "{key} moved off its pinned value; re-measure with {EMIT_ENV}=1"
    );
}

fn pin_hash(key: &str, observed: u64, expected: u64) {
    println!("TRIFAMILY {key}={observed:016x}");
    if emitting() {
        return;
    }
    assert_eq!(
        observed, expected,
        "{key} moved off its pinned value ({observed:016x} vs {expected:016x}); \
         re-measure with {EMIT_ENV}=1"
    );
}

/// FNV-1a over the raw bits of a float sequence.
fn sequence_hash(values: impl Iterator<Item = f64>) -> u64 {
    let mut hash: u64 = 1469598103934665603;
    for value in values {
        hash ^= value.to_bits();
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

/// Every node waveform keyed by upper-case name, so two decks that allocate
/// their nodes in different orders still compare.
fn waveforms_by_name(result: &TransientResult) -> BTreeMap<String, Vec<f64>> {
    result
        .node_names
        .iter()
        .zip(&result.voltages)
        .map(|(name, values)| (name.to_ascii_uppercase(), values.clone()))
        .collect()
}

/// Every digital trace keyed by upper-case node name, each point rendered as
/// `time=value` so a comparison needs no ordering assumption.
fn traces_by_name(result: &TransientResult) -> BTreeMap<String, Vec<String>> {
    result
        .digital_traces
        .iter()
        .map(|trace| {
            (
                trace.node_name.to_ascii_uppercase(),
                trace
                    .points
                    .iter()
                    .map(|point| format!("{:e}={:?}", point.time, point.value))
                    .collect(),
            )
        })
        .collect()
}

/// One node's waveform, linearly interpolated at `time`.
///
/// Two runs on different accepted grids have no sample in common, so a
/// comparison between them has to interpolate; the decks below are sampled
/// where they are settled, so the interpolation error is the waveform's own
/// curvature over one step and not a step-sized jump.
fn sample_at(result: &TransientResult, name: &str, time: f64) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no analog node {name} in {:?}", result.node_names));
    let values = &result.voltages[index];
    let position = result.time.partition_point(|sample| *sample < time);
    if position == 0 {
        return values[0];
    }
    if position >= result.time.len() {
        return *values.last().expect("a node has at least one sample");
    }
    let (before, after) = (result.time[position - 1], result.time[position]);
    let span = after - before;
    // A span that is not strictly positive — zero, negative or NaN — leaves
    // nothing to interpolate across, so the later sample stands.
    if span > 0.0 {
        let weight = (time - before) / span;
        values[position - 1] * (1.0 - weight) + values[position] * weight
    } else {
        values[position]
    }
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

//=============================================================================
// Deck A: the tri-family deck
//=============================================================================

const MIX_DIV: &str = r#"
`include "disciplines.vams"
module mix_div(p, n, clk, q);
    inout p, n; electrical p, n;
    input clk; output q; wire clk; reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= ~q;
    analog I(p,n) <+ V(p,n)/1000.0;
endmodule
"#;

const DECK_A_TSTOP: f64 = 60e-9;
const DECK_A_STEP: f64 = 0.1e-9;

/// The analog half of deck A's Verilog-A device: a resistor, a charge, and a
/// `$bound_step` so the deck also carries the stepper-bound path.
fn bounded_rc_module() -> String {
    "`include \"disciplines.vams\"\n\
     module m(p, n);\n\
     \x20inout p, n; electrical p, n;\n\
     \x20parameter real r = 1k from (0:inf);\n\
     \x20parameter real c = 1p from [0:inf);\n\
     \x20analog begin\n\
     \x20 I(p,n) <+ V(p,n)/r + ddt(c*V(p,n));\n\
     \x20 $bound_step(2n);\n\
     \x20end\n\
     endmodule\n"
        .to_string()
}

/// Deck A. A DC rail feeds a native R/C/diode node that also carries a
/// Verilog-A conductance and, where the build has it, the generated
/// `diode_cmc` card; an XSPICE adc/inverter/dac chain turns the clock into an
/// event net and back into an analog level; and a mixed Verilog-AMS module
/// divides that clock in its discrete half while conducting in its analog
/// half.
fn deck_a() -> (Vec<ModelFile>, Netlist) {
    let analog = ModelFile::new(&bounded_rc_module());
    let mixed = ModelFile::new(MIX_DIV);
    let generated = if DIODE_CMC { "xd2 a 0 diode_cmc\n" } else { "" };
    let deck = format!(
        "* tri-family: native SPICE, XSPICE code models, Verilog-A and Verilog-AMS\n\
         .param vcc=3.3\n\
         vdd vdd 0 dc 3.3\n\
         vclk clk 0 pulse(0 3.3 0 0.1n 0.1n 4.9n 10n)\n\
         a_adc [clk] [d_clk] adc\n\
         .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
         a_inv d_clk d_inv inv\n\
         .model inv d_inverter(rise_delay=0.5n fall_delay=0.5n)\n\
         a_dac [d_inv] [y] dac\n\
         .model dac dac_bridge(out_low=0 out_high=3.3 t_rise=0.1n t_fall=0.1n)\n\
         ry y 0 10k\n\
         r1 vdd a 1k\n\
         c1 a 0 1p\n\
         d1 a 0 dnat\n\
         .model dnat d(is=1e-14)\n\
         {generated}\
         xva a 0 m r=2k c=0.5p\n\
         .va \"{}\" m\n\
         rmix y p 1k\n\
         xmix p 0 clk q mix_div\n\
         .va \"{}\" mix_div\n\
         rq q 0 10k\n\
         .end\n",
        analog.path(),
        mixed.path()
    );
    let netlist = Netlist::parse(&deck).expect("deck A parses");
    (vec![analog, mixed], netlist)
}

fn run_deck_a() -> TransientResult {
    let (_models, netlist) = deck_a();
    let result = Engine::default()
        .run_tran(&netlist, DECK_A_TSTOP, DECK_A_STEP)
        .expect("deck A transient");
    digital_trace_invariants::assert_one_digital_value_per_instant(&result, "deck A");
    result
}

/// Point count and both sequence hashes, in the order the goldens list them.
fn deck_a_fingerprint(result: &TransientResult) -> (usize, u64, u64) {
    (
        result.time.len(),
        sequence_hash(result.time.iter().copied()),
        sequence_hash(result.voltages.iter().flatten().copied()),
    )
}

#[test]
fn deck_a_tri_family_transient_sequence_golden() {
    let result = run_deck_a();
    let (points, grid, voltages) = deck_a_fingerprint(&result);

    // Structure first: the golden is meaningless if the run lost a family.
    assert!(
        result
            .digital_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case("d_clk")),
        "the XSPICE event net must reach the result, got {:?}",
        result
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        result
            .digital_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case("q")),
        "the mixed module's discrete output must reach the result"
    );
    assert!(
        (result.time.last().copied().unwrap_or(0.0) - DECK_A_TSTOP).abs() < DECK_A_STEP,
        "the run must reach tstop, ended at {:?}",
        result.time.last()
    );
    assert!(
        result.time.windows(2).all(|pair| pair[1] > pair[0]),
        "the accepted grid must be strictly increasing"
    );

    if !DIODE_CMC {
        // Without the generated card the deck is a different circuit, so the
        // pinned sequence does not apply; the structure above still does.
        println!("TRIFAMILY deck_a_points_no_diode_cmc={points}");
        return;
    }
    pin_usize("deck_a_points", points, DECK_A_POINTS);
    pin_hash("deck_a_grid_hash", grid, DECK_A_GRID_HASH);
    pin_hash("deck_a_volt_hash", voltages, DECK_A_VOLT_HASH);
}

/// A digital edge must not restart the analog stepper anywhere near its floor.
///
/// The bound is ten femtoseconds rather than the picosecond this case was
/// first written with, and the difference is the deck rather than the defect:
/// ngspice's own restart policy is a tenth of the approach step capped by the
/// gap to the next breakpoint, and `a_adc` publishes `in_low` at 1.6 V and
/// `in_high` at 1.7 V, which on a 0.1 ns 3.3 V ramp are 2.5 ps apart, so the
/// restart between that pair is 0.25 ps by the rule every native deck uses.
/// Demanding a picosecond here would be demanding a different breakpoint policy
/// for every deck in the engine. Ten femtoseconds is six doublings above this
/// deck's hard floor and twenty-five times below the smallest legitimate
/// restart it takes, so it separates a ladder from a policy cleanly.
///
/// # The three mechanisms this bound has caught, measured on this deck
///
/// Two were node- and history-scoped, and
/// `deck_a_takes_a_controller_sized_step_after_every_digital_edge` pins both
/// live: the mixed module's D/A output inside the voltage-LTE norm, and
/// predictor history carried across a breakpoint.
///
/// The third was the first step after that breakpoint restart, and it owned all
/// 78 of the accepted steps this case used to find, every one of them at a `y`
/// ramp *leaving* zero volts — 6.55, 16.55 … 56.55 ns, thirteen each. There the
/// predictor is the single point it restarted from, so it holds `p` at 0 V at
/// every width while the candidate has `p` = 1.65e10·dt: `|curr − pred|` and
/// the reference are the same number, the reference floors at `abstol/reltol`
/// = 1e-3 V, and the estimate is `x/(1 + x)` with `x` = 1.65e10·dt/1e-3, which
/// reaches `reltol` at dt = 6.07e-17 s — thirteen doublings below this bound.
/// It was the estimator's own arithmetic rather than a stale-history artefact,
/// and any native `PULSE` into a divider crosses the same regime: a constant
/// predictor cannot agree with a ramp at any width, so the number it produces
/// bounds nothing. R2.24 stopped forming it, and the step after a restart is
/// now accepted on Newton convergence and the ngspice device truncation limits
/// instead — which is what `CKTterr` does, charge and flux states only.
#[test]
fn deck_a_accepts_no_step_near_the_solver_floor_after_a_digital_edge() {
    const FLOOR_LADDER_BOUND: f64 = 1e-14;
    let result = run_deck_a();
    let tiny: Vec<(usize, f64, f64)> = result
        .time
        .windows(2)
        .enumerate()
        .map(|(index, pair)| (index + 1, pair[0], pair[1] - pair[0]))
        .filter(|(_, _, step)| *step < FLOOR_LADDER_BOUND)
        .collect();
    let smallest = result
        .time
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .fold(f64::INFINITY, f64::min);
    println!(
        "TRIFAMILY deck_a_floor_steps={} smallest={smallest:e}",
        tiny.len()
    );
    assert!(
        tiny.is_empty(),
        "a digital edge must not restart the analog stepper near its floor; \
         {} accepted steps are below {FLOOR_LADDER_BOUND:e}, first few {:?}",
        tiny.len(),
        tiny.iter().take(6).collect::<Vec<_>>()
    );
}

/// The two retry ladders R2.14 closed, pinned where each of them happened.
///
/// Deck A's mixed module drives `q` from its discrete half, and every flip used
/// to cost seventy-odd rejections of one timepoint: the D/A output was inside
/// the generic voltage-LTE norm, so the predictor carried the pre-edge slope
/// across the edge and reported the jump as truncation error at every candidate
/// width, down to 1.1e-18 s. The same stale history at the far end of each `y`
/// ramp cost sixty-odd more. Both are node- and breakpoint-scoped facts about
/// the accepted grid, and this case states them as such.
///
/// A picosecond is the separator. The controller's own first proposal after an
/// edge here is 2.4 ps; the width-gated acceptance floor R2.14 first proposed
/// would have accepted 0.125 ps at the same instant and called the ladder gone,
/// so a bound between the two is what tells "ended" from "masked". The second
/// assertion is what a masked ladder cannot satisfy at all: the edge is taken
/// whole, in the one accepted point that lands on it, at the level `q` still
/// holds 2.5 ns later.
#[test]
fn deck_a_takes_a_controller_sized_step_after_every_digital_edge() {
    const CONTROLLER_STEP_BOUND: f64 = 1e-12;
    const FLOOR_LADDER_BOUND: f64 = 1e-14;
    let result = run_deck_a();
    let waveforms = waveforms_by_name(&result);
    let q = &waveforms["Q"];
    let y = &waveforms["Y"];

    let edges: Vec<usize> = (1..result.time.len())
        .filter(|index| (q[*index] - q[index - 1]).abs() > 1.0)
        .collect();
    assert_eq!(
        edges.len(),
        6,
        "the mixed module halves a 10 ns clock over 60 ns, so `q` flips six \
         times; got {:?}",
        edges
            .iter()
            .map(|index| result.time[*index])
            .collect::<Vec<_>>()
    );

    for index in edges {
        let edge = result.time[index];
        let step = result.time[index + 1] - edge;
        assert!(
            step >= CONTROLLER_STEP_BOUND,
            "the step after the D/A edge at {edge:e} is {step:e}, below \
             {CONTROLLER_STEP_BOUND:e}: the retry ladder at the edge is back, \
             or something is accepting a width the controller did not propose"
        );
        let settled = sample_at(&result, "Q", edge + DECK_A_INSTANT_OFFSET);
        assert!(
            (q[index] - settled).abs() < 1e-6,
            "V(q) at the edge instant {edge:e} is {}, and the level it settles \
             at is {settled}: the edge was not taken in the point that landed \
             on it",
            q[index]
        );
    }

    // Since R2.24 no step is below the floor bound at all, and
    // `deck_a_accepts_no_step_near_the_solver_floor_after_a_digital_edge`
    // asserts exactly that. This loop is the shape check that outlives it: if
    // the set ever refills, a returning edge on `q` or the far end of a `y`
    // ramp lands here and fails, because both are flat 50 ps later while the
    // one class that used to live here — a ramp leaving zero volts — is at
    // 1.65 V by then.
    for (index, times) in result.time.windows(2).enumerate() {
        if times[1] - times[0] >= FLOOR_LADDER_BOUND {
            continue;
        }
        let soon = sample_at(&result, "Y", times[0] + 5e-11);
        assert!(
            y[index] < 0.5 && soon > 1.0,
            "the accepted step {:e} at {:e} is below {FLOOR_LADDER_BOUND:e} \
             somewhere other than a `y` ramp leaving zero volts: V(y) is {} \
             there and {soon} fifty picoseconds later",
            times[1] - times[0],
            times[0],
            y[index]
        );
    }
}

/// The accepted grid is allowed to move; the waveform is not.
///
/// Deck A's analog solution is sampled at the instants the pre-fix run
/// published its XSPICE event net on, so a change that ends the refinement
/// storm has to reproduce the same voltages on a grid a thousand points
/// shorter. One micro-volt is four orders of magnitude below the smallest
/// feature any of these nodes carries.
#[test]
fn deck_a_digital_instants_keep_their_pre_fix_analog_solution() {
    let result = run_deck_a();
    let instants: Vec<f64> = trace_points(&result, "d_clk")
        .into_iter()
        .map(|(time, _)| time)
        .filter(|time| *time > 0.0)
        .collect();
    println!("TRIFAMILY deck_a_digital_instants={instants:?}");
    let mut samples = Vec::new();
    for instant in DECK_A_DIGITAL_INSTANTS {
        for node in DECK_A_INSTANT_NODES {
            samples.push(sample_at(&result, node, instant + DECK_A_INSTANT_OFFSET));
        }
    }
    println!("TRIFAMILY deck_a_instant_voltages={samples:?}");
    if !DIODE_CMC || emitting() {
        // Without the generated card the deck is a different circuit and the
        // pinned samples are not its samples.
        return;
    }
    assert_eq!(
        samples.len(),
        DECK_A_INSTANT_VOLTAGES.len(),
        "the pinned sample table must cover every instant and node"
    );
    for (index, (observed, expected)) in samples.iter().zip(DECK_A_INSTANT_VOLTAGES).enumerate() {
        let instant = DECK_A_DIGITAL_INSTANTS[index / DECK_A_INSTANT_NODES.len()];
        let node = DECK_A_INSTANT_NODES[index % DECK_A_INSTANT_NODES.len()];
        assert!(
            (observed - expected).abs() < 1e-6,
            "V({node}) {DECK_A_INSTANT_OFFSET:e} s after the digital instant {instant:e} is \
             {observed}, was {expected} before the accepted grid changed"
        );
    }
}

#[test]
#[ignore = "R2.4: .op refuses mixed decks"]
fn deck_a_operating_point_agrees_with_its_first_transient_point() {
    let (_models, netlist) = deck_a();
    let operating_point = Engine::default()
        .run_dc_op(&netlist)
        .expect("a deck with a mixed Verilog-AMS instance must have an operating point");
    let transient = run_deck_a();
    for (name, values) in transient.node_names.iter().zip(&transient.voltages) {
        let Some(index) = operating_point
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
        else {
            panic!(
                "operating point is missing node {name}: {:?}",
                operating_point.node_names
            );
        };
        let (op, first) = (operating_point.node_voltages[index], values[0]);
        assert!(
            (op - first).abs() < 1e-9,
            "node {name}: operating point {op} differs from the first transient point {first}"
        );
    }
}

//=============================================================================
// Decks C and C2: two mixed modules driving one deck node digitally
//=============================================================================

const DRIVER_MODULE: &str = r#"
`include "disciplines.vams"
module drv(p, n, q);
    inout p, n; electrical p, n;
    output q; reg q;
    initial begin q = 1'b0; end
    always #5 q = ~q;
    analog I(p,n) <+ V(p,n)/1000.0;
endmodule
"#;

const RECEIVER_MODULE: &str = r#"
`include "disciplines.vams"
module rcv(p, n, clk, c);
    inout p, n; electrical p, n;
    input clk; output c; wire clk; reg c;
    initial c = 1'b0;
    always @(posedge clk) c <= ~c;
    analog I(p,n) <+ V(p,n)/1000.0;
endmodule
"#;

/// Deck C carries a 10k load on the shared digital net `dq`; deck C2 is the
/// same deck with that load removed.
fn deck_c(load_dq: bool) -> (Vec<ModelFile>, Netlist) {
    deck_c_with_load(if load_dq { "rdq dq 0 10k\n" } else { "" })
}

/// Deck C's circuit with an arbitrary load on the shared digital net.
///
/// The load is the whole variable: it decides whether `dq` is an event net at
/// all, and — once it is analog — how far behind the driver's D/A flip the
/// receiver's A/D crossing falls.
fn deck_c_with_load(load: &str) -> (Vec<ModelFile>, Netlist) {
    let driver = ModelFile::new(DRIVER_MODULE);
    let receiver = ModelFile::new(RECEIVER_MODULE);
    let deck = format!(
        "* mixed module to mixed module over one deck node\n\
         .param vcc=3.3\n\
         x1 p 0 dq drv\n\
         .va \"{}\" drv\n\
         x2 p2 0 dq c rcv\n\
         .va \"{}\" rcv\n\
         rp p 0 1k\n\
         rp2 p2 0 1k\n\
         {load}\
         rc c 0 10k\n\
         .end\n",
        driver.path(),
        receiver.path()
    );
    let netlist = Netlist::parse(&deck).expect("deck C parses");
    (vec![driver, receiver], netlist)
}

/// Deck C, with the number of timepoints the step controller threw away.
fn run_deck_c_with_rejections(load_dq: bool) -> (TransientResult, usize) {
    let (_models, netlist) = deck_c(load_dq);
    let engine = Engine::default();
    let result = engine
        .run_tran(&netlist, 100e-9, 0.5e-9)
        .expect("deck C transient");
    let rejections = engine.convergence_quality().timestep_reductions;
    digital_trace_invariants::assert_one_digital_value_per_instant(
        &result,
        if load_dq { "deck C" } else { "deck C2" },
    );
    (result, rejections)
}

fn run_deck_c(load_dq: bool) -> TransientResult {
    run_deck_c_with_rejections(load_dq).0
}

#[test]
fn deck_c2_unloaded_digital_to_digital_net_carries_the_driver_period() {
    let result = run_deck_c(false);
    let points = trace_points(&result, "dq");
    assert_eq!(
        points.len(),
        20,
        "the driver toggles every 5 ns over 100 ns, got {points:?}"
    );
    for (time, _) in &points {
        let ticks = time / 5e-9;
        assert!(
            (ticks - ticks.round()).abs() < 1e-9,
            "every event on dq must land on a 5 ns multiple, got {time:e}"
        );
    }
    // The receiver halves it, so its own net toggles once per driver rise.
    let received = trace_points(&result, "c");
    assert!(
        received.len() * 2 >= points.len(),
        "the receiving module must see every rise of dq, got {received:?}"
    );
    pin_usize("deck_c2_points", result.time.len(), DECK_C2_POINTS);
}

#[test]
fn deck_c_loaded_digital_to_digital_net_still_runs() {
    let result = run_deck_c(true);
    assert!(!result.time.is_empty(), "deck C produced no points");
    assert!(
        !trace_points(&result, "dq").is_empty(),
        "deck C recorded no events on dq"
    );
}

/// A load on a digital net is a load, not a hundredfold cost.
///
/// Deck C and deck C2 are one circuit with one 10k resistor between them, and
/// both run the same two modules over the same 100 ns. Before the R2.2 fix the
/// loaded deck spent 1890 accepted points and ~504 rejections against C2's 265
/// and 0, because every D/A flip on `dq` was interpolated as an interior A/D
/// root and chased to the solver's hard floor. The bound is relational rather
/// than pinned: the load legitimately adds the edges themselves, so deck C may
/// keep half as many points again as C2 and reject once per published edge.
#[test]
fn deck_c_load_does_not_multiply_the_accepted_grid() {
    let (loaded, loaded_rejections) = run_deck_c_with_rejections(true);
    let (unloaded, unloaded_rejections) = run_deck_c_with_rejections(false);
    let edges: usize = ["dq", "c"]
        .into_iter()
        .map(|node| trace_points(&loaded, node).len())
        .sum();
    println!(
        "TRIFAMILY deck_c_points={} deck_c_rejections={loaded_rejections} \
         deck_c2_points={} deck_c2_rejections={unloaded_rejections} deck_c_edges={edges}",
        loaded.time.len(),
        unloaded.time.len()
    );
    assert!(
        loaded.time.len() * 2 <= unloaded.time.len() * 3,
        "loading the shared digital net kept {} accepted points against the unloaded deck's {}",
        loaded.time.len(),
        unloaded.time.len()
    );
    assert!(
        loaded_rejections <= unloaded_rejections + edges,
        "loading the shared digital net rejected {loaded_rejections} timepoints against the \
         unloaded deck's {unloaded_rejections}, with only {edges} published edges to pay for"
    );
}

/// Deck C3: deck C with the shared digital net slowed down by a capacitor.
///
/// Deck C's own load is resistive, so the receiver's A/D crossing falls inside
/// the very trial the driver's D/A bridge flipped in and both halves of `dq`
/// move at one accepted timepoint. That agreement is a property of the deck,
/// not of the trace channel. Add 100 pF to the same net and the crossing lands
/// a nanosecond and a half later — several accepted points after the flip —
/// so through that window the driver says one thing about `dq` and the reader
/// says the other. The net still holds exactly one value at each of those
/// instants: the one its driver put there.
const DECK_C3_LOAD: &str = "rdq dq 0 10k\ncdq dq 0 100p\n";

#[test]
fn deck_c3_a_lagging_reader_does_not_glitch_the_net_its_driver_drives() {
    let (_models, netlist) = deck_c_with_load(DECK_C3_LOAD);
    let result = Engine::default()
        .run_tran(&netlist, 100e-9, 0.5e-9)
        .expect("deck C3 transient");
    digital_trace_invariants::assert_one_digital_value_per_instant(&result, "deck C3");

    let points = trace_points(&result, "dq");
    let glitches: Vec<_> = points
        .windows(2)
        .filter(|pair| pair[0].0 == pair[1].0 && pair[0].1 != pair[1].1)
        .map(|pair| (pair[0].0, pair[0].1, pair[1].1))
        .collect();
    println!(
        "TRIFAMILY deck_c3_points={} deck_c3_dq_points={} deck_c3_glitches={}",
        result.time.len(),
        points.len(),
        glitches.len()
    );
    assert!(
        glitches.is_empty(),
        "a reader lagging its driver must not publish a second value for dq at one \
         instant; got {glitches:?}"
    );

    // And the value the net carries is the driver's schedule, not the sampled
    // crossing: the driver toggles on its own 5 ns timer.
    for (time, _) in &points {
        let ticks = time / 5e-9;
        assert!(
            (ticks - ticks.round()).abs() < 1e-9,
            "dq must carry the driver's 5 ns schedule, got an event at {time:e}; {points:?}"
        );
    }
}

#[test]
fn deck_c_and_c2_agree_on_the_shared_digital_net() {
    let loaded = run_deck_c(true);
    let unloaded = run_deck_c(false);

    let glitches: Vec<_> = trace_points(&loaded, "dq")
        .windows(2)
        .filter(|pair| pair[0].0 == pair[1].0 && pair[0].1 != pair[1].1)
        .map(|pair| (pair[0].0, pair[0].1, pair[1].1))
        .collect();
    assert!(
        glitches.is_empty(),
        "an analog load on a digital net must not add zero-width events; got {glitches:?}"
    );

    let (loaded_traces, unloaded_traces) = (traces_by_name(&loaded), traces_by_name(&unloaded));
    assert_eq!(
        loaded_traces.get("DQ"),
        unloaded_traces.get("DQ"),
        "loading dq changed the events the digital half published"
    );
}

//=============================================================================
// Decks D and F2: a mixed discrete port on an XSPICE event net
//=============================================================================

/// The same circuit twice: deck D writes the XSPICE A cards first, deck F2
/// writes the Verilog-AMS X card first. Card order must not reach the answer.
fn deck_d(x_card_first: bool) -> (Vec<ModelFile>, Netlist) {
    let receiver = ModelFile::new(RECEIVER_MODULE);
    let xspice = "a_adc [clk] [d_clk] adc\n\
                  .model adc adc_bridge(in_low=1.6 in_high=1.7)\n\
                  a_dac [d_clk] [y] dac\n\
                  .model dac dac_bridge(out_low=0 out_high=3.3)\n\
                  ry y 0 10k\n";
    let mixed = format!(
        "x2 p2 0 d_clk c rcv\n.va \"{}\" rcv\nrp2 p2 0 1k\nrc c 0 10k\n",
        receiver.path()
    );
    let (first, second) = if x_card_first {
        (mixed.as_str(), xspice)
    } else {
        (xspice, mixed.as_str())
    };
    let deck = format!(
        "* a mixed discrete port joined to an XSPICE event net\n\
         .param vcc=3.3\n\
         vclk clk 0 pulse(0 3.3 0 0.1n 0.1n 4.9n 10n)\n\
         {first}{second}.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck D parses");
    (vec![receiver], netlist)
}

fn run_deck_d(x_card_first: bool) -> TransientResult {
    let (_models, netlist) = deck_d(x_card_first);
    let result = Engine::default()
        .run_tran(&netlist, 30e-9, 0.5e-9)
        .expect("deck D transient");
    digital_trace_invariants::assert_one_digital_value_per_instant(&result, "deck D");
    result
}

#[test]
fn decks_d_and_f2_are_independent_of_card_order() {
    let a_first = run_deck_d(false);
    let x_first = run_deck_d(true);

    assert!(
        !trace_points(&a_first, "d_clk").is_empty(),
        "the shared event net recorded nothing"
    );
    assert_eq!(
        a_first.time, x_first.time,
        "card order changed the accepted time grid"
    );
    assert_eq!(
        traces_by_name(&a_first),
        traces_by_name(&x_first),
        "card order changed the digital traces"
    );

    let (left, right) = (waveforms_by_name(&a_first), waveforms_by_name(&x_first));
    assert_eq!(
        left.keys().collect::<Vec<_>>(),
        right.keys().collect::<Vec<_>>(),
        "card order changed which analog nodes the deck has"
    );
    for (name, values) in &left {
        let other = &right[name];
        let worst = values
            .iter()
            .zip(other)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            worst < 1e-12,
            "card order moved V({name}) by {worst:e} at worst"
        );
    }
}

//=============================================================================
// Cross-process determinism of the deck A golden
//=============================================================================

/// Four children, each re-running the deck A case with the emit variable set,
/// must report the hashes this process computes. A shared cache, an address,
/// or an iteration order that leaked into the answer shows up here and nowhere
/// else in the file.
#[test]
fn deck_a_hashes_are_identical_across_processes() {
    if !DIODE_CMC {
        println!(
            "TRIFAMILY determinism: SKIPPED because this build has no \
             veriloga-model-diode-cmc card, so deck A pins no sequence"
        );
        return;
    }
    let Ok(executable) = std::env::current_exe() else {
        println!("TRIFAMILY determinism: SKIPPED because the test binary has no path");
        return;
    };

    let parent = deck_a_fingerprint(&run_deck_a());
    for attempt in 0..4 {
        let output = Command::new(&executable)
            .args([
                "--exact",
                "deck_a_tri_family_transient_sequence_golden",
                "--nocapture",
            ])
            .env(EMIT_ENV, "1")
            .output();
        let Ok(output) = output else {
            println!("TRIFAMILY determinism: SKIPPED because the test binary could not be spawned");
            return;
        };
        assert!(
            output.status.success(),
            "child {attempt} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).expect("child stdout is utf-8");
        let read = |key: &str| -> String {
            let prefix = format!("TRIFAMILY {key}=");
            stdout
                .lines()
                .find_map(|line| line.trim().strip_prefix(prefix.as_str()))
                .unwrap_or_else(|| panic!("child {attempt} printed no {key}:\n{stdout}"))
                .to_string()
        };
        let child = (
            read("deck_a_points")
                .parse::<usize>()
                .expect("child point count"),
            u64::from_str_radix(&read("deck_a_grid_hash"), 16).expect("child grid hash"),
            u64::from_str_radix(&read("deck_a_volt_hash"), 16).expect("child voltage hash"),
        );
        assert_eq!(
            child, parent,
            "child {attempt} produced a different deck A sequence than this process"
        );
    }
}
