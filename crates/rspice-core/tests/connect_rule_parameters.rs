#![cfg(feature = "veriloga")]
//! The connect-rule converters' parameters, on decks the engine solves.
//!
//! A connect statement may set every parameter of the built-in converters
//! (thresholds, X timeout, per-state and per-strength resistances, levels,
//! edge times and delay, the unpowered supply). Each value is checked before
//! a converter exists, and the insertion record says where every value came
//! from. The same law runs on both routes: an XSPICE node a connect rule
//! reaches, and a Verilog port.
//!
//! Expected values are closed forms.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Once;

use rspice_core::circuit::{
    ConverterInsertion, ConverterKind, ConverterParameter, ConverterSource,
};
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::notice::{BoundarySupplyKind, EngineNotice, NoticeCollector};
use rspice_core::xspice::PortDirection;
use rspice_veriloga::connect::library::{BUILTIN_CONNECT_MODULES, LEGACY_BUILTIN_CONNECT_MODULES};

//=============================================================================
// Log capture
//=============================================================================

static LOG_INIT: Once = Once::new();

thread_local! {
    /// The lines logged on this thread while it captures. The logger is
    /// process-global and the tests run in parallel, so a capture keeps only
    /// its own thread's lines: another test's boundaries never reach it.
    static CAPTURED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

struct CapturingLogger;

impl log::Log for CapturingLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            CAPTURED.with(|captured| {
                if let Some(lines) = captured.borrow_mut().as_mut() {
                    lines.push(record.args().to_string());
                }
            });
        }
    }

    fn flush(&self) {}
}

static LOGGER: CapturingLogger = CapturingLogger;

/// Run `body` with the boundary lines it logs collected.
fn boundary_lines_of<T>(body: impl FnOnce() -> T) -> (T, Vec<String>) {
    lines_of("boundary", body)
}

/// Run `body` with the lines it logs that contain `needle` collected.
fn lines_of<T>(needle: &str, body: impl FnOnce() -> T) -> (T, Vec<String>) {
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("the capture logger installs");
        log::set_max_level(log::LevelFilter::Info);
    });
    CAPTURED.with(|captured| *captured.borrow_mut() = Some(Vec::new()));
    let value = body();
    let lines = CAPTURED
        .with(|captured| captured.borrow_mut().take())
        .unwrap_or_default()
        .into_iter()
        .filter(|line| line.contains(needle))
        .collect();
    (value, lines)
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
        let path =
            std::env::temp_dir().join(format!("rspice-cr-{label}-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&path).expect("create isolated test directory");
        Self(path)
    }

    fn write(&self, name: &str, source: &str) -> String {
        let path = self.0.join(name);
        std::fs::write(&path, source).expect("write the test source");
        slashed(&path)
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

/// The shipped modules (full or legacy signature) and one rules block.
fn library(modules: &[(&str, &str); 3], rules: &str) -> String {
    let mut source = String::new();
    for (_, module) in modules {
        source.push_str(module);
    }
    source.push_str(rules);
    source
}

/// An XSPICE inverter between two connect-rule boundaries: `din` (A/D) and
/// `dout` (D/A, loaded by `load`).
fn xspice_deck(library_path: &str, load: &str) -> String {
    format!(
        "* connect-rule parameters, XSPICE route\n\
         .param vcc=3.3\n\
         .veriloga \"{library_path}\"\n\
         vin din 0 pwl(0 0 100n 3.3)\n\
         rin din 0 1meg\n\
         ainv din dout inverter\n\
         .model inverter d_inverter\n\
         {load}\n\
         .end\n"
    )
}

fn build_error(deck: &str) -> String {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    match Engine::default().build_circuit(&netlist) {
        Ok(_) => panic!("the deck must be refused"),
        Err(error) => error.to_string(),
    }
}

fn run(deck: &str, tstop: f64, step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    Engine::default()
        .run_tran(&netlist, tstop, step)
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

/// `node` at `time`, linearly interpolated.
fn value_at(result: &TransientResult, node: &str, time: f64) -> f64 {
    let values = series(result, node);
    let after = result
        .time
        .iter()
        .position(|t| *t >= time)
        .unwrap_or_else(|| panic!("{time} is inside the run"));
    if after == 0 || result.time[after] == time {
        return values[after];
    }
    let (t0, t1) = (result.time[after - 1], result.time[after]);
    values[after - 1] + (values[after] - values[after - 1]) * (time - t0) / (t1 - t0)
}

/// The first instant after `from` at which `node` crosses `level`, linearly
/// interpolated.
fn crossing(result: &TransientResult, node: &str, level: f64, from: f64) -> f64 {
    let values = series(result, node);
    for index in 1..result.time.len() {
        if result.time[index] <= from {
            continue;
        }
        let (a, b) = (values[index - 1] - level, values[index] - level);
        if a == 0.0 {
            return result.time[index - 1];
        }
        if (a < 0.0) != (b < 0.0) || b == 0.0 {
            let (t0, t1) = (result.time[index - 1], result.time[index]);
            return t0 + (t1 - t0) * a / (a - b);
        }
    }
    panic!("{node} never crosses {level} after {from}")
}

//=============================================================================
// 1. Every refusal names its parameter
//=============================================================================

#[test]
fn every_refusal_names_its_parameter() {
    let directory = TempDirectory::new("refusals");
    let cases: [(&str, &str, &[&str]); 9] = [
        (
            "connect a2d #(.vlo(0.4)); connect d2a;",
            "an unknown name lists the accepted set",
            &[
                "'vlo'",
                "vsup, vsupmin, fthi, ftlo, vthi, vtlo, txdel, tdrise, tdfall, tdelay",
            ],
        ),
        (
            "connect a2d; connect d2a #(.rlo(-5));",
            "a resistance is positive",
            &["rlo = -5", "positive"],
        ),
        (
            "connect a2d; connect d2a #(.trise(-1n));",
            "a time is nonnegative",
            &["trise", "nonnegative"],
        ),
        (
            "connect a2d #(.vtlo(2.0), .vthi(1.0)); connect d2a;",
            "the band is not inverted",
            &["vtlo = 2", "vthi = 1"],
        ),
        (
            "connect a2d; connect d2a #(.rpull(5));",
            "a weaker strength is never stiffer",
            &["rpull = 5", "rpull <= rweak"],
        ),
        (
            "connect a2d #(.txdel(-1n)); connect d2a;",
            "a timeout is nonnegative",
            &["txdel", "nonnegative"],
        ),
        (
            "connect a2d; connect d2a #(.vx(inf));",
            "only txdel and rz may be infinite",
            &["vx = inf", "finite"],
        ),
        (
            "connect a2d #(.tdelay(1n), .tdrise(2n)); connect d2a;",
            "an alias is refused beside its target",
            &["'tdrise' twice", "'tdelay'"],
        ),
        (
            "connect a2d #(.vsupmin(5.0)); connect d2a;",
            "an unpowered threshold above the supply",
            &["vsupmin = 5", "never powered"],
        ),
    ];
    for (index, (statements, label, expected)) in cases.iter().enumerate() {
        let path = directory.write(
            &format!("case{index}.va"),
            &library(
                &BUILTIN_CONNECT_MODULES,
                &format!("connectrules deck;\n    {statements}\nendconnectrules\n"),
            ),
        );
        let error = build_error(&xspice_deck(&path, "rout dout 0 1meg"));
        for fragment in *expected {
            assert!(
                error.contains(fragment),
                "{label}: the refusal must contain {fragment:?}: {error}"
            );
        }
    }

    // The earlier signature takes only what it declares.
    let path = directory.write(
        "legacy.va",
        &library(
            &LEGACY_BUILTIN_CONNECT_MODULES,
            "connectrules deck;\n    connect a2d #(.vthi(2.0));\n    connect d2a;\nendconnectrules\n",
        ),
    );
    let error = build_error(&xspice_deck(&path, "rout dout 0 1meg"));
    assert!(
        error.contains("'vthi'")
            && error.contains("earlier signature")
            && error.contains("vsup, tdrise, tdfall"),
        "a legacy copy lists its own parameters: {error}"
    );
}

//=============================================================================
// 2. Precedence, and the record's sources
//=============================================================================

fn record_of<'a>(lines: &'a [String], module: &str) -> &'a str {
    lines
        .iter()
        .find(|line| line.contains(&format!(": {module} as ")))
        .unwrap_or_else(|| panic!("a record for {module}: {lines:?}"))
}

#[test]
fn a_statement_beats_the_module_default_which_beats_the_rule_set() {
    let directory = TempDirectory::new("precedence");
    let full = directory.write(
        "full.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            "connectrules deck;\n    connect a2d;\n    connect d2a #(.rhi(10), .tr(2n));\nendconnectrules\n",
        ),
    );
    let (_, lines) =
        boundary_lines_of(|| run(&xspice_deck(&full, "rout dout 0 1meg"), 20e-9, 1e-9));
    let d2a = record_of(&lines, "d2a");
    for fragment in [
        "vsup=3.3 V (.param vcc, static)",
        "rhi=10 Ω (connect statement)",
        "trise=2 ns (connect statement)",
        "rlo=20 Ω (module default)",
        "rx=10 Ω (module default min(rlo, rhi))",
        "vhi=3.3 V (module default vsup)",
        "rz=inf (module default)",
    ] {
        assert!(d2a.contains(fragment), "{fragment:?} in {d2a}");
    }
    let a2d = record_of(&lines, "a2d");
    assert!(a2d.contains("vthi=1.65 V (module default vsup/2)"), "{a2d}");
    assert!(a2d.contains("txdel=inf (module default)"), "{a2d}");

    // A legacy copy declares only a supply and the edge times; everything
    // else is the rule set's, and so are its edge times when unset.
    let legacy = directory.write(
        "legacy.va",
        &library(
            &LEGACY_BUILTIN_CONNECT_MODULES,
            "connectrules deck;\n    connect a2d;\n    connect d2a #(.trise(2n));\nendconnectrules\n",
        ),
    );
    let (_, lines) =
        boundary_lines_of(|| run(&xspice_deck(&legacy, "rout dout 0 1meg"), 20e-9, 1e-9));
    let d2a = record_of(&lines, "d2a");
    for fragment in [
        "earlier signature",
        "trise=2 ns (connect statement)",
        "tfall=0 s (rule set rspice_builtin)",
        "rlo=20 Ω (rule set rspice_builtin)",
    ] {
        assert!(d2a.contains(fragment), "{fragment:?} in {d2a}");
    }
}

