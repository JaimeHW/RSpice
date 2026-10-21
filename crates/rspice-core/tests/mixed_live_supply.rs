#![cfg(feature = "veriloga")]
//! A converter bound live to its supply net (plan ruling D-C, §8.2 rule 3).
//!
//! A connect statement naming a net (`#(.vsup(vdd))`), or a rail the resolver
//! infers, binds the boundary to that net's voltage: a D/A level is the
//! fraction of V(sup) its state selects (1 = V(sup), 0 = ground, X = V(sup)/2),
//! and an A/D threshold is the fraction of V(sup) the static rule states
//! (V(sup)/2). A numeric `vsup` or a `.param vcc` stays static, as Spectre's
//! default connect rules do.
//!
//! Every expected value here is a closed form of the deck, never an RSpice
//! output: an unloaded held 1 is V(vdd); a 20 ohm built-in driver into a 1k
//! load is V(vdd)·1000/1020, and so is its AC gain from the supply; a constant
//! input crosses a falling V(vdd)/2 threshold where V(vdd) = 2·V(in).

use rspice_core::engine::TransientResult;
use rspice_core::netlist::{AnalysisCommand, DcSweepSpec};
use rspice_core::{Complex64, Engine, Netlist};
use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(extension: &str, source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_live_supply_{}_{}.{extension}",
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

/// The shipped connect modules followed by `rules`, which select them.
fn connect_library(rules: &str) -> SourceFile {
    let mut source = String::new();
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(rules);
    SourceFile::new("va", &source)
}

/// Both converters bound live to `vdd`.
const LIVE_RULES: &str = "\
connectrules live;
    connect a2d #(.vsup(vdd));
    connect d2a #(.vsup(vdd));
endconnectrules
";

/// The A/D bound live to `vdd`, the D/A static at 5 V.
const LIVE_INPUT_RULES: &str = "\
connectrules live_input;
    connect a2d #(.vsup(vdd));
    connect d2a #(.vsup(5.0));
endconnectrules
";

/// A Verilog module holding a 1 from time zero.
const HOLD_ONE: &str = "module hold1(q);\n output q; reg q;\n initial q = 1'b1;\nendmodule\n";

/// A Verilog buffer: `y` follows `a`.
const BUFFER: &str = "module buf1(a, y);\n input a; output y;\n assign y = a;\nendmodule\n";

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn tran(deck: &Netlist, tstop: f64, step: f64) -> TransientResult {
    Engine::default()
        .run_tran(deck, tstop, step)
        .unwrap_or_else(|error| panic!("transient solves: {error}"))
}

fn series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[index]
}

/// `(time, v(node), v(vdd))` at every accepted point.
fn against_supply(result: &TransientResult, node: &str) -> Vec<(f64, f64, f64)> {
    let values = series(result, node);
    let supply = series(result, "vdd");
    result
        .time
        .iter()
        .zip(values.iter().zip(supply))
        .map(|(time, (value, supply))| (*time, *value, *supply))
        .collect()
}

/// The first time `node` rises through `level`, linearly interpolated.
fn rise_through(result: &TransientResult, node: &str, level: f64) -> f64 {
    let values = series(result, node);
    for index in 1..values.len() {
        let (before, after) = (values[index - 1], values[index]);
        if before < level && after >= level {
            let (t0, t1) = (result.time[index - 1], result.time[index]);
            return t0 + (t1 - t0) * (level - before) / (after - before);
        }
    }
    panic!("{node} never rises through {level}")
}

/// The first time `node` falls through `level`, linearly interpolated.
fn fall_through(result: &TransientResult, node: &str, level: f64) -> f64 {
    let values = series(result, node);
    for index in 1..values.len() {
        let (before, after) = (values[index - 1], values[index]);
        if before > level && after <= level {
            let (t0, t1) = (result.time[index - 1], result.time[index]);
            return t0 + (t1 - t0) * (before - level) / (before - after);
        }
    }
    panic!("{node} never falls through {level}")
}

/// The Verilog-AMS route: a held 1 through the built-in D/A, unloaded.
fn vams_hold_deck(module: &SourceFile, rules: Option<&SourceFile>, extra: &str) -> Netlist {
    let rules = rules.map_or(String::new(), |rules| {
        format!(".veriloga \"{}\"\n", rules.path())
    });
    parse(&format!(
        "* live supply, Verilog-AMS route\n\
         {extra}\
         vdd vdd 0 pwl(0 0 1u 3.3)\n\
         xh q hold1\n\
         .hdl \"{}\"\n\
         {rules}\
         .end\n",
        module.path()
    ))
}

