//! Terminal currents of Verilog-A instances, on both routes and in every
//! analysis that reports a current.
//!
//! A terminal a deck names is metered: an ideal zero-volt source in series
//! with it, whose branch unknown is the current entering the terminal in the
//! solved system. Each case here sets a Verilog-A device beside the native
//! SPICE device it describes, both driven from the same source, and holds the
//! instance's current to the native device's own: `I(r1)`, `I(d1)`, `I(c1)`
//! where the engine reports one, and in AC — which reports no device current
//! — the branch current of a zero-volt source in series with the native
//! device, AC having no nodal GMIN to leak around it.
//!
//! The decks are small, so an operating point or a DC sweep agrees to solver
//! round-off; a transient agrees to well inside its own tolerance, because
//! the two halves share every time point.
#![cfg(feature = "veriloga")]

use rspice_core::analysis::AcResult;
use rspice_core::engine::TransientResult;
use rspice_core::execution::{
    AnalysisResultKind, ProjectedSignals, ProjectionSource, ProjectionSourceSignal,
    ProjectionValues, SignalKind, SignalProjection, SignalUnit, dc_sweep_observable_series,
    observable_lookup, operating_point_observable_series, operating_point_projection_signals,
    transient_projection_signals,
};
use rspice_core::solver::SimulationResult;
use rspice_core::{Engine, Netlist, NoAbort};
use std::borrow::Cow;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Relative agreement asked of an operating point or a DC sweep point.
const STATIC_RELATIVE: f64 = 1.0e-12;
/// Absolute floor, so a current through zero is not compared by ratio.
const ABSOLUTE_FLOOR: f64 = 1.0e-18;
/// Relative agreement asked of a transient sample: SPICE's `RELTOL`.
const TRANSIENT_RELATIVE: f64 = 1.0e-3;

struct ModelFile(PathBuf);

impl ModelFile {
    fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_terminal_current_{name}_{}_{sequence}.va",
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

const RESISTOR: &str = "`include \"disciplines.vams\"\n\
module va_res(p, n); inout p, n; electrical p, n; parameter real r = 1k;\n\
analog I(p, n) <+ V(p, n) / r;\n\
endmodule\n";

/// The same resistor with ports named as a real module names them.
const PORTS_RESISTOR: &str = "`include \"disciplines.vams\"\n\
module va_ports(inp, out); inout inp, out; electrical inp, out; parameter real r = 1k;\n\
analog I(inp, out) <+ V(inp, out) / r;\n\
endmodule\n";

/// The native diode's equation with its thermal voltage and GMIN written
/// out, so the comparison measures the meter rather than two spellings of
/// Boltzmann's constant.
const DIODE: &str = "`include \"disciplines.vams\"\n\
module va_diode(a, c); inout a, c; electrical a, c;\n\
parameter real is = 1e-14; parameter real vt = 0.025; parameter real gmin = 1e-12;\n\
analog I(a, c) <+ is * (exp(V(a, c) / vt) - 1.0) + gmin * V(a, c);\n\
endmodule\n";

const CAPACITOR: &str = "`include \"disciplines.vams\"\n\
module va_cap(p, n); inout p, n; electrical p, n; parameter real c = 1p;\n\
analog I(p, n) <+ ddt(c * V(p, n));\n\
endmodule\n";

/// A conductance from `a` to `c` and a transconductance into `b`: a native
/// resistor and a native VCCS between the same three nodes.
const THREE_TERMINAL: &str = "`include \"disciplines.vams\"\n\
module va_tri(a, b, c); inout a, b, c; electrical a, b, c;\n\
parameter real g1 = 1m; parameter real g2 = 2m;\n\
analog begin\n\
  I(a, c) <+ g1 * V(a, c);\n\
  I(b, c) <+ g2 * V(a, c);\n\
end\n\
endmodule\n";

fn netlist(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn operating_point(deck: &str) -> SimulationResult {
    Engine::default()
        .run_dc_op(&netlist(deck))
        .unwrap_or_else(|error| panic!("operating point solves: {error}\n{deck}"))
}

fn observable(result: &SimulationResult, name: &str) -> f64 {
    result
        .try_dc_observable_named(name)
        .unwrap_or_else(|| panic!("{name} is published; have {:?}", result.dc_observables))
}

fn agrees(actual: f64, expected: f64, relative: f64) -> bool {
    (actual - expected).abs() <= relative * expected.abs().max(actual.abs()) + ABSOLUTE_FLOOR
}

fn assert_static(label: &str, actual: f64, expected: f64) {
    assert!(
        agrees(actual, expected, STATIC_RELATIVE),
        "{label}: {actual:e} against the native {expected:e} (relative {:e})",
        (actual - expected).abs() / expected.abs().max(f64::MIN_POSITIVE)
    );
}

fn transient(deck: &str, tstop: f64, step: f64) -> TransientResult {
    Engine::default()
        .run_tran(&netlist(deck), tstop, step)
        .unwrap_or_else(|error| panic!("transient runs: {error}\n{deck}"))
}

fn derived(result: &TransientResult, name: &str) -> Vec<f64> {
    result
        .try_branch_current_waveform_named(name)
        .unwrap_or_else(|| panic!("I({name}) is retained; have {:?}", result.branch_names))
        .to_vec()
}

fn lead(result: &TransientResult, device: &str, parameter: &str) -> Vec<f64> {
    result
        .try_device_op_waveform_named(device, parameter)
        .unwrap_or_else(|| panic!("@{device}[{parameter}] is retained"))
        .to_vec()
}

/// Hold a transient waveform to its native twin at every sample, and return
/// the largest relative deviation for the record.
fn assert_waveform(label: &str, actual: &[f64], expected: &[f64]) -> f64 {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{label}: one value per sample"
    );
    let scale = expected
        .iter()
        .fold(0.0_f64, |scale, value| scale.max(value.abs()));
    let mut worst = 0.0_f64;
    for (index, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert!(a.is_finite(), "{label}: sample {index} is {a}");
        let deviation = (a - e).abs() / scale.max(f64::MIN_POSITIVE);
        worst = worst.max(deviation);
        assert!(
            (a - e).abs() <= TRANSIENT_RELATIVE * e.abs().max(a.abs()) + 1e-9 * scale,
            "{label}: sample {index} is {a:e} against the native {e:e}"
        );
    }
    eprintln!("{label}: worst deviation {worst:e} of the peak");
    worst
}

fn ac(deck: &str, frequencies: &[f64]) -> Vec<AcResult> {
    Engine::default()
        .run_ac(&netlist(deck), frequencies)
        .unwrap_or_else(|error| panic!("AC runs: {error}\n{deck}"))
}

fn ac_branch(point: &AcResult, name: &str) -> rspice_core::Complex64 {
    let index = point
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("I({name}) is an AC branch; have {:?}", point.branch_names));
    point.currents[index]
}

