//! DC sweeps carry the held discrete state from point to point (P1 design S6,
//! S8), and `RESTART=YES` makes every point an independent operating point.
//!
//! The fixture is a set/reset latch: `posedge hi` sets `q`, `negedge lo`
//! clears it, and two behavioural comparators on `vin` drive `hi` above
//! 2.7 V and `lo` above 2.4 V. Between 2.4 V and 2.7 V both inputs are
//! steady, so `q` is whatever the sweep brought it in as — the hysteresis a
//! carried state shows in sweep order, and an independent re-solve cannot.
#![cfg(feature = "veriloga")]

use rspice_core::netlist::{AnalysisCommand, DcSweepSpec};
use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(extension: &str, source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_mixed_dc_{}_{}.{extension}",
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

const LATCH: &str = "`timescale 1ns/1ps\n\
module latch(hi, lo, q);\n\
  input hi, lo;\n\
  output q;\n\
  reg q;\n\
  initial q = 1'b0;\n\
  always @(posedge hi) q = 1'b1;\n\
  always @(negedge lo) q = 1'b0;\n\
endmodule\n";

/// The comparators on `vin`, which a test sweeps directly or through `r`.
/// Each threshold sits half a sweep step below its decade value, so the sweep
/// value that first reaches it is the 10 mV grid point itself rather than
/// whichever side of it floating point puts the grid point on.
const COMPARATORS: &str = "bhi hi 0 v = v(vin) >= 2.695 ? 3.3 : 0\n\
blo lo 0 v = v(vin) >= 2.395 ? 3.3 : 0\n\
rhi hi 0 1meg\n\
rlo lo 0 1meg\n\
rq q 0 1meg\n";

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn latch_deck_at(latch: &SourceFile, vin: f64, card: &str) -> Netlist {
    parse(&format!(
        "* carried latch\n\
         .param vcc=3.3\n\
         vin vin 0 {vin}\n\
         {COMPARATORS}\
         xl hi lo q latch\n\
         .hdl \"{}\"\n\
         {card}\n\
         .end\n",
        latch.path()
    ))
}

fn latch_deck(latch: &SourceFile, card: &str) -> Netlist {
    latch_deck_at(latch, 0.0, card)
}

/// The same latch built from XSPICE: `adc_bridge` inputs, a `d_dff` clocked
/// by `hi` with its data tied high and an asynchronous reset on `!lo`, and a
/// `dac_bridge` output. The flip-flop starts set (`ic=1`); the reset clears it
/// below the lower threshold.
fn xspice_latch_deck(card: &str) -> Netlist {
    parse(&format!(
        "* XSPICE latch\n\
         vin vin 0 0\n\
         vone one 0 3.3\n\
         {COMPARATORS}\
         aadc [hi lo one] [hid lod oned] adc\n\
         .model adc adc_bridge (in_low=1.65 in_high=1.65)\n\
         ainv lod lodn inv\n\
         .model inv d_inverter\n\
         aff oned hid NULL lodn qd qdn ff\n\
         .model ff d_dff (ic=1)\n\
         adac [qd] [q] dac\n\
         .model dac dac_bridge (out_low=0 out_high=3.3)\n\
         {card}\n\
         .end\n"
    ))
}

/// `(vin, v(q))` at every point of the deck's own `.dc` card.
fn sweep(netlist: &Netlist) -> Vec<(f64, f64)> {
    let Some(AnalysisCommand::Dc {
        source,
        start,
        stop,
        step,
        mode,
        sweep2,
        restart,
    }) = netlist.analyses.first()
    else {
        panic!("the deck carries a .dc card")
    };
    let spec = DcSweepSpec {
        start: *start,
        stop: *stop,
        step: *step,
        mode: mode.clone(),
        restart: *restart,
    };
    Engine::default()
        .run_dc_sweep2_spec_with_report_and_abort(
            netlist,
            source,
            &spec,
            sweep2.as_ref(),
            &rspice_core::NoAbort,
        )
        .unwrap_or_else(|error| panic!("the sweep runs: {error}"))
        .into_iter()
        .map(|point| {
            (
                point.sweep_value,
                point.result.try_voltage_named("q").expect("q is published"),
            )
        })
        .collect()
}

fn is_high(level: f64) -> bool {
    assert!(
        !(0.1..=3.2).contains(&level),
        "q must be a rail, not {level}"
    );
    level > 1.65
}

/// The first sweep value at which `q` reads `high`, and that `q` never
/// changes again after it.
fn single_edge(points: &[(f64, f64)], high: bool) -> f64 {
    let first = points
        .iter()
        .position(|&(_, q)| is_high(q) == high)
        .unwrap_or_else(|| panic!("q never reads {high}: {points:?}"));
    assert!(
        points[first..].iter().all(|&(_, q)| is_high(q) == high),
        "q changes back after the edge: {points:?}"
    );
    points[first].0
}

fn assert_edge_at(edge: f64, expected: f64) {
    assert!(
        (edge - expected).abs() < 1.0e-9,
        "the edge is at {edge} V, expected {expected} V"
    );
}

/// Point k starts from point k−1's committed equilibrium: q rises where the
/// upward sweep first reaches 2.70 V and falls where the downward sweep first
/// drops below 2.40 V, so the band between the two thresholds reads the
/// direction the sweep came from.
#[test]
fn a_sweep_carries_the_comparator_latch_in_sweep_order() {
    let latch = SourceFile::new("v", LATCH);
    let up = sweep(&latch_deck(&latch, ".dc vin 0 3 0.01"));
    assert!(!is_high(up[0].1), "q starts low below both thresholds");
    assert_edge_at(single_edge(&up, true), 2.70);

    let down = sweep(&latch_deck(&latch, ".dc vin 3 0 -0.01"));
    assert!(is_high(down[0].1), "q starts high above both thresholds");
    assert_edge_at(single_edge(&down, false), 2.39);
}

/// `RESTART=YES`: every point is an independent operating point, so both
/// directions read the same and each point equals a standalone `.op` there.
#[test]
fn restart_makes_every_point_an_independent_operating_point() {
    let latch = SourceFile::new("v", LATCH);
    let up = sweep(&latch_deck(&latch, ".dc vin 0 3 0.05 restart=yes"));
    let down = sweep(&latch_deck(&latch, ".dc vin 3 0 -0.05 restart=yes"));
    let engine = Engine::default();
    for &(vin, q) in &up {
        let mirrored = down
            .iter()
            .find(|(value, _)| (value - vin).abs() < 1.0e-9)
            .expect("both directions visit the same points");
        assert_eq!(
            q.to_bits(),
            mirrored.1.to_bits(),
            "restart points do not depend on direction at {vin} V"
        );
        let standalone = engine
            .run_dc_op(&latch_deck_at(&latch, vin, ""))
            .expect("the standalone operating point solves")
            .try_voltage_named("q")
            .expect("q is published");
        assert!(
            (q - standalone).abs() <= 1.0e-12 * standalone.abs().max(1.0e-12),
            "restart point at {vin} V is {q}, the standalone .op is {standalone}"
        );
    }
}

/// The XSPICE latch has no state to carry: SPICE evaluates every DC point at
/// time zero, where `d_dff` drives its `ic` unless reset or set holds it, so
/// the flip-flop reads low wherever `lo` resets it and high everywhere else,
/// whichever way the sweep runs. The held state a sweep carries belongs to
/// the discrete processes of a mixed module, which do run before their DC
/// point.
#[test]
fn the_xspice_latch_twin_reads_its_time_zero_state_at_every_point() {
    let up = sweep(&xspice_latch_deck(".dc vin 0 3 0.01"));
    assert_edge_at(single_edge(&up, true), 2.40);
    let down = sweep(&xspice_latch_deck(".dc vin 3 0 -0.01"));
    assert_edge_at(single_edge(&down, false), 2.39);
    for &(vin, q) in &up {
        let mirrored = down
            .iter()
            .find(|(value, _)| (value - vin).abs() < 1.0e-9)
            .expect("both directions visit the same points");
        assert_eq!(is_high(q), is_high(mirrored.1), "no hysteresis at {vin} V");
    }
}

/// S8: a parameter sweep rebuilds the circuit at every point, and the held
/// equilibrium travels into the rebuilt circuit when its digital structure is
/// unchanged. `r` moves `vin` above 2.7 V and then back into the band, where
/// only a carried latch still reads high.
#[test]
fn a_parameter_sweep_carries_the_latch_into_each_rebuilt_circuit() {
    let latch = SourceFile::new("v", LATCH);
    let deck = |card: &str| {
        parse(&format!(
            "* carried latch across rebuilt circuits\n\
             .param vcc=3.3\n\
             .param r=1k\n\
             vsup sup 0 3.3\n\
             rtop sup vin 1k\n\
             rbot vin 0 {{r}}\n\
             {COMPARATORS}\
             xl hi lo q latch\n\
             .hdl \"{}\"\n\
             {card}\n\
             .end\n",
            latch.path()
        ))
    };
    // vin = 3.3 r / (r + 1k): 1.65 V, 3.0 V, then 2.64 V inside the band.
    let carried = sweep(&deck(".dc r list 1000 10000 4000"));
    let levels = carried.iter().map(|&(_, q)| is_high(q)).collect::<Vec<_>>();
    assert_eq!(levels, [false, true, true], "{carried:?}");
    let restarted = sweep(&deck(".dc r list 1000 10000 4000 restart=yes"));
    let levels = restarted
        .iter()
        .map(|&(_, q)| is_high(q))
        .collect::<Vec<_>>();
    assert_eq!(levels, [false, true, false], "{restarted:?}");
}

/// S8, the other branch: a parameter that changes the digital elaboration —
/// here the width of a vector the module holds — gives the rebuilt point no
/// structure to carry into, so that point runs its initialization afresh.
#[test]
fn a_parameter_that_changes_the_digital_structure_reinitializes_the_point() {
    let latch = SourceFile::new(
        "v",
        "`timescale 1ns/1ps\n\
         module wlatch(hi, lo, q);\n\
           parameter integer w = 2;\n\
           input hi, lo;\n\
           output q;\n\
           reg q;\n\
           reg [w-1:0] seen;\n\
           initial begin q = 1'b0; seen = 0; end\n\
           always @(posedge hi) begin q = 1'b1; seen = seen + 1; end\n\
           always @(negedge lo) q = 1'b0;\n\
         endmodule\n",
    );
    let deck = |card: &str| {
        parse(&format!(
            "* carried latch, varying width\n\
             .param vcc=3.3\n\
             .param w=2\n\
             .param r=10000\n\
             vsup sup 0 3.3\n\
             rtop sup vin 1k\n\
             rbot vin 0 {{r}}\n\
             {COMPARATORS}\
             xl hi lo q wlatch w={{w}}\n\
             .hdl \"{}\"\n\
             {card}\n\
             .end\n",
            latch.path()
        ))
    };
    // The inner sweep leaves the band (r = 4k, vin = 2.64 V) for 3.0 V and
    // sets q; the next outer point starts back inside the band. With the
    // width unchanged that point carries q high; with the width changed it
    // re-initializes, and inside the band its initialization leaves q low.
    // The inner sweep is linear: a `LIST` reads every value it can, and `w`
    // names a parameter, so `r list 4000 10000 w ...` would read `w` as a
    // third point.
    let same = sweep(&deck(".dc r 4000 10000 6000 w list 2 2"));
    let levels = same.iter().map(|&(_, q)| is_high(q)).collect::<Vec<_>>();
    assert_eq!(levels, [false, true, true, true], "{same:?}");
    let changed = sweep(&deck(".dc r 4000 10000 6000 w list 2 4"));
    let levels = changed.iter().map(|&(_, q)| is_high(q)).collect::<Vec<_>>();
    assert_eq!(levels, [false, true, false, true], "{changed:?}");
}
