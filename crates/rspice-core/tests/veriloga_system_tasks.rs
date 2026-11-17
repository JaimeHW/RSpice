//! Verilog-AMS system tasks and functions in the analog context (LRM 2.4
//! sections 9.4 to 9.13), run by the engine.

#![cfg(feature = "veriloga")]

#[path = "common/model_outcome.rs"]
mod model_outcome;

use model_outcome::RunWithOutcome;
use rspice_core::config::{HdlOutputCollector, HdlOutputRecord, HdlStream};
use rspice_core::{Engine, Netlist, NoAbort, SimulationOutcome};
use std::path::PathBuf;
use std::sync::Arc;

/// A scratch directory of its own for each test.
fn scratch(name: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "rspice_va_system_tasks_{name}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// Run a transient of `body` in a one-port module across a ramp source, in
/// `directory`, and return the outcome and every HDL record.
fn run(
    directory: &std::path::Path,
    body: &str,
    stop: f64,
) -> (
    Result<SimulationOutcome<rspice_core::engine::TransientResult>, String>,
    Vec<HdlOutputRecord>,
) {
    run_with(directory, body, stop, |_| {})
}

/// [`run`], with the run's HDL options set by `hdl`.
fn run_with(
    directory: &std::path::Path,
    body: &str,
    stop: f64,
    hdl: impl FnOnce(&mut rspice_core::config::HdlRunConfig),
) -> (
    Result<SimulationOutcome<rspice_core::engine::TransientResult>, String>,
    Vec<HdlOutputRecord>,
) {
    let model = directory.join("tasks.va");
    std::fs::write(
        &model,
        format!(
            "`include \"disciplines.vams\"\nmodule tasks(x); inout x; electrical x;\n\
             integer fd, rc, k, seed, n, a, b, c, d, e, f, g, h, i, j; real r;\nanalog begin\n{body}\nI(x) <+ V(x) / 1k;\nend\nendmodule\n"
        ),
    )
    .unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        format!(
            "* system tasks\nvx s 0 pwl(0 0 {stop} 1)\nr1 s x 1k\nxa x tasks\n.hdl \"tasks.va\"\n.end\n"
        ),
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.to_path_buf());
    hdl(&mut config.hdl);
    let outcome = Engine::new(config)
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, stop, stop / 100.0, signal)
        })
        .map_err(|error| error.to_string());
    (outcome, collector.take())
}

fn stdout(records: &[HdlOutputRecord]) -> Vec<String> {
    records
        .iter()
        .filter(|record| record.stream == HdlStream::Stdout)
        .map(|record| record.text.clone())
        .collect()
}

/// `$debug` prints, `$fdisplay` to the standard output prints there, and
/// `$fwrite` to the standard error lands there.
#[test]
fn debug_and_standard_stream_file_output_print_from_accepted_points() {
    let directory = scratch("streams");
    let (outcome, records) = run(
        &directory,
        "@(initial_step) begin $debug(\"debug\"); $fdisplay(32'h8000_0001, \"out %0d\", 7); \
         $fwrite(32'h8000_0002, \"err\"); end",
        1e-9,
    );
    outcome.unwrap();
    let printed = stdout(&records);
    assert!(printed.iter().any(|text| text == "debug\n"), "{printed:?}");
    assert!(printed.iter().any(|text| text == "out 7\n"), "{printed:?}");
    assert!(
        records
            .iter()
            .any(|record| record.stream == HdlStream::Stderr && record.text == "err"),
        "{records:?}"
    );
}

/// `$fatal` ends the run at the accepted point with its formatted message
/// and source line; `$stop` ends it as a stop (LRM 2.4 section 9.7).
#[test]
fn fatal_and_stop_end_the_run_with_their_source_line() {
    use rspice_core::ModelFinishTask;
    let directory = scratch("control");
    for (body, task, message) in [
        (
            "@(cross(V(x) - 0.25, +1)) $fatal(1, \"crossed at %g\", 0.25);",
            ModelFinishTask::Fatal,
            Some("crossed at 0.25"),
        ),
        (
            "@(cross(V(x) - 0.25, +1)) $stop;",
            ModelFinishTask::Stop,
            None,
        ),
    ] {
        let (outcome, records) = run(&directory, body, 10e-9);
        let SimulationOutcome::Finished { finish, .. } = outcome.unwrap() else {
            panic!("{body}: the run must end at the call");
        };
        assert_eq!(finish.task, task, "{body}");
        assert_eq!(finish.message.as_deref(), message, "{body}");
        let source = finish.source.expect("the call's source");
        assert!(source.file.ends_with("tasks.va"), "{source:?}");
        assert_eq!(source.line, 5);
        assert!(
            records
                .iter()
                .any(|record| matches!(record.stream, HdlStream::Finish { .. })),
            "{records:?}"
        );
    }
}

