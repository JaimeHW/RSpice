#![cfg(feature = "veriloga")]
//! Where a mixed boundary's supply comes from when the deck does not say.
//!
//! Both boundary routes — the XSPICE auto-bridge and the Verilog-AMS mixed
//! module — convert between an event level and an analog one, and both need a
//! supply to do it. A deck that sets `.param vcc` says what it is. A deck that
//! did not used to get 3.3 V silently, whatever its rails were, so a 1 V core
//! and a 5 V I/O ring both converted at 3.3 V with nothing said.
//!
//! These cases pin the replacement. The rail that reaches the boundary net
//! through the passive network is the supply, said out loud; no rail is 3.3 V,
//! said out loud; two rails is a refusal naming both. A `.param vcc` and a
//! `connectrules` `vsup` both outrank the derivation and stay silent.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, Once};

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;

//=============================================================================
// Warning capture
//=============================================================================

static LOG_INIT: Once = Once::new();
static CAPTURED: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// Serializes the capture window; the logger is process-global.
static CAPTURE_LOCK: Mutex<()> = Mutex::new(());

struct CapturingLogger;

impl log::Log for CapturingLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            CAPTURED
                .lock()
                .expect("warning capture lock")
                .push(record.args().to_string());
        }
    }

    fn flush(&self) {}
}

static LOGGER: CapturingLogger = CapturingLogger;

/// Run `body` with the supply warnings it emits collected, deduplicated and
/// sorted: a build repeated for a second analysis says the same sentence
/// again, and what is pinned here is which sentences a run produces.
fn supply_warnings_of<T>(body: impl FnOnce() -> T) -> (T, Vec<String>) {
    let guard = CAPTURE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("the capture logger installs");
        log::set_max_level(log::LevelFilter::Warn);
    });
    CAPTURED.lock().expect("warning capture lock").clear();
    let value = body();
    let mut warnings: Vec<String> = CAPTURED
        .lock()
        .expect("warning capture lock")
        .iter()
        .filter(|line| line.contains("boundary net"))
        .cloned()
        .collect();
    drop(guard);
    warnings.sort();
    warnings.dedup();
    (value, warnings)
}

//=============================================================================
// Decks
//=============================================================================

struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new(label: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock follows Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-supply-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create isolated supply test directory");
        Self(path)
    }

    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, source).expect("write the test source");
        path
    }

    /// The shipped connect modules plus one `connectrules` block.
    fn write_library(&self, rules: &str) -> PathBuf {
        let mut source = String::new();
        for (_, module) in BUILTIN_CONNECT_MODULES {
            source.push_str(module);
        }
        source.push_str(rules);
        self.write("connect_lib.va", &source)
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn slashed(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

const FIVE_VOLT_RAIL: &str = "V1 vdd 0 DC 5\n";
const ONE_VOLT_RAIL: &str = "V1 vdd 0 DC 1\n";

/// A digital pull-up on an analog node, with `rail` the only rail in the deck
/// and a 10k path from it to the boundary net.
///
/// The generated `dac_bridge` drives `mix` through a branch equation, so the
/// pull-up sets no divider: `mix` reads the bridge's own high level.
fn auto_bridge_deck(rail: &str, parameters: &str) -> String {
    format!(
        "* auto-bridge output level against the deck's rail\n\
         {parameters}\
         {rail}\
         Rpull mix vdd 10k\n\
         .model pull d_pullup\n\
         apull [mix] pull\n\
         .end\n"
    )
}

/// A rising ramp into a digital inverter, with one rail reaching the
/// analog-to-discrete boundary and another reaching the discrete-to-analog
/// one. With `rails` naming the same node twice the two boundaries share it.
fn inverter_deck(rails: &str, input_rail: &str, output_rail: &str, library: &str) -> String {
    format!(
        "* auto-bridge thresholds and levels against the deck's rails\n\
         {library}\
         {rails}\
         Vin din 0 PWL(0 0 100n 5)\n\
         Rin din {input_rail} 1g\n\
         ainv din dout inverter\n\
         .model inverter d_inverter\n\
         Rout dout {output_rail} 1g\n\
         .end\n"
    )
}

/// A pure-discrete Verilog module whose output lands on a deck node: the mixed
/// route's discrete-to-analog boundary.
const TIMER_MODULE: &str =
    "module t4(q);\n output q; reg q;\n initial begin q=0; #4 q=1; end\nendmodule\n";

fn mixed_deck(model: &Path, rail: &str) -> String {
    format!(
        "* mixed boundary output level against the deck's rail\n\
         X1 q t4\n\
         .va \"{}\" t4\n\
         {rail}\
         Rpull q vdd 10k\n\
         .end\n",
        slashed(model)
    )
}

//=============================================================================
// Measurement
//=============================================================================

fn run(deck: &str, tstop: f64, step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    Engine::default()
        .run_tran(&netlist, tstop, step)
        .unwrap_or_else(|error| panic!("{error}\n--- deck ---\n{deck}"))
}

fn build_error(deck: &str) -> String {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    Engine::default()
        .build_circuit(&netlist)
        .err()
        .unwrap_or_else(|| panic!("the build must refuse\n--- deck ---\n{deck}"))
        .to_string()
}

fn series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[index]
}

fn value_at(result: &TransientResult, node: &str, time: f64) -> f64 {
    let values = series(result, node);
    result
        .time
        .iter()
        .zip(values)
        .find(|(sample, _)| **sample >= time)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| panic!("{node} has no sample at or after {time:e}"))
}