fn assert_complex(label: &str, actual: rspice_core::Complex64, expected: rspice_core::Complex64) {
    let scale = expected.norm().max(actual.norm());
    assert!(
        (actual - expected).norm() <= STATIC_RELATIVE * scale + ABSOLUTE_FLOOR,
        "{label}: {actual} against the native {expected}"
    );
}

/// Project a result onto the deck's `.SAVE` contract, as an export does.
fn project(netlist: &Netlist, source: &ProjectionSource<'_>) -> ProjectedSignals {
    SignalProjection::from_netlist(netlist)
        .expect("projection builds")
        .project(&netlist.params, source, &NoAbort)
        .unwrap_or_else(|error| panic!("the saved signals project: {error}"))
}

fn real_column<'a>(
    display: String,
    registry: &str,
    kind: SignalKind,
    values: Vec<f64>,
) -> ProjectionSourceSignal<'a> {
    ProjectionSourceSignal::new(
        display,
        registry,
        kind,
        ProjectionValues::Real(Cow::Owned(values)),
    )
    .expect("valid column")
}

fn op_projection(netlist: &Netlist, op: &SimulationResult) -> ProjectedSignals {
    let observables = operating_point_observable_series(op);
    let source = ProjectionSource::new(AnalysisResultKind::OperatingPoint, "DC OP")
        .with_axis(vec![0.0])
        .with_signals(operating_point_projection_signals(op).expect("op inventory"))
        .with_lookup(observable_lookup(&observables));
    project(netlist, &source)
}

/// Project a real-valued result whose deck may carry ordered `.PRINT`
/// columns, which the core resolver evaluates before the `.SAVE` ones.
fn project_ordered(
    netlist: &Netlist,
    source: ProjectionSource<'_>,
    ordered: impl FnOnce(&SignalProjection) -> Option<Vec<rspice_core::execution::ProjectedSignal>>,
) -> ProjectedSignals {
    let projection = SignalProjection::from_netlist(netlist).expect("projection builds");
    let source = source.with_ordered_print_columns(ordered(&projection));
    project(netlist, &source)
}

fn dc_projection(netlist: &Netlist, sweep: &[(f64, SimulationResult)]) -> ProjectedSignals {
    let first = &sweep[0].1;
    let mut signals = Vec::new();
    for (index, node) in first.node_names.iter().enumerate() {
        let values = sweep
            .iter()
            .map(|(_, point)| point.node_voltages[index])
            .collect();
        signals.push(real_column(
            format!("V({node})"),
            node,
            SignalKind::Voltage,
            values,
        ));
    }
    for (index, branch) in first.branch_names.iter().enumerate() {
        let values = sweep
            .iter()
            .map(|(_, point)| point.branch_currents[index])
            .collect();
        signals.push(real_column(
            format!("I({branch})"),
            branch,
            SignalKind::Current,
            values,
        ));
    }
    let observables = dc_sweep_observable_series(sweep);
    let axis = sweep.iter().map(|(value, _)| *value).collect::<Vec<_>>();
    let source = ProjectionSource::new(AnalysisResultKind::DcSweep, "DC")
        .with_axis(axis)
        .with_signals(signals)
        .with_lookup(observable_lookup(&observables));
    project_ordered(netlist, source, |projection| {
        projection
            .ordered_dc_columns(netlist, sweep, Default::default(), &NoAbort)
            .expect("the DC print cards evaluate")
    })
}

fn tran_projection(netlist: &Netlist, tran: &TransientResult) -> ProjectedSignals {
    let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
        .with_axis(tran.time.as_slice())
        .with_signals(transient_projection_signals(tran).expect("transient inventory"))
        .with_lookup(rspice_core::analysis::measure_signals::transient_signal_map(tran));
    project_ordered(netlist, source, |projection| {
        projection
            .ordered_transient_columns(netlist, tran, Default::default(), &NoAbort)
            .expect("the TRAN print cards evaluate")
    })
}

fn complex_column<'a>(
    points: &[AcResult],
    display: String,
    registry: &str,
    kind: SignalKind,
    pick: impl Fn(&AcResult) -> rspice_core::Complex64,
) -> ProjectionSourceSignal<'a> {
    ProjectionSourceSignal::new(
        display,
        registry,
        kind,
        ProjectionValues::Complex {
            real: Cow::Owned(points.iter().map(|point| pick(point).re).collect()),
            imag: Cow::Owned(points.iter().map(|point| pick(point).im).collect()),
        },
    )
    .expect("valid complex column")
}