//=============================================================================
// 3. A verbatim legacy copy still delegates
//=============================================================================

/// The deck converts, both ways, through the built-in converters: the
/// inverter's output falls once the ramp passes half the supply.
#[test]
fn a_verbatim_legacy_copy_still_delegates() {
    let directory = TempDirectory::new("legacy");
    let path = directory.write(
        "legacy.va",
        &library(
            &LEGACY_BUILTIN_CONNECT_MODULES,
            "connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n",
        ),
    );
    let (result, lines) =
        boundary_lines_of(|| run(&xspice_deck(&path, "rout dout 0 1meg"), 100e-9, 0.5e-9));
    assert!(lines.iter().any(|line| line.contains("earlier signature")));
    // 20 Ω into 1 MΩ: the high level is 3.3·1e6/(1e6 + 20).
    let high = 3.3 * 1e6 / (1e6 + 20.0);
    assert!((value_at(&result, "dout", 10e-9) - high).abs() < 1e-9);
    // The input crosses vsup/2 = 1.65 V at 50 ns; the A/D publishes at the
    // first accepted point past it (tdrise = 0; a deck without an HDL host
    // samples its converters at accepted points, as `adc_bridge` does), the
    // inverter after its 1 ns default delay, and the D/A switches at that
    // edge (trise = tfall = 0). So the fall is within one 0.5 ns step of 51 ns.
    let fall = crossing(&result, "dout", 1.65, 1e-9);
    assert!(
        (51e-9 - 5e-12..=51.5e-9 + 5e-12).contains(&fall),
        "dout falls at {fall}"
    );
}

//=============================================================================
// 4. The same law on both routes
//=============================================================================

/// The time constant of a first-order edge from the instants it passes 10%
/// and 63.2% of its swing: τ·(1 − ln(10/9)) apart.
fn time_constant(result: &TransientResult, node: &str, swing: f64, rising: bool, from: f64) -> f64 {
    let (first, second) = if rising {
        (0.1 * swing, (1.0 - (-1.0f64).exp()) * swing)
    } else {
        (0.9 * swing, (-1.0f64).exp() * swing)
    };
    let a = crossing(result, node, first, from);
    let b = crossing(result, node, second, a);
    (b - a) / (1.0 - (10.0f64 / 9.0).ln())
}

const RHI_RLO: &str = "connect d2a #(.rhi(10), .rlo(5));";

#[test]
fn an_xspice_node_and_a_verilog_port_convert_by_one_law() {
    let directory = TempDirectory::new("both-routes");
    let library_path = directory.write(
        "lib.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            &format!("connectrules deck;\n    connect a2d;\n    {RHI_RLO}\nendconnectrules\n"),
        ),
    );
    let xspice = run(
        &format!(
            "* d2a on an XSPICE node\n\
             .param vcc=3.3\n\
             .veriloga \"{library_path}\"\n\
             vin vin 0 pulse(0 3.3 50n 1p 1p 200n 1)\n\
             aadc [vin] [dd] adc1\n\
             .model adc1 adc_bridge(in_low=1.65 in_high=1.65)\n\
             abuf dd out buf1\n\
             .model buf1 d_buffer\n\
             c1 out 0 1n\n\
             .end\n"
        ),
        400e-9,
        0.02e-9,
    );

    let mut hdl_source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module drv(o);\n  output o; reg o;\n  initial begin o = 1'b0; #50 o = 1'b1; #200 o = 1'b0; end\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        hdl_source.push_str(module);
    }
    hdl_source.push_str(&format!(
        "connectrules deck;\n    connect a2d;\n    {RHI_RLO}\nendconnectrules\n"
    ));
    let hdl_path = directory.write("drv.vams", &hdl_source);
    let hdl = run(
        &format!(
            "* d2a on a Verilog port\n\
             .param vcc=3.3\n\
             .hdl \"{hdl_path}\" module=drv\n\
             .options connectrules=deck\n\
             x1 out drv\n\
             c1 out 0 1n\n\
             .end\n"
        ),
        400e-9,
        0.02e-9,
    );

    for (route, result) in [("XSPICE", &xspice), ("Verilog", &hdl)] {
        let rise = time_constant(result, "out", 3.3, true, 40e-9);
        let fall = time_constant(result, "out", 3.3, false, 240e-9);
        assert!(
            (rise - 10e-9).abs() <= 1e-3 * 10e-9,
            "{route} rise τ {rise}"
        );
        assert!((fall - 5e-9).abs() <= 1e-3 * 5e-9, "{route} fall τ {fall}");
    }
}

/// An HDL driver's strength reaches the converter: a pull drive of 1 into a
/// 1 kΩ load settles at `vsup·1k/(1k + rpull)` and a strong one at
/// `vsup·1k/(1k + rhi)`, at the `d2a` defaults (`vsup` 3.3 V, `rpull` 10 kΩ,
/// `rhi` 20 Ω).
#[test]
fn an_hdl_drive_strength_picks_the_converter_resistance() {
    let directory = TempDirectory::new("hdl-strength");
    let pulled = 3.3 * 1e3 / (1e3 + 10e3);
    let strong = 3.3 * 1e3 / (1e3 + 20.0);
    for (index, (body, expected)) in [
        ("pullup (o);", pulled),
        ("assign (pull1, pull0) o = 1'b1;", pulled),
        ("assign o = 1'b1;", strong),
    ]
    .into_iter()
    .enumerate()
    {
        let mut hdl_source = format!(
            "`include \"disciplines.vams\"\n\
             `timescale 1ns/1ps\n\
             module drv(o);\n  output o;\n  wire o;\n  {body}\nendmodule\n"
        );
        for (_, module) in BUILTIN_CONNECT_MODULES {
            hdl_source.push_str(module);
        }
        hdl_source
            .push_str("connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n");
        let hdl_path = directory.write(&format!("drv{index}.vams"), &hdl_source);
        let result = run(
            &format!(
                "* HDL drive strength at a converter\n\
                 .hdl \"{hdl_path}\" module=drv\n\
                 .options connectrules=deck\n\
                 x1 out drv\n\
                 r1 out 0 1k\n\
                 .end\n"
            ),
            20e-9,
            0.1e-9,
        );
        let settled = value_at(&result, "out", 10e-9);
        assert!(
            (settled - expected).abs() <= 1e-6 * 3.3,
            "{body}: {settled} V, expected {expected} V"
        );
    }
}

