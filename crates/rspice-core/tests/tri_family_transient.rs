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

const DECK_A_POINTS: usize = 1476;
const DECK_A_GRID_HASH: u64 = 0x1066_dc3f_f7ef_4fed;
const DECK_A_VOLT_HASH: u64 = 0xbe98_92ef_a7aa_6209;
const DECK_C2_POINTS: usize = 265;

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
    Engine::default()
        .run_tran(&netlist, DECK_A_TSTOP, DECK_A_STEP)
        .expect("deck A transient")
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

#[test]
#[ignore = "R2.2: post-event restart ladder starts at hard_min_dt"]
fn deck_a_accepts_no_step_below_a_picosecond_after_a_digital_edge() {
    let result = run_deck_a();
    let tiny: Vec<(usize, f64, f64)> = result
        .time
        .windows(2)
        .enumerate()
        .map(|(index, pair)| (index + 1, pair[0], pair[1] - pair[0]))
        .filter(|(_, _, step)| *step < 1e-12)
        .collect();
    assert!(
        tiny.is_empty(),
        "a digital edge must not restart the analog stepper at the hard floor; \
         {} accepted steps are below 1 ps, first few {:?}",
        tiny.len(),
        tiny.iter().take(6).collect::<Vec<_>>()
    );
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
    let driver = ModelFile::new(DRIVER_MODULE);
    let receiver = ModelFile::new(RECEIVER_MODULE);
    let load = if load_dq { "rdq dq 0 10k\n" } else { "" };
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

fn run_deck_c(load_dq: bool) -> TransientResult {
    let (_models, netlist) = deck_c(load_dq);
    Engine::default()
        .run_tran(&netlist, 100e-9, 0.5e-9)
        .expect("deck C transient")
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

#[test]
#[ignore = "R2.3: loaded digital-to-digital net records zero-width glitches"]
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
    Engine::default()
        .run_tran(&netlist, 30e-9, 0.5e-9)
        .expect("deck D transient")
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