fn ac_projection(netlist: &Netlist, points: &[AcResult]) -> ProjectedSignals {
    let first = &points[0];
    let mut signals = Vec::new();
    for (index, node) in first.node_names.iter().enumerate() {
        signals.push(complex_column(
            points,
            format!("V({node})"),
            node,
            SignalKind::Voltage,
            |point| point.voltages[index],
        ));
    }
    for (index, branch) in first.branch_names.iter().enumerate() {
        signals.push(complex_column(
            points,
            format!("I({branch})"),
            branch,
            SignalKind::Current,
            |point| point.currents[index],
        ));
    }
    let axis = points
        .iter()
        .map(|point| point.frequency)
        .collect::<Vec<_>>();
    let source = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
        .with_axis(axis)
        .with_signals(signals);
    project(netlist, &source)
}

/// Each named column is exported as a current in amperes, with these values.
fn assert_amperes(label: &str, projected: &ProjectedSignals, name: &str, expected: &[f64]) {
    let signal = projected
        .signals()
        .iter()
        .find(|signal| {
            signal
                .descriptor()
                .display_name()
                .eq_ignore_ascii_case(name)
        })
        .unwrap_or_else(|| {
            panic!(
                "{label}: {name} is exported; have {:?}",
                projected
                    .signals()
                    .iter()
                    .map(|signal| signal.descriptor().display_name())
                    .collect::<Vec<_>>()
            )
        });
    assert_eq!(
        signal.descriptor().kind(),
        SignalKind::Current,
        "{label}: {name}"
    );
    assert_eq!(
        signal.descriptor().unit(),
        &SignalUnit::Ampere,
        "{label}: {name}"
    );
    let values = signal
        .real()
        .map(<[f64]>::to_vec)
        .or_else(|| signal.complex().map(|(real, _)| real.to_vec()))
        .expect("a column of values");
    assert_eq!(values.len(), expected.len(), "{label}: {name}");
    for (actual, expected) in values.iter().zip(expected) {
        assert!(
            agrees(*actual, *expected, STATIC_RELATIVE),
            "{label}: {name} is {actual:e}, expected {expected:e}"
        );
    }
}

/// A Verilog-A resistor reports the native resistor's current in every
/// analysis: `I(x1)` and `@x1[ip]` name the current into `p`, `@x1[in]` the
/// current into `n`.
#[test]
fn a_verilog_a_resistor_reports_the_native_resistors_current() {
    let model = ModelFile::new("va_res", RESISTOR);
    let deck = |source: &str, analysis: &str| {
        format!(
            "* Verilog-A resistor against a native one\n\
             v1 in 0 {source}\n\
             x1 in a va_res r=1k\n\
             rl1 a 0 2k\n\
             vm in b 0\n\
             r1 b c 1k\n\
             rl2 c 0 2k\n\
             .save i(x1) @x1[ip] @x1[in] i(vm) i(r1)\n\
             {analysis}\n\
             .va \"{}\" va_res\n\
             .end\n",
            model.deck_path()
        )
    };

    let op = operating_point(&deck("dc 1.5", ".op"));
    let native = observable(&op, "I(r1)");
    assert!((native - 1.5 / 3000.0).abs() < 1e-15, "{native}");
    assert_static("op I(x1)", observable(&op, "I(x1)"), native);
    assert_static("op @x1[ip]", observable(&op, "x1:ip"), native);
    assert_static("op @x1[in]", observable(&op, "x1:in"), -native);

    let sweep = Engine::default()
        .run_dc_sweep(
            &netlist(&deck("dc 0", ".dc v1 -2 2 0.5")),
            "v1",
            -2.0,
            2.0,
            0.5,
        )
        .expect("DC sweep solves");
    assert_eq!(sweep.len(), 9);
    for (value, point) in &sweep {
        let native = observable(point, "I(r1)");
        assert_static(
            &format!("dc {value} I(x1)"),
            observable(point, "I(x1)"),
            native,
        );
        assert_static(
            &format!("dc {value} @x1[in]"),
            observable(point, "x1:in"),
            -native,
        );
    }

    let tran = transient(&deck("sin(0 1 10meg)", ".tran 1n 200n"), 200e-9, 1e-9);
    let native = derived(&tran, "r1");
    assert_waveform("tran I(x1)", &derived(&tran, "x1"), &native);
    assert_waveform("tran @x1[ip]", &lead(&tran, "x1", "ip"), &native);
    let reversed = native.iter().map(|value| -value).collect::<Vec<_>>();
    assert_waveform("tran @x1[in]", &lead(&tran, "x1", "in"), &reversed);

    for point in ac(&deck("dc 0 ac 1", ".ac dec 2 1k 1meg"), &[1e3, 1e6]) {
        let native = ac_branch(&point, "vm");
        assert_complex("ac I(x1)", ac_branch(&point, "x1"), native);
        assert_complex("ac @x1[ip]", ac_branch(&point, "x1:ip"), native);
        assert_complex("ac @x1[in]", ac_branch(&point, "x1:in"), -native);
    }
}

