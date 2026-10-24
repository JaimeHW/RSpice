//! `.ASSERT` over a published result.
//!
//! Every expected value is a closed form of its fixture, never an RSpice
//! output. A transient check is solved on the interpolant of each segment of
//! the timeline (the accepted rows and every event instant), so a crossing of
//! a piecewise-linear waveform is exact and an event edge is dated at its
//! instant.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::{
    AssertViolation, AssertionReport, evaluate_dc_assertions, evaluate_op_assertions,
    evaluate_tran_assertions,
};
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{AnalysisCommand, AssertAnalysis, AssertLevel, Netlist, ParseError};

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

fn tran_reports(netlist: &Netlist) -> Vec<AssertionReport> {
    let result = run(netlist);
    evaluate_tran_assertions(netlist, &result, &NoAbort).expect("assertions evaluate")
}

fn report<'a>(reports: &'a [AssertionReport], name: &str) -> &'a AssertionReport {
    let report = reports
        .iter()
        .find(|report| report.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} is reported: {reports:?}"));
    assert!(report.error.is_none(), "{name}: {:?}", report.error);
    report
}

/// An instant the run dated at a closed-form time, to 4 ulp.
#[cfg(feature = "veriloga")]
fn assert_instant(actual: f64, expected: f64, what: &str) {
    let ulp = f64::EPSILON * expected.abs().max(f64::MIN_POSITIVE);
    assert!(
        (actual - expected).abs() <= 4.0 * ulp,
        "{what}: expected {expected:e}, got {actual:e}"
    );
}

/// The violation stretches `[start, end)`, `end` `None` when open.
fn stretches(report: &AssertionReport) -> Vec<(f64, Option<f64>)> {
    report
        .violations
        .iter()
        .map(|violation| (violation.start.expect("a timed start"), violation.end))
        .collect()
}

/// RC with τ = 1 µs driven by a 1 V step with a 1 ps edge: V(out) passes
/// 0.5 V at τ·ln 2, and stays above it through the stop time.
#[test]
fn an_rc_charging_past_its_bound_violates_from_tau_ln_2_open_at_the_stop() {
    let netlist = parse(
        "* rc\nV1 in 0 PULSE(0 1 0 1p 1p 1 2)\nR1 in out 1k\nC1 out 0 1n\n\
         .options reltol=1e-6\n.tran 1n 2u\n\
         .ASSERT a EXPR='V(out)' MAX=0.5 LEVEL=ERROR MESSAGE='out above half'\n.end\n",
    );
    let reports = tran_reports(&netlist);
    assert_eq!(reports.len(), 1, "{reports:?}");
    let report = report(&reports, "a");
    assert_eq!(report.analysis, AssertAnalysis::Tran);
    assert_eq!(report.level, AssertLevel::Error);
    assert_eq!(report.message.as_deref(), Some("out above half"));
    assert_eq!(report.origin.line, 7);
    let [violation] = report.violations.as_slice() else {
        panic!("one violation: {:?}", report.violations);
    };
    let tau_ln2 = 1.0e-6 * std::f64::consts::LN_2;
    let start = violation.start.expect("a start");
    assert!(
        (start - tau_ln2).abs() <= 0.05e-9,
        "V(out) reaches 0.5 V at τ·ln 2 = {tau_ln2:e} s: {start:e}"
    );
    assert_eq!(violation.end, None, "still above at the stop time");
    // The farthest excursion is at the stop: 1 - e^-2.
    let extreme = violation.extreme.expect("an extreme");
    assert!(
        (extreme - (1.0 - (-2.0f64).exp())).abs() < 1e-4,
        "{extreme}"
    );
    assert_eq!(violation.at, Some(2.0e-6));
}

