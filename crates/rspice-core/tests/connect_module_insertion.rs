#![cfg(feature = "veriloga")]
//! A Verilog-AMS connect module, selected by clause 7 and materialized by the
//! engine, on a deck the engine solves.
//!
//! Every other test of this route checks one link of it. These check the chain:
//! a deck names a `.veriloga` file that declares connect modules and a
//! `connectrules` block, the bridge planner finds the mixed node, clause 7
//! selects the module for that node's direction, and the bridge the engine
//! stamps carries the module's own supply rather than the deck's.
//!
//! # What this cannot check yet, and why
//!
//! That the connect module's *body* ran. It did not: the engine delegates each
//! built-in connect module to the XSPICE bridge code model that implements it.
//! Running a connect module's own body needs the Verilog-AMS mixed host, which
//! is not wired to the engine and refuses any trial time off its
//! integer-nanosecond grid — and an LTE-controlled transient does not land
//! there. What the delegation makes checkable is that the module's parameters
//! reached the boundary and changed the conversion, which is what
//! `a_supplied_connect_module_moves_the_threshold` does by moving the threshold
//! far enough that the digital edge lands at a different time.

use std::path::{Path, PathBuf};

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;

struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new(label: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock follows Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-connect-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create isolated connect module test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    /// Write the built-in connect modules plus one `connectrules` block.
    ///
    /// The module sources are the shipped ones, verbatim: a test that wrote
    /// its own would pin its own copy rather than the library.
    fn write_library(&self, rules: &str) -> PathBuf {
        let mut source = String::new();
        for (_, module) in BUILTIN_CONNECT_MODULES {
            source.push_str(module);
        }
        source.push_str(rules);
        let path = self.0.join("connect_lib.va");
        std::fs::write(&path, source).expect("write the connect library");
        path
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A rising ramp into a digital inverter, and the inverter's output driven back
/// into the matrix.
///
/// Both boundaries are in one deck deliberately: `din` needs an
/// analog-to-discrete bridge and `dout` a discrete-to-analog one, so one solve
/// exercises both of Table 7-2's unidirectional rows.
fn deck(library: &Path) -> String {
    format!(
        "connect module insertion\n\
         .veriloga \"{}\"\n\
         vin din 0 pwl(0 0 100n 3.3)\n\
         rin din 0 1meg\n\
         ainv din dout inverter\n\
         .model inverter d_inverter\n\
         rout dout 0 1meg\n\
         .tran 1n 100n\n\
         .end\n",
        library.display().to_string().replace('\\', "/")
    )
}

fn transient_waveform(deck: &str, node: &str) -> Vec<(f64, f64)> {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    let result = Engine::default()
        .run_tran(&netlist, 100.0e-9, 1.0e-9)
        .unwrap_or_else(|error| panic!("transient solves: {error}"));
    let series = node_series(&result, node).to_vec();
    result.time.iter().copied().zip(series).collect()
}

fn node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[index]
}

const DEFAULT_RULES: &str = "\
connectrules deck;
    connect a2d;
    connect d2a;
endconnectrules
";

const SUPPLIED_RULES: &str = "\
connectrules deck;
    connect a2d #(.vsup(1.0));
    connect d2a #(.vsup(1.0));
endconnectrules
";

/// The whole chain, end to end: the conversion happens, both ways.
///
/// `din` ramps from 0 to 3.3 V over the run. The analog-to-discrete bridge the
/// `a2d` rule selects turns that into a four-state edge when the ramp passes
/// half the supply, and the discrete-to-analog bridge the `d2a` rule selects
/// turns the inverter's answer back into a ramped analog transition on `dout` —
/// which starts high and falls, because the gate is an inverter.
#[test]
fn a_connect_module_bridges_a_deck_both_ways() {
    let directory = TempDirectory::new("both-ways");
    let library = directory.write_library(DEFAULT_RULES);
    let waveform = transient_waveform(&deck(&library), "dout");

    let (_, first) = waveform.first().copied().expect("a first point");
    let (_, last) = waveform.last().copied().expect("a last point");
    assert!(
        first > 3.3 / 2.0,
        "the inverter drives dout high while din is below threshold, got {first}"
    );
    assert!(
        last < 3.3 / 2.0,
        "the inverter drives dout low once din crosses, got {last}"
    );

    // The fall is a ramp, not a step: `dac_bridge`'s `t_rise`/`t_fall` shape
    // it, which is the "ramped analog transition" half of the conversion.
    let intermediate = waveform
        .iter()
        .filter(|(_, voltage)| *voltage > 0.1 && *voltage < 3.2)
        .count();
    assert!(
        intermediate > 0,
        "the discrete-to-analog transition is ramped, not stepped: {waveform:?}"
    );
}

/// Section 7.7.3's parameter reaches the boundary and changes the conversion.
///
/// This is the test that would pass just as well without any of this work if
/// the connect module were decoration: at `vsup = 1.0` the analog-to-discrete
/// threshold is 0.5 V rather than 1.65 V, so the ramp crosses it earlier and
/// the inverter's output falls earlier. Nothing about the deck changes but the
/// `connect` statement.
#[test]
fn a_supplied_connect_module_moves_the_threshold() {
    let default_directory = TempDirectory::new("default");
    let default_waveform = transient_waveform(
        &deck(&default_directory.write_library(DEFAULT_RULES)),
        "dout",
    );

    let supplied_directory = TempDirectory::new("supplied");
    let supplied_waveform = transient_waveform(
        &deck(&supplied_directory.write_library(SUPPLIED_RULES)),
        "dout",
    );

    let default_fall = fall_time(&default_waveform);
    let supplied_fall = fall_time(&supplied_waveform);
    assert!(
        supplied_fall < default_fall,
        "a 1.0 V supply crosses earlier than a 3.3 V one: {supplied_fall} vs {default_fall}"
    );

    // The ramp is `pwl(0 0 100n 3.3)`, so the crossing time is the threshold's
    // fraction of the ramp. 0.5 V of 3.3 V over 100 ns is about 15 ns; 1.65 V
    // is about 50 ns. The assertion is loose enough for the step controller to
    // choose its own points and tight enough that a threshold that did not
    // move would fail it.
    assert!(
        supplied_fall < 30e-9,
        "a 0.5 V threshold is crossed in the first third of the ramp, got {supplied_fall}"
    );
    assert!(
        default_fall > 30e-9,
        "a 1.65 V threshold is not, got {default_fall}"
    );
}

/// The time `dout` first falls below half of the 3.3 V the discrete-to-analog
/// bridge drives it between.
fn fall_time(waveform: &[(f64, f64)]) -> f64 {
    waveform
        .iter()
        .find(|(_, voltage)| *voltage < 3.3 / 2.0)
        .map(|(time, _)| *time)
        .unwrap_or_else(|| panic!("dout falls: {waveform:?}"))
}

/// A `.veriloga` file that declares only connect modules is a connect library:
/// it contributes rules and no device, and the deck must not have to give it a
/// module it does not have.
#[test]
fn a_connect_library_needs_no_device_module() {
    let directory = TempDirectory::new("library-only");
    let library = directory.write_library(DEFAULT_RULES);
    let source = std::fs::read_to_string(&library).expect("the library is written");
    assert!(
        !source.contains("\nmodule "),
        "the library declares no ordinary module"
    );
    // Solving at all is the assertion: compiling this file for a device model
    // would fail before the first bridge was planned.
    let waveform = transient_waveform(&deck(&library), "din");
    assert!(!waveform.is_empty());
}

/// A connect module RSpice cannot execute is refused by name, with the reason,
/// rather than silently bridged as if the deck had asked for nothing.
#[test]
fn a_connect_module_outside_the_library_is_refused() {
    let directory = TempDirectory::new("refused");
    let path = directory.path().join("connect_lib.va");
    std::fs::write(
        &path,
        "\
connectmodule my_a2d(a, d);
    input a;
    output d;
    electrical a;
    logic d;
endmodule
connectmodule d2a(d, a);
    input d;
    output a;
    logic d;
    electrical a;
endmodule
connectrules deck;
    connect my_a2d;
    connect d2a;
endconnectrules
",
    )
    .expect("write the connect library");

    let netlist = Netlist::parse(&deck(&path)).expect("deck parses");
    let error = Engine::default()
        .run_tran(&netlist, 100.0e-9, 1.0e-9)
        .expect_err("an unrunnable connect module is refused");
    let error = format!("{error}");
    assert!(error.contains("my_a2d"), "names the module: {error}");
    assert!(
        error.contains("integer-nanosecond grid"),
        "names the blocker: {error}"
    );
}
