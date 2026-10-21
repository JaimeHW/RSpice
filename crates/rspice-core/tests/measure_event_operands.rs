#![cfg(feature = "veriloga")]
//! `.MEASURE TRAN` on a deck whose HDL drives event nets.
//!
//! Every expected value is a closed form of the fixture, never an RSpice
//! output: `q` toggles every 10 ns from 0, so it rises at 10, 30, 50, 70 and
//! 90 ns and its period is 20 ns.

use rspice_core::analysis::measure::MeasureResult;
use rspice_core::analysis::measure_signals::evaluate_tran_measurements;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{AnalysisCommand, Netlist};
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
    Engine::new(SimulationConfig::default())
        .run_tran(netlist, stop, max_step)
        .unwrap_or_else(|error| panic!("transient runs: {error}"))
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