/// The time `node` first falls below one volt, in nanoseconds.
fn fall_time_ns(result: &TransientResult, node: &str) -> f64 {
    result
        .time
        .iter()
        .zip(series(result, node))
        .find(|(_, voltage)| **voltage < 1.0)
        .map(|(time, _)| *time * 1e9)
        .unwrap_or_else(|| panic!("{node} never falls"))
}

fn assert_level(label: &str, observed: f64, expected: f64) {
    println!("SUPPLY {label}={observed:.6}");
    assert!(
        (observed - expected).abs() < 0.02,
        "{label} is {observed}, expected the {expected} V supply"
    );
}

fn the_only_warning(warnings: &[String]) -> &str {
    match warnings {
        [only] => only.as_str(),
        other => panic!("expected one supply warning, got {other:?}"),
    }
}

//=============================================================================
// The derivation: one rail reaching the boundary is the supply
//=============================================================================

#[test]
fn a_one_volt_rail_sets_the_auto_bridge_output_level() {
    let (result, warnings) =
        supply_warnings_of(|| run(&auto_bridge_deck(ONE_VOLT_RAIL, ""), 10e-9, 0.5e-9));
    assert_level(
        "auto_bridge_1v_rail_output",
        value_at(&result, "mix", 5e-9),
        1.0,
    );
    let warning = the_only_warning(&warnings);
    println!("SUPPLY warning={warning}");
    assert!(
        warning.contains("derived from V1 = 1 V") && warning.to_lowercase().contains("'mix'"),
        "the warning must name the source, the level and the net, got: {warning}"
    );
}

#[test]
fn a_five_volt_rail_sets_the_auto_bridge_output_level() {
    let (result, warnings) =
        supply_warnings_of(|| run(&auto_bridge_deck(FIVE_VOLT_RAIL, ""), 10e-9, 0.5e-9));
    assert_level(
        "auto_bridge_5v_rail_output",
        value_at(&result, "mix", 5e-9),
        5.0,
    );
    let warning = the_only_warning(&warnings);
    println!("SUPPLY warning={warning}");
    assert!(
        warning.contains("derived from V1 = 5 V") && warning.contains(".param vcc=5"),
        "the warning must name the level and the parameter that fixes it, got: {warning}"
    );
}