/// Supply and weak drives keep their own resistances through an HDL port's
/// converter, which XSPICE's strong and resistive classes would not: a 1
/// into a 1 kΩ load at the `d2a` defaults (`vsup` 3.3 V, `rsup` 2 Ω, `rpull`
/// 10 kΩ, `rweak` 100 kΩ) settles at `3.3·1k/(1k + r)`, so 3.293413 V,
/// 0.300000 V and 32.673 mV, on the `.hdl` and `.veriloga` routes alike.
#[test]
fn supply_pull_and_weak_drives_take_their_own_resistance_on_both_routes() {
    let directory = TempDirectory::new("hdl-strength-classes");
    let settled = |resistance: f64| 3.3 * 1e3 / (1e3 + resistance);
    for (class, body, expected) in [
        (
            "supply",
            "assign (supply1, supply0) o = 1'b1;",
            settled(2.0),
        ),
        ("pull", "assign (pull1, pull0) o = 1'b1;", settled(10e3)),
        ("weak", "assign (weak1, weak0) o = 1'b1;", settled(100e3)),
    ] {
        let mut source = format!(
            "`include \"disciplines.vams\"\n\
             `timescale 1ns/1ps\n\
             module drv(o);\n  output o;\n  wire o;\n  {body}\nendmodule\n"
        );
        for (_, module) in BUILTIN_CONNECT_MODULES {
            source.push_str(module);
        }
        source
            .push_str("connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n");
        let path = directory.write(&format!("drv-{class}.vams"), &source);
        for (route, load) in [
            ("hdl", format!(".hdl \"{path}\" module=drv\n")),
            ("veriloga", format!(".veriloga \"{path}\"\n")),
        ] {
            let result = run(
                &format!(
                    "* a {class} 1 through an HDL port's converter, {route} route\n\
                     {load}\
                     .options connectrules=deck\n\
                     x1 out drv\n\
                     r1 out 0 1k\n\
                     .end\n"
                ),
                20e-9,
                0.1e-9,
            );
            let observed = value_at(&result, "out", 10e-9);
            assert!(
                (observed - expected).abs() <= 1e-6 * 3.3,
                "{class} on the {route} route: {observed} V, expected {expected} V"
            );
        }
    }
}

//=============================================================================
// 5. A live bidirectional converter
//=============================================================================

#[test]
fn a_live_bidirectional_port_tracks_its_supply() {
    let directory = TempDirectory::new("live-bidir");
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module od(io);\n  inout io;\n  reg d;\n  assign io = d ? 1'bz : 1'b0;\n  initial begin d = 1'b0; #1000 d = 1'b1; end\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(
        "connectrules live;\n    connect a2d #(.vsup(vdd));\n    connect d2a #(.vsup(vdd));\n    connect bidir #(.vsup(vdd));\nendconnectrules\n",
    );
    let path = directory.write("od.vams", &source);
    let (result, lines) = boundary_lines_of(|| {
        run(
            &format!(
                "* open-drain port, live supply\n\
                 .hdl \"{path}\" module=od\n\
                 .options connectrules=live\n\
                 vdd vdd 0 pwl(0 3.3 2u 2.0)\n\
                 rp io vdd 2.2k\n\
                 x1 io od\n\
                 .end\n"
            ),
            2e-6,
            5e-9,
        )
    });
    let bidir = record_of(&lines, "bidir");
    assert!(
        bidir.contains("vsup=net vdd, live (connect statement)"),
        "{bidir}"
    );
    assert!(
        bidir.contains("vthi=1.65 V (module default vsup/2, tracks)"),
        "{bidir}"
    );

    for time in [0.2e-6, 0.5e-6, 0.8e-6] {
        let supply = value_at(&result, "vdd", time);
        let expected = supply * 20.0 / 2220.0;
        let observed = value_at(&result, "io", time);
        assert!(
            (observed - expected).abs() <= 1e-4 * expected,
            "V_OL at {time}: {observed} against {expected}"
        );
    }
    for time in [1.2e-6, 1.5e-6, 1.8e-6] {
        let supply = value_at(&result, "vdd", time);
        let observed = value_at(&result, "io", time);
        assert!(
            (observed - supply).abs() <= 1e-4 * supply,
            "released at {time}: {observed} against {supply}"
        );
    }
}

//=============================================================================
// 6. A template boundary is static
//=============================================================================

#[test]
fn a_template_boundary_reports_static_and_is_not_bound_live() {
    let deck = "\
* template boundary against an inferred rail
V1 vdd 0 DC 5
Rpull mix vdd 10k
.model pull d_pullup
apull [mix] pull
.control
set auto_bridge_d_out = ( \".model tdac dac_bridge(out_low = 0 out_high = %g)\" \"atmpl%d [ %s ] [ %s ] tdac\" 1 )
.endc
.end
";
    let (_, lines) = boundary_lines_of(|| run(deck, 10e-9, 1e-9));
    assert!(
        lines
            .iter()
            .any(|line| line.contains("template") && line.contains("static")),
        "the template boundary says it is static: {lines:?}"
    );
    assert!(
        !lines.iter().any(|line| line.contains("bound live")),
        "a template is never bound live: {lines:?}"
    );

    // The record says the same: the template's key, and the inferred 5 V
    // rail as a static level, not a live net. The run's notices say the
    // supply was inferred and that the template holds it static.
    let collector = NoticeCollector::new();
    let records = {
        let _installed = collector.install();
        insertions(&Netlist::parse(deck).expect("deck parses"))
    };
    let kinds: Vec<_> = collector
        .take()
        .into_iter()
        .filter_map(|recorded| match recorded.notice {
            EngineNotice::BoundarySupply { boundary, kind, .. }
                if boundary.eq_ignore_ascii_case("mix") =>
            {
                Some(kind)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        kinds,
        [
            BoundarySupplyKind::Inferred,
            BoundarySupplyKind::TemplateStatic
        ]
    );
    assert_eq!(records.len(), 1, "{records:#?}");
    let record = &records[0];
    assert!(record.boundary.eq_ignore_ascii_case("mix"), "{record:#?}");
    assert_eq!(record.kind, ConverterKind::Template);
    assert_eq!(record.direction, PortDirection::Out);
    assert_eq!(
        record.rules,
        ConverterSource::Template("auto_bridge_d_out".into())
    );
    let supply = record
        .supply
        .as_ref()
        .expect("a digital boundary has a supply");
    assert_eq!((supply.level, supply.net.as_deref()), (5.0, None));
    assert!(
        matches!(&supply.source, ConverterSource::Inferred { source, level }
            if source.eq_ignore_ascii_case("v1") && *level == 5.0),
        "{supply:#?}"
    );
}

//=============================================================================
// The insertion record, as data
//=============================================================================

/// A corpus deck, read from its file so its relative includes resolve.
fn corpus_deck(design: &str, deck: &str) -> Netlist {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(design)
        .join("deck")
        .join(deck);
    Netlist::parse_file(&path).unwrap_or_else(|error| panic!("{} parses: {error}", path.display()))
}

fn insertions(netlist: &Netlist) -> Vec<ConverterInsertion> {
    Engine::default()
        .build_circuit(netlist)
        .unwrap_or_else(|error| panic!("the deck builds: {error}"))
        .converter_insertions()
        .to_vec()
}

/// The one converter on `node`, which `port` of the deck's HDL instance
/// meets.
fn only_on<'a>(
    records: &'a [ConverterInsertion],
    node: &str,
    port: &str,
) -> &'a ConverterInsertion {
    let here: Vec<_> = records
        .iter()
        .filter(|record| {
            record.boundary.eq_ignore_ascii_case(node)
                && record
                    .port
                    .as_deref()
                    .is_some_and(|named| named.eq_ignore_ascii_case(port))
        })
        .collect();
    assert_eq!(
        here.len(),
        1,
        "one converter on {node} for {port}: {records:#?}"
    );
    here[0]
}

fn parameter<'a>(record: &'a ConverterInsertion, name: &str) -> &'a ConverterParameter {
    record
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .unwrap_or_else(|| panic!("{name} in {record:#?}"))
}