/// `$random` and `$dist_*` in analog code draw IEEE 1364-2005 section
/// 17.9.3's sequence: the same values Icarus Verilog 13.0 prints for
///
/// ```verilog
/// seed = 42; a = $random(seed); ... j = $dist_erlang(seed, 2, 5);
/// s = 3; x = $random(s);
/// ```
///
/// and a `$random` without a seed starts from the run's `hdlseed`, as the
/// digital kernel's does.
#[test]
fn random_and_integer_distributions_draw_what_icarus_draws() {
    let directory = scratch("random");
    let (outcome, records) = run_with(
        &directory,
        "@(initial_step) begin seed = 42;\n\
         a = $random(seed); b = $random(seed); c = $random(seed);\n\
         d = $dist_uniform(seed, -10, 10); e = $dist_normal(seed, 5, 3);\n\
         f = $dist_exponential(seed, 4); g = $dist_poisson(seed, 6);\n\
         h = $dist_chi_square(seed, 3); i = $dist_t(seed, 4); j = $dist_erlang(seed, 2, 5);\n\
         $display(\"%0d %0d %0d %0d %0d %0d %0d %0d %0d %0d seed=%0d\", a, b, c, d, e, f, g, h, i, j, seed);\n\
         n = $random; $display(\"hdlseed %0d\", n); end",
        1e-9,
        |hdl| hdl.seed = 3,
    );
    outcome.unwrap();
    let printed = stdout(&records);
    assert_eq!(
        printed,
        [
            "-2144582656 646214477 38602500 -5 7 1 4 7 -3 6 seed=2039465103\n",
            "hdlseed -2147276288\n",
        ],
        "{records:?}"
    );
}

/// `$fopen`, `$fscanf`, `$test$plusargs` and `$value$plusargs` read the run's
/// files and plusargs, `$fdisplay` and `$fstrobe` write through the same
/// table, and `$fstrobe` writes once per accepted point. A path outside the
/// run directory opens as 0 with a warning, as the digital kernel's does.
#[test]
fn files_and_plusargs_are_the_runs() {
    let directory = scratch("files");
    std::fs::write(directory.join("in.txt"), "3 2.5\n").unwrap();
    let (outcome, records) = run_with(
        &directory,
        "@(initial_step) begin fd = $fopen(\"out.txt\", \"w\");\n\
         k = $fopen(\"in.txt\", \"r\"); rc = $fscanf(k, \"%d %g\", n, r);\n\
         a = $test$plusargs(\"fast\"); b = $value$plusargs(\"gain=%d\", c);\n\
         e = $fopen(\"../escape.txt\", \"w\");\n\
         $fdisplay(fd, \"n=%0d r=%g rc=%0d fast=%0d gain=%0d/%0d escape=%0d\", n, r, rc, a, b, c, e);\n\
         $fclose(k); end\n\
         $fstrobe(fd, \"t\");",
        10e-9,
        |hdl| hdl.plusargs = vec!["+gain=7".into(), "+fast".into()],
    );
    let SimulationOutcome::Completed(result) = outcome.unwrap() else {
        panic!("the run completes");
    };
    let written = std::fs::read_to_string(directory.join("out.txt")).unwrap();
    // `$fdisplay` writes once, at the accepted first point; `$fstrobe` once
    // per accepted point.
    let (displayed, strobed): (Vec<&str>, Vec<&str>) =
        written.lines().partition(|line| *line != "t");
    assert_eq!(
        displayed,
        ["n=3 r=2.5 rc=2 fast=1 gain=1/7 escape=0"],
        "{written}"
    );
    assert!(written.starts_with(displayed[0]), "{written}");
    assert_eq!(strobed.len(), result.time.len(), "{written}");
    assert!(
        records.iter().any(
            |record| matches!(record.stream, HdlStream::Diagnostic { .. })
                && record.text.contains("$fopen(\"../escape.txt\")")
        ),
        "{records:?}"
    );
    assert!(!directory.join("../escape.txt").exists());
}

/// `$fdisplay` writes as `$display` does, once per accepted point: an
/// `@(initial_step)` call writes one line though the first point takes
/// several Newton evaluations, which the `$debug` beside it counts. The
/// module reads its file back and drives the number of lines it holds, which
/// the browser worker's twin of this test reads.
#[test]
fn an_initial_step_file_display_writes_once_however_many_evaluations_the_point_takes() {
    let directory = scratch("once");
    std::fs::write(directory.join("once.va"), ONCE_PER_POINT).unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* one write per accepted point\n.hdl \"once.va\"\nxa q once\nr1 q 0 1k\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.clone());
    let outcome = Engine::new(config)
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, 1e-9, 1e-11, signal)
        })
        .unwrap();
    let SimulationOutcome::Completed(result) = outcome else {
        panic!("the run completes");
    };
    let records = collector.take();
    let evaluations = stdout(&records)
        .iter()
        .filter(|text| text.starts_with("evaluation"))
        .count();
    assert!(
        evaluations >= 2,
        "the first point takes several evaluations: {records:?}"
    );
    let written = std::fs::read_to_string(directory.join("once.txt")).unwrap();
    assert_eq!(written, "7\n");
    let q = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("q"))
        .unwrap();
    let lines = *result.voltages[q].last().unwrap();
    assert!((lines - 1.0).abs() < 1e-9, "V(q) = {lines}");
}

/// A current source into a diode across the deck's 1k resistor, a first
/// point Newton solves in several evaluations, until the module reads back
/// how many lines its `@(initial_step)` wrote and drives that count.
const ONCE_PER_POINT: &str = "`include \"disciplines.vams\"\nmodule once(q); inout q; electrical q;\n\
     integer fd, k, n, a, b, c;\nanalog begin\n\
     @(initial_step) begin fd = $fopen(\"once.txt\", \"w\"); $fdisplay(fd, \"%0d\", 7);\n\
     $debug(\"evaluation at %g\", $abstime); $fclose(fd); end\n\
     @(timer(0.5n)) begin k = $fopen(\"once.txt\", \"r\"); n = $fscanf(k, \"%d %d %d\", a, b, c);\n\
     $fclose(k); end\n\
     if (n > 0) V(q) <+ n;\n\
     else I(q) <+ 1e-14 * (limexp(V(q) / 0.02585) - 1) - 5m;\nend\nendmodule\n";