/// The XSPICE auto-bridge route: a `d_pullup` held 1 through the connect
/// rule's D/A. The pull-up is a resistive 1, so the converter drives V(vdd)
/// behind `rpull` = 10 kΩ, and the 1 Meg load (which makes `q` an analog
/// node) divides it: v(q) = V(vdd)·[`PULL_DIVIDER`].
fn xspice_hold_deck(rules: &SourceFile) -> Netlist {
    parse(&format!(
        "* live supply, XSPICE route\n\
         vdd vdd 0 pwl(0 0 1u 3.3)\n\
         apull [q] pull\n\
         .model pull d_pullup\n\
         rq q 0 1meg\n\
         .veriloga \"{}\"\n\
         .end\n",
        rules.path()
    ))
}

/// The ramp's own value at `time`.
fn ramp(time: f64) -> f64 {
    3.3 * (time / 1.0e-6).min(1.0)
}

// ---------------------------------------------------------------------------
// Transient
// ---------------------------------------------------------------------------

/// A held 1 is V(vdd) at every accepted point of a 0 → 3.3 V ramp: the
/// converter is unloaded, so its 20 ohm drives no current and the level is
/// the supply itself.
#[test]
fn a_held_one_follows_a_supply_ramp() {
    let module = SourceFile::new("v", HOLD_ONE);
    let rules = connect_library(LIVE_RULES);
    let result = tran(&vams_hold_deck(&module, Some(&rules), ""), 2.0e-6, 10.0e-9);
    let points = against_supply(&result, "q");
    assert!(
        points.len() > 50,
        "the ramp is resolved: {} points",
        points.len()
    );
    for (time, level, supply) in points {
        assert!(
            (supply - ramp(time)).abs() < 1.0e-9,
            "v(vdd) is the PWL at {time:e}: {supply}"
        );
        assert!(
            (level - supply).abs() <= 1.0e-9 + 1.0e-9 * supply.abs(),
            "a held 1 is V(vdd) at {time:e}: v(q) = {level}, v(vdd) = {supply}"
        );
    }
}

/// The resistive 1 of [`xspice_hold_deck`] into its 1 Meg load:
/// 1 MΩ/(1 MΩ + 10 kΩ).
const PULL_DIVIDER: f64 = 1.0e6 / (1.0e6 + 10.0e3);

/// The XSPICE auto-bridge route binds the same way, and the two routes agree
/// point for point on the supply's own value, the XSPICE route's resistive
/// pull-up through its load divider.
#[test]
fn the_xspice_and_verilog_ams_routes_agree() {
    let module = SourceFile::new("v", HOLD_ONE);
    let rules = connect_library(LIVE_RULES);
    let xspice = tran(&xspice_hold_deck(&rules), 2.0e-6, 10.0e-9);
    let points = against_supply(&xspice, "q");
    assert!(
        points.len() > 50,
        "q is an analog node: {} points",
        points.len()
    );
    for (time, level, supply) in points {
        let expected = supply * PULL_DIVIDER;
        assert!(
            (level - expected).abs() <= 1.0e-9 + 1.0e-9 * supply.abs(),
            "the auto D/A's held resistive 1 is V(vdd)·{PULL_DIVIDER} at {time:e}: \
             v(q) = {level}, v(vdd) = {supply}"
        );
    }
    let vams = tran(&vams_hold_deck(&module, Some(&rules), ""), 2.0e-6, 10.0e-9);
    for time in [0.25e-6, 0.5e-6, 0.75e-6, 1.5e-6] {
        let sample = |result: &TransientResult| {
            let index = result
                .time
                .iter()
                .position(|sample| *sample >= time)
                .expect("a sample at or after the time");
            let (t0, t1) = (result.time[index.max(1) - 1], result.time[index]);
            let values = series(result, "q");
            let (v0, v1) = (values[index.max(1) - 1], values[index]);
            if t1 == t0 {
                v1
            } else {
                v0 + (v1 - v0) * (time - t0) / (t1 - t0)
            }
        };
        let (a, b) = (sample(&xspice) / PULL_DIVIDER, sample(&vams));
        assert!(
            (a - ramp(time)).abs() < 1.0e-6 && (b - ramp(time)).abs() < 1.0e-6,
            "both routes drive V(vdd) = {} at {time:e}: XSPICE {a}, Verilog-AMS {b}",
            ramp(time)
        );
    }
}