/// C9 `live`: no `.param vcc`, and its rules (`net/por_ctrl_live.vams:54-57`)
/// give `a2d` and `d2a` the net `vdd` as `vsup`. The controller's inputs `hi`
/// and `lo_n` each read through one `a2d`, and its outputs `por_n`, `rst_n`
/// and `tie_hi` each drive through one `d2a`; `clk` is an event net the
/// XSPICE counter shares, not a boundary. The modules are the earlier
/// signature's verbatim copies (`vsup` and the two edge times), so the
/// statement binds the supply live and the built-in rule set gives every
/// parameter the statement does not: the edge times are its 0, not the copy's
/// 1 ns. With no rail reaching the boundary (`vdd` is a PWL source) the
/// nominal level is the 3.3 V default, the thresholds its half, tracking.
#[test]
fn c9_live_records_the_statement_binding_vdd_live() {
    let records = insertions(&corpus_deck("c09-por", "live.tran.cir"));
    assert_eq!(records.len(), 5, "{records:#?}");
    let rule_set = ConverterSource::RuleSet("rspice_builtin".into());
    for (node, kind, direction, line) in [
        ("hi", ConverterKind::A2d, PortDirection::In, 55),
        ("lo_n", ConverterKind::A2d, PortDirection::In, 55),
        ("por_n", ConverterKind::D2a, PortDirection::Out, 56),
        ("rst_n", ConverterKind::D2a, PortDirection::Out, 56),
        ("tie_hi", ConverterKind::D2a, PortDirection::Out, 56),
    ] {
        let record = only_on(&records, node, &format!("xpor.{node}"));
        assert_eq!(
            (record.kind, record.direction),
            (kind, direction),
            "{record:#?}"
        );
        let module = if kind == ConverterKind::A2d {
            "a2d"
        } else {
            "d2a"
        };
        assert_eq!(record.module, module);
        assert_eq!(
            record.rules,
            ConverterSource::ConnectRule {
                block: "por_live".into(),
                earlier_signature: true,
            }
        );
        assert!(
            record
                .location
                .as_deref()
                .is_some_and(|at| at.ends_with(&format!("por_ctrl_live.vams:{line}"))),
            "{record:#?}"
        );
        let supply = record.supply.as_ref().expect("a supply");
        assert_eq!(supply.net.as_deref(), Some("vdd"));
        assert_eq!(supply.level, 3.3);
        assert_eq!(supply.source, ConverterSource::ConnectStatement);
        assert!(
            record
                .parameters
                .iter()
                .all(|parameter| parameter.source == rule_set),
            "the statement gives nothing but vsup: {record:#?}"
        );
        if kind == ConverterKind::A2d {
            let tdrise = parameter(record, "tdrise");
            assert_eq!((tdrise.value, tdrise.tracks), (0.0, false));
            let vthi = parameter(record, "vthi");
            assert_eq!((vthi.value, vthi.tracks), (1.65, true));
            assert_eq!(vthi.rule.as_deref(), Some("vsup/2"));
        } else {
            let vhi = parameter(record, "vhi");
            assert_eq!((vhi.value, vhi.tracks), (3.3, true));
            assert_eq!(parameter(record, "trise").value, 0.0);
            assert_eq!(parameter(record, "rlo").value, 20.0);
        }
        assert!(
            record
                .to_string()
                .contains("vsup=net vdd, live (connect statement)"),
            "{record}"
        );
    }
}

/// C7 `cont`: three Verilog instances on the I2C bus, no connect rules, and
/// `.param vcc=3.3` (`net/bus.cir`). Every port converts through the built-in
/// rule set at the deck's static 3.3 V: the controller's `scl` and `sda` and
/// both targets' `sda` are `inout`, so `bidir`; the targets' `scl` is an
/// input, so `a2d`. Three converters on each net.
#[test]
fn c7_cont_records_three_converters_on_each_bus_net() {
    let records = insertions(&corpus_deck("c07-i2c", "cont.tran.cir"));
    assert_eq!(records.len(), 6, "{records:#?}");
    let rule_set = ConverterSource::RuleSet("rspice_builtin".into());
    for (node, port, kind) in [
        ("scl", "xctrl.scl", ConverterKind::Bidir),
        ("scl", "xdev2a.scl", ConverterKind::A2d),
        ("scl", "xdev51.scl", ConverterKind::A2d),
        ("sda", "xctrl.sda", ConverterKind::Bidir),
        ("sda", "xdev2a.sda", ConverterKind::Bidir),
        ("sda", "xdev51.sda", ConverterKind::Bidir),
    ] {
        let record = only_on(&records, node, port);
        let (module, direction) = if kind == ConverterKind::A2d {
            ("a2d", PortDirection::In)
        } else {
            ("bidir", PortDirection::InOut)
        };
        assert_eq!(
            (record.kind, record.direction, record.module.as_str()),
            (kind, direction, module),
            "{record:#?}"
        );
        assert_eq!(record.rules, rule_set);
        assert_eq!(record.location, None);
        let supply = record.supply.as_ref().expect("a supply");
        assert_eq!((supply.level, supply.net.as_deref()), (3.3, None));
        assert_eq!(supply.source, ConverterSource::Parameter("vcc".into()));
        assert!(
            record
                .parameters
                .iter()
                .all(|parameter| parameter.source == rule_set && !parameter.tracks),
            "{record:#?}"
        );
        assert_eq!(parameter(record, "vthi").value, 1.65);
        assert_eq!(parameter(record, "txdel").value, f64::INFINITY);
        if kind == ConverterKind::Bidir {
            assert_eq!(parameter(record, "rlo").value, 20.0);
            assert_eq!(parameter(record, "vhi").value, 3.3);
        }
    }
    let instances: std::collections::BTreeSet<_> = records
        .iter()
        .map(|record| record.instance.as_str())
        .collect();
    assert_eq!(instances.len(), 6, "each port has its own converter");
}

//=============================================================================
// Checkpoints across a connect-rule converter
//=============================================================================

/// A Verilog output into 1 kΩ through `d2a` with 100 ns edges started 20 ns
/// after each event: 1 at 50 ns (ramp over [70, 170] ns), 0 at 250 ns.
fn ramped_port_deck(directory: &TempDirectory) -> Netlist {
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module drv(o);\n  output o; reg o;\n  initial begin o = 1'b0; #50 o = 1'b1; #200 o = 1'b0; end\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(
        "connectrules deck;\n    connect a2d;\n    connect d2a #(.trise(100n), .tfall(100n), .tdelay(20n));\nendconnectrules\n",
    );
    let path = directory.write("drv.vams", &source);
    Netlist::parse(&format!(
        "* ramped port\n.param vcc=3.3\n.hdl \"{path}\" module=drv\n.options connectrules=deck\nx1 out drv\nr1 out 0 1k\n.end\n"
    ))
    .expect("deck parses")
}

/// A checkpoint taken while a converter waits out `tdelay` (60 ns) or ramps
/// (100 ns) resumes onto the uninterrupted run; and an image whose converter
/// is another model, as an image taken before the connect-rule converters
/// is, is refused with the instance named.
#[test]
fn a_checkpoint_across_a_converter_resumes_or_refuses_by_name() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let directory = TempDirectory::new("checkpoint");
    let netlist = ramped_port_deck(&directory);
    let engine = Engine::default();
    let whole = engine
        .run_tran(&netlist, 400e-9, 1e-9)
        .expect("the uninterrupted run");
    for stop in [60e-9, 100e-9] {
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, stop, 1e-9)
            .expect("checkpoints");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 400e-9, 1e-9)
            .expect("resumes");
        for time in [80e-9, 120e-9, 169e-9, 200e-9, 280e-9, 350e-9]
            .into_iter()
            .filter(|time| *time > stop)
        {
            let (a, b) = (
                value_at(&whole, "out", time),
                value_at(&resumed, "out", time),
            );
            assert!(
                (a - b).abs() <= 1e-9,
                "resumed from {stop:e} at {time:e}: {b} against {a}"
            );
        }

        let text = String::from_utf8(
            checkpoint
                .to_bytes(TransientCheckpointEncoding::Unpacked)
                .expect("serializes"),
        )
        .expect("canonical text");
        assert!(
            text.contains("__rspice_connect_dac"),
            "the image names its model"
        );
        // An image taken before the converter changed model carries another
        // instance identity: the structural hash of the instance, its model
        // name included. Perturb the converter's (the first identity in the
        // mixed image; its context's follows) without changing the text's
        // length.
        let at = text.find("\"identity\":[").expect("an instance identity") + 12;
        let digits = text[at..]
            .find(|c: char| !c.is_ascii_digit())
            .expect("a number");
        let byte: u32 = text[at..at + digits].parse().expect("a byte");
        let other = if byte % 10 == 9 { byte - 1 } else { byte + 1 };
        let older = format!("{}{other}{}", &text[..at], &text[at + digits..]);
        let refusal = match TransientCheckpoint::from_bytes(older.as_bytes()) {
            Err(error) => error,
            Ok(image) => match engine.run_tran_resume(&netlist, &image, 400e-9, 1e-9) {
                Ok(_) => panic!("an image with another converter model is refused"),
                Err(error) => error.to_string(),
            },
        };
        assert!(
            refusal.contains("__rspice_auto_dac"),
            "the refusal names the instance: {refusal}"
        );
    }
}

