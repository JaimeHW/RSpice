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

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
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
        .filter(|line| line.contains("boundary"))
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
                "vsup, vsupmin, vthi, vtlo, txdel, tdrise, tdfall, tdelay",
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
        assert_eq!(header, "RSPICE-CHECKPOINT 61");
        let mut older = String::from("RSPICE-CHECKPOINT 60\n");
        for line in body
            .lines()
            .filter(|line| !line.starts_with("pending_model_breakpoints "))
        {
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