/// A port named with more than one letter gives a longer label — `out` gives
/// `@x1[iout]` — and every spelling of it (`@x1[iout]`, `x1:iout`,
/// `N(x1:iinp)`) is exported as a current in amperes in every analysis, as a
/// one-letter port's is, never as a dimensionless device parameter.
#[test]
fn a_multi_letter_port_is_a_current_in_amperes() {
    let model = ModelFile::new("va_ports", PORTS_RESISTOR);
    let deck = |source: &str, saves: &str, analysis: &str| {
        format!(
            "* Verilog-A resistor with multi-letter ports against a native one\n\
             v1 in 0 {source}\n\
             x1 in a va_ports r=1k\n\
             rl1 a 0 2k\n\
             vm in b 0\n\
             r1 b c 1k\n\
             rl2 c 0 2k\n\
             .save i(x1) @x1[iinp] @x1[iout] x1:iout i(vm) {saves}\n\
             {analysis}\n\
             .va \"{}\" va_ports\n\
             .end\n",
            model.deck_path()
        )
    };
    let reversed = |values: &[f64]| values.iter().map(|value| -value).collect::<Vec<_>>();
    // `N(x1:iinp)` is the same signal as the saved `@x1[iinp]`, so where a
    // `.PRINT` card spells it the export carries it under that spelling.
    let out = ["@x1[iout]", "x1:iout"];

    let source = deck("dc 1.5", "i(r1)", ".op");
    let op = operating_point(&source);
    let native = observable(&op, "I(r1)");
    assert_static("op I(x1)", observable(&op, "I(x1)"), native);
    assert_static("op @x1[iinp]", observable(&op, "x1:iinp"), native);
    assert_static("op @x1[iout]", observable(&op, "x1:iout"), -native);
    let projected = op_projection(&netlist(&source), &op);
    assert_amperes("op", &projected, "@x1[iinp]", &[native]);
    for name in out {
        assert_amperes("op", &projected, name, &[-native]);
    }

    let source = deck("dc 0", "i(r1)", ".dc v1 -2 2 1\n.print dc n(x1:iinp)");
    let sweep = Engine::default()
        .run_dc_sweep(&netlist(&source), "v1", -2.0, 2.0, 1.0)
        .expect("DC sweep solves");
    let native = sweep
        .iter()
        .map(|(_, point)| observable(point, "I(r1)"))
        .collect::<Vec<_>>();
    for ((value, point), native) in sweep.iter().zip(&native) {
        assert_static(
            &format!("dc {value} @x1[iout]"),
            observable(point, "x1:iout"),
            -native,
        );
    }
    let projected = dc_projection(&netlist(&source), &sweep);
    assert_amperes("dc", &projected, "n(x1:iinp)", &native);
    for name in out {
        assert_amperes("dc", &projected, name, &reversed(&native));
    }

    let source = deck(
        "sin(0 1 10meg)",
        "i(r1)",
        ".tran 1n 100n\n.print tran n(x1:iinp)",
    );
    let tran = transient(&source, 100e-9, 1e-9);
    let native = derived(&tran, "r1");
    assert_waveform("tran @x1[iinp]", &lead(&tran, "x1", "iinp"), &native);
    assert_waveform(
        "tran @x1[iout]",
        &lead(&tran, "x1", "iout"),
        &reversed(&native),
    );
    let projected = tran_projection(&netlist(&source), &tran);
    assert_amperes("tran", &projected, "n(x1:iinp)", &lead(&tran, "x1", "iinp"));
    for name in out {
        assert_amperes("tran", &projected, name, &lead(&tran, "x1", "iout"));
    }

    // AC reports no resistor current, so the native side is the meter `vm`.
    let source = deck("dc 0 ac 1", "", ".ac dec 2 1k 1meg\n.print ac n(x1:iinp)");
    let points = ac(&source, &[1e3, 1e6]);
    for point in &points {
        assert_complex(
            "ac @x1[iout]",
            ac_branch(point, "x1:iout"),
            -ac_branch(point, "vm"),
        );
    }
    let projected = ac_projection(&netlist(&source), &points);
    let expected = |branch: &str| {
        points
            .iter()
            .map(|point| ac_branch(point, branch).re)
            .collect::<Vec<_>>()
    };
    assert_amperes("ac", &projected, "n(x1:iinp)", &expected("vm"));
    for name in out {
        assert_amperes("ac", &projected, name, &expected("x1:iout"));
    }
}

/// A terminal current saved only by its bare `x1:iout` spelling — Spectre's
/// `save x1:p` form — is metered and exported like the bracketed one.
#[test]
fn a_bare_terminal_current_spelling_alone_is_metered() {
    let model = ModelFile::new("va_ports_bare", PORTS_RESISTOR);
    let deck = |analysis: &str| {
        format!(
            "* only the bare spelling names the terminal\n\
             v1 in 0 sin(0 1 10meg)\n\
             x1 in a va_ports r=1k\n\
             r1 a 0 1k\n\
             .save x1:iout i(r1)\n\
             {analysis}\n\
             .va \"{}\" va_ports\n\
             .end\n",
            model.deck_path()
        )
    };
    let source = deck(".tran 1n 100n");
    let tran = transient(&source, 100e-9, 1e-9);
    let into_terminal = derived(&tran, "r1")
        .iter()
        .map(|value| -value)
        .collect::<Vec<_>>();
    assert_waveform("bare x1:iout", &lead(&tran, "x1", "iout"), &into_terminal);
    let projected = tran_projection(&netlist(&source), &tran);
    assert_amperes(
        "bare tran",
        &projected,
        "x1:iout",
        &lead(&tran, "x1", "iout"),
    );
}