/// A Verilog buffer between a banded A/D with an X timeout and a delayed,
/// ramped D/A into 1 kΩ:
///
/// * the A/D: `vtlo` 1 V, `vthi` 2 V, `txdel` 1 µs. The input ramps to
///   1.5 V over 1 µs and enters the band at 2/3 µs; it is still there at the
///   deadline 5/3 µs, where the A/D publishes X. It rises through `vthi` at
///   3 µs + 0.5 V / 3.6 V/µs, and the A/D publishes 1.
/// * the D/A: each change starts `tdelay` = 50 ns after the buffer's event
///   and takes the full `trise` or `tfall`, 100 ns: 0 → X ramps over
///   [5/3 µs + 50 ns, + 150 ns], X → 1 over [rise + 50 ns, + 150 ns].
///
/// `stock` puts both ports on explicit ngspice cards instead, so no connect
/// rule reaches them: an `adc_bridge` with the same thresholds, which reads X
/// as soon as the input is inside them and publishes 1 ns later, and a
/// `dac_bridge` with 100 ns full-swing edges, so a 0 → X or X → 1 ramp ends
/// 50 ns after its event.
fn deadline_deck(directory: &TempDirectory, stock: bool) -> Netlist {
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module buffer(i, o);\n  input i; output o;\n  assign o = i;\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(
        "connectrules deck;\n    connect a2d #(.vtlo(1.0), .vthi(2.0), .txdel(1u));\n    connect d2a #(.trise(100n), .tfall(100n), .tdelay(50n));\nendconnectrules\n",
    );
    let path = directory.write("buffer.vams", &source);
    let ports = if stock {
        "x1 ind od buffer\n\
         aadc [in] [ind] stockadc\n\
         .model stockadc adc_bridge(in_low=1.0 in_high=2.0)\n\
         adac [od] [out] stockdac\n\
         .model stockdac dac_bridge(out_low=0 out_high=3.3 out_undef=1.65 t_rise=100n t_fall=100n)\n"
    } else {
        "x1 in out buffer\n"
    };
    Netlist::parse(&format!(
        "* banded A/D and ramped D/A\n\
         .param vcc=3.3\n\
         .hdl \"{path}\" module=buffer\n\
         .options connectrules=deck\n\
         vin in 0 pwl(0 0 1u 1.5 3u 1.5 3.5u 3.3)\n\
         {ports}\
         r1 out 0 1k\n\
         .end\n"
    ))
    .expect("deck parses")
}

/// The run resumed from `stop` against the uninterrupted one: every event
/// after `stop` at the same instant with the same value, and `out` within
/// 1e-9 V at every point either run accepted after it. The first difference
/// is the error.
fn resumes_onto(
    whole: &TransientResult,
    resumed: &TransientResult,
    stop: f64,
) -> Result<(), String> {
    for trace in &whole.digital_traces {
        let after = |points: &[rspice_core::engine::DigitalTracePoint]| {
            points
                .iter()
                .filter(|point| point.time > stop)
                .copied()
                .collect::<Vec<_>>()
        };
        let other = resumed
            .digital_traces
            .iter()
            .find(|other| other.node_name == trace.node_name)
            .ok_or_else(|| format!("the resumed run does not trace {}", trace.node_name))?;
        let (resumed_events, whole_events) = (after(&other.points), after(&trace.points));
        if resumed_events != whole_events {
            return Err(format!(
                "events on {} resumed from {stop:e}: {resumed_events:?} against {whole_events:?}",
                trace.node_name
            ));
        }
    }
    let end = *whole.time.last().expect("a run");
    for time in whole
        .time
        .iter()
        .chain(&resumed.time)
        .copied()
        .filter(|time| *time > stop && *time <= end)
    {
        let (a, b) = (value_at(whole, "out", time), value_at(resumed, "out", time));
        if (a - b).abs() > 1e-9 {
            return Err(format!(
                "out resumed from {stop:e} at {time:e}: {b} against {a}"
            ));
        }
    }
    Ok(())
}

fn assert_resumes_onto(whole: &TransientResult, resumed: &TransientResult, stop: f64) {
    if let Err(difference) = resumes_onto(whole, resumed, stop) {
        panic!("{difference}");
    }
}

/// The instant `node`'s digital trace first takes `value` after `from`.
fn event_time(
    result: &TransientResult,
    node: &str,
    value: rspice_core::xspice::DigitalValue,
    from: f64,
) -> f64 {
    result
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("no trace of {node}: {:?}", result.digital_traces))
        .points
        .iter()
        .find(|point| point.time > from && point.value == value)
        .unwrap_or_else(|| panic!("{node} never reads {value:?} after {from}"))
        .time
}

/// A checkpoint taken inside the A/D's band with its X deadline 2/3 µs
/// ahead, inside the D/A's `tdelay` wait, and inside its ramp resumes onto
/// the uninterrupted run: the same events at the same instants, and the same
/// analog output. The A/D's band entry is its own state, and the delayed
/// start is the buffer's event time plus `tdelay`, which the converter reads
/// from the event time the image restores.
#[test]
fn a_checkpoint_inside_a_pending_deadline_resumes_onto_the_uninterrupted_run() {
    use rspice_core::xspice::DigitalValue;
    let directory = TempDirectory::new("deadlines");
    let netlist = deadline_deck(&directory, false);
    let engine = Engine::default();
    let whole = engine
        .run_tran(&netlist, 4e-6, 10e-9)
        .expect("the uninterrupted run");
    let names = whole
        .digital_traces
        .iter()
        .map(|trace| trace.node_name.clone())
        .collect::<Vec<_>>();
    let x = event_time(&whole, &names[0], DigitalValue::unknown(), 0.0);
    assert!(
        (x - 5.0e-6 / 3.0).abs() <= 1e-12,
        "X at the deadline: {x} ({names:?})"
    );
    let start = x + 50e-9;
    for stop in [1.0e-6, x + 25e-9, start + 50e-9] {
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, stop, 10e-9)
            .expect("checkpoints");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 4e-6, 10e-9)
            .expect("resumes");
        assert_resumes_onto(&whole, &resumed, stop);
    }
}