#[test]
fn a_five_volt_rail_sets_the_auto_bridge_input_threshold() {
    let deck = inverter_deck(FIVE_VOLT_RAIL, "vdd", "vdd", "");
    let (result, warnings) = supply_warnings_of(|| run(&deck, 100e-9, 1e-9));
    assert_level(
        "threshold_5v_dout_high",
        value_at(&result, "dout", 0.0),
        5.0,
    );
    // The ramp is 0 to 5 V over 100 ns and the threshold is half the supply,
    // so it is crossed just past the halfway point rather than at the 33 ns a
    // 3.3 V supply's 1.65 V put it. The inverter's own delay and the D/A ramp
    // add a few nanoseconds on top, so the window is loose on the late side.
    let fall = fall_time_ns(&result, "dout");
    println!("SUPPLY threshold_5v_fall_time_ns={fall:.3}");
    assert!(
        (45.0..60.0).contains(&fall),
        "a 2.5 V threshold on a 5 V/100 ns ramp falls just past 50 ns, got {fall} ns"
    );
    assert_eq!(
        warnings.len(),
        2,
        "one sentence per boundary net, and this deck has two: {warnings:?}"
    );
}

#[test]
fn a_five_volt_rail_sets_the_mixed_boundary_output_level() {
    let directory = TempDirectory::new("mixed-5v");
    let model = directory.write("t4.va", TIMER_MODULE);
    let (result, warnings) =
        supply_warnings_of(|| run(&mixed_deck(&model, FIVE_VOLT_RAIL), 6e-9, 0.05e-9));
    assert_level("mixed_5v_rail_output", value_at(&result, "q", 5e-9), 5.0);
    let warning = the_only_warning(&warnings);
    println!("SUPPLY warning={warning}");
    assert!(
        warning.contains("derived from V1 = 5 V"),
        "the mixed route derives the same rail the auto-bridge route does, got: {warning}"
    );
}

/// One deck, two boundary nets, two rails: the supply is a property of the net.
#[test]
fn one_deck_answers_each_boundary_net_from_its_own_rail() {
    let deck = inverter_deck(
        "Vone vone 0 DC 1\nVfive vfive 0 DC 5\n",
        "vone",
        "vfive",
        "",
    );
    let (result, warnings) = supply_warnings_of(|| run(&deck, 100e-9, 1e-9));
    assert_level("two_net_dout_high", value_at(&result, "dout", 0.0), 5.0);
    // `din` sits on the 1 V rail, so its threshold is 0.5 V and the 0-to-5 V
    // ramp crosses it in the first tenth of the run.
    let fall = fall_time_ns(&result, "dout");
    println!("SUPPLY two_net_fall_time_ns={fall:.3}");
    assert!(
        fall < 20.0,
        "a 0.5 V threshold is crossed early in the ramp, got {fall} ns"
    );
    assert_eq!(
        warnings.len(),
        2,
        "one sentence per boundary net, not per route: {warnings:?}"
    );
    assert!(
        warnings.iter().any(|line| line.contains("VONE = 1 V"))
            && warnings.iter().any(|line| line.contains("VFIVE = 5 V")),
        "each net names its own rail: {warnings:?}"
    );
}

//=============================================================================
// What outranks the derivation, and what has no rail at all
//=============================================================================

#[test]
fn an_explicit_supply_parameter_outranks_the_rail_and_says_nothing() {
    let (result, warnings) = supply_warnings_of(|| {
        run(
            &auto_bridge_deck(FIVE_VOLT_RAIL, ".param vcc=3.3\n"),
            10e-9,
            0.5e-9,
        )
    });
    assert_level("explicit_vcc_output", value_at(&result, "mix", 5e-9), 3.3);
    assert!(
        warnings.is_empty(),
        "a deck that states its supply is not told what its rails are: {warnings:?}"
    );
}