/// A Verilog-A diode reports the native diode's current, where the current
/// is exponential in the solved terminal voltage.
#[test]
fn a_verilog_a_diode_reports_the_native_diodes_current() {
    let model = ModelFile::new("va_diode", DIODE);
    // The thermal voltage the native diode uses at the default 27 C, from the
    // constants its junction equations read.
    let constants = rspice_core::constants::NoisePhysicalConstants::NGSPICE_46;
    let vt =
        constants.boltzmann * rspice_core::constants::TEMP_REFERENCE / constants.electron_charge;
    // AC reports no diode current, so the native diode returns to ground
    // through a zero-volt source. On the ground side its node sits at 0 V,
    // where a GMIN floor leaks nothing around the diode.
    let deck = |source: &str, analysis: &str| {
        format!(
            "* Verilog-A diode against a native one\n\
             v1 in 0 {source}\n\
             rs1 in a 1k\n\
             x1 a 0 va_diode is=1e-14 vt={vt:.17e}\n\
             rs2 in b 1k\n\
             d1 b c dmod\n\
             vm c 0 0\n\
             .model dmod d(is=1e-14 n=1)\n\
             .save i(x1) @x1[ia] @x1[ic] i(d1)\n\
             {analysis}\n\
             .va \"{}\" va_diode\n\
             .end\n",
            model.deck_path()
        )
    };

    let op = operating_point(&deck("dc 1.2", ".op"));
    let native = observable(&op, "I(d1)");
    assert!(native > 1e-4, "the diode conducts: {native}");
    assert_static("op I(x1)", observable(&op, "I(x1)"), native);
    assert_static("op @x1[ia]", observable(&op, "x1:ia"), native);
    assert_static("op @x1[ic]", observable(&op, "x1:ic"), -native);

    // Forward bias only: below -3 Vt the native diode leaves the exponential
    // for SPICE's cubic reverse-region approximation, which this module does
    // not write.
    let sweep = Engine::default()
        .run_dc_sweep(
            &netlist(&deck("dc 0", ".dc v1 0 2 0.25")),
            "v1",
            0.0,
            2.0,
            0.25,
        )
        .expect("DC sweep solves");
    for (value, point) in &sweep {
        let native = observable(point, "I(d1)");
        assert_static(
            &format!("dc {value} I(x1)"),
            observable(point, "I(x1)"),
            native,
        );
        assert_static(
            &format!("dc {value} @x1[ic]"),
            observable(point, "x1:ic"),
            -native,
        );
    }

    let tran = transient(&deck("sin(0.6 0.6 1meg)", ".tran 10n 2u"), 2e-6, 10e-9);
    let native = derived(&tran, "d1");
    assert_waveform("tran I(x1)", &derived(&tran, "x1"), &native);
    assert_waveform("tran @x1[ia]", &lead(&tran, "x1", "ia"), &native);

    for point in ac(&deck("dc 0.65 ac 1", ".ac dec 2 1k 1meg"), &[1e3, 1e6]) {
        let native = ac_branch(&point, "vm");
        assert!(native.norm() > 0.0);
        assert_complex("ac I(x1)", ac_branch(&point, "x1"), native);
        assert_complex("ac @x1[ic]", ac_branch(&point, "x1:ic"), -native);
    }
}

/// A Verilog-A capacitor's current is its charge's rate, and the meter
/// carries it: zero at the operating point, the native capacitor's current
/// in a transient, and `jωC·V` in AC.
#[test]
fn a_verilog_a_capacitor_reports_its_reactive_current() {
    let model = ModelFile::new("va_cap", CAPACITOR);
    let deck = |source: &str, analysis: &str| {
        format!(
            "* Verilog-A capacitor against a native one\n\
             v1 in 0 {source}\n\
             r1 in a 1k\n\
             x1 a 0 va_cap c=1n\n\
             r2 in b 1k\n\
             vm b c 0\n\
             c1 c 0 1n\n\
             .save v(a) i(x1) @x1[ip] @x1[in] i(c1) i(vm)\n\
             {analysis}\n\
             .va \"{}\" va_cap\n\
             .end\n",
            model.deck_path()
        )
    };

    let op = operating_point(&deck("dc 1", ".op"));
    assert_eq!(observable(&op, "I(x1)"), 0.0);
    assert_eq!(observable(&op, "x1:in"), 0.0);

    let tran = transient(
        &deck("pulse(0 1 10n 5n 5n 1u 2u)", ".tran 1n 3u"),
        3e-6,
        20e-9,
    );
    let native = derived(&tran, "c1");
    assert!(
        native.iter().any(|current| current.abs() > 1e-4),
        "the capacitor charges"
    );
    assert_waveform("tran I(x1)", &derived(&tran, "x1"), &native);
    assert_waveform("tran @x1[ip]", &lead(&tran, "x1", "ip"), &native);
    let reversed = native.iter().map(|value| -value).collect::<Vec<_>>();
    assert_waveform("tran @x1[in]", &lead(&tran, "x1", "in"), &reversed);

    for point in ac(&deck("dc 0 ac 1", ".ac dec 2 1k 1meg"), &[1e3, 1.59e5, 1e7]) {
        let native = ac_branch(&point, "vm");
        assert!(native.im.abs() > 0.0, "the current leads the voltage");
        assert_complex("ac I(x1)", ac_branch(&point, "x1"), native);
        assert_complex("ac @x1[ip]", ac_branch(&point, "x1:ip"), native);
    }
}