/// The same with the checkpoint 30 ps before each deadline, inside the
/// 100 ps first step the resumed run takes (a hundredth of the 10 ns step
/// hint): the X timeout, the delayed start and both ramp ends on the
/// connect-rule converters, and both ramp ends of a stock `dac_bridge`. A
/// model asks for each at every accepted point and a resumed run evaluates
/// none at its first, so only the image can hold them; without it the
/// resumed run stepped over every one and published the X, started the ramp
/// or ended it 70 ps late.
#[test]
fn a_checkpoint_just_before_a_deadline_resumes_onto_the_uninterrupted_run() {
    use rspice_core::xspice::DigitalValue;
    let directory = TempDirectory::new("deadline-edges");
    let mut failures = Vec::new();
    for stock in [false, true] {
        let netlist = deadline_deck(&directory, stock);
        let engine = Engine::default();
        let whole = engine
            .run_tran(&netlist, 4e-6, 10e-9)
            .expect("the uninterrupted run");
        let names = whole
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.clone())
            .collect::<Vec<_>>();
        let x = event_time(&whole, &names[0], DigitalValue::unknown(), 0.0);
        let rise = event_time(&whole, &names[0], DigitalValue::one(), x);
        let stops = if stock {
            vec![
                ("stock ramp end", x + 50e-9 - 30e-12),
                ("stock second ramp end", rise + 50e-9 - 30e-12),
            ]
        } else {
            vec![
                ("X deadline", x - 30e-12),
                ("delayed start", x + 50e-9 - 30e-12),
                ("ramp end", x + 150e-9 - 30e-12),
                ("second ramp end", rise + 150e-9 - 30e-12),
            ]
        };
        for (what, stop) in stops {
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&netlist, stop, 10e-9)
                .expect("checkpoints");
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &checkpoint, 4e-6, 10e-9)
                .unwrap_or_else(|error| panic!("resumes from {stop:e} (stock {stock}): {error}"));
            if let Err(difference) = resumes_onto(&whole, &resumed, stop) {
                failures.push(format!("{what}: {difference}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// An image from before the format that carries those breakpoints is
/// refused when the circuit holds a connect-rule converter, naming it: one
/// of its deadlines may have been pending. A circuit on stock XSPICE cards
/// resumes from one as it always did.
#[test]
fn an_image_without_the_model_breakpoints_resumes_only_stock_converters() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let directory = TempDirectory::new("older-image");
    let engine = Engine::default();
    for stock in [false, true] {
        let netlist = deadline_deck(&directory, stock);
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 1e-6, 10e-9)
            .expect("checkpoints");
        let text = String::from_utf8(
            checkpoint
                .to_bytes(TransientCheckpointEncoding::Unpacked)
                .expect("serializes"),
        )
        .expect("canonical text");
        let (header, body) = text.split_once('\n').expect("a header line");
        assert_eq!(header, "RSPICE-CHECKPOINT 62");
        let mut older = String::from("RSPICE-CHECKPOINT 60\n");
        for line in body.lines().filter(|line| {
            !line.starts_with("pending_model_breakpoints ") && !line.starts_with("vcd_files ")
        }) {
            older.push_str(line);
            older.push('\n');
        }
        let image = TransientCheckpoint::from_bytes(older.as_bytes()).expect("a format-60 image");
        let outcome = engine.run_tran_resume(&netlist, &image, 4e-6, 10e-9);
        if stock {
            outcome.expect("an image on stock cards resumes");
        } else {
            let refusal = match outcome {
                Ok(_) => panic!("an older image with a connect-rule converter is refused"),
                Err(error) => error.to_string(),
            };
            assert!(
                refusal.contains("__rspice_auto_") && refusal.contains("(__rspice_connect_"),
                "the refusal names the converter and its model: {refusal}"
            );
        }
    }
}

//=============================================================================
// 7. A driver-sensitive D/A on an XSPICE node
//=============================================================================

/// A resistive 1 (`d_pullup`, `rpull` = 10 kΩ) and a strong 0 (a buffer of a
/// pulled-down input, 20 Ω) on one event node: each converts as its own
/// branch, and the node divides between them, 3.3·20/10020 = 6.587 mV. The
/// digitally resolved strong 0 would read exactly 0 V.
#[test]
fn a_weak_one_and_a_strong_zero_divide_on_an_xspice_node() {
    let directory = TempDirectory::new("driver-sensitive");
    let path = directory.write(
        "lib.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            "connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n",
        ),
    );
    let result = run(
        &format!(
            "* weak 1 against strong 0\n\
             .param vcc=3.3\n\
             .veriloga \"{path}\"\n\
             .model pull d_pullup\n\
             .model down d_pulldown\n\
             .model buf1 d_buffer\n\
             apull [mix] pull\n\
             adown [lo] down\n\
             abuf lo mix buf1\n\
             rload mix 0 1e12\n\
             .end\n"
        ),
        10e-9,
        1e-9,
    );
    let expected = 3.3 * 20.0 / 10_020.0;
    let observed = value_at(&result, "mix", 5e-9);
    assert!(
        (observed - expected).abs() <= 1e-6,
        "driver-sensitive divider: {observed} against {expected}"
    );
}

//=============================================================================
// 8. A tracking A/D's band in the operating point's census
//=============================================================================

/// A supply-tracking A/D declares each band edge as the zero of its own
/// crossing function. With V(vdd) = 4 V, `vtlo` = vsup/2 tracks to 2 V and
/// `vthi` = 3 V, and an input held at 2.5 V lies inside the band: the
/// operating point's census reports the band in volts, (2 V, 3 V), as it does
/// a static converter's.
#[test]
fn a_tracking_converter_inside_its_band_is_reported_in_volts() {
    let directory = TempDirectory::new("tracking-band");
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module probe(din, q);\n  input din; output q; reg q;\n  initial q = 1'b0;\n  always @(din) q = din;\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(
        "connectrules track;\n    connect a2d #(.vsup(vdd), .vthi(3.0));\n    connect d2a #(.vsup(vdd));\nendconnectrules\n",
    );
    let path = directory.write("probe.vams", &source);
    let netlist = Netlist::parse(&format!(
        "* tracking A/D inside its band\n\
         .hdl \"{path}\" module=probe\n\
         .options connectrules=track\n\
         vdd vdd 0 4\n\
         vin din 0 2.5\n\
         x1 din q probe\n\
         rq q 0 1k\n\
         .end\n"
    ))
    .unwrap_or_else(|error| panic!("deck parses: {error}"));
    let (solved, lines) = lines_of("[mixed.threshold]", || {
        Engine::default().run_dc_op(&netlist)
    });
    solved.unwrap_or_else(|error| panic!("the operating point solves: {error}"));
    assert_eq!(lines.len(), 1, "one finding: {lines:?}");
    assert!(
        lines[0].contains("lies inside its undefined band (2e0 V, 3e0 V)")
            && lines[0].contains("(v = 2.5e0 V)"),
        "{}",
        lines[0]
    );
}

//=============================================================================
// The shipped real converters
//=============================================================================

/// `source` (an HDL design), the shipped real modules and one rules block, as
/// a library file.
fn real_library(directory: &TempDirectory, source: &str, rules: &str) -> String {
    let mut text = format!("`include \"disciplines.vams\"\n`timescale 1ns/1ps\n{source}");
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_REAL_CONNECT_MODULES {
        text.push_str(module);
    }
    text.push_str(rules);
    directory.write("real.vams", &text)
}

/// An HDL real output of 0.6 behind `r2e` at `rout` = 1 kΩ, on node `n` of
/// `vin` (AC 1) through 1 kΩ with 1 kΩ to ground; the twin replaces the
/// converter with its resistor to ground, which is what a held ideal source
/// behind `rout` is to small signals.
fn r2e_divider(directory: &TempDirectory, twin: bool) -> Netlist {
    let converter = if twin {
        "rr n 0 1k\n".to_string()
    } else {
        let path = real_library(
            directory,
            "module level(r);\n  output r; wreal r;\n  assign r = 0.6;\nendmodule\n",
            "connectrules deck;\n    connect r2e #(.rout(1k));\nendconnectrules\n",
        );
        format!(".hdl \"{path}\" module=level\n.options connectrules=deck\nx1 n level\n")
    };
    Netlist::parse(&format!(
        "* r2e behind rout\nvin in 0 dc 0 ac 1\nrin in n 1k\nrl n 0 1k\n{converter}.end\n"
    ))
    .expect("deck parses")
}

/// `r2e` at `rout` = 1 kΩ: the operating point divides 0.6 V by three, the
/// small-signal gain from `in` is (1/1k)/(3/1k) = 1/3, and the noise at `n`
/// is the resistor twin's, 4kT/`rout` across the converter.
#[test]
fn r2e_is_its_value_behind_rout_in_every_analysis() {
    let directory = TempDirectory::new("r2e-small-signal");
    let netlist = r2e_divider(&directory, false);
    let records = insertions(&netlist);
    let record = only_on(&records, "n", "x1.r");
    assert_eq!(record.kind, ConverterKind::R2e);
    assert_eq!(record.module, "r2e");
    assert!(record.supply.is_none());
    let rout = parameter(record, "rout");
    assert_eq!(
        (rout.value, &rout.source),
        (1e3, &ConverterSource::ConnectStatement)
    );
    assert_eq!(
        parameter(record, "trise").source,
        ConverterSource::ModuleDefault
    );

    let engine = Engine::default();
    let tran = engine
        .run_tran(&netlist, 10e-9, 1e-9)
        .expect("transient solves");
    assert!((value_at(&tran, "n", 5e-9) - 0.2).abs() < 1e-9);
    let ac = engine.run_ac(&netlist, &[1e3]).expect("AC solves");
    let n = ac[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("n"))
        .expect("n is published");
    let gain = ac[0].voltages[n];
    assert!(
        (gain.re - 1.0 / 3.0).abs() < 1e-12 && gain.im.abs() < 1e-12,
        "{gain}"
    );

    let twin = r2e_divider(&directory, true);
    let density = |netlist: &Netlist| {
        let node = engine
            .build_circuit(netlist)
            .expect("builds")
            .get_node_by_name("n")
            .expect("n exists");
        engine
            .run_noise(netlist, node, &[1e3], 300.15)
            .expect("noise solves")[0]
            .output_noise_density
    };
    let (converter, resistor) = (density(&netlist), density(&twin));
    // 4kT·(R/3) at 300.15 K: three 1 kΩ in parallel.
    let expected = 4.0 * 1.380649e-23 * 300.15 * (1e3 / 3.0);
    assert!((resistor - expected).abs() < 1e-3 * expected, "{resistor}");
    assert!(
        (converter - resistor).abs() < 1e-12 * resistor,
        "{converter} against {resistor}"
    );
}