#[test]
fn a_connect_rule_supply_outranks_the_rail_and_says_nothing() {
    let directory = TempDirectory::new("vsup");
    let library = directory.write_library(
        "\
connectrules deck;
    connect a2d #(.vsup(1.0));
    connect d2a #(.vsup(1.0));
endconnectrules
",
    );
    let deck = inverter_deck(
        FIVE_VOLT_RAIL,
        "vdd",
        "vdd",
        &format!(".veriloga \"{}\"\n", slashed(&library)),
    );
    let (result, warnings) = supply_warnings_of(|| run(&deck, 100e-9, 1e-9));
    assert_level("vsup_dout_high", value_at(&result, "dout", 0.0), 1.0);
    assert!(
        warnings.is_empty(),
        "a boundary whose connect statement states vsup derives nothing: {warnings:?}"
    );
}

#[test]
fn a_boundary_no_rail_reaches_defaults_to_three_point_three() {
    let deck = "* nothing but a load on the boundary net\n\
                Rload mix 0 1k\n\
                .model pull d_pullup\n\
                apull [mix] pull\n\
                .end\n";
    let (result, warnings) = supply_warnings_of(|| run(deck, 10e-9, 0.5e-9));
    assert_level("no_rail_output", value_at(&result, "mix", 5e-9), 3.3);
    let warning = the_only_warning(&warnings);
    println!("SUPPLY warning={warning}");
    assert!(
        warning.contains("no supply reaches boundary net")
            && warning.contains("defaulting to 3.3 V"),
        "the default is said out loud too, got: {warning}"
    );
}

/// A rail only reaches a boundary through the passive network. A source card
/// carrying a waveform is stimulus rather than supply, and ground is not a
/// path: every rail touches it, so a walk through it would reach everything.
#[test]
fn a_stimulus_source_is_not_a_supply() {
    let deck = "* a ramp reaching the boundary net, and no rail anywhere\n\
                Vramp in 0 PWL(0 0 10n 5)\n\
                Rin in mix 1k\n\
                Rload mix 0 1k\n\
                .model pull d_pullup\n\
                apull [mix] pull\n\
                .end\n";
    let (_, warnings) = supply_warnings_of(|| run(deck, 10e-9, 0.5e-9));
    let warning = the_only_warning(&warnings);
    println!("SUPPLY warning={warning}");
    assert!(
        warning.contains("no supply reaches boundary net"),
        "a PWL card is not a rail, got: {warning}"
    );
}

//=============================================================================
// Two rails is a refusal
//=============================================================================

#[test]
fn two_rails_reaching_one_boundary_are_refused() {
    let deck = "* a 1.8 V core rail and a 5 V I/O rail on one boundary net\n\
                Vcore vcore 0 DC 1.8\n\
                Vio vio 0 DC 5\n\
                Rcore mix vcore 10k\n\
                Rio mix vio 10k\n\
                .model pull d_pullup\n\
                apull [mix] pull\n\
                .end\n";
    let error = build_error(deck);
    println!("SUPPLY refusal={error}");
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("boundary net") && lowered.contains("'mix'"),
        "the refusal names the net, got: {error}"
    );
    assert!(
        error.contains("VCORE = 1.8 V") && error.contains("VIO = 5 V"),
        "the refusal names both rails and both levels, got: {error}"
    );
    assert!(
        error.contains(".param vcc=<volts>") && error.contains("vsup"),
        "the refusal names both fixes, got: {error}"
    );
}

/// The refusal is about ambiguity, not about having two rails: a deck that
/// states its supply is never asked which rail it meant.
#[test]
fn two_rails_with_an_explicit_supply_parameter_build() {
    let deck = "* two rails and a deck that says which supply the boundary uses\n\
                .param vcc=1.8\n\
                Vcore vcore 0 DC 1.8\n\
                Vio vio 0 DC 5\n\
                Rcore mix vcore 10k\n\
                Rio mix vio 10k\n\
                .model pull d_pullup\n\
                apull [mix] pull\n\
                .end\n";
    let (result, warnings) = supply_warnings_of(|| run(deck, 10e-9, 0.5e-9));
    assert_level(
        "two_rails_explicit_output",
        value_at(&result, "mix", 5e-9),
        1.8,
    );
    assert!(warnings.is_empty(), "nothing is derived: {warnings:?}");
}