/// A pulse with linear edges above 0.5 V from 10.5 ns: 6 ns is shorter than
/// `DURATION=10n` and is dropped; with a 15 ns width the 16 ns stretch
/// `[10.5n, 26.5n)` is reported, its ends exact.
#[test]
fn a_violation_shorter_than_its_duration_is_dropped_and_a_longer_one_is_exact() {
    let deck = |width: &str| {
        parse(&format!(
            "* pulse\nV1 n 0 PULSE(0 1 10n 1n 1n {width} 100n)\nR1 n 0 1k\n.tran 0.1n 40n\n\
             .ASSERT high EXPR='V(n)' MAX=0.5 DURATION=10n\n.end\n"
        ))
    };
    let reports = tran_reports(&deck("5n"));
    assert!(
        report(&reports, "high").violations.is_empty(),
        "6 ns above is shorter than 10 ns: {reports:?}"
    );

    let reports = tran_reports(&deck("15n"));
    let high = report(&reports, "high");
    let [(start, end)] = stretches(high)[..] else {
        panic!("one violation: {:?}", high.violations);
    };
    assert!((start - 10.5e-9).abs() <= 1e-18, "{start:e}");
    let end = end.expect("closed before the stop");
    assert!((end - 26.5e-9).abs() <= 1e-18, "{end:e}");
    assert_eq!(high.violations[0].extreme, Some(1.0));

    // Exactly as long as DURATION is reported.
    let netlist = parse(
        "* pulse\nV1 n 0 PULSE(0 1 10n 1n 1n 5n 100n)\nR1 n 0 1k\n.tran 0.1n 40n\n\
         .ASSERT six EXPR='V(n)' MAX=0.5 DURATION=6n\n.end\n",
    );
    let reports = tran_reports(&netlist);
    assert_eq!(report(&reports, "six").violations.len(), 1, "{reports:?}");
}

/// A level-1 NMOS biased at Vds = 1 V: the device forms expand to the
/// instance the model names and read its operating-point quantity at the
/// operating point and on every accepted transient row.
#[test]
fn a_device_form_reads_each_instance_its_target_names() {
    let netlist = parse(
        "* nmos\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nm W=1u L=1u\nM2 d g 0 0 nm W=2u L=1u\n\
         .model nm nmos level=1 vto=0.5 kp=1e-4\n.op\n.tran 1n 10n\n\
         .ASSERT sat MOD=nm PARAM=vds MAX=0.5\n\
         .ASSERT one DEV=m2 PARAM=vds MIN=0.9 MAX=1.1\n.end\n",
    );
    let (op, devices) = Engine::new(SimulationConfig::default())
        .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
        .expect("operating point");
    let reports = evaluate_op_assertions(&netlist, &op, &devices);
    let sat = reports
        .iter()
        .filter(|report| report.name == "sat")
        .collect::<Vec<_>>();
    assert_eq!(
        sat.iter()
            .map(|report| report
                .instance
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase())
            .collect::<Vec<_>>(),
        ["m1", "m2"],
        "elaboration order: {reports:?}"
    );
    for report in &sat {
        assert!(report.error.is_none(), "{report:?}");
        assert_eq!(
            report.violations,
            [AssertViolation {
                start: None,
                end: None,
                extreme: Some(1.0),
                at: None,
            }]
        );
    }
    assert!(report(&reports, "one").violations.is_empty());

    let reports = tran_reports(&netlist);
    for report in reports.iter().filter(|report| report.name == "sat") {
        assert!(report.error.is_none(), "{report:?}");
        assert_eq!(stretches(report), [(0.0, None)], "{report:?}");
    }
}

/// Each solved point of a sweep is checked, and consecutive failing points
/// report the range of the swept value they span.
#[test]
fn a_dc_sweep_reports_the_range_its_failing_points_span() {
    let netlist = parse(
        "* dc\nV1 in 0 0\nR1 in 0 1k\n.dc V1 0 1 0.25\n\
         .ASSERT lim EXPR='V(in)' MAX=0.3\n.end\n",
    );
    let sweep = Engine::new(SimulationConfig::default())
        .run_dc_sweep(&netlist, "v1", 0.0, 1.0, 0.25)
        .expect("the sweep runs");
    let reports = evaluate_dc_assertions(&netlist, &sweep);
    let [violation] = report(&reports, "lim").violations.as_slice() else {
        panic!("one range: {reports:?}");
    };
    assert_eq!(violation.start, Some(0.5));
    assert_eq!(violation.end, Some(1.0));
    assert_eq!(violation.extreme, Some(1.0));
    assert_eq!(violation.at, Some(1.0));
}

/// A card names the analyses it applies to; with none it applies to every
/// analysis that produces its operands, and an operand an analysis does not
/// produce is refused naming it.
#[test]
fn an_assertion_applies_to_the_analyses_that_produce_its_operands() {
    let netlist = parse(
        "* scope\nV1 in 0 1\nR1 in 0 1k\n.op\n\
         .ASSERT any EXPR='V(in)' MAX=2\n\
         .ASSERT tran_only EXPR='V(in)' MAX=2 ANALYSES=tran\n\
         .ASSERT events COND='D(q)==1'\n\
         .ASSERT forced COND='D(q)==1' ANALYSES=op\n\
         .ASSERT missing EXPR='V(nowhere)' MAX=1\n.end\n",
    );
    let (op, devices) = Engine::new(SimulationConfig::default())
        .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
        .expect("operating point");
    let reports = evaluate_op_assertions(&netlist, &op, &devices);
    let names = reports
        .iter()
        .map(|report| report.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["any", "forced", "missing"]);
    assert!(reports[0].violations.is_empty() && reports[0].error.is_none());
    let forced = reports[1].error.as_deref().expect("refused");
    assert!(
        forced.contains("D(Q) is an event operand") && forced.contains("op"),
        "{forced}"
    );
    let missing = reports[2].error.as_deref().expect("refused");
    assert!(missing.contains("V(NOWHERE) names no signal"), "{missing}");
}