/// An HDL real stepping 0 → 1 at 100 ns and back at 400 ns behind `r2e` with
/// `rout` into 900 Ω: the rise takes `trise` = 200 ns and the fall `tfall` =
/// 50 ns, divided by 0.9 at `rout` = 100 Ω.
fn r2e_ramp_deck(directory: &TempDirectory, rout: &str) -> Netlist {
    let path = real_library(
        directory,
        "module step(r);\n  output r; wreal r;\n  real v;\n  initial begin v = 0.0; #100 v = 1.0; #300 v = 0.0; end\n  assign r = v;\nendmodule\n",
        &format!(
            "connectrules deck;\n    connect r2e #(.rout({rout}), .trise(200n), .tfall(50n));\nendconnectrules\n"
        ),
    );
    Netlist::parse(&format!(
        "* r2e ramps\n.hdl \"{path}\" module=step\n.options connectrules=deck\nx1 out step\nrl out 0 900\n.end\n"
    ))
    .expect("deck parses")
}

/// Each edge takes its own time, behind `rout` or ideal; and a checkpoint
/// taken inside the rise resumes onto the uninterrupted run.
#[test]
fn r2e_ramps_each_edge_and_resumes_inside_a_ramp() {
    use rspice_core::engine::TransientCheckpointEncoding;
    let directory = TempDirectory::new("r2e-ramp");
    let engine = Engine::default();
    for (rout, scale) in [("100", 0.9), ("0", 1.0)] {
        let netlist = r2e_ramp_deck(&directory, rout);
        let whole = engine
            .run_tran(&netlist, 600e-9, 1e-9)
            .expect("the uninterrupted run");
        for (time, level) in [
            (50e-9, 0.0),
            (150e-9, 0.25),
            (200e-9, 0.5),
            (350e-9, 1.0),
            (412.5e-9, 0.75),
            (500e-9, 0.0),
        ] {
            let value = value_at(&whole, "out", time);
            assert!(
                (value - level * scale).abs() < 1e-9,
                "rout {rout} at {time:e}: {value}"
            );
        }
        if rout == "0" {
            continue;
        }
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 150e-9, 1e-9)
            .expect("checkpoints inside the rise");
        let text = String::from_utf8(
            checkpoint
                .to_bytes(TransientCheckpointEncoding::Unpacked)
                .expect("serializes"),
        )
        .expect("canonical text");
        assert!(
            text.contains("__rspice_connect_r2e"),
            "the image names its model"
        );
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 600e-9, 1e-9)
            .expect("resumes");
        assert_resumes_onto(&whole, &resumed, 150e-9);
    }
}

/// `e2r` with `vdelta` = 10 mV on a 1 V/ms ramp publishes once per 10 mV:
/// 100 publications over the millisecond, give or take the last.
#[test]
fn e2r_publishes_once_per_vdelta_on_a_ramp() {
    let directory = TempDirectory::new("e2r-vdelta");
    let path = real_library(
        &directory,
        "",
        "connectrules deck;\n    connect e2r #(.vdelta(10m));\n    connect r2e;\nendconnectrules\n",
    );
    let netlist = Netlist::parse(&format!(
        "* e2r on a ramp\n.veriloga \"{path}\"\nvin in 0 pwl(0 0 1m 1)\na1 in out g\n.model g real_gain\nrl out 0 1k\n.end\n"
    ))
    .expect("deck parses");
    let records = insertions(&netlist);
    let sample = records
        .iter()
        .find(|record| record.kind == ConverterKind::E2r)
        .unwrap_or_else(|| panic!("an e2r: {records:#?}"));
    assert_eq!(parameter(sample, "vdelta").value, 10e-3);
    let drive = records
        .iter()
        .find(|record| record.kind == ConverterKind::R2e)
        .unwrap_or_else(|| panic!("an r2e: {records:#?}"));
    assert_eq!(
        parameter(drive, "rout").source,
        ConverterSource::ModuleDefault
    );

    let result = Engine::default()
        .run_tran(&netlist, 1e-3, 1e-6)
        .expect("transient solves");
    // The event side of the boundary keeps the deck's name for it.
    let trace = result
        .real_trace_named("in")
        .expect("the sampled real net is traced");
    let published: Vec<_> = trace.iter().filter(|point| point.time > 0.0).collect();
    assert!(
        (99..=101).contains(&published.len()),
        "{} publications",
        published.len()
    );
    // Each publication is the node's value where it was taken.
    for point in &published {
        assert!((point.value - point.time * 1e3).abs() < 1e-9, "{point:?}");
    }
}

/// An HDL real inout `p` on node `n`, which `supply` feeds through 1 kΩ with
/// 1 kΩ to ground, through `rbidir` at its default `rout` = 20 Ω. `drive` is
/// the port's own driver, if it has one; `q` copies what the port reads onto
/// `qn` through the stock converter (`r2e` at its defaults).
fn rbidir_deck(directory: &TempDirectory, drive: &str, supply: &str) -> Netlist {
    let path = real_library(
        directory,
        &format!(
            "module port(p, q);\n  inout p; wreal p;\n  output q; wreal q;\n{drive}  assign q = p;\nendmodule\n"
        ),
        "connectrules deck;\n    connect rbidir;\n    connect r2e;\nendconnectrules\n",
    );
    Netlist::parse(&format!(
        "* rbidir\n.hdl \"{path}\" module=port\n.options connectrules=deck\nvs s 0 {supply}\nrs s n 1k\nrg n 0 1k\nx1 n qn port\nrq qn 0 1k\n.end\n"
    ))
    .expect("deck parses")
}

/// While its port's own drivers have not driven (here: it has none),
/// `rbidir` reads the node into it, 1.2 V, and drives nothing. A port that
/// drives 1.0 V is driven behind 20 Ω: (2.4/1k + 1.0/20)/(1/1k + 1/1k +
/// 1/20) = 0.0524/0.052 = 1.007692 V, and the port reads the net its drivers
/// resolve, 1.0. A port that drives exactly the voltage it would read takes
/// the node over all the same: when the supply steps to 3.6 V the node is
/// (3.6/1k + 1.2/20)/0.052 = 1.223077 V and the port reads 1.2, where a
/// port still reading the node would see 1.8 V.
///
/// A continuous assignment drives from time zero, and an assignment delay
/// has no lowered form yet, so a port's drivers have run from the start or
/// never; the store's own test drives a receiving net mid-run.
#[test]
fn rbidir_reads_the_node_until_its_port_drives_and_then_drives_behind_rout() {
    let directory = TempDirectory::new("rbidir");
    let netlist = rbidir_deck(&directory, "", "2.4");
    let records = insertions(&netlist);
    let record = only_on(&records, "n", "x1.p");
    assert_eq!(record.kind, ConverterKind::Rbidir);
    assert_eq!(record.direction, PortDirection::InOut);
    let rout = parameter(record, "rout");
    assert_eq!(
        (rout.value, &rout.source),
        (20.0, &ConverterSource::ModuleDefault)
    );

    let engine = Engine::default();
    let step = "pwl(0 2.4 20n 2.4 21n 3.6)";
    let driven = 0.0524 / 0.052;
    // Each deck rewrites the one library file, so each runs before the next
    // is written.
    for (drive, supply, before, after) in [
        ("", "2.4", (1.2, 1.2), (1.2, 1.2)),
        ("  assign p = 1.0;\n", "2.4", (driven, 1.0), (driven, 1.0)),
        (
            "  assign p = 1.2;\n",
            step,
            (1.2, 1.2),
            ((3.6e-3 + 1.2 / 20.0) / 0.052, 1.2),
        ),
        ("", step, (1.2, 1.2), (1.8, 1.8)),
    ] {
        let deck = rbidir_deck(&directory, drive, supply);
        let result = engine
            .run_tran(&deck, 40e-9, 0.1e-9)
            .expect("transient solves");
        for (time, (node, read)) in [(10e-9, before), (35e-9, after)] {
            let (n, qn) = (value_at(&result, "n", time), value_at(&result, "qn", time));
            assert!((n - node).abs() < 1e-9, "n at {time:e}: {n} against {node}");
            assert!(
                (qn - read).abs() < 1e-9,
                "qn at {time:e}: {qn} against {read}"
            );
        }
    }
}