/// A constant 1 V input and a supply falling 3.3 → 1.0 V over 1 µs: the
/// threshold V(vdd)/2 falls through the input where V(vdd) = 2.0 V, at
/// t = (3.3 − 2.0)/2.3 µs = 0.565217 µs. Nothing about the input moves, so the
/// edge is the supply's alone, and it is dated at that instant like any
/// other crossing. The buffer and the converters add no delay (the built-in
/// converters' defaults), and the static 5 V D/A shows the edge.
#[test]
fn a_crossing_the_supply_causes_is_dated_at_the_analytic_instant() {
    let module = SourceFile::new("v", BUFFER);
    let rules = connect_library(LIVE_INPUT_RULES);
    let deck = parse(&format!(
        "* supply-caused crossing\n\
         vdd vdd 0 pwl(0 3.3 1u 1.0)\n\
         vin a 0 dc 1.0\n\
         xb a y buf1\n\
         .hdl \"{}\"\n\
         .veriloga \"{}\"\n\
         .end\n",
        module.path(),
        rules.path()
    ));
    let result = tran(&deck, 1.0e-6, 10.0e-9);
    let first = series(&result, "y")[0];
    assert!(first.abs() < 1.0e-9, "y starts low (1 V < 1.65 V): {first}");
    let edge = rise_through(&result, "y", 2.5);
    let expected = 1.3 / 2.3 * 1.0e-6;
    println!("LIVE crossing={edge:e} expected={expected:e}");
    assert!(
        (edge - expected).abs() < 2.0e-12,
        "the supply-caused crossing is dated at {expected:e}, got {edge:e}"
    );
}

/// The built-in A/D of an HDL input whose `supplySensitivity` names `vddl`, a
/// 1.2 V rail, converts against V(vddl) although the deck says
/// `.param vcc=3.3`: the triangle 0 -> 1.2 -> 0 V over 2 us crosses
/// V(vddl)/2 = 0.6 V at 0.5 us and 1.5 us, so the buffered output (a D/A at
/// vcc = 3.3 V, unloaded) passes 1.65 V at exactly those instants. Against
/// vcc/2 = 1.65 V the input would never cross at all.
#[test]
fn a_supply_sensitivity_attribute_outranks_the_deck_supply() {
    use rspice_core::circuit::ConverterSource;

    let module = SourceFile::new(
        "v",
        &BUFFER.replace(" input a;", " (* supplySensitivity = \"vddl\" *) input a;"),
    );
    let deck = parse(&format!(
        "* supplySensitivity on a built-in A/D\n\
         .param vcc=3.3\n\
         vddl vddl 0 1.2\n\
         vin a 0 pwl(0 0 1u 1.2 2u 0)\n\
         xb a y buf1\n\
         .hdl \"{}\"\n\
         .end\n",
        module.path()
    ));
    let result = tran(&deck, 2.0e-6, 10.0e-9);
    let rise = rise_through(&result, "y", 1.65);
    let fall = fall_through(&result, "y", 1.65);
    for (label, edge, expected) in [("rise", rise, 0.5e-6), ("fall", fall, 1.5e-6)] {
        println!("ATTRIBUTE {label}={edge:e} expected={expected:e}");
        assert!(
            (edge - expected).abs() < 2.0e-12,
            "the {label} is dated at {expected:e}, got {edge:e}"
        );
    }

    let circuit = Engine::default()
        .build_circuit(&deck)
        .expect("the deck builds");
    let supplies: Vec<_> = circuit
        .converter_insertions()
        .iter()
        .filter_map(|record| record.supply.as_ref())
        .collect();
    assert!(
        supplies.iter().any(|supply| {
            supply.source == ConverterSource::PortAttribute("supplySensitivity".into())
                && supply
                    .net
                    .as_deref()
                    .is_some_and(|net| net.eq_ignore_ascii_case("vddl"))
                && supply.level == 1.2
        }),
        "the A/D's record names the attribute and its net: {supplies:?}"
    );
    assert!(
        supplies.iter().any(|supply| {
            supply.source == ConverterSource::Parameter("vcc".into()) && supply.net.is_none()
        }),
        "the output's D/A keeps .param vcc: {supplies:?}"
    );
}