/// The three refusals, each in its own sentence.
#[test]
fn level_fatal_modelparam_and_an_unknown_param_are_refused() {
    let deck = |card: &str| {
        Netlist::parse(&format!(
            "* refusals\nV1 in 0 1\nR1 in 0 1k\n.op\n{card}\n.end\n"
        ))
    };
    let Err(ParseError::UnsupportedCapability { capability, .. }) =
        deck(".ASSERT a EXPR='V(in)' MAX=1 LEVEL=FATAL")
    else {
        panic!("LEVEL=FATAL is a capability refusal");
    };
    assert_eq!(capability, "netlist.assert.level_fatal");
    let Err(ParseError::UnsupportedCapability { capability, .. }) =
        deck(".ASSERT a MODELPARAM=vth0 MAX=1")
    else {
        panic!("MODELPARAM= is a capability refusal");
    };
    assert_eq!(capability, "netlist.assert.modelparam");
    let error = deck(".ASSERT mpsat DEV=mp PARAM=nosuch MIN=-0.6")
        .expect_err("an unknown PARAM is refused")
        .to_string();
    assert!(
        error.contains("PARAM=nosuch is not an operating-point quantity"),
        "{error}"
    );
}

#[cfg(feature = "veriloga")]
mod hdl {
    //! The event fixtures of the `.MEASURE` event-operand suite: `q` toggles
    //! every 10 ns from 0 and `p` every 7 ns from 0; `r` is X until 3 ns, 0 to
    //! 5 ns, X to 8 ns and 1 after.

    use super::*;
    use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQUENCE: AtomicU64 = AtomicU64::new(0);

    /// A scratch directory removed on drop.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(prefix: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "rspice_assertions_{prefix}_{}_{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&dir).expect("create scratch directory");
            Self(dir)
        }

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

    const CLOCKS: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module clocks(q, p);\n\
  output q; reg q;\n\
  output p; reg p;\n\
  initial begin q = 1'b0; p = 1'b0; end\n\
  always #10 q = ~q;\n\
  always #7 p = ~p;\n\
endmodule\n";

    const UNDEFINED: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module undefined(r);\n\
  output r; reg r;\n\
  initial begin r = 1'bx; #3 r = 1'b0; #2 r = 1'bx; #3 r = 1'b1; end\n\