/// Each terminal of a three-terminal instance reports its own current, and
/// the three satisfy KCL; `I(x1)` names no single one of them and is refused.
#[test]
fn a_three_terminal_instance_reports_each_terminal_and_refuses_i_of_the_instance() {
    let model = ModelFile::new("va_tri", THREE_TERMINAL);
    let deck = |source: &str, saves: &str, analysis: &str| {
        format!(
            "* three-terminal Verilog-A device against a resistor and a VCCS\n\
             v1 in 0 {source}\n\
             ra in a 1k\n\
             x1 a b 0 va_tri g1=1m g2=2m\n\
             rb b 0 500\n\
             rna in na 1k\n\
             vma na na2 0\n\
             r1 na2 0 1k\n\
             vmb nb nb2 0\n\
             g1 nb2 0 na2 0 2m\n\
             rnb nb 0 500\n\
             .save {saves}\n\
             {analysis}\n\
             .va \"{}\" va_tri\n\
             .end\n",
            model.deck_path()
        )
    };
    let saves = "@x1[ia] @x1[ib] @x1[ic] i(r1) i(g1) i(vma) i(vmb)";
    // The native pair's own currents: the resistor's, and the VCCS's law at
    // its solved control voltage (the engine reports a VCCS current in a
    // transient only).
    let native = |point: &SimulationResult| {
        let control = point.try_voltage_named("na2").expect("V(na2)");
        (observable(point, "I(r1)"), 2e-3 * control)
    };

    let op = operating_point(&deck("dc 2", saves, ".op"));
    let (ia, ib) = native(&op);
    assert!(ia > 0.0 && ib > 0.0, "{ia} {ib}");
    assert_static("op @x1[ia]", observable(&op, "x1:ia"), ia);
    assert_static("op @x1[ib]", observable(&op, "x1:ib"), ib);
    assert_static("op @x1[ic]", observable(&op, "x1:ic"), -(ia + ib));

    let sweep = Engine::default()
        .run_dc_sweep(
            &netlist(&deck("dc 0", saves, ".dc v1 -1 1 0.5")),
            "v1",
            -1.0,
            1.0,
            0.5,
        )
        .expect("DC sweep solves");
    for (value, point) in &sweep {
        let (ia, ib) = native(point);
        assert_static(
            &format!("dc {value} @x1[ia]"),
            observable(point, "x1:ia"),
            ia,
        );
        assert_static(
            &format!("dc {value} @x1[ib]"),
            observable(point, "x1:ib"),
            ib,
        );
    }

    let tran = transient(&deck("sin(0 1 5meg)", saves, ".tran 2n 400n"), 400e-9, 2e-9);
    assert_waveform(
        "tran @x1[ia]",
        &lead(&tran, "x1", "ia"),
        &derived(&tran, "r1"),
    );
    assert_waveform(
        "tran @x1[ib]",
        &lead(&tran, "x1", "ib"),
        &derived(&tran, "g1"),
    );

    for point in ac(&deck("dc 0 ac 1", saves, ".ac dec 2 1k 1meg"), &[1e4]) {
        assert_complex(
            "ac @x1[ia]",
            ac_branch(&point, "x1:ia"),
            ac_branch(&point, "vma"),
        );
        assert_complex(
            "ac @x1[ib]",
            ac_branch(&point, "x1:ib"),
            ac_branch(&point, "vmb"),
        );
    }

    let error = Engine::default()
        .run_dc_op(&netlist(&deck("dc 2", "i(x1)", ".op")))
        .expect_err("I(x1) names no single current of a three-terminal instance");
    let rspice_core::SimulationError::UnsupportedCapability(refusal) = &error else {
        panic!("expected a typed capability refusal, got {error:?}");
    };
    assert_eq!(
        refusal.capability, "output.veriloga.terminal_current",
        "{error}"
    );
    assert!(
        refusal
            .instance
            .as_deref()
            .is_some_and(|instance| instance.eq_ignore_ascii_case("x1")),
        "{error}"
    );
    for form in ["@x1[ia]", "@x1[ib]", "@x1[ic]"] {
        assert!(refusal.detail.contains(form), "{form}: {error}");
    }
}

/// A runtime instance reports nothing but its terminal currents, so a
/// parameter that names none of its terminals is refused rather than left
/// out of the result.
#[test]
fn a_parameter_naming_no_terminal_of_a_runtime_instance_is_refused() {
    let model = ModelFile::new("va_res_typo", RESISTOR);
    let deck = format!(
        "* a misspelled terminal current\n\
         v1 in 0 1\n\
         x1 in 0 va_res\n\
         .save @x1[iq]\n\
         .va \"{}\" va_res\n\
         .end\n",
        model.deck_path()
    );
    let error = Engine::default()
        .run_tran(&netlist(&deck), 2e-9, 1e-9)
        .expect_err("@x1[iq] names no terminal");
    let rspice_core::SimulationError::UnsupportedCapability(refusal) = &error else {
        panic!("expected a typed capability refusal, got {error:?}");
    };
    assert_eq!(
        refusal.capability, "output.veriloga.terminal_current",
        "{error}"
    );
    assert!(refusal.detail.contains("@x1[ip]"), "{error}");
    assert!(refusal.detail.contains("@x1[in]"), "{error}");
}

/// A terminal a deck does not name gets no meter: the circuit is the one an
/// ordinary run builds, node for node.
#[test]
fn an_unnamed_terminal_adds_nothing_to_the_circuit() {
    let model = ModelFile::new("va_res_quiet", RESISTOR);
    let deck = |saves: &str| {
        format!(
            "* no terminal current asked for\n\
             v1 in 0 1\n\
             x1 in a va_res\n\
             r1 a 0 1k\n\
             {saves}\n\
             .va \"{}\" va_res\n\
             .end\n",
            model.deck_path()
        )
    };
    let plain = operating_point(&deck(".save v(a)"));
    let metered = operating_point(&deck(".save v(a) @x1[ip]"));
    assert_eq!(plain.node_names.len() + 1, metered.node_names.len());
    assert_eq!(plain.branch_names.len() + 1, metered.branch_names.len());
    assert!(plain.try_dc_observable_named("x1:ip").is_none());
    assert_static(
        "metered @x1[ip]",
        observable(&metered, "x1:ip"),
        observable(&metered, "I(r1)"),
    );
    for node in ["in", "a"] {
        assert_eq!(
            plain.try_voltage_named(node),
            metered.try_voltage_named(node),
            "V({node}) is the unmetered circuit's"
        );
    }
}