/// A `.param vcc` and a numeric `vsup` are static: the same ramp leaves a
/// held 1 at the stated level at every point (to the solver's own residual,
/// 1e-13 V here, nowhere near the ramp's volts).
#[test]
fn an_explicit_supply_stays_static() {
    let module = SourceFile::new("v", HOLD_ONE);
    let parameter = tran(
        &vams_hold_deck(&module, None, ".param vcc=3.3\n"),
        2.0e-6,
        10.0e-9,
    );
    let numeric = connect_library(
        "connectrules numeric;\n    connect a2d #(.vsup(3.3));\n    connect d2a #(.vsup(3.3));\nendconnectrules\n",
    );
    let stated = tran(
        &vams_hold_deck(&module, Some(&numeric), ""),
        2.0e-6,
        10.0e-9,
    );
    for (label, result) in [(".param vcc", &parameter), ("vsup = 3.3", &stated)] {
        for (time, level, _) in against_supply(result, "q") {
            assert!(
                (level - 3.3).abs() < 1.0e-9,
                "{label} is static: v(q) = {level} at {time:e}, not the ramp"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// DC and small-signal
// ---------------------------------------------------------------------------

/// `(sweep value, v(node))` at every point of the deck's `.dc` card.
fn sweep(netlist: &Netlist, node: &str) -> Vec<(f64, f64)> {
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
                point
                    .result
                    .try_voltage_named(node)
                    .expect("the node is published"),
            )
        })
        .collect()
}

/// An inferred rail binds live, which a sweep of the rail itself shows.
///
/// `V1` is the only rail; `r1`/`r2` divide it to 2/3·V(vdd) at `a`, and
/// `rpull` ties `y` to it, so both boundaries are reached by `V1` and infer it.
/// Live, `a` (2/3·V(vdd)) is above V(vdd)/2 at every point, so `y` is 1 and
/// drives V(vdd) into a node `rpull` also holds at V(vdd): v(y) = V(vdd)
/// exactly. A static 3.3 V supply would read `a` low below 2.475 V and drive
/// `y` to (3.3/20 + V/10k)/(1/20 + 1/10k) above it.
///
/// A deck that names no node 0 takes `V1`'s negative net, `ref`, as its
/// reference, and a rail stated against that net is a rail exactly as one
/// stated against `0` is.
#[test]
fn an_inferred_rail_binds_live_in_a_sweep_of_the_rail() {
    let module = SourceFile::new("v", BUFFER);
    for ground in ["0", "ref"] {
        let deck = parse(&format!(
            "* inferred live rail\n\
             V1 vdd {ground} DC 3.3\n\
             r1 vdd a 1k\n\
             r2 a {ground} 2k\n\
             rpull y vdd 10k\n\
             xb a y buf1\n\
             .hdl \"{}\"\n\
             .dc V1 0.1 3.6 0.1\n\
             .end\n",
            module.path()
        ));
        let points = sweep(&deck, "y");
        assert_eq!(
            points.len(),
            36,
            "every sweep point solves with ground {ground}: {points:?}"
        );
        for (supply, level) in points {
            assert!(
                (level - supply).abs() <= 1.0e-9 + 1.0e-9 * supply,
                "v(y) = V(vdd) = {supply} V live with ground {ground}, got {level}"
            );
        }
    }
}

/// A supply ripple reaches a held 1's output through its live level: the
/// held level's gain from V(sup) is 1, behind 20 ohm into 1k, so
/// v(q)/v(vdd) = 1000/1020 at every frequency, with no phase. Static, the
/// same deck has no path from the supply to `q` at all.
#[test]
fn a_supply_ripple_reaches_a_held_output_through_its_live_level() {
    let module = SourceFile::new("v", HOLD_ONE);
    let deck = |rules: &str| {
        let library = connect_library(rules);
        let netlist = parse(&format!(
            "* supply ripple\n\
             vdd vdd 0 dc 3.3 ac 1\n\
             xh q hold1\n\
             rl q 0 1k\n\
             .hdl \"{}\"\n\
             .veriloga \"{}\"\n\
             .end\n",
            module.path(),
            library.path()
        ));
        (netlist, library)
    };
    let engine = Engine::default();
    let ac_q = |netlist: &Netlist| -> Vec<Complex64> {
        engine
            .run_ac(netlist, &[1.0e3, 1.0e6, 1.0e9])
            .expect(".ac runs on the held state")
            .iter()
            .map(|point| {
                let index = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("q"))
                    .expect("q is published");
                point.voltages[index]
            })
            .collect()
    };

    let (live, _live_library) = deck(LIVE_RULES);
    let op = engine.run_dc_op(&live).expect("the operating point solves");
    let held = op.try_voltage_named("q").expect("q is published");
    let expected_op = 3.3 * 1000.0 / 1020.0;
    assert!(
        (held - expected_op).abs() < 1.0e-9,
        "the held 1 is V(vdd)·1000/1020 = {expected_op}: {held}"
    );
    let gain = 1000.0 / 1020.0;
    for value in ac_q(&live) {
        assert!(
            (value.re - gain).abs() < 1.0e-9 && value.im.abs() < 1.0e-9,
            "v(q)/v(vdd) = {gain} with no phase, got {value}"
        );
    }

    let (stated, _stated_library) = deck(
        "connectrules numeric;\n    connect a2d #(.vsup(3.3));\n    connect d2a #(.vsup(3.3));\nendconnectrules\n",
    );
    for value in ac_q(&stated) {
        assert!(
            value.norm() < 1.0e-12,
            "a static supply carries no ripple to q, got {value}"
        );
    }
}