endmodule\n";

    /// Connect rules for a 1 V supply: a D/A that ramps linearly over 1 ns.
    const RULES: &str = "connectrules deck;\n\
    connect a2d #(.vsup(1), .vthi(0.5), .vtlo(0.5));\n\
    connect d2a #(.vsup(1), .trise(1n), .tfall(1n));\n\
endconnectrules\n";

    /// `p` is 1 on [7, 14) and [21, 28) ns, so `D(p)==0` fails there.
    #[test]
    fn a_digital_condition_fails_on_exact_event_intervals() {
        let scratch = Scratch::new("digital");
        let module = scratch.write("clocks.vams", CLOCKS);
        let netlist = parse(&format!(
            "* clocks\n.hdl \"{module}\"\nx1 q p clocks\nr1 q 0 1meg\n.tran 0.1n 100n\n\
             .ASSERT low COND='D(p)==0' TO=28n\n.end\n"
        ));
        let reports = tran_reports(&netlist);
        let stretches = stretches(report(&reports, "low"));
        let [(s0, Some(e0)), (s1, Some(e1))] = stretches[..] else {
            panic!("two closed violations: {stretches:?}");
        };
        assert_instant(s0, 7.0e-9, "p rises at 7 ns");
        assert_instant(e0, 14.0e-9, "p falls at 14 ns");
        assert_instant(s1, 21.0e-9, "p rises at 21 ns");
        assert_instant(e1, 28.0e-9, "the window ends at TO=28n");
    }

    /// X fails a check unless `XZ=IGNORE` counts it instead.
    #[test]
    fn an_undefined_stretch_fails_unless_it_is_ignored_and_counted() {
        let scratch = Scratch::new("undefined");
        let module = scratch.write("undefined.vams", UNDEFINED);
        let netlist = parse(&format!(
            "* undefined\n.hdl \"{module}\"\nx1 r undefined\n.tran 0.1n 10n\n\
             .ASSERT known COND='!isunknown(D(r))'\n\
             .ASSERT one_ignoring COND='D(r)==1' XZ=IGNORE TO=10n\n\
             .ASSERT one COND='D(r)==1' TO=10n\n.end\n"
        ));
        let reports = tran_reports(&netlist);

        let known = stretches(report(&reports, "known"));
        let [(s0, Some(e0)), (s1, Some(e1))] = known[..] else {
            panic!("two X stretches: {known:?}");
        };
        assert_eq!(s0, 0.0);
        assert_instant(e0, 3.0e-9, "r leaves X at 3 ns");
        assert_instant(s1, 5.0e-9, "r is X again at 5 ns");
        assert_instant(e1, 8.0e-9, "r is 1 at 8 ns");

        let ignoring = report(&reports, "one_ignoring");
        let [(start, Some(end))] = stretches(ignoring)[..] else {
            panic!("one violation: {ignoring:?}");
        };
        assert_instant(start, 3.0e-9, "r is 0 from 3 ns");
        assert_instant(end, 5.0e-9, "to 5 ns");
        assert_eq!(ignoring.unknown_stretches, 2);

        let one = report(&reports, "one");
        let [(start, Some(end))] = stretches(one)[..] else {
            panic!("one violation: {one:?}");
        };
        assert_eq!(start, 0.0);
        assert_instant(end, 8.0e-9, "X, 0 and X again, then 1 at 8 ns");
        assert_eq!(one.unknown_stretches, 0);
    }

    /// `D(p)==0 || V(q)<0.5` fails where `p` is 1 and the D/A has carried
    /// `q` to at least 0.5 V. `q` rises at 10 and 30 ns and falls at 20 and
    /// 40 ns; its D/A ramps over 1 ns through 20 Ω into 1 MΩ, so V(q) is
    /// at least 0.5 V from 0.5·(1 + 20/10⁶) ns after each rise to the same
    /// before 1 ns after each fall. With `p` 1 on [7, 14) and [35, 42) ns:
    /// `[10.50001n, 14n)` and `[35n, 40.49999n)`.
    #[test]
    fn a_mixed_condition_is_the_union_of_event_edges_and_ramp_crossings() {
        let scratch = Scratch::new("mixed");
        let mut source = CLOCKS.to_string();
        for (_, connect) in BUILTIN_CONNECT_MODULES {
            source.push_str(connect);
        }
        source.push_str(RULES);
        let module = scratch.write("clocks.vams", &source);
        let netlist = parse(&format!(
            "* mixed\n.hdl \"{module}\" module=clocks\n.options connectrules=deck\n\
             x1 q p clocks\nr1 q 0 1meg\n.tran 0.01n 45n\n\
             .ASSERT either COND='D(p)==0 || V(q)<0.5'\n.end\n"
        ));
        let reports = tran_reports(&netlist);
        let stretches = stretches(report(&reports, "either"));
        let [(s0, Some(e0)), (s1, Some(e1))] = stretches[..] else {
            panic!("two violations: {stretches:?}");
        };
        let crossing = 0.5e-9 * (1.0 + 20.0 / 1.0e6);
        assert!((s0 - (10.0e-9 + crossing)).abs() <= 1e-15, "{s0:e}");
        assert_instant(e0, 14.0e-9, "p falls at 14 ns");
        assert_instant(s1, 35.0e-9, "p rises at 35 ns");
        assert!((e1 - (41.0e-9 - crossing)).abs() <= 1e-15, "{e1:e}");
    }
}

/// HSPICE `.VEC` vector files: inputs drive ideal piecewise-linear sources,
/// expected outputs are sampled checks in the assertion report. The files are
/// registered as data files, as a prepared run registers what it sealed, under
/// paths no other test uses.
mod vector_files {
    use super::*;
    use rspice_core::netlist::AssertLevel;

    fn register(path: &str, contents: &str) {
        rspice_core::xspice::register_data_file(path, contents)
            .unwrap_or_else(|error| panic!("register {path}: {error}"));
    }

    fn build_error(deck: &str) -> String {
        let netlist = parse(deck);
        match Engine::new(SimulationConfig::default()).run_tran(&netlist, 20e-9, 1e-9) {
            Ok(_) => panic!("the build refuses the vector file:\n{deck}"),
            Err(error) => error.to_string(),
        }
    }