/// The first sample of a resumed transient carries the terminal current the
/// checkpoint's last accepted point had.
#[test]
fn a_resumed_transient_reports_the_terminal_current_at_its_first_point() {
    let model = ModelFile::new("va_cap_resume", CAPACITOR);
    let deck = format!(
        "* resume a metered Verilog-A capacitor\n\
         v1 in 0 sin(0 1 5meg)\n\
         r1 in a 1k\n\
         x1 a 0 va_cap c=100p\n\
         r2 in b 1k\n\
         vm b c 0\n\
         c1 c 0 100p\n\
         .save i(x1) @x1[ip] i(vm)\n\
         .va \"{}\" va_cap\n\
         .end\n",
        model.deck_path()
    );
    let netlist = netlist(&deck);
    let engine = Engine::default();
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 150e-9, 2e-9)
        .expect("first window runs");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 300e-9, 2e-9)
        .expect("resumed window runs");
    for (label, result) in [("first", &first), ("resumed", &resumed)] {
        let native = derived(result, "vm");
        assert_waveform(&format!("{label} I(x1)"), &derived(result, "x1"), &native);
        assert_waveform(
            &format!("{label} @x1[ip]"),
            &lead(result, "x1", "ip"),
            &native,
        );
    }
    let last = *derived(&first, "x1").last().unwrap();
    let resumed_first = derived(&resumed, "x1")[0];
    assert!(
        agrees(resumed_first, last, 1e-9),
        "the resumed first point {resumed_first:e} is the checkpoint's {last:e}"
    );
}

/// A mixed module's discrete output reaches the deck through a converter that
/// belongs to the instance, so the instance's terminal current is what the
/// converter drives into the load.
#[test]
fn a_mixed_instance_reports_the_current_its_boundary_drives() {
    let model = ModelFile::new(
        "digital_drive",
        "`timescale 1ns/1ps\n\
         module digital_drive(q); output q; reg q;\n\
         initial begin q = 1'b0; #10 q = 1'b1; end\n\
         endmodule\n",
    );
    let deck = format!(
        "* a digital output driving an analog load\n\
         .param vcc=3.3\n\
         x1 out digital_drive\n\
         r1 out 0 1k\n\
         .save v(out) i(x1) @x1[iq] i(r1)\n\
         .va \"{}\" digital_drive\n\
         .end\n",
        model.deck_path()
    );
    let tran = transient(&deck, 20e-9, 0.5e-9);
    let load = derived(&tran, "r1");
    assert!(
        load.iter().any(|current| *current > 3.0e-3),
        "the boundary drives the load high: {load:?}"
    );
    let into_terminal = load.iter().map(|value| -value).collect::<Vec<_>>();
    assert_waveform("mixed I(x1)", &derived(&tran, "x1"), &into_terminal);
    assert_waveform("mixed @x1[iq]", &lead(&tran, "x1", "iq"), &into_terminal);
    assert!(
        tran.digital_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case("out")),
        "the bit the instance drives keeps the deck's net name"
    );
}

/// The generated route meters the same terminals under the same names, and
/// its answer is the runtime route's for the same source.
#[cfg(feature = "veriloga-model-diode-cmc")]
mod generated {
    use super::*;

    const DIODE_CMC_SOURCE: &str = "models/veriloga/cmc/diode_cmc_3.0_20250714/vacode/diode_cmc.va";

    fn model_source() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join(DIODE_CMC_SOURCE);
        path.canonicalize()
            .expect("the generated model's own source is in the tree")
            .display()
            .to_string()
            .replace('\\', "/")
            .replace("//?/", "")
    }

    fn deck(include: &str, source: &str, saves: &str, analysis: &str) -> String {
        format!(
            "* diode_cmc terminal currents\n\
             v1 in 0 {source}\n\
             rs in a 1k\n\
             x1 a 0 diode_cmc\n\
             .save {saves}\n\
             {analysis}\n\
             {include}\n\
             .end\n"
        )
    }

    /// `@x1[ia]` and `I(x1)` of a generated diode are the current through its
    /// series resistor, on the generated route and on the runtime route of
    /// the same source alike.
    #[test]
    fn a_generated_instance_reports_its_metered_terminal_currents() {
        let runtime = format!(".va \"{}\"", model_source());
        let saves = "i(x1) @x1[ia] @x1[ik] i(rs)";
        let gmin = rspice_core::SimulationConfig::default()
            .convergence_config
            .gmin_target;
        let mut op_currents = Vec::new();
        for include in ["", runtime.as_str()] {
            let op = operating_point(&deck(include, "dc 1", saves, ".op"));
            // KCL at the anode: the series resistor's current is the
            // diode's and the node's GMIN floor's.
            let anode = op.try_voltage_named("a").expect("V(a)");
            let native = observable(&op, "I(rs)") - gmin * anode;
            assert!(native > 1e-7, "the diode conducts: {native}");
            assert_static("op I(x1)", observable(&op, "I(x1)"), native);
            assert_static("op @x1[ia]", observable(&op, "x1:ia"), native);
            assert_static("op @x1[ik]", observable(&op, "x1:ik"), -native);
            op_currents.push(native);

            let tran = transient(
                &deck(include, "sin(0.5 0.5 20meg)", saves, ".tran 1n 100n"),
                100e-9,
                1e-9,
            );
            let native = derived(&tran, "rs");
            assert_waveform("tran I(x1)", &derived(&tran, "x1"), &native);
            assert_waveform("tran @x1[ia]", &lead(&tran, "x1", "ia"), &native);

            for point in ac(
                &deck(include, "dc 0.7 ac 1", saves, ".ac dec 1 1meg 1g"),
                &[1e8],
            ) {
                let terminal = ac_branch(&point, "x1:ia");
                assert!(terminal.norm() > 0.0);
                assert_complex("ac I(x1)", ac_branch(&point, "x1"), terminal);
                assert_complex("ac @x1[ik]", ac_branch(&point, "x1:ik"), -terminal);
            }
        }
        assert!(
            agrees(op_currents[0], op_currents[1], 1e-12),
            "the two routes agree: {op_currents:?}"
        );
    }
}