// ---------------------------------------------------------------------------
// Supply ports on authored connect modules
// ---------------------------------------------------------------------------

/// An authored D/A with a supply port: a 1 drives V(vsup) behind 20 ohm, a 0
/// drives ground. Into a 1k load a held 1 is V(vsup)·1000/1020.
const D2A_SUP: &str = "\
connectmodule d2a_sup(d, a, vsup);
    input d; logic d;
    output a; electrical a;
    inout vsup; electrical vsup;
    analog I(a) <+ (V(a) - (d ? V(vsup) : 0.0)) / 20.0;
endmodule
";

/// [`D2A_SUP`] with its supply declared by `declaration`, selected by
/// `statement`.
fn d2a_sup_library(declaration: &str, statement: &str) -> SourceFile {
    SourceFile::new(
        "va",
        &format!(
            "{}connectrules supplied;\n    {statement}\nendconnectrules\n",
            D2A_SUP.replace("electrical vsup;", declaration)
        ),
    )
}

/// A held 1 into 1k beside a 2.5 V rail `vdd` and a 1.8 V rail `v18`, with
/// `extra` cards; `hold` is the HDL module holding the 1.
fn supplied_deck(hold: &SourceFile, library: &SourceFile, supply: &str, extra: &str) -> Netlist {
    parse(&format!(
        "* supply port\n\
         vdd vdd 0 {supply}\n\
         v18 v18 0 1.8\n\
         xh q hold1\n\
         rl q 0 1k\n\
         {extra}\
         .hdl \"{}\"\n\
         .veriloga \"{}\"\n\
         .end\n",
        hold.path(),
        library.path()
    ))
}

/// [`HOLD_ONE`] with its port's `supplySensitivity` naming `net`.
fn hold_one_sensing(net: &str) -> String {
    HOLD_ONE.replace(
        " output q;",
        &format!(" (* supplySensitivity = \"{net}\" *) output q;"),
    )
}