    /// VIH 1.8 V, VIL 0, TRISE 1 ns, a 0 -> 1 row at 10 ns into 1 MΩ: the
    /// source ramps from 10 ns to 11 ns, so V(a) crosses 0.9 V at 10.5 ns,
    /// read off the accepted points around it.
    #[test]
    fn an_input_ramps_through_half_its_swing_half_a_rise_time_after_its_row() {
        register(
            "assertions-vec/drive.vec",
            "radix 1\nvname a\nio i\ntunit ns\nvih 1.8\nvil 0\ntrise 1\n0 0\n10 1\n",
        );
        let netlist = parse(
            "* vec drive\nr1 a 0 1meg\n.vec 'assertions-vec/drive.vec'\n.tran 0.1n 20n\n.end\n",
        );
        let result = run(&netlist);
        let v = result
            .try_voltage_waveform_named("a")
            .expect("V(a) is recorded");
        let row = v
            .iter()
            .position(|value| *value >= 0.9)
            .expect("V(a) reaches 0.9 V");
        let (t0, t1) = (result.time[row - 1], result.time[row]);
        let (v0, v1) = (v[row - 1], v[row]);
        let crossing = t0 + (0.9 - v0) * (t1 - t0) / (v1 - v0);
        // The accepted points lie on the source's line, so the crossing is
        // exact to the rounding of the interpolation: a few ulp.
        let ulp = f64::EPSILON * 10.5e-9;
        assert!(
            (crossing - 10.5e-9).abs() <= 8.0 * ulp,
            "V(a) crosses 0.9 V at {crossing:e}, between {t0:e} and {t1:e}"
        );
        assert_eq!(v[0], 0.0);
        assert_eq!(*v.last().unwrap(), 1.8);
        // An input file has no output column, so nothing is reported.
        let reports = evaluate_tran_assertions(&netlist, &result, &NoAbort).unwrap();
        assert!(reports.is_empty(), "{reports:?}");
    }

    #[test]
    fn the_card_sets_the_level_and_an_unreadable_file_is_reported() {
        register(
            "assertions-vec/level.vec",
            "radix 1\nio o\nvname a\nvoh 1\nvol 0.5\n0 1\n",
        );
        let netlist = parse(
            "* vec level\nva a 0 0\n.vec 'assertions-vec/level.vec' LEVEL=warning\n\
             .tran 1n 10n\n.end\n",
        );
        let reports = tran_reports(&netlist);
        let [report] = reports.as_slice() else {
            panic!("{reports:?}")
        };
        assert_eq!(report.level, AssertLevel::Warning);
        assert_eq!(report.violations.len(), 1, "V(a) = 0 is not >= VOH");
        let error = build_error(
            "* vec missing\nr1 a 0 1k\n.vec 'assertions-vec/no-such-file.vec'\n.tran 1n 10n\n.end\n",
        );
        assert!(
            error.contains("the vector file 'assertions-vec/no-such-file.vec' cannot be read"),
            "{error}"
        );
    }

    /// Each refusal names the file and line, as the build reports it.
    #[test]
    fn a_vector_file_is_refused_naming_its_file_and_line() {
        for (name, text, needle) in [
            (
                "bidirectional",
                "radix 1 4\nio i b\nvname en x[3:0]\nenable en 0 F\n0 0 0\n",
                "bidirectional.vec:2: column 2 (x3) is bidirectional (IO b)",
            ),
            (
                "input-z",
                "radix 1 1\nio i o\nvname a b\n0 0 0\n10 Z 1\n",
                "input-z.vec:5: input a (column 1) is Z",
            ),
            (
                "keyword",
                "radix 1\nvname a\nhlz 1\n0 0\n",
                "keyword.vec:3: hlz is not a vector file keyword",
            ),
            (
                "digit",
                "radix 2\nvname a[1:0]\n0 4\n",
                "digit.vec:3: '4' in column 1 is not a digit of its 2-bit",
            ),
            (
                "columns",
                "radix 1 1\nvname a b\n0 0 0\n10 1\n",
                "columns.vec:4: the row gives 1 value(s), but RADIX declares 2",
            ),
        ] {
            let path = format!("assertions-vec/{name}.vec");
            register(&path, text);
            let error = build_error(&format!(
                "* vec {name}\nr1 a 0 1k\nr2 b 0 1k\n.vec '{path}'\n.tran 1n 10n\n.end\n"
            ));
            assert!(error.contains(needle), "{name}: {error}");
            assert!(error.contains("the .VEC card at line 4"), "{name}: {error}");
        }
    }
}