/// `rbidir` refuses an ideal drive: its node has other sources.
#[test]
fn rbidir_refuses_an_ideal_drive_by_name() {
    let directory = TempDirectory::new("rbidir-ideal");
    let path = real_library(
        &directory,
        "module port(p);\n  inout p; wreal p;\n  assign p = 1.0;\nendmodule\n",
        "connectrules deck;\n    connect rbidir #(.rout(0));\nendconnectrules\n",
    );
    let error = build_error(&format!(
        "* rbidir ideal\n.hdl \"{path}\" module=port\n.options connectrules=deck\nvs n 0 1\nx1 n port\n.end\n"
    ));
    assert!(
        error.contains("rout = 0") && error.contains("rbidir"),
        "{error}"
    );
}

//=============================================================================
// Shipped rule sets
//=============================================================================

/// A Verilog buffer whose input and output carry `attributes`.
fn shipped_buffer(directory: &TempDirectory, attributes: &str) -> String {
    directory.write(
        "buf1.v",
        &format!(
            "module buf1(a, y);\n  {attributes}input a;\n  {attributes}output y;\n  assign y = \
             a;\nendmodule\n"
        ),
    )
}

/// Each fixed-supply shipped set converts a triangle 0 -> v -> 0 over 2 us
/// at its fractions of v: the input reaches fthi*v = 0.7*v at 0.7 us and
/// falls to ftlo*v = 0.3*v at 1.7 us (for rspice_1v8_fast, 1.26 V and
/// 0.54 V). The buffered output leaves the X level v/2 for v, or for 0, over
/// the set's full edge time, so it passes 0.75*v at 0.7 us + tr/2 and 0.25*v
/// at 1.7 us + tf/2.
#[test]
fn each_fixed_shipped_set_switches_at_its_fractions_of_the_supply() {
    let directory = TempDirectory::new("shipped-sets");
    let buffer = shipped_buffer(&directory, "");
    for (spelled, volts) in [
        ("1v0", 1.0),
        ("1v2", 1.2),
        ("1v8", 1.8),
        ("2v5", 2.5),
        ("3v3", 3.3),
        ("5v0", 5.0),
    ] {
        for (speed, edge) in [("fast", 100.0e-12), ("slow", 1.0e-9)] {
            let set = format!("rspice_{spelled}_{speed}");
            let result = run(
                &format!(
                    "* shipped set {set}\n.options connectrules={set}\nvin a 0 pwl(0 0 1u \
                     {volts} 2u 0)\nxb a y buf1\n.hdl \"{buffer}\"\n.end\n"
                ),
                2.0e-6,
                10.0e-9,
            );
            let rise = crossing(&result, "y", 0.75 * volts, 0.6e-6);
            let fall = crossing(&result, "y", 0.25 * volts, 1.6e-6);
            for (label, edge_at, expected) in [
                ("rise", rise, 0.7e-6 + edge / 2.0),
                ("fall", fall, 1.7e-6 + edge / 2.0),
            ] {
                assert!(
                    (edge_at - expected).abs() <= 1.0e-12,
                    "{set} {label}: {edge_at:e}, expected {expected:e}"
                );
            }
        }
    }
}

/// `rspice_track_fast` against a supply bound live: the input held at 1.0 V
/// while V(vdd) ramps 1.8 -> 1.2 V over 1 us reads 1 once 0.7*V(vdd) falls
/// to 1.0 V, at V(vdd) = 1/0.7 = 1.428571 V, which is
/// t = (1.8 - 1/0.7)/0.6 us = 0.619048 us. The output then leaves the X level
/// (half the supply) for the supply over 100 ps, so v(y)/v(vdd) passes 0.75
/// 50 ps later.
#[test]
fn the_tracking_shipped_set_switches_at_its_fraction_of_a_live_supply() {
    let directory = TempDirectory::new("shipped-track");
    let buffer = shipped_buffer(&directory, "(* supplySensitivity = \"vdd\" *) ");
    let result = run(
        &format!(
            "* rspice_track_fast\n.options connectrules=rspice_track_fast\nvdd vdd 0 pwl(0 1.8 \
             1u 1.2)\nvin a 0 1.0\nxb a y buf1\n.hdl \"{buffer}\"\n.end\n"
        ),
        1.0e-6,
        10.0e-9,
    );
    let output = series(&result, "y");
    let supply = series(&result, "vdd");
    let ratio: Vec<f64> = output.iter().zip(supply).map(|(y, v)| y / v).collect();
    let flip = (1.8 - 1.0 / 0.7) / 0.6 * 1.0e-6;
    let expected = flip + 50.0e-12;
    let mut crossed = None;
    for index in 1..ratio.len() {
        let (a, b) = (ratio[index - 1] - 0.75, ratio[index] - 0.75);
        if a < 0.0 && b >= 0.0 {
            let (t0, t1) = (result.time[index - 1], result.time[index]);
            crossed = Some(t0 + (t1 - t0) * a / (a - b));
            break;
        }
    }
    let crossed = crossed.expect("v(y)/v(vdd) passes 0.75");
    assert!(
        (crossed - expected).abs() <= 1.0e-12,
        "the flip is dated at {expected:e}, got {crossed:e}"
    );
}

/// A shipped set is selected by name, and the ways of selecting one wrongly
/// are refused by name: an unknown name lists the declared and the shipped
/// sets, a deck block may not take the shipped prefix, a shipped name does
/// not combine with CONNECTRULES_SOURCE, and a threshold given both as a
/// fraction and in volts is refused naming the fraction.
#[test]
fn shipped_set_selection_errors_name_the_choices() {
    let directory = TempDirectory::new("shipped-errors");
    let declared = directory.write(
        "mine.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            "connectrules mine;\n    connect a2d;\n    connect d2a;\nendconnectrules\n",
        ),
    );
    let unknown = build_error(&format!(
        "* unknown set\n.options connectrules=nosuch\n.veriloga \"{declared}\"\nvin din 0 \
         1\nrin din 0 1meg\nainv din dout inverter\n.model inverter d_inverter\n.end\n"
    ));
    for named in [
        "nosuch",
        "'mine'",
        "rspice_builtin",
        "rspice_3v3_fast",
        "rspice_track_slow",
    ] {
        assert!(unknown.contains(named), "lists {named}: {unknown}");
    }

    let shadow = directory.write(
        "shadow.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            "connectrules rspice_3v3_fast;\n    connect a2d;\nendconnectrules\n",
        ),
    );
    let shadowing = build_error(&xspice_deck(&shadow, "rload dout 0 1k"));
    assert!(
        shadowing.contains("rspice_3v3_fast") && shadowing.contains("reserves"),
        "{shadowing}"
    );

    let sourced = build_error(&format!(
        "* shipped set and source\n.options connectrules=rspice_3v3_fast \
         connectrules_source=lib\n.veriloga \"{declared}\" lib\nvin din 0 1\nrin din 0 \
         1meg\nainv din dout inverter\n.model inverter d_inverter\n.end\n"
    ));
    assert!(
        sourced.contains("rspice_3v3_fast") && sourced.contains("CONNECTRULES_SOURCE"),
        "{sourced}"
    );

    let both = directory.write(
        "both.va",
        &library(
            &BUILTIN_CONNECT_MODULES,
            "connectrules both;\n    connect a2d #(.fthi(0.7), .vthi(1.0));\n    connect \
             d2a;\nendconnectrules\n",
        ),
    );
    let doubled = build_error(&xspice_deck(&both, "rload dout 0 1k"));
    assert!(
        doubled.contains("fthi = 0.7") && doubled.contains("vthi"),
        "{doubled}"
    );
}