/// Each way of binding the supply port wires it to `vdd`: the held 1 is
/// 2.5·1000/1020 = 2.450980 V, and the record names what bound it. The
/// statement outranks the port's attribute: `v18` against `vdd` gives
/// 1.8·1000/1020 = 1.764706 V.
#[test]
fn every_binding_source_wires_the_supply_port_to_its_net() {
    use rspice_core::circuit::ConverterSource;

    let plain = SourceFile::new("v", HOLD_ONE);
    let sensing = SourceFile::new("v", &hold_one_sensing("vdd"));
    let bare = "connect d2a_sup;";
    let cases = [
        (
            "statement",
            &plain,
            d2a_sup_library("electrical vsup;", "connect d2a_sup #(.vsup(vdd));"),
            "",
            ConverterSource::ConnectStatement,
            "vdd",
            2.5,
        ),
        (
            "attribute",
            &sensing,
            d2a_sup_library("electrical vsup;", bare),
            "",
            ConverterSource::PortAttribute("supplySensitivity".into()),
            "vdd",
            2.5,
        ),
        (
            "module default",
            &plain,
            d2a_sup_library(
                "electrical (* integer inh_conn_def_value = \"cds_globals.\\\\vdd \"; *) vsup;",
                bare,
            ),
            "",
            ConverterSource::ModuleDefault,
            "vdd",
            2.5,
        ),
        (
            // A 1e15 ohm leak is the resistive path that makes vdd the one
            // rail reaching q; it moves v(q) by 2e-14 relative.
            "inference",
            &plain,
            d2a_sup_library("electrical vsup;", bare),
            "rleak q vdd 1e15\n",
            ConverterSource::Inferred {
                source: "VDD".into(),
                level: 2.5,
            },
            "vdd",
            2.5,
        ),
        (
            "statement over attribute",
            &sensing,
            d2a_sup_library("electrical vsup;", "connect d2a_sup #(.vsup(v18));"),
            "",
            ConverterSource::ConnectStatement,
            "v18",
            1.8,
        ),
    ];
    let engine = Engine::default();
    for (case, hold, library, extra, source, net, supply) in cases {
        let deck = supplied_deck(hold, &library, "2.5", extra);
        let op = engine
            .run_dc_op(&deck)
            .unwrap_or_else(|error| panic!("{case}: the operating point solves: {error}"));
        let held = op.try_voltage_named("q").expect("q is published");
        let expected = supply * 1000.0 / 1020.0;
        assert!(
            (held - expected).abs() <= 1.0e-9 * expected,
            "{case}: the held 1 is V({net})·1000/1020 = {expected}, got {held}"
        );

        let circuit = engine.build_circuit(&deck).expect("the deck builds");
        let record = circuit
            .converter_insertions()
            .iter()
            .find(|record| record.module == "d2a_sup")
            .unwrap_or_else(|| panic!("{case}: d2a_sup is recorded"));
        let [port] = record.supply_ports.as_slice() else {
            panic!("{case}: one supply port: {record:?}");
        };
        assert_eq!(port.port.as_deref(), Some("vsup"), "{case}");
        assert!(
            port.net
                .as_deref()
                .is_some_and(|bound| bound.eq_ignore_ascii_case(net)),
            "{case}: bound to {net}: {port:?}"
        );
        assert_eq!(port.source, source, "{case}");
        assert_eq!(record.supply.as_ref(), Some(port), "{case}");
    }
}

/// A supply port nothing binds is refused, naming the module, the port, the
/// boundary node and the instance: a waveform source is stimulus, never an
/// inferred rail, and no level is assumed.
#[test]
fn an_unbound_supply_port_is_refused_by_name() {
    let hold = SourceFile::new("v", HOLD_ONE);
    let library = d2a_sup_library("electrical vsup;", "connect d2a_sup;");
    let deck = supplied_deck(&hold, &library, "pwl(0 0 1n 2.5)", "");
    let error = Engine::default()
        .run_dc_op(&deck)
        .expect_err("an unbound supply port is refused")
        .to_string();
    let lower = error.to_ascii_lowercase();
    for named in [
        "'d2a_sup'",
        "supply port 'vsup'",
        "node 'q'",
        "xh",
        "supplysensitivity",
        "inh_conn_def_value",
    ] {
        assert!(lower.contains(named), "names {named}: {error}");
    }
}

/// A supply port that takes its net from a Cadence netSet property, with no
/// default beside it, is refused naming the property.
#[test]
fn a_supply_port_asking_for_a_netset_property_is_refused_by_name() {
    let hold = SourceFile::new("v", HOLD_ONE);
    let library = d2a_sup_library(
        "electrical (* integer inh_conn_prop_name = \"vdd\"; *) vsup;",
        "connect d2a_sup;",
    );
    let deck = supplied_deck(&hold, &library, "2.5", "");
    let error = Engine::default()
        .run_dc_op(&deck)
        .expect_err("a netSet binding is refused")
        .to_string();
    assert!(
        error.contains("inherited-connection property 'vdd'") && error.contains("netSet"),
        "names the property: {error}"
    );
}

/// The supply port carries ripple: the held 1's output follows V(vdd) behind
/// 20 ohm into 1k, so v(q)/v(vdd) = 1000/1020 = 0.980392 with no phase.
#[test]
fn a_supply_port_carries_the_supply_ripple() {
    let hold = SourceFile::new("v", HOLD_ONE);
    let library = d2a_sup_library("electrical vsup;", "connect d2a_sup #(.vsup(vdd));");
    let deck = supplied_deck(&hold, &library, "dc 2.5 ac 1", "");
    let gain = 1000.0 / 1020.0;
    let points = Engine::default()
        .run_ac(&deck, &[1.0e3, 1.0e6])
        .expect(".ac runs on the held state");
    for point in points {
        let index = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("q"))
            .expect("q is published");
        let value = point.voltages[index];
        assert!(
            (value.re - gain).abs() <= 1.0e-9 && value.im.abs() <= 1.0e-9,
            "v(q)/v(vdd) = {gain} with no phase, got {value}"
        );
    }
}