/// The generated route labels a terminal as its catalog does, and a catalog
/// label longer than two letters is a current all the same: R2_CMC's ports
/// `n1` and `n2` give `@x1[in1]` and `@x1[in2]`.
#[cfg(feature = "veriloga-model-r2-cmc")]
mod generated_multi_letter {
    use super::*;

    fn deck(source: &str, analysis: &str) -> String {
        format!(
            "* generated R2_CMC terminal currents\n\
             v1 in 0 {source}\n\
             x1 in a r2_cmc r=1k\n\
             vm a b 0\n\
             rl b 0 2k\n\
             .save i(x1) @x1[in1] @x1[in2] x1:in2 i(vm)\n\
             {analysis}\n\
             .end\n"
        )
    }

    #[test]
    fn a_generated_instance_with_multi_letter_labels_reports_currents_in_amperes() {
        // A `.PRINT` card's `N(x1:in1)` is the saved `@x1[in1]`, exported
        // under the card's spelling.
        let spellings = ["@x1[in2]", "x1:in2"];
        let gmin = rspice_core::SimulationConfig::default()
            .convergence_config
            .gmin_target;

        // KCL at `a`: what the instance drives out of `n2` leaves through the
        // meter `vm` and the node's GMIN floor.
        let kcl = |point: &SimulationResult| {
            point.branch_current_named("vm").expect("I(vm)")
                + gmin * point.try_voltage_named("a").expect("V(a)")
        };
        let source = deck("dc 1.5", ".op");
        let op = operating_point(&source);
        let native = kcl(&op);
        assert!(
            (native - 1.5 / 3000.0).abs() < 1e-9,
            "the instance is 1k: {native}"
        );
        assert_static("op I(x1)", observable(&op, "I(x1)"), native);
        assert_static("op @x1[in1]", observable(&op, "x1:in1"), native);
        assert_static("op @x1[in2]", observable(&op, "x1:in2"), -native);
        let projected = op_projection(&netlist(&source), &op);
        assert_amperes("op", &projected, "@x1[in1]", &[native]);
        for name in spellings {
            assert_amperes("op", &projected, name, &[-native]);
        }

        let source = deck("dc 0", ".dc v1 -1 1 1\n.print dc n(x1:in1)");
        let sweep = Engine::default()
            .run_dc_sweep(&netlist(&source), "v1", -1.0, 1.0, 1.0)
            .expect("DC sweep solves");
        let mut native = Vec::new();
        for (value, point) in &sweep {
            let current = kcl(point);
            assert_static(
                &format!("dc {value} @x1[in2]"),
                observable(point, "x1:in2"),
                -current,
            );
            native.push(current);
        }
        let reversed = native.iter().map(|value| -value).collect::<Vec<_>>();
        let projected = dc_projection(&netlist(&source), &sweep);
        assert_amperes("dc", &projected, "n(x1:in1)", &native);
        for name in spellings {
            assert_amperes("dc", &projected, name, &reversed);
        }

        let source = deck("sin(0 1 10meg)", ".tran 1n 100n\n.print tran n(x1:in1)");
        let tran = transient(&source, 100e-9, 1e-9);
        let native = derived(&tran, "vm");
        assert_waveform("tran I(x1)", &derived(&tran, "x1"), &native);
        assert_waveform("tran @x1[in1]", &lead(&tran, "x1", "in1"), &native);
        let projected = tran_projection(&netlist(&source), &tran);
        assert_amperes("tran", &projected, "n(x1:in1)", &lead(&tran, "x1", "in1"));
        for name in spellings {
            assert_amperes("tran", &projected, name, &lead(&tran, "x1", "in2"));
        }

        let source = deck("dc 0 ac 1", ".ac dec 2 1k 1meg\n.print ac n(x1:in1)");
        let points = ac(&source, &[1e3, 1e6]);
        for point in &points {
            assert_complex(
                "ac @x1[in1]",
                ac_branch(point, "x1:in1"),
                ac_branch(point, "vm"),
            );
        }
        let projected = ac_projection(&netlist(&source), &points);
        let expected = |branch: &str| {
            points
                .iter()
                .map(|point| ac_branch(point, branch).re)
                .collect::<Vec<_>>()
        };
        assert_amperes("ac", &projected, "n(x1:in1)", &expected("x1:in1"));
        for name in spellings {
            assert_amperes("ac", &projected, name, &expected("x1:in2"));
        }
    }
}
